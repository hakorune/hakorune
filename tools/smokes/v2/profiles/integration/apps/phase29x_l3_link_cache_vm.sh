#!/bin/bash
# Phase 29x X45: L3 link cache smoke
#
# Contract pin:
# 1) First run materializes link artifact (miss).
# 2) Second run reuses link artifact (hit).
# 3) Runtime ABI diff causes link key miss while L2 object stays hit.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! can_run_llvm; then
    test_skip "phase29x_l3_link_cache: LLVM backend not available"
    exit 0
fi

GUARD="$NYASH_ROOT/tools/checks/phase29x_l3_link_cache_guard.sh"
if [ ! -x "$GUARD" ]; then
    test_fail "phase29x_l3_link_cache: guard script missing or not executable: $GUARD"
    exit 1
fi

if ! "$GUARD"; then
    test_fail "phase29x_l3_link_cache: l3 cache guard failed"
    exit 1
fi

test_pass "phase29x_l3_link_cache: PASS (L3 link cache miss->hit + runtime ABI diff miss contract)"
