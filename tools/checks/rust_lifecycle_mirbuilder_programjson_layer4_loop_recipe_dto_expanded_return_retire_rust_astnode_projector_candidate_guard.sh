#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-loop-recipe-dto-expanded-return-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-loop-recipe-dto-expanded-return-retire-rust-astnode-projector-candidate-v0.json"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_loop_recipe_dto_expanded_return_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PARITY_GATE"

PARITY_OUT="$(bash "$PARITY_GATE")"
for required in \
  '^runtime_parity_green=1$' \
  '^mir_json_route_green=1$' \
  '^expanded_rows=6$' \
  '^legacy_loop_parity_guard_still_green=1$' \
  '^expanded_if_payload_prerequisite_green=1$'
do
  if ! grep -q "$required" <<<"$PARITY_OUT"; then
    printf '%s\n' "$PARITY_OUT" >&2
    guard_fail "$TAG" "expanded Return Loop Recipe DTO prerequisite drift: $required"
  fi
done

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if fixture.get("kind") != "MirBuilderProgramJsonLayer4LoopRecipeDtoExpandedReturnRustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-EXPANDED-RETURN-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001":
    raise SystemExit("bad fixture token")

expected_hashes = {
    "hako_snapshot_source_hash": "sha256:55936df225b43256df55023d83c4ed486ab3f9e7a70b4d43b7935ef482327c68",
    "loop_handler_source_hash": "sha256:2bd4e9719078fcb547d9549beaf606efaf1aa1a99558267551e0aedf6cf2dc95",
    "parity_fixture_hash": "sha256:67a4ccac9172fbe57b5109656590c6dffc7e90febdf98602e94db499aa9b5589",
    "parity_gate_hash": "sha256:84ddd0a76a500acfc7ff43c14f163245f817843522d763be1388c118ad242c3c",
}
evidence = fixture.get("evidence") or {}
for key, expected in expected_hashes.items():
    if evidence.get(key) != expected:
        raise SystemExit(f"hash drift: {key}")

scope = fixture.get("retire_candidate_scope") or {}
if scope.get("retire_candidate") != "LoopRecipeDtoSnapshotV1":
    raise SystemExit("bad retire candidate")
if scope.get("shape_scope") != "expanded Return payload ProgramJSON Layer4 Loop Recipe DTO rows":
    raise SystemExit("bad shape scope")
if scope.get("programjson_snapshot_owner") != "ProgramJsonLoopRecipeDtoSnapshotV1":
    raise SystemExit("bad snapshot owner")
expected_rows = [
    "loop_if_then_return_new_stringbox_abc_assignment_final_return_var",
    "loop_if_then_return_call_id0_assignment_final_return_var",
    "loop_if_then_return_call_id1_int9_assignment_final_return_var",
    "loop_if_then_return_call_id1_int7_assignment_final_return_var",
    "loop_if_then_return_method_stringbox_length_abc_assignment_final_return_var",
    "loop_if_then_return_method_stringbox_indexof_b_abc_assignment_final_return_var",
]
if scope.get("covered_rows") != expected_rows:
    raise SystemExit("covered rows drift")
if scope.get("deferred_rows") != []:
    raise SystemExit("deferred rows drift")
if scope.get("rust_projector_runtime_dependency_removed") != 0:
    raise SystemExit("runtime dependency removal must stay unclaimed")
if scope.get("rust_projector_oracle_only") != 1:
    raise SystemExit("rust oracle marker missing")
if scope.get("full_astnode_projector_retired") != 0:
    raise SystemExit("full ASTNode projector retirement must stay unclaimed")

criteria = fixture.get("criteria") or {}
expected_criteria = {
    "expanded_return_loop_snapshot_parity_gate": "Green",
    "programjson_runtime_parity_green": 1,
    "programjson_route_traverses_programjson": 1,
    "structured_recipe_dto_constructed": 1,
    "covered_row_count": 6,
    "legacy_loop_parity_guard_still_green": 1,
    "expanded_if_payload_prerequisite_green": 1,
    "programjson_route_uses_string_only_facade": 0,
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
if decision.get("selected_next_card") != "MIRBUILDER-PROGRAMJSON-LAYER4-NEXT-RECIPE-DTO-CAPABILITY-SELECTION-001":
    raise SystemExit("bad selected next card")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-loop-recipe-dto-expanded-return-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-EXPANDED-RETURN-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate=LoopRecipeDtoSnapshotV1
shape_scope=expanded Return payload ProgramJSON Layer4 Loop Recipe DTO rows
covered_rows=6
deferred_rows=
decision=RetireCandidateScoped
expanded_return_loop_parity_gate=green
programjson_runtime_parity_green=1
legacy_loop_parity_guard_still_green=1
expanded_if_payload_prerequisite_green=1
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering=0
source_selfhost_claim=0
hako_adopted_decision=0
programjson_full_parser_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-LAYER4-NEXT-RECIPE-DTO-CAPABILITY-SELECTION-001
summary=ok
REPORT
