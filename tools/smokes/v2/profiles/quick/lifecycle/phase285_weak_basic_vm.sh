#!/bin/bash
# phase285_weak_basic_vm.sh - Phase 285W-Syntax-0: WeakRef basic smoke test (VM)
#
# Verifies weak <expr> and weak_to_strong() work correctly in VM backend.
# Note: Full drop semantics test deferred (needs GC/scope analysis)
# SSOT: docs/reference/language/lifecycle.md:179

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase285_weak_basic.hako"

output=$(NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$FIXTURE" 2>&1)
exit_code=$?

# Check for success marker
if ! echo "$output" | grep -q "ok: weak and weak_to_strong work correctly"; then
    log_error "phase285_weak_basic_vm: Expected 'ok: weak and weak_to_strong work correctly'"
    echo "$output"
    exit 1
fi

# Check for failure markers
if echo "$output" | grep -q "ng:"; then
    log_error "phase285_weak_basic_vm: Found 'ng:' in output (test failure)"
    echo "$output"
    exit 1
fi

# Check exit code (Phase 285 P2: exit 2 = success)
if [ "$exit_code" -ne 2 ]; then
    log_error "phase285_weak_basic_vm: Expected exit code 2 (non-zero success), got: $exit_code"
    echo "$output"
    exit 1
fi

log_success "phase285_weak_basic_vm: WeakRef basic test passed"
exit 0
