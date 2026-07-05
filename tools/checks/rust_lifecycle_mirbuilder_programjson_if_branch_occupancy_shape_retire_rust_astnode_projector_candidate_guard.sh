#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-if-branch-occupancy-shape-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-if-branch-occupancy-shape-retire-rust-astnode-projector-candidate-v0.json"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_if_branch_occupancy_shape_scan_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PARITY_GATE"

bash "$PARITY_GATE" >/dev/null

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))

if fixture.get("kind") != "MirBuilderProgramJsonIfBranchOccupancyShapeRustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-IF-BRANCH-OCCUPANCY-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001":
    raise SystemExit("bad fixture token")

evidence = fixture.get("evidence") or {}
expected_hashes = {
    "hako_snapshot_source_hash": "sha256:2cff5c771d2cd69de22019fbb03c146a2715bfb64646041ffe6090dc2a49a754",
    "parity_fixture_hash": "sha256:bb475b3fe0ed02264e5c1f712cb46b9a427008e42dee561ca116c0dfc9bfd8e0",
    "parity_gate_hash": "sha256:4dc382c4a80d255ec01f740a46233608e25e4a4c4fcf0fead2a38b8e2e666ec3",
}
for key, expected in expected_hashes.items():
    if evidence.get(key) != expected:
        raise SystemExit(f"hash drift: {key}")

scope = fixture.get("retire_candidate_scope") or {}
if scope.get("retire_candidate") != "IfBranchOccupancyShapeSnapshotV1":
    raise SystemExit("bad retire candidate")
if scope.get("shape_scope") != "covered ProgramJSON If branch occupancy rows":
    raise SystemExit("bad shape scope")
if scope.get("programjson_snapshot_owner") != "ProgramJsonIfBranchOccupancyShapeScanV1":
    raise SystemExit("bad snapshot owner")

rows = scope.get("covered_rows") or []
expected_rows = [
    "then_empty_else_null",
    "then_one_else_null",
    "then_empty_else_empty",
    "then_empty_else_one",
    "then_one_else_one",
    "then_two_else_null",
    "then_one_else_two",
    "then_many_else_null",
    "first_stmt_return_unsupported",
    "if_else_scalar_unsupported",
]
if rows != expected_rows:
    raise SystemExit("covered rows drift")

if scope.get("rust_projector_runtime_dependency_removed") != 0:
    raise SystemExit("runtime dependency removal must stay unclaimed")
if scope.get("rust_projector_oracle_only") != 1:
    raise SystemExit("rust oracle marker missing")
if scope.get("full_astnode_projector_retired") != 0:
    raise SystemExit("full ASTNode projector retirement must stay unclaimed")

criteria = fixture.get("criteria") or {}
if criteria.get("programjson_snapshot_parity_gate") != "Green":
    raise SystemExit("parity gate must be green")
if criteria.get("programjson_route_traverses_programjson") != 1:
    raise SystemExit("programjson traversal marker missing")
if criteria.get("programjson_route_uses_string_only_facade") != 0:
    raise SystemExit("string-only facade must remain 0")
if criteria.get("covered_row_count") != 10:
    raise SystemExit("covered row count drift")
if criteria.get("if_lowering") != 0:
    raise SystemExit("if lowering must stay unclaimed")
if criteria.get("branch_recipe_construction") != 0:
    raise SystemExit("branch recipe construction must stay unclaimed")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "mir_mutation",
    "id_allocation",
    "backend_lowering",
    "route_selection",
    "full_recipe_matcher_execution",
    "if_lowering",
    "branch_recipe_construction",
    "hako_adopted_decision",
    "programjson_full_parser_claim",
    "programjson_all_shapes_supported",
    "rust_astnode_projector_fully_retired",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

decision = fixture.get("decision") or {}
if decision.get("kind") != "RetireCandidateScoped":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != "MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION":
    raise SystemExit("bad selected next card")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-if-branch-occupancy-shape-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-IF-BRANCH-OCCUPANCY-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate=IfBranchOccupancyShapeSnapshotV1
shape_scope=covered ProgramJSON If branch occupancy rows
covered_rows=10
decision=RetireCandidateScoped
parity_gate=green
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
if_lowering=0
branch_recipe_construction=0
source_selfhost_claim=0
hako_adopted_decision=0
programjson_full_parser_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
summary=ok
REPORT
