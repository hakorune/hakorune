#!/bin/bash
# RC/GC alignment G-RC-2: fast/milestone gate matrix
#
# Contract pin:
# - Replay RC/GC non-semantic-drift checks through one command.
# - Keep one fast lane and one milestone lane in the same matrix.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

CASES_FILE="$NYASH_ROOT/tools/checks/rc_gc_alignment_g2_gate_matrix_cases.txt"

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "rc_gc_alignment_g2_fast_milestone_gate: step failed: $cmd"
        exit 1
    fi
}

if [ ! -f "$CASES_FILE" ]; then
    test_fail "rc_gc_alignment_g2_fast_milestone_gate: matrix file missing: $CASES_FILE"
    exit 1
fi

run_step "tools/checks/rc_gc_alignment_g2_gate_matrix_guard.sh"

while IFS= read -r row || [ -n "$row" ]; do
    [ -z "$row" ] && continue
    IFS='|' read -r case_id gate_rel tier <<<"$row"

    if [ ! -x "$NYASH_ROOT/$gate_rel" ]; then
        test_fail "rc_gc_alignment_g2_fast_milestone_gate: gate missing or not executable (case=$case_id gate=$gate_rel)"
        exit 1
    fi

    if ! bash "$NYASH_ROOT/$gate_rel"; then
        test_fail "rc_gc_alignment_g2_fast_milestone_gate: matrix case failed (case=$case_id tier=$tier gate=$gate_rel)"
        exit 1
    fi
done < <(grep -v '^[[:space:]]*#' "$CASES_FILE" | sed '/^[[:space:]]*$/d')

test_pass "rc_gc_alignment_g2_fast_milestone_gate: PASS (G-RC-2 fast/milestone matrix locked)"
