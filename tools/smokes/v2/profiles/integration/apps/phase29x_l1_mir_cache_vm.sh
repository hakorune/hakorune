#!/bin/bash
# Phase 29x X43: L1 MIR cache smoke
#
# Contract pin:
# 1) First run creates MIR/ABI artifacts (miss).
# 2) Second run reuses same artifacts (hit).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

GUARD="$NYASH_ROOT/tools/checks/phase29x_l1_mir_cache_guard.sh"
if [ ! -x "$GUARD" ]; then
    test_fail "phase29x_l1_mir_cache: guard script missing or not executable: $GUARD"
    exit 1
fi

if ! "$GUARD"; then
    test_fail "phase29x_l1_mir_cache: l1 cache guard failed"
    exit 1
fi

test_pass "phase29x_l1_mir_cache: PASS (L1 MIR cache miss->hit contract)"
