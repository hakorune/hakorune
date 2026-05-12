#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-record-local-scalar-metadata"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-211-C203C-RECORD-LOCAL-SCALAR-METADATA.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
AGG_LOCAL="src/mir/agg_local_scalarization.rs"
PLACEMENT="src/mir/placement_effect.rs"
AGG_JSON="src/runner/mir_json_emit/agg_local.rs"
PLACEMENT_TESTS="src/runner/mir_json_emit/tests/placement.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_record_local_scalar_metadata_guard.sh"

echo "[$TAG] checking C203c record local scalar metadata"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$PHASE_README" \
  "$AGG_LOCAL" \
  "$PLACEMENT" \
  "$AGG_JSON" \
  "$PLACEMENT_TESTS" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C203c card must be complete"
guard_expect_in_file "$TAG" 'C203c status:' "$PLAN" "mimalloc plan must record C203c status"
guard_expect_in_file "$TAG" '`C203c` is complete as `293x-211`' "$RECORD_SSOT" "record SSOT must mark C203c complete"
guard_expect_in_file "$TAG" '`293x-211`' "$PHASE_README" "phase README must list C203c row"
guard_expect_in_file "$TAG" 'RecordLocalLayout' "$AGG_LOCAL" "agg-local owner must define record-local layout kind"
guard_expect_in_file "$TAG" 'collect_record_layout_routes' "$AGG_LOCAL" "agg-local owner must collect record layouts"
guard_expect_in_file "$TAG" 'record_local_layout' "$AGG_JSON" "MIR JSON agg-local emitter must expose record_local_layout"
guard_expect_in_file "$TAG" 'RecordLocalLayout' "$PLACEMENT" "placement/effect owner must fold record-local layout"
guard_expect_in_file "$TAG" 'record_local_layout\(7\)' "$PLACEMENT" "placement/effect test must preserve record detail"
guard_expect_in_file "$TAG" 'record_local_layout' "$PLACEMENT_TESTS" "MIR JSON placement test must cover record-local row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C203c guard"

if rg -n 'record_local_layout|RecordLocalLayout|record scalar|record local' lang/c-abi/shims src/llvm_py/instructions >/tmp/"$TAG".backend 2>&1; then
  echo "[$TAG] ERROR: C203c record-local matcher leaked into backend/user-box lowering" >&2
  cat /tmp/"$TAG".backend >&2
  rm -f /tmp/"$TAG".backend
  exit 1
fi
rm -f /tmp/"$TAG".backend

cargo test -q refresh_function_collects_folded_agg_local_routes
cargo test -q refresh_function_collects_folded_placement_effect_routes
cargo test -q build_mir_json_root_emits_agg_local_scalarization_routes

echo "[$TAG] ok"
