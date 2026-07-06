#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-stmt-only-block-recipe-snapshot-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-stmt-only-block-recipe-snapshot-retire-rust-astnode-projector-candidate-v0.json"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_stmt_only_block_recipe_snapshot_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PARITY_GATE"

PARITY_OUT="$(bash "$PARITY_GATE")"
for required in \
  '^runtime_parity_green=1$' \
  '^mir_json_route_green=1$' \
  '^row_count=4$' \
  '^recipe_root_traversal_used=1$' \
  '^stmt_only_reducer_called=1$' \
  '^prebuilt_token_snapshot_input=0$' \
  '^string_only_facade=0$'
do
  if ! grep -q "$required" <<<"$PARITY_OUT"; then
    printf '%s\n' "$PARITY_OUT" >&2
    guard_fail "$TAG" "ProgramJSON StmtOnly block recipe prerequisite drift: $required"
  fi
done

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if fixture.get("kind") != "MirBuilderProgramJsonStmtOnlyBlockRecipeSnapshotRustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-SNAPSHOT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001":
    raise SystemExit("bad fixture token")

expected_hashes = {
    "hako_snapshot_source_hash": "sha256:dd70fdadcebdb9abf64982adf1139c0142541d868f5dbf37a9855a442a853c62",
    "target_reducer_source_hash": "sha256:09bd9913e8cc9b3ecfb70b6f83973ed666d160d050b85e81e24f983ddbf80e3a",
    "parity_fixture_hash": "sha256:cf87a34c628b2661295720c7feb7c244b9a3d0a8420f2636104db7cb8799acca",
    "parity_gate_hash": "sha256:76ecf68608b086185976ecc22757ca1eadc2d8c5798548d1f6aba4f2de49d773",
}
evidence = fixture.get("evidence") or {}
for key, expected in expected_hashes.items():
    if evidence.get(key) != expected:
        raise SystemExit(f"hash drift: {key}")

scope = fixture.get("retire_candidate_scope") or {}
if scope.get("retire_candidate") != "ProgramJsonStmtOnlyBlockRecipeSnapshotV1":
    raise SystemExit("bad retire candidate")
if scope.get("shape_scope") != "covered ProgramJSON stmt-only block recipe direct stmt rows":
    raise SystemExit("bad shape scope")
if scope.get("programjson_snapshot_owner") != "ProgramJsonStmtOnlyBlockRecipeSnapshotV1":
    raise SystemExit("bad snapshot owner")
if scope.get("covered_rows") != ["local_print", "local_assignment", "local_return_reject", "return_only_reject"]:
    raise SystemExit("covered rows drift")
if scope.get("deferred_rows") != ["if_no_exit", "loop_no_exit", "no_exit_block", "exit_allowed_block"]:
    raise SystemExit("deferred rows drift")
if scope.get("rust_projector_runtime_dependency_removed") != 0:
    raise SystemExit("runtime dependency removal must stay unclaimed")
if scope.get("rust_projector_oracle_only") != 1:
    raise SystemExit("rust oracle marker missing")
if scope.get("full_astnode_projector_retired") != 0:
    raise SystemExit("full ASTNode projector retirement must stay unclaimed")

criteria = fixture.get("criteria") or {}
expected_criteria = {
    "programjson_stmt_only_block_recipe_snapshot_parity_gate": "Green",
    "programjson_runtime_parity_green": 1,
    "programjson_route_traverses_programjson": 1,
    "recipe_root_traversal_used": 1,
    "stmt_only_reducer_called": 1,
    "programjson_route_uses_string_only_facade": 0,
    "covered_row_count": 4,
    "recipe_bodies_materialization": 0,
    "full_recipe_matcher_execution": 0,
    "route_selection": 0,
    "mir_mutation": 0,
    "id_allocation": 0,
    "backend_lowering": 0,
}
for key, expected in expected_criteria.items():
    if criteria.get(key) != expected:
        raise SystemExit(f"criteria drift: {key}")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

decision = fixture.get("decision") or {}
if decision.get("kind") != "RetireCandidateScoped":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != "MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-NEXT-CAPABILITY-SELECTION-001":
    raise SystemExit("bad selected next card")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-stmt-only-block-recipe-snapshot-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-SNAPSHOT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate=ProgramJsonStmtOnlyBlockRecipeSnapshotV1
shape_scope=covered ProgramJSON stmt-only block recipe direct stmt rows
covered_rows=4
deferred_rows=if_no_exit,loop_no_exit,no_exit_block,exit_allowed_block
decision=RetireCandidateScoped
programjson_stmt_only_block_recipe_snapshot_parity_gate=green
programjson_runtime_parity_green=1
recipe_root_traversal_used=1
stmt_only_reducer_called=1
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
recipe_bodies_materialization=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering=0
source_selfhost_claim=0
hako_adopted_decision=0
programjson_full_parser_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-NEXT-CAPABILITY-SELECTION-001
summary=ok
REPORT
