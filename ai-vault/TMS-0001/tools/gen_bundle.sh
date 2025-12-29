#!/usr/bin/env bash
set -euo pipefail

# ------------------------------------------------------------------------------
# gen_bundle.sh
# - Updates AUTO blocks in context_bundle.md:
#   * requirements_snapshot (Section 6)
#   * contract_snapshot (Section 7)
#   * tasking_snapshot (Section 8) - NEW
#   * preflight_checks (Section 9) - NEW
# - Extracts data from:
#   * requirements.md (index table + Given/When/Then per REQ)
#   * openapi.yaml (operationId + x-requirements)
#   * asyncapi.yaml (messageId + x-requirements + channel mapping)
#   * tasks.md (TASK-IDs + Status + mappings) - NEW
#
# Usage:
#   tools/gen_bundle.sh \
#     --bundle 90_review/context_bundle.md \
#     --requirements 10_prd/requirements.md \
#     --openapi 30_contract/openapi.yaml \
#     --asyncapi 30_contract/asyncapi.yaml \
#     --tasks 40_design/tasks.md \
#     --max-req 10 \
#     [--phase Contract]
#
# Optional:
#   --req-ids REQ-0001,REQ-0007   # override auto selection
# ------------------------------------------------------------------------------

BUNDLE="90_review/context_bundle.md"
REQUIREMENTS="10_prd/requirements.md"
OPENAPI="30_contract/openapi.yaml"
ASYNCAPI="30_contract/asyncapi.yaml"
TASKS="40_design/tasks.md"
DESIGN="40_design/design.md"
MAX_REQ=10
PHASE=""
REQ_IDS_OVERRIDE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --bundle) BUNDLE="$2"; shift 2;;
    --requirements) REQUIREMENTS="$2"; shift 2;;
    --openapi) OPENAPI="$2"; shift 2;;
    --asyncapi) ASYNCAPI="$2"; shift 2;;
    --tasks) TASKS="$2"; shift 2;;
    --design) DESIGN="$2"; shift 2;;
    --max-req) MAX_REQ="$2"; shift 2;;
    --phase) PHASE="$2"; shift 2;;
    --req-ids) REQ_IDS_OVERRIDE="$2"; shift 2;;
    -h|--help)
      grep '^# ' "$0" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *)
      echo "Unknown arg: $1" >&2
      exit 1
      ;;
  esac
done

if [[ ! -f "$BUNDLE" ]]; then echo "Bundle not found: $BUNDLE" >&2; exit 1; fi
if [[ ! -f "$REQUIREMENTS" ]]; then echo "Requirements not found: $REQUIREMENTS" >&2; exit 1; fi
if [[ ! -f "$OPENAPI" ]]; then echo "OpenAPI not found: $OPENAPI" >&2; exit 1; fi
if [[ ! -f "$ASYNCAPI" ]]; then echo "AsyncAPI not found: $ASYNCAPI" >&2; exit 1; fi

# Backup
cp "$BUNDLE" "${BUNDLE}.bak.$(date +%Y%m%d%H%M%S)"

# Update timestamp in header (Updated: ...)
# Assumes line like: > Updated: <...>
TZ=Asia/Tokyo
NOW="$(TZ=Asia/Tokyo date '+%Y-%m-%d %H:%M (JST)')"
# macOS sed needs -i '' for in-place
sed -i '' -E "s/^(> Updated: ).*$/\1${NOW}/" "$BUNDLE" || true

# Optionally update Phase
if [[ -n "$PHASE" ]]; then
  sed -i '' -E "s/^(> Phase: ).*$/\1${PHASE}/" "$BUNDLE" || true
fi

export BUNDLE REQUIREMENTS OPENAPI ASYNCAPI MAX_REQ REQ_IDS_OVERRIDE TASKS DESIGN

python3 - <<'PY'
import os, re, sys, json
from pathlib import Path

bundle = Path(os.environ.get("BUNDLE","90_review/context_bundle.md"))
requirements = Path(os.environ.get("REQUIREMENTS","10_prd/requirements.md"))
openapi = Path(os.environ.get("OPENAPI","30_contract/openapi.yaml"))
asyncapi = Path(os.environ.get("ASYNCAPI","30_contract/asyncapi.yaml"))
tasks_path = Path(os.environ.get("TASKS","40_design/tasks.md"))
design_path = Path(os.environ.get("DESIGN","40_design/design.md"))
max_req = int(os.environ.get("MAX_REQ","10"))
req_ids_override = os.environ.get("REQ_IDS_OVERRIDE","").strip()

