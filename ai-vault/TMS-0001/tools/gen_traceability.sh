#!/usr/bin/env bash
set -euo pipefail

# ------------------------------------------------------------------------------
# gen_traceability.sh
# - Automatically generates traceability.md from:
#   * requirements.md (REQ-IDs)
#   * openapi.yaml (operationId + x-requirements)
#   * asyncapi.yaml (messageId + x-requirements)
#   * design.md (Components)
#   * tasks.md (TASK-IDs + mappings)
#
# Usage:
#   tools/gen_traceability.sh \
#     --traceability 90_review/traceability.md \
#     --requirements 10_prd/requirements.md \
#     --openapi 30_contract/openapi.yaml \
#     --asyncapi 30_contract/asyncapi.yaml \
#     --design 40_design/design.md \
#     --tasks 40_design/tasks.md
# ------------------------------------------------------------------------------

TRACEABILITY="90_review/traceability.md"
REQUIREMENTS="10_prd/requirements.md"
OPENAPI="30_contract/openapi.yaml"
ASYNCAPI="30_contract/asyncapi.yaml"
DESIGN="40_design/design.md"
TASKS="40_design/tasks.md"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --traceability) TRACEABILITY="$2"; shift 2;;
    --requirements) REQUIREMENTS="$2"; shift 2;;
    --openapi) OPENAPI="$2"; shift 2;;
    --asyncapi) ASYNCAPI="$2"; shift 2;;
    --design) DESIGN="$2"; shift 2;;
    --tasks) TASKS="$2"; shift 2;;
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

if [[ ! -f "$REQUIREMENTS" ]]; then echo "Requirements not found: $REQUIREMENTS" >&2; exit 1; fi
if [[ ! -f "$OPENAPI" ]]; then echo "OpenAPI not found: $OPENAPI" >&2; exit 1; fi
if [[ ! -f "$ASYNCAPI" ]]; then echo "AsyncAPI not found: $ASYNCAPI" >&2; exit 1; fi

# Backup if exists
if [[ -f "$TRACEABILITY" ]]; then
  cp "$TRACEABILITY" "${TRACEABILITY}.bak.$(date +%Y%m%d%H%M%S)"
fi

export TRACEABILITY REQUIREMENTS OPENAPI ASYNCAPI DESIGN TASKS

python3 - <<'PY'
import os, re, sys, json
from pathlib import Path
from datetime import datetime

traceability = Path(os.environ.get("TRACEABILITY","90_review/traceability.md"))
requirements = Path(os.environ.get("REQUIREMENTS","10_prd/requirements.md"))
openapi = Path(os.environ.get("OPENAPI","30_contract/openapi.yaml"))
asyncapi = Path(os.environ.get("ASYNCAPI","30_contract/asyncapi.yaml"))
design = Path(os.environ.get("DESIGN","40_design/design.md"))
tasks = Path(os.environ.get("TASKS","40_design/tasks.md"))

# Load PyYAML
try:
    import yaml  # type: ignore
except Exception as e:
    print("ERROR: PyYAML is required.", file=sys.stderr)
    print("Install: python3 -m pip install pyyaml", file=sys.stderr)
    sys.exit(2)

def read_text(p: Path) -> str:
    return p.read_text(encoding="utf-8") if p.exists() else ""

def write_text(p: Path, s: str) -> None:
    p.write_text(s, encoding="utf-8")

# Parse requirements.md - extract REQ-IDs
req_ids = []
req_md = read_text(requirements)
for line in req_md.splitlines():
    if line.strip().startswith("| REQ-"):
        cols = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cols) >= 4 and cols[0].startswith("REQ-"):
            req_ids.append(cols[0])

# Parse openapi.yaml - build REQ -> operationId mapping
openapi_obj = yaml.safe_load(read_text(openapi))
req_to_ops = {}  # REQ-ID -> [operationIds]
op_to_reqs = {}  # operationId -> [REQ-IDs]
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
            op_to_reqs[op_id] = [str(x) for x in xreq]
            for req in xreq:
                req_to_ops.setdefault(str(req), []).append(op_id)

