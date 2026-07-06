#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-if-handler-then-local-no-else-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-if-handler-then-local-no-else-retire-rust-astnode-projector-candidate-v0.json"
CAPABILITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_if_handler_then_local_no_else_capability_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CAPABILITY_GATE"

CAPABILITY_OUT="$(guard_cached_run "$TAG" bash "$CAPABILITY_GATE")"
for required in \
  '^runtime_parity_green=1$' \
  '^mir_json_route_green=1$' \
  '^row_count=1$' \
  '^if_handler_then_local_no_else_supported=1$' \
  '^recipe_root_traversal_used=1$' \
  '^stmt_only_reducer_called=1$' \
  '^if_no_exit_token_projected=1$' \
  '^prebuilt_token_snapshot_input=0$' \
  '^string_only_facade=0$'
do
  if ! grep -q "$required" <<<"$CAPABILITY_OUT"; then
    printf '%s\n' "$CAPABILITY_OUT" >&2
    guard_fail "$TAG" "then-local/no-else capability prerequisite drift: $required"
  fi
done

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if fixture.get("kind") != "MirBuilderProgramJsonIfHandlerThenLocalNoElseRustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-IF-HANDLER-THEN-LOCAL-NO-ELSE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001":
    raise SystemExit("bad fixture token")

expected_hashes = {
    "if_handler_source_hash": "sha256:2200fe485925e6adb4c2432b2b930d7a2aaa59a6510b7e060ff56943ddc508a5",
    "hako_snapshot_source_hash": "sha256:559cf51fb400c1250310931560f0d8e5b8e24b11e6ecf0c84d2a29ee8198aa6e",
    "target_reducer_source_hash": "sha256:09bd9913e8cc9b3ecfb70b6f83973ed666d160d050b85e81e24f983ddbf80e3a",
    "capability_fixture_hash": "sha256:850d0d6755e43e1d7a967d081a029613d4dbf6b3b5573f8eac53bc05c729454c",
    "capability_gate_hash": "sha256:5fd6901d961a5324a358baa6e4c2a60195974bdd43131c4df2290222948e7e5a",
}
evidence = fixture.get("evidence") or {}
for key, expected in expected_hashes.items():
    if evidence.get(key) != expected:
        raise SystemExit(f"hash drift: {key}")

scope = fixture.get("retire_candidate_scope") or {}
if scope.get("retire_candidate") != "ProgramJsonIfHandlerThenLocalNoElseV1":
    raise SystemExit("bad retire candidate")
if scope.get("shape_scope") != "covered ProgramJSON If handler then-local/no-else row":
    raise SystemExit("bad shape scope")
if scope.get("covered_rows") != ["local_if_then_local_no_else"]:
    raise SystemExit("covered rows drift")
if scope.get("deferred_rows") != ["no_exit_block", "exit_allowed_block"]:
    raise SystemExit("deferred rows drift")
if scope.get("rust_projector_runtime_dependency_removed") != 0:
    raise SystemExit("runtime dependency removal must stay unclaimed")
if scope.get("rust_projector_oracle_only") != 1:
    raise SystemExit("rust oracle marker missing")
if scope.get("full_astnode_projector_retired") != 0:
    raise SystemExit("full ASTNode projector retirement must stay unclaimed")

criteria = fixture.get("criteria") or {}
expected_criteria = {
    "programjson_if_handler_then_local_no_else_capability_gate": "Green",
    "programjson_runtime_parity_green": 1,
    "programjson_route_traverses_programjson": 1,
    "recipe_root_traversal_used": 1,
    "stmt_only_reducer_called": 1,
    "if_no_exit_token_projected": 1,
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
if decision.get("selected_next_card") != "MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-RECURSIVE-CONTRACT-CAPABILITY-SELECTION-001":
    raise SystemExit("bad selected next card")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-if-handler-then-local-no-else-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-IF-HANDLER-THEN-LOCAL-NO-ELSE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate=ProgramJsonIfHandlerThenLocalNoElseV1
shape_scope=covered ProgramJSON If handler then-local/no-else row
covered_rows=1
deferred_rows=no_exit_block,exit_allowed_block
decision=RetireCandidateScoped
programjson_if_handler_then_local_no_else_capability_gate=green
programjson_runtime_parity_green=1
recipe_root_traversal_used=1
stmt_only_reducer_called=1
if_no_exit_token_projected=1
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
selected_next_card=MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-RECURSIVE-CONTRACT-CAPABILITY-SELECTION-001
summary=ok
REPORT
