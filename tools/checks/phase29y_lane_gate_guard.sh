#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29y/50-LANE-GATE-SSOT.md"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh"
QUICK_GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh"
source "$(dirname "$0")/lib/guard_common.sh"

CORE_GATE="tools/smokes/v2/profiles/integration/apps/phase29y_core_contracts_vm.sh"
OPT_GC_ENTRY_GATE="tools/smokes/v2/profiles/integration/apps/phase29y_optional_gc_lane_entry_vm.sh"
NO_COMPAT_GATE="tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh"
PIPELINE_PARITY_GATE="tools/smokes/v2/profiles/integration/apps/phase29y_hako_using_resolver_parity_vm.sh"
RUN_BINARY_ONLY_PORTED_GATE="tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_ported_vm.sh"
MIR_SHAPE_GUARD_GATE="tools/smokes/v2/profiles/integration/apps/mir_shape_guard_vm.sh"
RING1_GATE="tools/smokes/v2/profiles/integration/apps/phase29y_ring1_gate_vm.sh"
DIRECT_V0_GUARD_GATE="tools/smokes/v2/profiles/integration/apps/phase29y_direct_v0_bridge_guard_vm.sh"
DIRECT_V0_RETIREMENT_GUARD="tools/checks/phase29y_direct_v0_retirement_guard.sh"

TAG="phase29y-lane-gate-guard"

cd "$ROOT_DIR"

echo "[$TAG] checking phase29y lane gate wiring"

guard_require_command "$TAG" rg
guard_require_files "$TAG" "$DOC" "$GATE" "$QUICK_GATE"
guard_require_exec_files "$TAG" "$GATE" "$QUICK_GATE"

guard_expect_in_file "$TAG" 'phase29y_lane_gate_guard.sh' "$DOC" "doc missing guard reference"
guard_expect_in_file "$TAG" 'phase29y_lane_gate_vm.sh' "$DOC" "doc missing full gate reference"
guard_expect_in_file "$TAG" 'phase29y_lane_gate_quick_vm.sh' "$DOC" "doc missing quick gate reference"

for dep in "$RING1_GATE" "$PIPELINE_PARITY_GATE" "$RUN_BINARY_ONLY_PORTED_GATE" "$MIR_SHAPE_GUARD_GATE" "$DIRECT_V0_GUARD_GATE" "$DIRECT_V0_RETIREMENT_GUARD" "$NO_COMPAT_GATE" "$CORE_GATE" "$OPT_GC_ENTRY_GATE"; do
  guard_require_exec_files "$TAG" "$ROOT_DIR/$dep"
  guard_expect_in_file "$TAG" "$dep" "$DOC" "doc missing dependency gate reference: $dep"
done

for dep in "$RING1_GATE" "$PIPELINE_PARITY_GATE" "$RUN_BINARY_ONLY_PORTED_GATE" "$MIR_SHAPE_GUARD_GATE" "$DIRECT_V0_GUARD_GATE" "$DIRECT_V0_RETIREMENT_GUARD" "$NO_COMPAT_GATE" "$CORE_GATE"; do
  guard_expect_in_file "$TAG" "$dep" "$QUICK_GATE" "quick gate missing dependency step: $dep"
done

guard_expect_in_file "$TAG" 'phase29y_lane_gate_quick_vm.sh' "$GATE" "full gate missing quick gate dependency step"
guard_expect_in_file "$TAG" "$OPT_GC_ENTRY_GATE" "$GATE" "full gate missing optional GC dependency step"

guard_expect_in_file "$TAG" 'phase29y_lane_gate_guard.sh' "$QUICK_GATE" "quick gate missing guard precondition step"

echo "[$TAG] ok"
