#!/bin/bash
# Phase 29x X57: NewClosure runtime lane decision refresh gate
#
# Contract pin:
# - Runtime boundary stays fail-fast for new_closure at X57.
# - X56 parity gate stays green before NewClosure decision is confirmed.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

assert_gate_contract() {
    local gate="$1"
    local allowlist="$NYASH_ROOT/tools/checks/phase29x_vm_hako_s6_vocab_allowlist.txt"
    local doc_x50="$NYASH_ROOT/docs/development/current/main/phases/phase-29x/29x-77-newclosure-contract-lock-ssot.md"
    local doc_x57="$NYASH_ROOT/docs/development/current/main/phases/phase-29x/29x-83-vm-hako-newclosure-runtime-lane-decision-refresh-ssot.md"
    if ! command -v rg >/dev/null 2>&1; then
        test_fail "phase29x_vm_hako_newclosure_decision_refresh_vm: rg is required"
        exit 1
    fi
    if [ ! -f "$doc_x50" ] || [ ! -f "$doc_x57" ] || [ ! -f "$allowlist" ]; then
        test_fail "phase29x_vm_hako_newclosure_decision_refresh_vm: missing SSOT or allowlist input"
        exit 1
    fi
    if ! rg -q '^Decision: accepted$' "$doc_x50"; then
        test_fail "phase29x_vm_hako_newclosure_decision_refresh_vm: X50 decision drift"
        exit 1
    fi
    if ! rg -q '^Decision: accepted$' "$doc_x57"; then
        test_fail "phase29x_vm_hako_newclosure_decision_refresh_vm: X57 decision drift"
        exit 1
    fi
    if ! rg -q 'Decision owner: .*29x-77-newclosure-contract-lock-ssot.md' "$doc_x57"; then
        test_fail "phase29x_vm_hako_newclosure_decision_refresh_vm: X57 decision owner drift"
        exit 1
    fi
    if rg -q '^new_closure$' "$allowlist"; then
        test_fail "phase29x_vm_hako_newclosure_decision_refresh_vm: allowlist drift (new_closure promoted)"
        exit 1
    fi
    if ! rg -q 'phase29x_vm_hako_s6_parity_gate_vm.sh' "$gate"; then
        test_fail "phase29x_vm_hako_newclosure_decision_refresh_vm: gate missing parity step"
        exit 1
    fi
    if ! rg -q 'phase29x_vm_hako_newclosure_contract_vm.sh' "$gate"; then
        test_fail "phase29x_vm_hako_newclosure_decision_refresh_vm: gate missing NewClosure contract step"
        exit 1
    fi
}

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_vm_hako_newclosure_decision_refresh_vm: step failed: $cmd"
        exit 1
    fi
}

assert_gate_contract "$0"
run_step "tools/smokes/v2/profiles/integration/phase29x/vm_hako/phase29x_vm_hako_s6_parity_gate_vm.sh"
run_step "tools/smokes/v2/profiles/integration/phase29x/vm_hako/phase29x_vm_hako_newclosure_contract_vm.sh"

test_pass "phase29x_vm_hako_newclosure_decision_refresh_vm: PASS (X57 NewClosure runtime decision locked)"
