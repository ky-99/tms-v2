#!/usr/bin/env bash
set -euo pipefail

# ------------------------------------------------------------------------------
# gen_context.sh
# - Generates/updates context-specific sections in context_bundle.md
# - Auto-fills Phase-specific Objective, Manual Instructions, etc.
#
# Usage:
#   tools/gen_context.sh --phase <Phase> [--feature-title "<title>"]
#
# Phases: Requirements | Domain | Contract | Design | Implementation | Review
# ------------------------------------------------------------------------------

BUNDLE="90_review/context_bundle.md"
PHASE=""
FEATURE_TITLE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --bundle) BUNDLE="$2"; shift 2;;
    --phase) PHASE="$2"; shift 2;;
    --feature-title) FEATURE_TITLE="$2"; shift 2;;
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

if [[ -z "$PHASE" ]]; then
  echo "Error: --phase is required" >&2
  echo "Usage: $0 --phase <Phase>" >&2
  exit 1
fi

if [[ ! -f "$BUNDLE" ]]; then
  echo "Bundle not found: $BUNDLE" >&2
  exit 1
fi

export BUNDLE PHASE FEATURE_TITLE

python3 - <<'PY'
import os, re, sys
from pathlib import Path
from datetime import datetime

bundle = Path(os.environ.get("BUNDLE", "90_review/context_bundle.md"))
phase = os.environ.get("PHASE", "")
feature_title = os.environ.get("FEATURE_TITLE", "")

def read_text(p: Path) -> str:
    return p.read_text(encoding="utf-8")

def write_text(p: Path, s: str) -> None:
    p.write_text(s, encoding="utf-8")

# Phase別の標準的な Objective と Manual Instructions
PHASE_CONFIGS = {
    "Requirements": {
        "objective": "PRD と requirements.md を作成し、全ての要件を Given/When/Then 形式で定義する",
        "dod": [
            "DoD-1: PRD.md に背景、目標、スコープ、制約が記載されている",
            "DoD-2: requirements.md に全 REQ が Given/When/Then 形式で定義されている",
            "DoD-3: 全 REQ の依存関係が明確になっている"
        ],
        "deliverable_type": "PRD, requirements",
        "target_files": "`10_prd/PRD.md`, `10_prd/requirements.md`",
        "work_steps": [
            "PRD.md を作成：背景、ユーザーペルソナ、目標、スコープ、制約を定義",
            "requirements.md を作成：機能を細分化して REQ に分解",
            "各 REQ に Given/When/Then を記述",
            "REQ 間の依存関係を明示"
        ],
        "constraints": {
            "operationId": "N/A",
            "messageId": "N/A",
            "glossary": "Yes",
            "traceability": "No (later phase)"
        }
    },
    "Domain": {
        "objective": "ドメイン用語と概念モデルを確立し、glossary.md と domain.md を完成させる",
        "dod": [
            "DoD-1: glossary.md に主要用語が Canonical term として定義されている",
            "DoD-2: domain.md に Bounded Context、Entity、Aggregate が定義されている",
            "DoD-3: ドメイン不変条件（Invariants）が明記されている"
        ],
        "deliverable_type": "glossary, domain",
        "target_files": "`20_domain/glossary.md`, `20_domain/domain.md`",
        "work_steps": [
            "requirements.md から用語を抽出",
            "glossary.md に Canonical terms を定義（1用語=1定義）",
            "domain.md で Bounded Context を定義",
            "Entity、Value Object、Aggregate を定義",
            "ドメイン不変条件を明記"
        ],
        "constraints": {
            "operationId": "N/A",
            "messageId": "N/A",
            "glossary": "Yes (this is the phase to create it)",
            "traceability": "No"
        }
    },
    "Contract": {
        "objective": "HTTP API 仕様（OpenAPI）とイベント仕様（AsyncAPI）を確定し、全契約項目を REQ にマッピングする",
        "dod": [
            "DoD-1: openapi.yaml に全 operationId が定義され、x-requirements が設定されている",
            "DoD-2: asyncapi.yaml に全 messageId が定義され、x-requirements が設定されている",
            "DoD-3: Pre-flight Checks（Section 9）が全て合格している"
        ],
        "deliverable_type": "openapi, asyncapi",
        "target_files": "`30_contract/openapi.yaml`, `30_contract/asyncapi.yaml`",
        "work_steps": [
            "Focus REQs（Section 6）から必要な API エンドポイントを抽出",
            "openapi.yaml を作成：operationId + x-requirements を必ず設定",
            "asyncapi.yaml を作成：messageId + x-requirements を必ず設定",
            "gen_bundle.sh を実行して Contract Snapshot を更新",
            "Section 9 の Pre-flight Checks で漏れがないか確認"
        ],
        "constraints": {
            "operationId": "Yes (stability required)",
            "messageId": "Yes (version management: *.v1, *.v2)",
            "glossary": "Yes",
            "traceability": "No (Design phase)"
        }
    },
    "Design": {
        "objective": "システムアーキテクチャと実装設計を確定し、タスク分解を完了する",
        "dod": [
            "DoD-1: architecture.md に System Context と NFR が定義されている",
            "DoD-2: design.md に Component Model と End-to-end Flows が定義されている",
            "DoD-3: tasks.md に全 TASK が REQ/operationId/messageId にマッピングされている"
        ],
        "deliverable_type": "architecture, design, tasks, decisions",
        "target_files": "`40_design/*.md`",
        "work_steps": [
            "architecture.md を作成：System Context（C4 L1-L2）、NFR 定義",
            "design.md を作成：Component Model（C4 L3）、Flows、エラーハンドリング",
            "tasks.md を作成：TASK 分解、DoD 定義、REQ/Contract マッピング",
            "decisions.md を作成：ADR 記録",
            "gen_bundle.sh を実行して Tasking Snapshot を確認"
        ],
        "constraints": {
            "operationId": "Yes (must be stable)",
            "messageId": "Yes (must be stable)",
            "glossary": "Yes",
            "traceability": "Partial (start mapping)"
        }
    },
    "Implementation": {
        "objective": "タスク単位でコードを実装し、テストを含めて DoD を満たす",
        "dod": [
            "DoD-1: 全 TASK が Done または Processing 状態",
            "DoD-2: 各 TASK の DoD が満たされている（テスト含む）",
            "DoD-3: traceability.md が最新状態に保たれている"
        ],
        "deliverable_type": "Code implementation",
        "target_files": "Source code, tests",
        "work_steps": [
            "Section 8.1 の Current Phase Tasks から次の TASK を選択",
            "選択した TASK を実装（コード + テスト）",
            "tasks.md の Status を更新（UnDone → Processing → Done）",
            "gen_all.sh Implementation を実行して進捗を反映",
            "次の TASK に進む"
        ],
        "constraints": {
            "operationId": "Yes (no changes)",
            "messageId": "Yes (no changes)",
            "glossary": "Yes",
            "traceability": "Yes (auto-updated)"
        }
    },
    "Review": {
        "objective": "トレーサビリティを検証し、Pre-flight Checks を全て合格させて PR 準備を完了する",
        "dod": [
            "DoD-1: traceability.md の全マッピングが完了している",
            "DoD-2: Contract Drift Checks（Section 4）が空である",
            "DoD-3: Pre-flight Checks（Section 9）が全て合格（✅）している"
        ],
        "deliverable_type": "traceability, review",
        "target_files": "`90_review/traceability.md`",
        "work_steps": [
            "gen_all.sh Review を実行",
            "traceability.md の自動生成を確認",
            "Section 9 の Pre-flight Checks を確認（全て ✅ か）",
            "Contract Drift Checks が空であることを確認",
            "問題があれば修正して再実行",
            "全て OK なら PR 作成"
        ],
        "constraints": {
            "operationId": "Yes (frozen)",
            "messageId": "Yes (frozen)",
            "glossary": "Yes (frozen)",
            "traceability": "Yes (final check)"
        }
    }
}