# Load PyYAML
try:
    import yaml  # type: ignore
except Exception as e:
    print("ERROR: PyYAML is required for parsing openapi/asyncapi YAML.", file=sys.stderr)
    print("Install: python3 -m pip install pyyaml", file=sys.stderr)
    sys.exit(2)

def read_text(p: Path) -> str:
    return p.read_text(encoding="utf-8")

def write_text(p: Path, s: str) -> None:
    p.write_text(s, encoding="utf-8")

def replace_auto_block(doc: str, block_name: str, new_content: str) -> str:
    """
    Replace content between:
      <!-- AUTO:BEGIN <block_name> -->
      ...
      <!-- AUTO:END <block_name> -->
    """
    begin = f"<!-- AUTO:BEGIN {block_name} -->"
    end = f"<!-- AUTO:END {block_name} -->"
    pattern = re.compile(re.escape(begin) + r".*?" + re.escape(end), re.DOTALL)
    if not pattern.search(doc):
        raise RuntimeError(f"AUTO block not found: {block_name}")
    replacement = begin + "\n" + new_content.rstrip() + "\n" + end
    return pattern.sub(replacement, doc, count=1)

# -------------------------
# requirements.md parsing
# -------------------------
req_md = read_text(requirements)

# Parse Requirement Index table:
# | REQ-ID | Title | Priority | Status | Area | Depends on |
index_rows = []
in_index = False
for line in req_md.splitlines():
    if line.strip().startswith("| REQ-ID") and "Priority" in line and "Status" in line:
        in_index = True
        continue
    if in_index:
        if not line.strip().startswith("|"):
            # end of table
            if index_rows:
                break
            continue
        # skip separator
        if re.match(r"^\|\s*-+\s*\|", line.strip()):
            continue
        cols = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cols) < 4:
            continue
        req_id, title, priority, status = cols[0], cols[1], cols[2], cols[3]
        area = cols[4] if len(cols) > 4 else ""
        depends = cols[5] if len(cols) > 5 else ""
        if req_id.startswith("REQ-"):
            index_rows.append({
                "req": req_id,
                "title": title,
                "priority": priority,
                "status": status,
                "area": area,
                "depends": depends,
            })

# Auto select Focus REQs:
# Default rule (you can tune later):
# - include statuses not in {Done, Deprecated}
# - sort MUST first, then SHOULD, then COULD
status_exclude = {"Done", "Deprecated"}
priority_rank = {"MUST": 0, "SHOULD": 1, "COULD": 2}

selected_req_ids = []
if req_ids_override:
    selected_req_ids = [x.strip() for x in req_ids_override.split(",") if x.strip()]
else:
    filtered = [r for r in index_rows if r["status"] not in status_exclude]
    filtered.sort(key=lambda r: (priority_rank.get(r["priority"], 9), r["req"]))
    selected_req_ids = [r["req"] for r in filtered[:max_req]]

# Parse per-REQ acceptance (Given/When/Then):
# Expect block:
# ### REQ-0001: Title
# ...
# - **Acceptance (the only one)**:
#   - **Given**: ...
#   - **When**: ...
#   - **Then**: ...
req_blocks = {}
# split by headings
parts = re.split(r"^###\s+(REQ-\d+):\s*(.+?)\s*$", req_md, flags=re.MULTILINE)
# parts: [pre, reqid1, title1, body1, reqid2, title2, body2, ...]
for i in range(1, len(parts), 3):
    req_id = parts[i].strip()
    title = parts[i+1].strip()
    body = parts[i+2]
    def find_field(label: str) -> str:
        # matches "- **Given**: ..." possibly with spaces
        m = re.search(rf"\-\s+\*\*{re.escape(label)}\*\*:\s*(.+)", body)
        return (m.group(1).strip() if m else "")
    given = find_field("Given")
    when = find_field("When")
    then = find_field("Then")
    req_blocks[req_id] = {"title": title, "given": given, "when": when, "then": then}

# Build Requirements Snapshot markdown
def md_escape(s: str) -> str:
    return s.replace("\n", " ").strip()

focus_table_lines = [
    "### 6.1 Focus REQs (This phase)",
    "| REQ | Title | Priority | Status |",
    "|---|---|---|---|",
]
req_meta = {r["req"]: r for r in index_rows}
if selected_req_ids:
    for rid in selected_req_ids:
        meta = req_meta.get(rid, {"title": req_blocks.get(rid,{}).get("title",""), "priority":"", "status":""})
        focus_table_lines.append(f"| {rid} | {md_escape(meta.get('title',''))} | {meta.get('priority','')} | {meta.get('status','')} |")
