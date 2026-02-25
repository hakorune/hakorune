#!/bin/bash
# phase29ap_pattern4_continue_min_vm.sh - Pattern4 continue via plan routing (VM)

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29ap_pattern4_continue_min.hako"

output=$(run_nyash_vm "$FIXTURE")

expected=$(cat << 'TXT'
6
TXT
)

compare_outputs "$expected" "$output" "phase29ap_pattern4_continue_min_vm" || exit 1
