#!/bin/bash
# loop_simple_while_stringutils_tolower_vm.sh - StringUtils.to_lower via plan/composer (VM)

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29ap_stringutils_tolower_min.hako"
export HAKO_JOINIR_STRICT=0
export NYASH_JOINIR_STRICT=0
export HAKO_JOINIR_DEV=0
export NYASH_JOINIR_DEV=0
export NYASH_DISABLE_PLUGINS=0
export NYASH_ALLOW_USING_FILE=1

output=$(run_nyash_vm "$FIXTURE")

expected=$(cat << 'TXT'
world
TXT
)

compare_outputs "$expected" "$output" "loop_simple_while_stringutils_tolower_vm" || exit 1
