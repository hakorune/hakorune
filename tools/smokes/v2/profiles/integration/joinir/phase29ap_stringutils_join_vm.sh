#!/bin/bash
# phase29ap_stringutils_join_vm.sh - StringUtils.join via plan/composer (VM)

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29ap_stringutils_join_min.hako"
export HAKO_JOINIR_STRICT=0
export NYASH_JOINIR_STRICT=0
export HAKO_JOINIR_DEV=0
export NYASH_JOINIR_DEV=0
export NYASH_DISABLE_PLUGINS=0
export NYASH_ALLOW_USING_FILE=1

output=$(run_nyash_vm "$FIXTURE")

expected=$(cat << 'TXT'
a,b,c
TXT
)

compare_outputs "$expected" "$output" "phase29ap_stringutils_join_vm" || exit 1