# Parse asyncapi.yaml - build REQ -> messageId mapping
async_obj = yaml.safe_load(read_text(asyncapi))
messages = (((async_obj.get("components") or {}).get("messages")) or {}) or {}
req_to_msgs = {}  # REQ-ID -> [messageIds]
msg_to_reqs = {}  # messageId -> [REQ-IDs]
for name, msg in messages.items():
    if not isinstance(msg, dict):
        continue
    mid = msg.get("messageId") or ""
    xreq = msg.get("x-requirements", []) or []
    if isinstance(xreq, str):
        xreq = [xreq]
    if mid:
        msg_to_reqs[str(mid)] = [str(x) for x in xreq]
        for req in xreq:
            req_to_msgs.setdefault(str(req), []).append(str(mid))

# Parse tasks.md - build REQ -> TASK mapping
task_rows = []
req_to_tasks = {}  # REQ-ID -> [TASK-IDs]
tasks_md = read_text(tasks)
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
            task_id = cols[0]
            status = cols[2]
            maps_to = cols[6]
            task_rows.append({
                "task_id": task_id,
                "status": status,
                "maps_to": maps_to
            })
            # Extract REQ-IDs from maps_to
            for req_match in re.finditer(r"REQ-\d+", maps_to):
                req_id = req_match.group(0)
                req_to_tasks.setdefault(req_id, []).append(task_id)

# Parse design.md - extract Components (basic)
components = set()
design_md = read_text(design)
for match in re.finditer(r"Component[:\s]+([A-Z][A-Za-z0-9_]+)", design_md):
    components.add(match.group(1))

# Build traceability matrix
matrix_rows = []
for req_id in req_ids:
    ops = req_to_ops.get(req_id, [])
    msgs = req_to_msgs.get(req_id, [])
    op_str = ", ".join(ops) if ops else "N/A"
    msg_str = ", ".join(msgs) if msgs else "N/A"
    
    # Extract components from maps_to in tasks
    task_ids = req_to_tasks.get(req_id, [])
    task_str = ", ".join(task_ids) if task_ids else "N/A"
    
    # Status (based on tasks)
    if task_ids:
        task_statuses = [t["status"] for t in task_rows if t["task_id"] in task_ids]
        if all(s == "Done" for s in task_statuses):
            status = "Done"
        elif any(s == "Processing" for s in task_statuses):
            status = "InProgress"
        else:
            status = "Planned"
    else:
        status = "Planned"
    
    matrix_rows.append({
        "req_id": req_id,
        "op": op_str,
        "msg": msg_str,
        "components": "TBD",  # TODO: extract from design.md
        "tasks": task_str,
        "status": status,
        "verification": "None"
    })

# Build drift checks
unmapped_ops = [op for op, reqs in op_to_reqs.items() if not reqs]
unmapped_msgs = [msg for msg, reqs in msg_to_reqs.items() if not reqs]

