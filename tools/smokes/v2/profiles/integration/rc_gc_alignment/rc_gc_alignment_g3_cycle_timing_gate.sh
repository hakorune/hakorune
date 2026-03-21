#!/bin/bash
# RC/GC alignment G-RC-3: weak/strong cycle + explicit-drop timing gate
#
# Contract pin:
# - Replay weak/drop parity, strong-cycle observability, and drop timing checks
#   through one matrix-driven command.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

CASES_FILE="$NYASH_ROOT/tools/checks/rc_gc_alignment_g3_cycle_timing_cases.txt"

if [ ! -f "$CASES_FILE" ]; then
    test_fail "rc_gc_alignment_g3_cycle_timing_gate: matrix file missing: $CASES_FILE"
    exit 1
fi

if ! bash "$NYASH_ROOT/tools/checks/rc_gc_alignment_g3_cycle_timing_guard.sh"; then
    test_fail "rc_gc_alignment_g3_cycle_timing_gate: guard precondition failed"
    exit 1
fi

while IFS= read -r row || [ -n "$row" ]; do
    [ -z "$row" ] && continue
    IFS='|' read -r case_id gate_rel focus <<<"$row"

    if [ ! -x "$NYASH_ROOT/$gate_rel" ]; then
        test_fail "rc_gc_alignment_g3_cycle_timing_gate: gate missing or not executable (case=$case_id gate=$gate_rel)"
        exit 1
    fi

    if ! bash "$NYASH_ROOT/$gate_rel"; then
        test_fail "rc_gc_alignment_g3_cycle_timing_gate: matrix case failed (case=$case_id focus=$focus gate=$gate_rel)"
        exit 1
    fi
done < <(grep -v '^[[:space:]]*#' "$CASES_FILE" | sed '/^[[:space:]]*$/d')

test_pass "rc_gc_alignment_g3_cycle_timing_gate: PASS (G-RC-3 cycle/timing matrix locked)"
