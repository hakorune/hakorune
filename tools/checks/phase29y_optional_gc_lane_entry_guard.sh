#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29y/40-OPTIONAL-GC-LANE-ENTRY-SSOT.md"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29y_optional_gc_lane_entry_vm.sh"
RC_GC_GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g2_fast_milestone_gate.sh"
RC_GC_CASES="$ROOT_DIR/tools/checks/rc_gc_alignment_g2_gate_matrix_cases.txt"
source "$(dirname "$0")/lib/guard_common.sh"

TAG="phase29y-optional-gc-entry-guard"

cd "$ROOT_DIR"

echo "[$TAG] checking optional GC lane entry wiring"

guard_require_command "$TAG" rg
guard_require_files "$TAG" "$DOC" "$GATE" "$RC_GC_GATE" "$RC_GC_CASES"
guard_require_exec_files "$TAG" "$GATE" "$RC_GC_GATE"

guard_expect_in_file "$TAG" 'RC/GC matrix single-entry' "$DOC" "doc missing RC/GC matrix precondition section"
guard_expect_in_file "$TAG" 'g1_lifecycle_parity' "$DOC" "doc missing g1_lifecycle_parity precondition"
guard_expect_in_file "$TAG" 'g3_cycle_timing_matrix' "$DOC" "doc missing g3_cycle_timing_matrix precondition"
guard_expect_in_file "$TAG" 'g5_gc_mode_semantics_invariance' "$DOC" "doc missing g5_gc_mode_semantics_invariance precondition"
guard_expect_in_file "$TAG" 'phase29y_optional_gc_lane_entry_guard.sh' "$DOC" "doc missing guard reference"
guard_expect_in_file "$TAG" 'phase29y_optional_gc_lane_entry_vm.sh' "$DOC" "doc missing gate reference"

guard_expect_in_file "$TAG" 'g1_lifecycle_parity' "$RC_GC_CASES" "RC/GC matrix missing g1_lifecycle_parity case"
guard_expect_in_file "$TAG" 'g3_cycle_timing_matrix' "$RC_GC_CASES" "RC/GC matrix missing g3_cycle_timing_matrix case"
guard_expect_in_file "$TAG" 'g5_gc_mode_semantics_invariance' "$RC_GC_CASES" "RC/GC matrix missing g5_gc_mode_semantics_invariance case"

guard_expect_in_file "$TAG" 'phase29y_optional_gc_lane_entry_guard.sh' "$GATE" "gate missing guard precondition step"
guard_expect_in_file "$TAG" 'rc_gc_alignment_g2_fast_milestone_gate.sh' "$GATE" "gate missing RC/GC single-entry step"

echo "[$TAG] ok"
