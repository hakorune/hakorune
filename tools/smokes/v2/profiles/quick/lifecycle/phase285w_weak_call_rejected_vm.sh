#!/bin/bash
# phase285w_weak_call_rejected_vm.sh - Verify weak(x) syntax is rejected with helpful error
# SSOT: Phase 285W-Syntax-0.1

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/result_checker.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase285w_weak_call_rejected.hako"

# Expect parse error AND helpful message mentioning 'weak <expr>' or unary operator
# Pattern matches: "Unexpected token LPAREN" OR "Use 'weak expr'"
if check_error_pattern "$FIXTURE" "Unexpected token LPAREN.*weak.*expr|Use.*weak expr" "phase285w_weak_call_rejected"; then
    log_success "phase285w_weak_call_rejected: weak(x) correctly rejected with helpful error"
    exit 0
else
    log_error "phase285w_weak_call_rejected: weak(x) not rejected or error message unhelpful"
    exit 1
fi
