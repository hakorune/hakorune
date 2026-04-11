#!/bin/bash
# phase29y vm-hako capability gate (app-first runtime lane)
#
# Contract pin:
# - Keep the retired phase29y vm-hako acceptance entrypoint stable while
#   blocking rows are moved into monitor-only shadow buckets.
# - This gate is now a compatibility stub; active vm-hako shadow rows live in
#   `vm-hako-core.txt`.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/gate_steps.sh"
require_env || exit 2

test_pass "phase29y_vm_hako_caps_gate_vm: PASS (retired compatibility stub)"
