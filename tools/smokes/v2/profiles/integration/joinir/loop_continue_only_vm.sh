#!/bin/bash
# current semantic wrapper; canonical entry for loop_continue_only smoke

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SEMANTIC_STEM="loop_continue_only_vm"
LEGACY_STEM="${LEGACY_STEM_OVERRIDE:-}"
LABEL_PREFIX="$SEMANTIC_STEM"
if [ -n "$LEGACY_STEM" ]; then
    LABEL_PREFIX="${SEMANTIC_STEM} (legacy stem ${LEGACY_STEM})"
fi

FIXTURE="$NYASH_ROOT/apps/tests/phase29ap_pattern4_continue_min.hako"

output=$(run_nyash_vm "$FIXTURE")

expected=$(cat << 'TXT'
6
TXT
)

compare_outputs "$expected" "$output" "${LABEL_PREFIX}" || exit 1
