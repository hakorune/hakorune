#!/bin/bash
# phase29ap_pattern4_continue_min_vm.sh - loop_continue_only via plan routing (VM)
# legacy compat stem; current semantic entry = loop_continue_only_vm.sh

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

LEGACY_STEM="phase29ap_pattern4_continue_min_vm"
SEMANTIC_STEM="loop_continue_only_vm"
LABEL_PREFIX="${SEMANTIC_STEM} (legacy stem ${LEGACY_STEM})"

FIXTURE="$NYASH_ROOT/apps/tests/phase29ap_pattern4_continue_min.hako"

output=$(run_nyash_vm "$FIXTURE")

expected=$(cat << 'TXT'
6
TXT
)

compare_outputs "$expected" "$output" "${LABEL_PREFIX}" || exit 1
