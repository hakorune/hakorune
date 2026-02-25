#!/bin/bash
# Phase 252 P0: this.methodcall in break condition (VM)
set -euo pipefail

HAKO_PATH="apps/tests/phase252_p0_this_methodcall_break_cond_min.hako"

# Phase 252: Test StringUtils.count_leading_digits with this.is_digit break condition
EXPECTED_EXIT=3  # "123abc" has 3 leading digits

$HAKORUNE_BIN --backend vm "$HAKO_PATH"
actual_exit=$?

if [[ $actual_exit -eq $EXPECTED_EXIT ]]; then
    echo "✅ phase252_p0_this_methodcall_break_cond_vm: PASS (exit=$actual_exit)"
    exit 0
else
    echo "❌ phase252_p0_this_methodcall_break_cond_vm: FAIL (expected=$EXPECTED_EXIT, got=$actual_exit)"
    exit 1
fi
