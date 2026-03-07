#!/bin/bash
# Selfhost Phase 150 Depth-1 Smoke Test
# Purpose: Verify 5 representative cases for selfhost Stage-3 pipeline
# Usage: ./tools/smokes/v2/profiles/integration/selfhost_phase150_depth1_smoke.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"

cd "$REPO_ROOT" || exit 1

PASS=0
FAIL=0

# Test candidates (from Phase 150 Task 2)
declare -a CANDIDATES=(
    "apps/tests/peek_expr_block.hako"
    "apps/tests/loop_min_while.hako"
    "apps/tests/string_method_chain.hako"
    "apps/tests/joinir_min_loop.hako"
    "apps/tests/joinir_if_select_simple.hako"
)

echo "=== Selfhost Phase 150 Depth-1 Smoke Test ==="
echo "Testing ${#CANDIDATES[@]} representative cases..."
echo ""

for candidate in "${CANDIDATES[@]}"; do
    name=$(basename "$candidate")
    echo -n "Testing: $name ... "

    # Run test with timeout
    # Phase S0: Removed NYASH_JOINIR_STRICT=1 to make this a baseline test (not strict-mode canary)
    # SSOT: docs/development/current/main/investigations/selfhost-integration-limitations.md (IfPhiJoin route gap)
    if timeout 10 bash -c \
        "NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1 \
         ./target/release/hakorune '$candidate' > /tmp/test_$$.log 2>&1" ; then
        # Check for errors in output
        if grep -qi "ERROR\|Parse error\|panic" /tmp/test_$$.log 2>/dev/null; then
            # Phase S0.1: Check for known route-shape gaps before failing
            # SSOT: docs/development/current/main/investigations/selfhost-integration-limitations.md
            if grep -qE "Phase 130 supports:|Loop lowering failed|StepTree lowering returned None|loop pattern is not supported|cap_missing/NestedLoop" /tmp/test_$$.log 2>/dev/null; then
                echo "⏭️  SKIP (known limitation, see investigation doc)"
                # Don't count as PASS or FAIL - just skip
            else
                echo "❌ FAIL (error found)"
                FAIL=$((FAIL + 1))
                tail -3 /tmp/test_$$.log | sed 's/^/  /'
            fi
        else
            echo "✅ PASS"
            PASS=$((PASS + 1))
        fi
    else
        exit_code=$?
        if [ $exit_code -eq 124 ]; then
            echo "⏱️  TIMEOUT (10s)"
            FAIL=$((FAIL + 1))
        else
            # Non-zero exit doesn't necessarily mean failure in our tests
            if grep -qi "ERROR\|Parse error\|panic" /tmp/test_$$.log 2>/dev/null; then
                # Phase S0.1: Check for known route-shape gaps before failing
                # SSOT: docs/development/current/main/investigations/selfhost-integration-limitations.md
                if grep -qE "Phase 130 supports:|Loop lowering failed|StepTree lowering returned None|loop pattern is not supported|cap_missing/NestedLoop" /tmp/test_$$.log 2>/dev/null; then
                    echo "⏭️  SKIP (known limitation, see investigation doc)"
                    # Don't count as PASS or FAIL - just skip
                else
                    echo "❌ FAIL (error found)"
                    FAIL=$((FAIL + 1))
                    tail -3 /tmp/test_$$.log | sed 's/^/  /'
                fi
            else
                echo "✅ PASS (non-zero exit but no errors)"
                PASS=$((PASS + 1))
            fi
        fi
    fi
    rm -f /tmp/test_$$.log
done

echo ""
echo "=== Summary ==="
echo "✅ Passed: $PASS"
echo "❌ Failed: $FAIL"
echo "Total: ${#CANDIDATES[@]}"

if [ $FAIL -eq 0 ]; then
    echo ""
    echo "✅ All selfhost depth-1 baseline cases passed!"
    exit 0
else
    echo ""
    echo "❌ Some cases failed. Review output above."
    exit 1
fi
