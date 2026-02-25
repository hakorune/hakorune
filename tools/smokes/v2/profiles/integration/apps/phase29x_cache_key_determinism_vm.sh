#!/bin/bash
# Phase 29x X42: cache key determinism smoke
#
# Contract pin:
# 1) module/object/link keys are deterministic for identical inputs.
# 2) profile change updates module key.
# 3) ABI boundary change updates object/link keys.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

GUARD="$NYASH_ROOT/tools/checks/phase29x_cache_key_determinism_guard.sh"
if [ ! -x "$GUARD" ]; then
    test_fail "phase29x_cache_key_determinism: guard script missing or not executable: $GUARD"
    exit 1
fi

if ! "$GUARD"; then
    test_fail "phase29x_cache_key_determinism: determinism guard failed"
    exit 1
fi

test_pass "phase29x_cache_key_determinism: PASS (Module/Object/Link key determinism contract)"
