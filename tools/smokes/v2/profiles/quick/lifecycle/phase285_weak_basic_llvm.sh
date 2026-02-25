#!/bin/bash
# phase285_weak_basic_llvm.sh - Phase 285 P2: WeakRef basic smoke test (LLVM harness)
#
# Verifies weak <expr> and weak_to_strong() work correctly in LLVM harness (P1: WeakRef New/Load implemented)
# Expected: exit code 2 (non-zero success, VM/LLVM parity)
# SSOT: docs/reference/language/lifecycle.md:179

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

# LLVM feature check (SKIP if not available) - Phase 287 P4: Use can_run_llvm SSOT
if ! can_run_llvm; then
    test_skip "phase285_weak_basic_llvm" "LLVM backend not available in this build"
    exit 0
fi

FIXTURE="$NYASH_ROOT/apps/tests/phase285_weak_basic.hako"

output=$(NYASH_LLVM_USE_HARNESS=1 NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" --backend llvm "$FIXTURE" 2>&1)
exit_code=$?

# Phase 285 P4: In LLVM harness, stdout can be polluted by harness/provider logs.
# Gate on exit code only (fixture returns 2 on success).
if [ "$exit_code" -ne 2 ]; then
    log_error "phase285_weak_basic_llvm: Expected exit code 2 (success), got: $exit_code"
    echo "$output"
    exit 1
fi

log_success "phase285_weak_basic_llvm: WeakRef basic test passed (LLVM harness)"
exit 0
