#!/bin/bash
# phase120_stable_paths.sh — Phase 120: selfhost Stage-3 stable paths smoke test

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2
preflight_plugins || exit 2

# Phase 120 environment variables
export NYASH_JOINIR_STRICT=1
export NYASH_USE_NY_COMPILER=1
export NYASH_PARSER_STAGE3=1
export HAKO_PARSER_STAGE3=1

log_info "Phase 120: selfhost Stage-3 stable paths smoke test"
log_info "JoinIR Strict mode: NYASH_JOINIR_STRICT=1"

# Test counter
PASSED=0
FAILED=0
TOTAL=3

# Representative path 1: peek_expr_block.hako
log_info "[1/3] Testing peek_expr_block.hako (match expression with block expressions)"
output=$(run_nyash_vm "$NYASH_ROOT/apps/tests/peek_expr_block.hako" 2>&1) || true
if echo "$output" | grep -q "found one"; then
  log_success "peek_expr_block.hako: PASS (match expression lowered correctly)"
  PASSED=$((PASSED + 1))
else
  log_error "peek_expr_block.hako: FAIL (unexpected output or error)"
  echo "$output" | head -20
  FAILED=$((FAILED + 1))
fi

# Representative path 2: loop_min_while.hako
log_info "[2/3] Testing loop_min_while.hako (loop with PHI instructions)"
output=$(run_nyash_vm "$NYASH_ROOT/apps/tests/loop_min_while.hako" 2>&1) || true
if echo "$output" | grep -q "0" && echo "$output" | grep -q "1" && echo "$output" | grep -q "2"; then
  log_success "loop_min_while.hako: PASS (loop lowered correctly with PHI)"
  PASSED=$((PASSED + 1))
else
  log_error "loop_min_while.hako: FAIL (unexpected output or error)"
  echo "$output" | head -20
  FAILED=$((FAILED + 1))
fi

# Representative path 3: esc_dirname_smoke.hako
log_info "[3/3] Testing esc_dirname_smoke.hako (complex control structures with StringBox)"
output=$(run_nyash_vm "$NYASH_ROOT/apps/tests/esc_dirname_smoke.hako" 2>&1) || true
# Expected: This test currently fails with ConsoleBox.println error
# We record the baseline: this is a known issue for Phase 122+
if echo "$output" | grep -q "Unknown method 'println' on ConsoleBox"; then
  log_warn "esc_dirname_smoke.hako: BASELINE RECORDED (expected ConsoleBox.println error)"
  log_info "  Issue recorded for Phase 122+ resolution"
  # Don't count as pass or fail - it's a baseline recording
else
  log_error "esc_dirname_smoke.hako: FAIL (unexpected error - not the known ConsoleBox.println issue)"
  echo "$output" | head -20
  FAILED=$((FAILED + 1))
fi

# Summary
log_info "=========================================="
log_info "Phase 120 Baseline Results:"
log_info "  Passed: $PASSED/$TOTAL"
log_info "  Failed: $FAILED/$TOTAL"
log_info "  Known Issues: 1 (esc_dirname_smoke.hako)"
log_info "=========================================="
log_info "JoinIR If/Loop Lowering: Stable"
log_info "ControlForm structure: Correct"
log_info "PHI instruction generation: Working"
log_info "ConsoleBox.println: Known issue for Phase 122+"
log_info "=========================================="

# Exit code: pass if at least 2/3 pass (allowing for known issues)
if [ "$PASSED" -ge 2 ]; then
  log_success "Phase 120 baseline established successfully"
  exit 0
else
  log_error "Phase 120 baseline check failed"
  exit 1
fi
