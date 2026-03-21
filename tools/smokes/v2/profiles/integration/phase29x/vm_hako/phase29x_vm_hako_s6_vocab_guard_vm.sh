#!/bin/bash
# Phase 29x X55: vm-hako S6 vocabulary inventory guard smoke
#
# Contract pin:
# - vm-hako subset op vocabulary is locked by allowlist+guard.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! bash "$NYASH_ROOT/tools/checks/phase29x_vm_hako_s6_vocab_guard.sh"; then
    test_fail "phase29x_vm_hako_s6_vocab_guard_vm: guard failed"
    exit 1
fi

test_pass "phase29x_vm_hako_s6_vocab_guard_vm: PASS (vm-hako S6 vocabulary inventory locked)"
