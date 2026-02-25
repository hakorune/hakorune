#!/bin/bash
# phase29aq_string_parse_integer_sign_min_vm.sh - StringUtils.parse_integer sign variant (VM)

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29aq_string_parse_integer_sign_min.hako"
export NYASH_ALLOW_USING_FILE=1

output=$(run_nyash_vm "$FIXTURE")

expected=$(cat << 'TXT'
-42
TXT
)

compare_outputs "$expected" "$output" "phase29aq_string_parse_integer_sign_min_vm" || exit 1
