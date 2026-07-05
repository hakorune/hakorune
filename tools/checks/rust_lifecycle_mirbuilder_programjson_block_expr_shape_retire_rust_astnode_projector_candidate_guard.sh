#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-block-expr-shape-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-block-expr-shape-retire-rust-astnode-projector-candidate-v0.json"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_block_expr_shape_scan_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PARITY_GATE"

bash "$PARITY_GATE" >/dev/null

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))

if fixture.get("kind") != "MirBuilderProgramJsonBlockExprShapeRustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-BLOCK-EXPR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001":
    raise SystemExit("bad fixture token")

evidence = fixture.get("evidence") or {}
expected_hashes = {
    "hako_snapshot_source_hash": "sha256:a7c094f730341a6119bccc855ca205e2c8ae439d25ef002890dd03329b61bf8b",
    "parity_fixture_hash": "sha256:8a22a63f57a7bf313133947e7b4d47470f260fef0079f0e72fe10e1c253b6aea",
    "parity_gate_hash": "sha256:808eb21b174343bded89e4321e3b79d327c3a6469a78dd030801dd12e63f3d93",
    "scanner_source_hash": "sha256:13cddeca59ba8bbb50c8fadaecaba71b7a4ae4a8dc659dec86625b5d63813be9",
}
for key, expected in expected_hashes.items():
    if evidence.get(key) != expected:
        raise SystemExit(f"hash drift: {key}")

scope = fixture.get("retire_candidate_scope") or {}
if scope.get("retire_candidate") != "BlockExprShapeSnapshotV1":
    raise SystemExit("bad retire candidate")
if scope.get("shape_scope") != "covered ProgramJSON top-level BlockExpr rows":
    raise SystemExit("bad shape scope")
if scope.get("programjson_snapshot_owner") != "ProgramJsonBlockExprShapeScanV1":
    raise SystemExit("bad snapshot owner")

rows = scope.get("covered_rows") or []
expected_rows = [
    "empty_prelude_tail_int",
    "empty_prelude_tail_string",
    "empty_prelude_tail_bool",
    "local_prelude_tail_int",
    "expr_prelude_tail_string",
    "return_prelude_tail_var",
    "local_expr_prelude_tail_var",
    "many_prelude_tail_int",
    "tail_unsupported",
    "first_stmt_local_unsupported",
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
if criteria.get("block_expr_lowering") != 0:
    raise SystemExit("block expr lowering must stay unclaimed")
if criteria.get("prelude_execution_semantics") != 0:
    raise SystemExit("prelude execution semantics must stay unclaimed")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "mir_mutation",
    "id_allocation",
    "backend_lowering",
    "route_selection",
    "full_recipe_matcher_execution",
    "block_expr_lowering",
    "prelude_execution_semantics",
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
output_contract=rust-lifecycle-mirbuilder-programjson-block-expr-shape-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-BLOCK-EXPR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate=BlockExprShapeSnapshotV1
shape_scope=covered ProgramJSON top-level BlockExpr rows
covered_rows=10
decision=RetireCandidateScoped
parity_gate=green
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
block_expr_lowering=0
prelude_execution_semantics=0
source_selfhost_claim=0
hako_adopted_decision=0
programjson_full_parser_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
summary=ok
REPORT
