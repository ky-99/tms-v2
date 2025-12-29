#!/usr/bin/env bash
set -euo pipefail

# ------------------------------------------------------------------------------
# gen_all.sh
# - Master script that orchestrates all generation scripts
# - Updates context_bundle.md, generates traceability.md, validates phase
#
# Usage:
#   tools/gen_all.sh [phase]
#   
#   phase: Requirements | Domain | Contract | Design | Implementation | Review
#          (default: Contract)
# ------------------------------------------------------------------------------

PHASE="${1:-Contract}"

echo "🚀 Starting full generation pipeline for Phase: $PHASE"
echo ""

# Step 0: Update context-specific sections (Objective, Manual Instructions)
echo "📝 Step 0/5: Updating Phase-specific context..."
if ./tools/gen_context.sh --phase "$PHASE" 2>&1; then
    echo "   ✅ Phase-specific context updated"
else
    echo "   ⚠️  gen_context.sh not found or failed (continuing...)"
fi
echo ""

# Step 1: Update context_bundle.md
echo "📋 Step 1/5: Updating context_bundle.md..."
if ./tools/gen_bundle.sh --phase "$PHASE" 2>&1; then
    echo "   ✅ context_bundle.md updated"
else
    echo "   ⚠️  context_bundle.md update had warnings (continuing...)"
fi
echo ""

# Step 2: Validate phase DoD (if applicable)
echo "✅ Step 2/5: Validating phase DoD..."
if ./tools/validate_phase.sh "$PHASE" 2>&1; then
    echo "   ✅ Phase $PHASE DoD satisfied"
    PHASE_VALID=true
else
    echo "   ⚠️  Phase $PHASE DoD not fully satisfied"
    PHASE_VALID=false
fi
echo ""

# Step 3: Generate traceability (if in Review or Implementation phase)
if [[ "$PHASE" == "Review" ]] || [[ "$PHASE" == "Implementation" ]]; then
    echo "🔍 Step 3/5: Generating traceability.md..."
    if ./tools/gen_traceability.sh 2>&1; then
        echo "   ✅ traceability.md generated"
    else
        echo "   ⚠️  traceability.md generation had warnings (continuing...)"
    fi
else
    echo "⏭️  Step 3/5: Skipping traceability (not Review/Implementation phase)"
fi
echo ""

# Step 4: Summary
echo "📊 Step 4/5: Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [[ "$PHASE_VALID" == true ]]; then
    echo "✅ Phase: $PHASE"
    echo "✅ Status: Ready for next phase"
    echo ""
    echo "Next steps:"
    case "$PHASE" in
        "Requirements")
            echo "  1. Review PRD.md and requirements.md"
            echo "  2. Run: ./tools/gen_all.sh Domain"
            ;;
        "Domain")
            echo "  1. Review glossary.md and domain.md"
            echo "  2. Run: ./tools/gen_all.sh Contract"
            ;;
        "Contract")
            echo "  1. Review openapi.yaml and asyncapi.yaml"
            echo "  2. Verify x-requirements in context_bundle.md"
            echo "  3. Run: ./tools/gen_all.sh Design"
            ;;
        "Design")
            echo "  1. Review architecture.md, design.md, tasks.md"
            echo "  2. Run: ./tools/gen_all.sh Implementation"
            ;;
        "Implementation")
            echo "  1. Implement tasks (update Status in tasks.md)"
            echo "  2. Run tests"
            echo "  3. Run: ./tools/gen_all.sh Review"
            ;;
        "Review")
            echo "  1. Review traceability.md"
            echo "  2. Check Pre-flight in context_bundle.md"
            echo "  3. Create Pull Request"
            ;;
    esac
else
    echo "⚠️  Phase: $PHASE"
    echo "⚠️  Status: Issues found"
    echo ""
    echo "Action required:"
    echo "  1. Fix issues listed above"
    echo "  2. Run: ./tools/gen_all.sh $PHASE"
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "✨ Generation pipeline completed!"

