#!/bin/bash
# phase29aq_string_trim_start_min_vm.sh - StringUtils.trim_start via plan/composer (VM)

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29aq_string_trim_start_min.hako"
export NYASH_ALLOW_USING_FILE=1

output=$(run_nyash_vm "$FIXTURE")

expected=$(cat << 'TXT'
ab 
TXT
)

compare_outputs "$expected" "$output" "phase29aq_string_trim_start_min_vm" || exit 1
