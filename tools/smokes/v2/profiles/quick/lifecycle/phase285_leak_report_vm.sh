#!/bin/bash
# phase285_leak_report_vm.sh - Phase 285: Exit-time leak report smoke test
#
# Verifies NYASH_LEAK_LOG={1,2} produces [lifecycle/leak] output at exit.

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase285_leak_report.hako"

# Test 1: Without NYASH_LEAK_LOG - no leak output
output_no_log=$(NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$FIXTURE" 2>&1)
if echo "$output_no_log" | grep -q "\[lifecycle/leak\]"; then
    log_error "phase285_leak_no_log: [lifecycle/leak] should NOT appear without NYASH_LEAK_LOG"
    exit 1
fi
if ! echo "$output_no_log" | grep -q "ok: cycle-created"; then
    log_error "phase285_leak_no_log: Expected 'ok: cycle-created' output"
    exit 1
fi
log_success "phase285_leak_no_log: No leak output when NYASH_LEAK_LOG is unset"

# Test 2: With NYASH_LEAK_LOG=1 - summary leak output
output_log1=$(NYASH_LEAK_LOG=1 NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$FIXTURE" 2>&1)
if ! echo "$output_log1" | grep -q "\[lifecycle/leak\] Roots still held at exit:"; then
    log_error "phase285_leak_log1: Expected '[lifecycle/leak] Roots still held at exit:' with NYASH_LEAK_LOG=1"
    exit 1
fi
if ! echo "$output_log1" | grep -q "\[lifecycle/leak\].*modules:"; then
    log_error "phase285_leak_log1: Expected '[lifecycle/leak]   modules: N' with NYASH_LEAK_LOG=1"
    exit 1
fi
if ! echo "$output_log1" | grep -q "ok: cycle-created"; then
    log_error "phase285_leak_log1: Expected 'ok: cycle-created' output"
    exit 1
fi
log_success "phase285_leak_log1: Summary leak output with NYASH_LEAK_LOG=1"

# Test 3: With NYASH_LEAK_LOG=2 - verbose leak output (module names)
output_log2=$(NYASH_LEAK_LOG=2 NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$FIXTURE" 2>&1)
if ! echo "$output_log2" | grep -q "\[lifecycle/leak\].*module names:"; then
    log_error "phase285_leak_log2: Expected '[lifecycle/leak]   module names:' with NYASH_LEAK_LOG=2"
    exit 1
fi
log_success "phase285_leak_log2: Verbose leak output with NYASH_LEAK_LOG=2"

log_success "phase285_leak_report_vm: All tests passed"
exit 0