else:
    focus_table_lines.append("")
    focus_table_lines.append("N/A")

accept_lines = ["", "### 6.2 Acceptance Details (Only for focus REQs)"]
if selected_req_ids:
    for rid in selected_req_ids:
        b = req_blocks.get(rid, {})
        accept_lines.append(f"- **{rid}**")
        accept_lines.append(f"  - Given: {md_escape(b.get('given','')) or 'TBD'}")
        accept_lines.append(f"  - When: {md_escape(b.get('when','')) or 'TBD'}")
        accept_lines.append(f"  - Then: {md_escape(b.get('then','')) or 'TBD'}")
else:
    accept_lines.append("N/A")

requirements_snapshot = "\n".join(focus_table_lines + accept_lines)

# -------------------------
# openapi.yaml parsing
# -------------------------
openapi_obj = yaml.safe_load(read_text(openapi))
http_ops = []
paths = openapi_obj.get("paths", {}) or {}
for path, item in paths.items():
    if not isinstance(item, dict):
        continue
    for method, op in item.items():
        if method.lower() not in {"get","post","put","patch","delete","options","head"}:
            continue
        if not isinstance(op, dict):
            continue
        op_id = op.get("operationId")
        xreq = op.get("x-requirements", []) or []
        if isinstance(xreq, str):
            xreq = [xreq]
        if op_id:
            http_ops.append({
                "operationId": op_id,
                "method": method.upper(),
                "path": path,
                "xreq": [str(x) for x in xreq],
            })
http_ops.sort(key=lambda x: x["operationId"])

# -------------------------
# asyncapi.yaml parsing (event notifications)
# -------------------------
async_obj = yaml.safe_load(read_text(asyncapi))
channels = async_obj.get("channels", {}) or {}
messages = (((async_obj.get("components") or {}).get("messages")) or {}) or {}

# messageName -> {messageId, xreq}
msg_info = {}
for name, msg in messages.items():
    if not isinstance(msg, dict):
        continue
    mid = msg.get("messageId") or ""
    xreq = msg.get("x-requirements", []) or []
    if isinstance(xreq, str):
        xreq = [xreq]
    if mid:
        msg_info[name] = {"messageId": str(mid), "xreq": [str(x) for x in xreq]}

# attempt to map message refs to channels
# channel -> subscribe -> message -> {$ref} or {oneOf:[{$ref},...]}
msg_to_channels = {}  # messageId -> set(channels)
def ref_to_name(ref: str) -> str:
    # "#/components/messages/MessageName"
    return ref.split("/")[-1].strip()

for ch_name, ch in channels.items():
    if not isinstance(ch, dict):
        continue
    sub = ch.get("subscribe")
    if not isinstance(sub, dict):
        continue
    msg = sub.get("message")
    if not isinstance(msg, dict):
        continue
    refs = []
    if "$ref" in msg:
        refs.append(msg["$ref"])
    if "oneOf" in msg and isinstance(msg["oneOf"], list):
        for it in msg["oneOf"]:
            if isinstance(it, dict) and "$ref" in it:
                refs.append(it["$ref"])
    for ref in refs:
        nm = ref_to_name(ref)
        info = msg_info.get(nm)
        if not info:
            continue
        mid = info["messageId"]
        msg_to_channels.setdefault(mid, set()).add(ch_name)

event_rows = []
# collect by messageId (not by channel)
for nm, info in msg_info.items():
    mid = info["messageId"]
    chs = sorted(msg_to_channels.get(mid, set()))
    channel = ", ".join(chs) if chs else "TBD"
    event_rows.append({"messageId": mid, "channel": channel, "xreq": info["xreq"]})
event_rows.sort(key=lambda x: x["messageId"])

# Build Contract Snapshot markdown
contract_lines = ["### 7.1 HTTP Operations (OpenAPI)",
                  "| operationId | Path/Method | x-requirements | Notes |",
                  "|---|---|---|---|"]
if http_ops:
    for op in http_ops:
        xreq = ", ".join(op["xreq"]) if op["xreq"] else "TBD"
        contract_lines.append(f"| {op['operationId']} | {op['method']} {op['path']} | {xreq} | - |")
else:
    contract_lines.append("")
    contract_lines.append("N/A")

contract_lines += ["", "### 7.2 Event Notifications (AsyncAPI)",
                   "| messageId | Channel | x-requirements | Notes |",
                   "|---|---|---|---|"]