if phase not in PHASE_CONFIGS:
    print(f"Error: Unknown phase: {phase}", file=sys.stderr)
    print(f"Valid phases: {', '.join(PHASE_CONFIGS.keys())}", file=sys.stderr)
    sys.exit(1)

config = PHASE_CONFIGS[phase]

# Read current bundle
doc = read_text(bundle)

# Update Section 1: Objective
objective_text = f"""## 1) Objective (What we are doing now)
- **Current objective**: {config['objective']}
- **Definition of Done**:
"""
for dod in config['dod']:
    objective_text += f"  - {dod}\n"

# Replace Section 1 (between "## 1)" and "---")
pattern_obj = re.compile(
    r"## 1\) Objective.*?(?=\n---)",
    re.DOTALL
)
if pattern_obj.search(doc):
    doc = pattern_obj.sub(objective_text.rstrip(), doc)
    print(f"✅ Updated Section 1: Objective for Phase {phase}")
else:
    print(f"⚠️  Warning: Could not find Section 1 to update", file=sys.stderr)

# Update Section 8.4: Manual Instructions
manual_text = f"""### 8.4 Manual Instructions (Edit as needed)
> **このセクションは Phase に応じて自動生成**：必要に応じて編集可能

#### 8.4.1 Primary deliverable
- Deliverable type: {config['deliverable_type']}
- Target file path: {config['target_files']}
- Required sections to modify:
  - (See work steps below)

#### 8.4.2 Work steps (Follow in order)
"""
for i, step in enumerate(config['work_steps'], 1):
    manual_text += f"{i}. {step}\n"

manual_text += f"""
#### 8.4.3 Output format (Strict)
- Provide:
  1. **Proposed edits** (copy-pastable content)
  2. **Diff summary** (Added/Changed/Removed bullets)
  3. **Open questions** (if any; max 10, targeted)

#### 8.4.4 Constraints for this task
- Must keep operationId stable: {config['constraints']['operationId']}
- Must keep messageId stable: {config['constraints']['messageId']}
- Must not introduce new terms unless added to glossary first: {config['constraints']['glossary']}
- Must update traceability: {config['constraints']['traceability']}
"""

# Replace Section 8.4 (between "### 8.4" and "---")
pattern_manual = re.compile(
    r"### 8\.4 Manual Instructions.*?(?=\n---)",
    re.DOTALL
)
if pattern_manual.search(doc):
    doc = pattern_manual.sub(manual_text.rstrip(), doc)
    print(f"✅ Updated Section 8.4: Manual Instructions for Phase {phase}")
else:
    print(f"⚠️  Warning: Could not find Section 8.4 to update", file=sys.stderr)

# Write back
write_text(bundle, doc)
print(f"✅ Context updated for Phase: {phase}")
PY

