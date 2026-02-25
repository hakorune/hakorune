#!/bin/bash
# Phase 29x X44: L2 object cache smoke
#
# Contract pin:
# 1) First run materializes object artifact (miss).
# 2) Second run reuses object artifact (hit).
# 3) ABI diff causes object key miss and re-materialization.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! can_run_llvm; then
    test_skip "phase29x_l2_object_cache: LLVM backend not available"
    exit 0
fi

GUARD="$NYASH_ROOT/tools/checks/phase29x_l2_object_cache_guard.sh"
if [ ! -x "$GUARD" ]; then
    test_fail "phase29x_l2_object_cache: guard script missing or not executable: $GUARD"
    exit 1
fi

if ! "$GUARD"; then
    test_fail "phase29x_l2_object_cache: l2 cache guard failed"
    exit 1
fi

test_pass "phase29x_l2_object_cache: PASS (L2 object cache miss->hit + ABI diff miss contract)"