if event_rows:
    for ev in event_rows:
        xreq = ", ".join(ev["xreq"]) if ev["xreq"] else "TBD"
        contract_lines.append(f"| {ev['messageId']} | {ev['channel']} | {xreq} | - |")
else:
    contract_lines.append("")
    contract_lines.append("N/A")

contract_snapshot = "\n".join(contract_lines)

# -------------------------
# tasks.md parsing (NEW)
# -------------------------
tasking_snapshot = ""
if tasks_path.exists():
    try:
        tasks_md = read_text(tasks_path)
        
        # Parse TASK Index table
        task_rows = []
        in_task_index = False
        for line in tasks_md.splitlines():
            if line.strip().startswith("| TASK-ID") and "Status" in line:
                in_task_index = True
                continue
            if in_task_index:
                if not line.strip().startswith("|"):
                    if task_rows:
                        break
                    continue
                if re.match(r"^\|\s*-+\s*\|", line.strip()):
                    continue
                cols = [c.strip() for c in line.strip().strip("|").split("|")]
                if len(cols) >= 7 and cols[0].startswith("TASK-"):
                    task_rows.append({
                        "task_id": cols[0],
                        "title": cols[1],
                        "status": cols[2],
                        "priority": cols[3],
                        "owner": cols[4],
                        "depends_on": cols[5],
                        "maps_to": cols[6]
                    })
        
        # Calculate statistics
        done_count = sum(1 for t in task_rows if t["status"] == "Done")
        processing_count = sum(1 for t in task_rows if t["status"] == "Processing")
        undone_count = sum(1 for t in task_rows if t["status"] == "UnDone")
        total_count = len(task_rows)
        progress = (done_count / total_count * 100) if total_count > 0 else 0
        
        # Current phase tasks (Processing or UnDone, limit to 5)
        active_tasks = [t for t in task_rows if t["status"] in {"UnDone", "Processing"}][:5]
        
        tasking_lines = [
            "### 8.1 Current Phase Tasks (from tasks.md)",
            "| TASK-ID | Title | Status | Priority | Maps to |",
            "|---|---|---|---|---|"
        ]
        if active_tasks:
            for task in active_tasks:
                maps_to = task["maps_to"] if task["maps_to"] != "-" else "N/A"
                tasking_lines.append(
                    f"| {task['task_id']} | {md_escape(task['title'])} | {task['status']} | "
                    f"{task['priority']} | {md_escape(maps_to)} |"
                )
        else:
            tasking_lines.append("")
            tasking_lines.append("N/A - No active tasks")
        
        tasking_lines += [
            "",
            "### 8.2 Task Progress",
            f"- Total Tasks: {total_count}",
            f"- Done: {done_count}",
            f"- Processing: {processing_count}",
            f"- UnDone: {undone_count}",
            f"- Progress: {progress:.0f}% ({done_count}/{total_count})"
        ]
        
        # Next actions (first 3 UnDone tasks)
        next_tasks = [t for t in task_rows if t["status"] == "UnDone"][:3]
        if next_tasks:
            tasking_lines += ["", "### 8.3 Next Actions"]
            for i, task in enumerate(next_tasks, 1):
                tasking_lines.append(f"{i}. Start {task['task_id']} ({md_escape(task['title'])})")
        else:
            tasking_lines += ["", "### 8.3 Next Actions", "All tasks completed or in progress"]
        
        tasking_snapshot = "\n".join(tasking_lines)
    except Exception as e:
        print(f"Warning: Failed to parse tasks.md: {e}", file=sys.stderr)
        tasking_snapshot = "### 8.1 Current Phase Tasks (from tasks.md)\nN/A - tasks.md parsing failed"
else:
    tasking_snapshot = "### 8.1 Current Phase Tasks (from tasks.md)\nN/A - tasks.md not found"

# -------------------------
# Pre-flight Checks (NEW)
# -------------------------
preflight_lines = ["### 9.1 Coverage Checks"]

# Check x-requirements coverage
ops_with_xreq = sum(1 for op in http_ops if op["xreq"])
ops_total = len(http_ops)
msgs_with_xreq = sum(1 for ev in event_rows if ev["xreq"])
msgs_total = len(event_rows)

check1 = ops_total == 0 or ops_with_xreq == ops_total
check2 = msgs_total == 0 or msgs_with_xreq == msgs_total
check3 = len(selected_req_ids) > 0  # Focus REQs exist

