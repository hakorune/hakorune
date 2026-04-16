#!/bin/bash
# phase21_5_perf_const_hoist_contract_vm.sh
#
# Retired contract:
# - bench_box_create_destroy now folds literal StringBox boxing away entirely.
# - active integration coverage lives in loop/hotspot perf smokes instead.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

test_skip "[SKIP:retired] phase21_5_perf_const_hoist_contract_vm: retired to archive; active coverage moved to loop/hotspot contracts"
