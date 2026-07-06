#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-no-exit-block-recipe-loop-v0-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-no-exit-block-recipe-loop-v0-retire-rust-astnode-projector-candidate-v0.json"
CAPABILITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_no_exit_block_recipe_loop_v0_snapshot_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CAPABILITY_GATE"

CAPABILITY_OUT="$(guard_cached_run "$TAG" bash "$CAPABILITY_GATE")"
for required in \
  '^runtime_parity_green=1$' \
  '^mir_json_route_green=1$' \
  '^row_count=1$' \
  '^recipe_root_traversal_used=1$' \
  '^no_exit_reducer_called=1$' \
  '^loop_v0_token_projected=1$' \
  '^exit_allowed_body_shape_observed=1$' \
  '^join_then_else_contract=0$' \
  '^additional_exit_allowed_if_modes=0$'
do
  if ! grep -q "$required" <<<"$CAPABILITY_OUT"; then
    printf '%s\n' "$CAPABILITY_OUT" >&2
    guard_fail "$TAG" "LoopV0 capability prerequisite drift: $required"
  fi
done

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if fixture.get("kind") != "MirBuilderProgramJsonNoExitBlockRecipeLoopV0RustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-LOOP-V0-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001":
    raise SystemExit("bad fixture token")

scope = fixture.get("retire_candidate_scope") or {}
if scope.get("retire_candidate") != "ProgramJsonNoExitBlockRecipeLoopV0SnapshotV1":
    raise SystemExit("bad retire candidate")
if scope.get("covered_rows") != ["local_loop_if_then_return_assignment_final_return_var"]:
    raise SystemExit("covered rows drift")
if scope.get("deferred_rows") != ["join_then_else", "additional_exit_allowed_if_modes", "recipe_bodies_materialization"]:
    raise SystemExit("deferred rows drift")
if scope.get("rust_projector_runtime_dependency_removed") != 0:
    raise SystemExit("runtime dependency removal must stay unclaimed")
if scope.get("rust_projector_oracle_only") != 1:
    raise SystemExit("rust oracle marker missing")
if scope.get("full_astnode_projector_retired") != 0:
    raise SystemExit("full ASTNode projector retirement must stay unclaimed")

criteria = fixture.get("criteria") or {}
expected = {
    "programjson_loop_v0_snapshot_parity_gate": "Green",
    "programjson_runtime_parity_green": 1,
    "recipe_root_traversal_used": 1,
    "no_exit_reducer_called": 1,
    "loop_v0_token_projected": 1,
    "exit_allowed_body_shape_observed": 1,
    "covered_row_count": 1,
    "join_then_else_contract": 0,
    "additional_exit_allowed_if_modes": 0,
    "recipe_bodies_materialization": 0,
    "full_recipe_matcher_execution": 0,
    "route_selection": 0,
    "mir_mutation": 0,
    "id_allocation": 0,
    "backend_lowering": 0,
}
for key, value in expected.items():
    if criteria.get(key) != value:
        raise SystemExit(f"criteria drift: {key}")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

decision = fixture.get("decision") or {}
if decision.get("kind") != "RetireCandidateScoped":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != "MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-POST-LOOP-V0-NEXT-CONTRACT-SELECTION-001":
    raise SystemExit("bad selected next card")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-no-exit-block-recipe-loop-v0-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-LOOP-V0-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate=ProgramJsonNoExitBlockRecipeLoopV0SnapshotV1
shape_scope=parseable ProgramJSON Loop.body If.then Return + Assignment row through NoExitBlockRecipeBox
covered_rows=1
deferred_rows=join_then_else,additional_exit_allowed_if_modes,recipe_bodies_materialization
decision=RetireCandidateScoped
programjson_loop_v0_snapshot_parity_gate=green
programjson_runtime_parity_green=1
recipe_root_traversal_used=1
no_exit_reducer_called=1
loop_v0_token_projected=1
exit_allowed_body_shape_observed=1
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
join_then_else_contract=0
additional_exit_allowed_if_modes=0
recipe_bodies_materialization=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering=0
source_selfhost_claim=0
hako_adopted_decision=0
programjson_full_parser_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-POST-LOOP-V0-NEXT-CONTRACT-SELECTION-001
summary=ok
REPORT