preflight_lines.append(
    f"- [{'x' if check1 else ' '}] All operationIds have x-requirements ({ops_with_xreq}/{ops_total})"
)
preflight_lines.append(
    f"- [{'x' if check2 else ' '}] All messageIds have x-requirements ({msgs_with_xreq}/{msgs_total})"
)
preflight_lines.append(
    f"- [{'x' if check3 else ' '}] Focus REQs defined ({len(selected_req_ids)} REQs)"
)

# Drift checks
preflight_lines += ["", "### 9.2 Drift Checks"]
unmapped_ops = [op for op in http_ops if not op["xreq"]]
unmapped_msgs = [ev for ev in event_rows if not ev["xreq"]]

if not unmapped_ops:
    preflight_lines.append("- [x] No unmapped operations")
else:
    preflight_lines.append(f"- [ ] ⚠️  {len(unmapped_ops)} unmapped operations found")
    for op in unmapped_ops[:3]:
        preflight_lines.append(f"  - {op['operationId']} ({op['method']} {op['path']})")
    if len(unmapped_ops) > 3:
        preflight_lines.append(f"  - ... and {len(unmapped_ops)-3} more")

if not unmapped_msgs:
    preflight_lines.append("- [x] No unmapped messages")
else:
    preflight_lines.append(f"- [ ] ⚠️  {len(unmapped_msgs)} unmapped messages found")
    for msg in unmapped_msgs[:3]:
        preflight_lines.append(f"  - {msg['messageId']}")
    if len(unmapped_msgs) > 3:
        preflight_lines.append(f"  - ... and {len(unmapped_msgs)-3} more")

# Check for undefined terms (basic check: look for TBD in x-requirements)
has_tbd_reqs = any("TBD" in str(op.get("xreq", [])) for op in http_ops)
has_tbd_msgs = any("TBD" in str(ev.get("xreq", [])) for ev in event_rows)
no_undefined = not (has_tbd_reqs or has_tbd_msgs)
preflight_lines.append(
    f"- [{'x' if no_undefined else ' '}] No undefined terms in requirements"
)

# Quality Metrics
preflight_lines += ["", "### 9.3 Quality Metrics"]
req_coverage = f"{len(selected_req_ids)} REQs" if selected_req_ids else "N/A"
api_coverage = f"{ops_with_xreq}/{ops_total}" if ops_total > 0 else "N/A"
event_coverage = f"{msgs_with_xreq}/{msgs_total}" if msgs_total > 0 else "N/A"

preflight_lines.append(f"- REQ Coverage: {req_coverage}")
preflight_lines.append(f"- API Coverage: {api_coverage}")
preflight_lines.append(f"- Event Coverage: {event_coverage}")

# Task completion (if tasks.md exists)
if tasks_path.exists() and 'total_count' in locals() and total_count > 0:
    task_completion = f"{progress:.0f}% ({done_count}/{total_count})"
    preflight_lines.append(f"- Task Completion: {task_completion}")
else:
    preflight_lines.append("- Task Completion: N/A")

# Overall status
preflight_lines += ["", "### 9.4 Overall Status"]
all_checks_pass = check1 and check2 and check3 and not unmapped_ops and not unmapped_msgs and no_undefined
status_icon = "✅" if all_checks_pass else "⚠️"
status_text = "Ready for next phase" if all_checks_pass else "Issues found - review needed"
preflight_lines.append(f"**{status_icon} {status_text}**")

if not all_checks_pass:
    preflight_lines += ["", "**Action Required:**"]
    if unmapped_ops:
        preflight_lines.append(f"- Add x-requirements to {len(unmapped_ops)} operation(s)")
    if unmapped_msgs:
        preflight_lines.append(f"- Add x-requirements to {len(unmapped_msgs)} message(s)")
    if not check3:
        preflight_lines.append("- Define Focus REQs in requirements.md")

preflight_snapshot = "\n".join(preflight_lines)

# -------------------------
# Apply to bundle
# -------------------------
doc = read_text(bundle)
doc = replace_auto_block(doc, "requirements_snapshot", requirements_snapshot)
doc = replace_auto_block(doc, "contract_snapshot", contract_snapshot)

# Apply new AUTO blocks (with error handling)
try:
    doc = replace_auto_block(doc, "tasking_snapshot", tasking_snapshot)
except RuntimeError as e:
    print(f"Warning: {e} - skipping tasking_snapshot update", file=sys.stderr)

try:
    doc = replace_auto_block(doc, "preflight_checks", preflight_snapshot)
except RuntimeError as e:
    print(f"Warning: {e} - skipping preflight_checks update", file=sys.stderr)

write_text(bundle, doc)
print("OK: updated AUTO blocks in", bundle)
PY
