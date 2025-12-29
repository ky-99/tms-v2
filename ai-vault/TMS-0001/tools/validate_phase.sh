#!/usr/bin/env bash
set -euo pipefail

# ------------------------------------------------------------------------------
# validate_phase.sh
# - Validates if current phase's Definition of Done (DoD) is satisfied
# - Returns exit code 0 if satisfied, 1 if not
#
# Usage:
#   tools/validate_phase.sh <phase>
#   
#   Phases: Requirements | Domain | Contract | Design | Implementation | Review
# ------------------------------------------------------------------------------

PHASE="${1:-}"

if [[ -z "$PHASE" ]]; then
  echo "Usage: $0 <phase>" >&2
  echo "Phases: Requirements | Domain | Contract | Design | Implementation | Review" >&2
  exit 1
fi

export PHASE

python3 - <<'PY'
import os, sys
from pathlib import Path

phase = os.environ.get("PHASE", "")

def file_exists(path: str) -> bool:
    return Path(path).exists()

def validate_requirements_phase() -> tuple[bool, list]:
    """PRD & requirements.md exist and complete"""
    issues = []
    
    if not file_exists("10_prd/PRD.md"):
        issues.append("Missing: 10_prd/PRD.md")
    if not file_exists("10_prd/requirements.md"):
        issues.append("Missing: 10_prd/requirements.md")
    else:
        # Check if requirements have Given/When/Then
        req_md = Path("10_prd/requirements.md").read_text()
        if "Given" not in req_md or "When" not in req_md or "Then" not in req_md:
            issues.append("requirements.md missing Given/When/Then format")
    
    return (len(issues) == 0, issues)

def validate_domain_phase() -> tuple[bool, list]:
    """glossary.md & domain.md exist"""
    issues = []
    
    if not file_exists("20_domain/glossary.md"):
        issues.append("Missing: 20_domain/glossary.md")
    if not file_exists("20_domain/domain.md"):
        issues.append("Missing: 20_domain/domain.md")
    
    return (len(issues) == 0, issues)

def validate_contract_phase() -> tuple[bool, list]:
    """openapi.yaml & asyncapi.yaml exist with x-requirements"""
    issues = []
    
    if not file_exists("30_contract/openapi.yaml"):
        issues.append("Missing: 30_contract/openapi.yaml")
    if not file_exists("30_contract/asyncapi.yaml"):
        issues.append("Missing: 30_contract/asyncapi.yaml")
    
    # Check x-requirements (basic check)
    try:
        import yaml
        if file_exists("30_contract/openapi.yaml"):
            openapi = yaml.safe_load(Path("30_contract/openapi.yaml").read_text())
            paths = openapi.get("paths", {}) or {}
            for path, methods in paths.items():
                for method, op in methods.items():
                    if isinstance(op, dict) and "operationId" in op:
                        if "x-requirements" not in op:
                            issues.append(f"Missing x-requirements: {op['operationId']}")
        
        if file_exists("30_contract/asyncapi.yaml"):
            asyncapi = yaml.safe_load(Path("30_contract/asyncapi.yaml").read_text())
            messages = asyncapi.get("components", {}).get("messages", {}) or {}
            for name, msg in messages.items():
                if isinstance(msg, dict) and "messageId" in msg:
                    if "x-requirements" not in msg:
                        issues.append(f"Missing x-requirements: {msg['messageId']}")
    except ImportError:
        issues.append("PyYAML not installed - cannot validate x-requirements")
    
    return (len(issues) == 0, issues)

def validate_design_phase() -> tuple[bool, list]:
    """architecture.md, design.md, tasks.md exist"""
    issues = []
    
    if not file_exists("40_design/architecture.md"):
        issues.append("Missing: 40_design/architecture.md")
    if not file_exists("40_design/design.md"):
        issues.append("Missing: 40_design/design.md")
    if not file_exists("40_design/tasks.md"):
        issues.append("Missing: 40_design/tasks.md")
    
    return (len(issues) == 0, issues)

def validate_implementation_phase() -> tuple[bool, list]:
    """Tasks are in progress or done"""
    issues = []
    
    if not file_exists("40_design/tasks.md"):
        issues.append("Missing: 40_design/tasks.md")
        return (False, issues)
    
    tasks_md = Path("40_design/tasks.md").read_text()
    # Check if any tasks are marked as Done or Processing
    if "| Done |" not in tasks_md and "| Processing |" not in tasks_md:
        issues.append("No tasks in Done or Processing status")
    
    return (len(issues) == 0, issues)

def validate_review_phase() -> tuple[bool, list]:
    """traceability.md exists and Pre-flight checks pass"""
    issues = []
    
    if not file_exists("90_review/traceability.md"):
        issues.append("Missing: 90_review/traceability.md")
    
    if not file_exists("90_review/context_bundle.md"):
        issues.append("Missing: 90_review/context_bundle.md")
    else:
        # Check Pre-flight checks in context_bundle
        bundle = Path("90_review/context_bundle.md").read_text()
        if "⚠️" in bundle:
            issues.append("Pre-flight checks not all passed (⚠️ found in context_bundle.md)")
    
    return (len(issues) == 0, issues)

# Validate based on phase
validators = {
    "Requirements": validate_requirements_phase,
    "Domain": validate_domain_phase,
    "Contract": validate_contract_phase,
    "Design": validate_design_phase,
    "Implementation": validate_implementation_phase,
    "Review": validate_review_phase
}

if phase not in validators:
    print(f"❌ Unknown phase: {phase}", file=sys.stderr)
    print(f"Valid phases: {', '.join(validators.keys())}", file=sys.stderr)
    sys.exit(1)

print(f"🔍 Validating Phase: {phase}")
passed, issues = validators[phase]()

if passed:
    print(f"✅ Phase {phase} DoD satisfied - ready for next phase")
    sys.exit(0)
else:
    print(f"❌ Phase {phase} DoD not satisfied:")
    for issue in issues:
        print(f"  - {issue}")
    sys.exit(1)
PY

