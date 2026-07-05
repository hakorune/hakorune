#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-trycatch-shape-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-trycatch-shape-retire-rust-astnode-projector-candidate-v0.json"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_trycatch_shape_scan_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PARITY_GATE"

bash "$PARITY_GATE" >/dev/null

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))

if fixture.get("kind") != "MirBuilderProgramJsonTryCatchShapeRustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-TRYCATCH-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001":
    raise SystemExit("bad fixture token")

evidence = fixture.get("evidence") or {}
expected_hashes = {
    "hako_snapshot_source_hash": "sha256:15f9cb6ac8ce36851ba48c5d3dbddb89a7e9cebf263cde83802bd1fa578a9578",
    "parity_fixture_hash": "sha256:6e6a2ed9693860f819f77909b072b95bb8efcb6c422fff6d01906faa41aef43b",
    "parity_gate_hash": "sha256:36b6eb4f4a398619b61124511139a6c5bbdf90d1990af36806840903d6ff2f9e",
    "scanner_source_hash": "sha256:f8cb2ad2daadccfe10ee15907c8ad2418f9c204e55bf36a7cb1f204a574e0a50",
}
for key, expected in expected_hashes.items():
    if evidence.get(key) != expected:
        raise SystemExit(f"hash drift: {key}")

scope = fixture.get("retire_candidate_scope") or {}
if scope.get("retire_candidate") != "TryCatchShapeSnapshotV1":
    raise SystemExit("bad retire candidate")
if scope.get("shape_scope") != "covered ProgramJSON TryCatch rows":
    raise SystemExit("bad shape scope")
if scope.get("programjson_snapshot_owner") != "ProgramJsonTryCatchShapeScanV1":
    raise SystemExit("bad snapshot owner")

expected_rows = [
    "try_throw_no_catch_no_cleanup",
    "try_return_one_catch_no_cleanup",
    "try_expr_one_catch_cleanup_expr",
    "try_empty_many_catches_no_cleanup",
    "try_return_no_catch_cleanup_return",
    "nested_trycatch_unsupported",
    "cleanup_scalar_unsupported",
    "first_stmt_local_unsupported",
]
if scope.get("covered_rows") != expected_rows:
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
if criteria.get("covered_row_count") != 8:
    raise SystemExit("covered row count drift")
for key in ["exception_runtime_semantics", "catch_matching", "cleanup_execution_semantics"]:
    if criteria.get(key) != 0:
        raise SystemExit(f"forbidden criteria drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "mir_mutation",
    "id_allocation",
    "backend_lowering",
    "route_selection",
    "full_recipe_matcher_execution",
    "exception_runtime_semantics",
    "catch_matching",
    "cleanup_execution_semantics",
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
if decision.get("selected_next_card") != "MIRBUILDER-PROGRAMJSON-LAYER4-STRUCTURED-PLAN-RECIPE-DTO-PILOT-SELECTION":
    raise SystemExit("bad selected next card")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-trycatch-shape-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-TRYCATCH-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate=TryCatchShapeSnapshotV1
shape_scope=covered ProgramJSON TryCatch rows
covered_rows=8
decision=RetireCandidateScoped
parity_gate=green
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
exception_runtime_semantics=0
catch_matching=0
cleanup_execution_semantics=0
source_selfhost_claim=0
hako_adopted_decision=0
programjson_full_parser_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-LAYER4-STRUCTURED-PLAN-RECIPE-DTO-PILOT-SELECTION
summary=ok
REPORT