# Generate traceability.md
now = datetime.now().strftime("%Y-%m-%d")
output = f"""# Traceability Matrix: <Feature Title>

> Confidentiality: <Public / Internal / Confidential>  
> Repo: <repo-name>  
> Ticket: <TICKET-ID>  
> Branch: <branch-name>  
> Owner: <name>  
> Created: {now}  
> Last Updated: {now}

---

## 0. Purpose (Non-negotiable)
This document provides an auditable mapping:
- **REQ (requirements.md)** → **Contract (OpenAPI/AsyncAPI)** → **Design components** → **Implementation tasks**
Used for:
- Coverage checks (no missing REQs)
- Drift checks (no unscoped operations/components)
- Review gating

Sources of truth:
- requirements: `10_prd/requirements.md`
- OpenAPI: `30_contract/openapi.yaml`
- AsyncAPI: `30_contract/asyncapi.yaml`
- Architecture: `40_design/architecture.md`
- Design: `40_design/design.md`
- Decisions: `40_design/decisions.md`
- Tasks: `40_design/tasks.md`

---

## 1. Rules (Do not violate)
1. Every row MUST include a **REQ-ID**.
2. Each row MUST include at least one of:
   - **operationId** (HTTP), OR
   - **messageId** (Event)
3. Contract mappings MUST be consistent with:
   - OpenAPI `x-requirements`
   - AsyncAPI `x-requirements`
4. "Component" MUST be a stable name used in design documents.
5. "TASK-ID" MUST exist in tasks.md once design phase starts.
6. If a REQ is intentionally not mapped, mark it explicitly as:
   - `CoverageStatus = N/A` with a reason in Notes (rare, but allowed)

---

## 2. Status Definitions
### 2.1 CoverageStatus
- **Planned**: mapping exists, not implemented yet
- **InProgress**: implementation ongoing
- **Done**: implemented and verified against acceptance criterion
- **Deferred**: intentionally postponed (must have reason)
- **N/A**: not applicable for this feature scope (must have reason)

### 2.2 Verification
- **Unit**: unit tests cover acceptance
- **Integration**: integration tests cover acceptance
- **E2E**: end-to-end tests cover acceptance
- **Manual**: manual verification only (discouraged; must justify)
- **None**: not verified yet

---

## 3. Traceability Matrix (REQ -> Contract -> Design -> Tasks)

> Notes:
> - Use **one row per REQ** as the default.
> - If a single REQ maps to multiple operations/events, either:
>   - list multiple IDs in the same cell (comma-separated), OR
>   - split into multiple rows with the same REQ-ID and different Contract IDs (choose one style and stick to it).
> - Avoid "TBD" in review phase. Use it only in early phases.

| REQ-ID | operationId (HTTP) | messageId (Event) | Components | TASK-IDs | CoverageStatus | Verification | Notes |
|---|---|---|---|---|---|---|---|
"""

for row in matrix_rows:
    output += f"| {row['req_id']} | {row['op']} | {row['msg']} | {row['components']} | {row['tasks']} | {row['status']} | {row['verification']} | - |\n"

output += """
---

## 4. Contract Drift Checks (Must be empty at Review)
> ここが埋まっている限り、レビューで「判定OK」は出せない。

### 4.1 HTTP operations without REQ mapping
"""

if unmapped_ops:
    for op in unmapped_ops:
        output += f"- operationId: {op} / Reason: Missing x-requirements\n"
else:
    output += "(None - all operations mapped)\n"

output += """
### 4.2 Event messages without REQ mapping
"""

if unmapped_msgs:
    for msg in unmapped_msgs:
        output += f"- messageId: {msg} / Reason: Missing x-requirements\n"
else:
    output += "(None - all messages mapped)\n"

output += """
### 4.3 Design components without REQ mapping
(Manual check required - components extracted from design.md)

---

## 5. Coverage Summary (Optional but recommended)
"""

total_reqs = len(matrix_rows)
mapped_to_http = sum(1 for r in matrix_rows if r["op"] != "N/A")
mapped_to_events = sum(1 for r in matrix_rows if r["msg"] != "N/A")
done = sum(1 for r in matrix_rows if r["status"] == "Done")
in_progress = sum(1 for r in matrix_rows if r["status"] == "InProgress")
planned = sum(1 for r in matrix_rows if r["status"] == "Planned")

output += f"""- Total REQs: {total_reqs}
- Mapped to HTTP: {mapped_to_http}
- Mapped to Events: {mapped_to_events}
- Done: {done}
- InProgress: {in_progress}
- Planned: {planned}
- Deferred/N/A: 0

---

## 6. Change Log
- {now} Auto-generated by gen_traceability.sh
"""

# Write output
write_text(traceability, output)
print(f"✅ Generated traceability.md with {total_reqs} REQs")
if unmapped_ops:
    print(f"⚠️  Warning: {len(unmapped_ops)} unmapped operations")
if unmapped_msgs:
    print(f"⚠️  Warning: {len(unmapped_msgs)} unmapped messages")
PY

