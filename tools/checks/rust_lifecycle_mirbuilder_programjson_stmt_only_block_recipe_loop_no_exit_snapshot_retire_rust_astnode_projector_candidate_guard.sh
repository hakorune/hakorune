#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-stmt-only-block-recipe-loop-no-exit-snapshot-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-stmt-only-block-recipe-loop-no-exit-snapshot-retire-rust-astnode-projector-candidate-v0.json"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_stmt_only_block_recipe_loop_no_exit_snapshot_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PARITY_GATE"

PARITY_OUT="$(guard_cached_run "$TAG" bash "$PARITY_GATE")"
for required in \
  '^runtime_parity_green=1$' \
  '^mir_json_route_green=1$' \
  '^row_count=1$' \
  '^recipe_root_traversal_used=1$' \
  '^stmt_only_reducer_called=1$' \
  '^loop_no_exit_token_projected=1$' \
  '^prebuilt_token_snapshot_input=0$' \
  '^string_only_facade=0$'
do
  if ! grep -q "$required" <<<"$PARITY_OUT"; then
    printf '%s\n' "$PARITY_OUT" >&2
    guard_fail "$TAG" "ProgramJSON LoopNoExit prerequisite drift: $required"
  fi
done

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if fixture.get("kind") != "MirBuilderProgramJsonStmtOnlyBlockRecipeLoopNoExitSnapshotRustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-LOOP-NO-EXIT-SNAPSHOT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001":
    raise SystemExit("bad fixture token")

expected_hashes = {
    "hako_snapshot_source_hash": "sha256:ff5d79ab0376f0e71c66d2daeea5b90098c2ebbf5cda15c8458f87a4e5e663d7",
    "target_reducer_source_hash": "sha256:09bd9913e8cc9b3ecfb70b6f83973ed666d160d050b85e81e24f983ddbf80e3a",
    "parity_fixture_hash": "sha256:29e98cee0920b889cd5c8c67d70b055509240caebc4c974efc71e4b78ab48bfd",
    "parity_gate_hash": "sha256:ffd15a4dc957f026af4092b114bdbd782aec600b85625bad9b4530ffa9240892",
}
evidence = fixture.get("evidence") or {}
for key, expected in expected_hashes.items():
    if evidence.get(key) != expected:
        raise SystemExit(f"hash drift: {key}")

scope = fixture.get("retire_candidate_scope") or {}
if scope.get("retire_candidate") != "ProgramJsonStmtOnlyBlockRecipeLoopNoExitSnapshotV1":
    raise SystemExit("bad retire candidate")
if scope.get("shape_scope") != "covered ProgramJSON stmt-only block recipe LoopNoExit row":
    raise SystemExit("bad shape scope")
if scope.get("programjson_snapshot_owner") != "ProgramJsonStmtOnlyBlockRecipeSnapshotV1":
    raise SystemExit("bad snapshot owner")
if scope.get("covered_rows") != ["local_loop_no_exit"]:
    raise SystemExit("covered rows drift")
if scope.get("deferred_rows") != ["if_no_exit", "then_local_no_else_if", "no_exit_block", "exit_allowed_block"]:
    raise SystemExit("deferred rows drift")
if scope.get("rust_projector_runtime_dependency_removed") != 0:
    raise SystemExit("runtime dependency removal must stay unclaimed")
if scope.get("rust_projector_oracle_only") != 1:
    raise SystemExit("rust oracle marker missing")
if scope.get("full_astnode_projector_retired") != 0:
    raise SystemExit("full ASTNode projector retirement must stay unclaimed")

criteria = fixture.get("criteria") or {}
expected_criteria = {
    "programjson_loop_no_exit_snapshot_parity_gate": "Green",
    "programjson_runtime_parity_green": 1,
    "programjson_route_traverses_programjson": 1,
    "recipe_root_traversal_used": 1,
    "stmt_only_reducer_called": 1,
    "loop_no_exit_token_projected": 1,
    "programjson_route_uses_string_only_facade": 0,
    "covered_row_count": 1,
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
if decision.get("selected_next_card") != "MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-IF-NO-EXIT-SNAPSHOT-PARITY-001":
    raise SystemExit("bad selected next card")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-stmt-only-block-recipe-loop-no-exit-snapshot-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-LOOP-NO-EXIT-SNAPSHOT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate=ProgramJsonStmtOnlyBlockRecipeLoopNoExitSnapshotV1
shape_scope=covered ProgramJSON stmt-only block recipe LoopNoExit row
covered_rows=1
deferred_rows=if_no_exit,then_local_no_else_if,no_exit_block,exit_allowed_block
decision=RetireCandidateScoped
programjson_loop_no_exit_snapshot_parity_gate=green
programjson_runtime_parity_green=1
recipe_root_traversal_used=1
stmt_only_reducer_called=1
loop_no_exit_token_projected=1
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
selected_next_card=MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-IF-NO-EXIT-SNAPSHOT-PARITY-001
summary=ok
REPORT
