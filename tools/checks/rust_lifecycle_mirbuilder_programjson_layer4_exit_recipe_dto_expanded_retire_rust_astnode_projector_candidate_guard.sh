#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-exit-recipe-dto-expanded-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-exit-recipe-dto-expanded-retire-rust-astnode-projector-candidate-v0.json"
ROOT_PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_exit_recipe_dto_parity_gate.sh"
LOOP_PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_exit_recipe_dto_loop_body_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$ROOT_PARITY_GATE" "$LOOP_PARITY_GATE"

ROOT_OUT="$(bash "$ROOT_PARITY_GATE")"
if ! grep -q '^runtime_parity_green=1$' <<<"$ROOT_OUT"; then
  printf '%s\n' "$ROOT_OUT" >&2
  guard_fail "$TAG" "root Exit Recipe DTO runtime parity is not green"
fi
if ! grep -q '^mir_json_route_green=1$' <<<"$ROOT_OUT"; then
  printf '%s\n' "$ROOT_OUT" >&2
  guard_fail "$TAG" "root Exit Recipe DTO MIR JSON route is not green"
fi

LOOP_OUT="$(bash "$LOOP_PARITY_GATE")"
if ! grep -q '^runtime_parity_green=1$' <<<"$LOOP_OUT"; then
  printf '%s\n' "$LOOP_OUT" >&2
  guard_fail "$TAG" "loop-body Exit Recipe DTO runtime parity is not green"
fi
if ! grep -q '^loop_exit_dto_green=1$' <<<"$LOOP_OUT"; then
  printf '%s\n' "$LOOP_OUT" >&2
  guard_fail "$TAG" "loop-body Exit DTO row is not green"
fi

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if fixture.get("kind") != "MirBuilderProgramJsonLayer4ExitRecipeDtoExpandedRustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-LAYER4-EXIT-RECIPE-DTO-EXPANDED-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001":
    raise SystemExit("bad fixture token")

expected_hashes = {
    "hako_snapshot_source_hash": "sha256:80fc5b28aa8bc228440a3523c7e3d0370bee9f594c1567aae9cb5a3da74f0576",
    "loop_handler_source_hash": "sha256:98e98da2c0654eda3df45b3a4b36d4ead6af4562bf55d88eb96b1968c74428ef",
    "loop_body_parity_fixture_hash": "sha256:702a2ce51316aa499d0550374cef5a3a7b1c26726722520d996b17967ef7ac7b",
    "loop_body_parity_gate_hash": "sha256:55e9e80a12afa41107e5f44d13ad22a3ed004dda5955464431f690e5afb02e9f",
}
evidence = fixture.get("evidence") or {}
for key, expected in expected_hashes.items():
    if evidence.get(key) != expected:
        raise SystemExit(f"hash drift: {key}")

scope = fixture.get("retire_candidate_scope") or {}
if scope.get("retire_candidate") != "ExitRecipeDtoSnapshotV1":
    raise SystemExit("bad retire candidate")
if scope.get("shape_scope") != "expanded ProgramJSON Layer4 Exit Recipe DTO rows":
    raise SystemExit("bad shape scope")
if scope.get("programjson_snapshot_owner") != "ProgramJsonExitRecipeDtoSnapshotV1":
    raise SystemExit("bad snapshot owner")
expected_rows = [
    "local_if_then_return_int_final_return_int",
    "local_if_then_return_int_final_return_var",
    "local_if_then_else_assignment_no_exit_reject",
    "local_loop_if_then_return_int_assignment_final_return_var",
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
    "root_exit_snapshot_parity_gate": "Green",
    "loop_body_exit_snapshot_parity_gate": "Green",
    "programjson_runtime_parity_green": 1,
    "programjson_route_traverses_programjson": 1,
    "structured_recipe_dto_constructed": 1,
    "programjson_route_uses_string_only_facade": 0,
    "covered_row_count": 4,
    "loop_exit_dto_green": 1,
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
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-exit-recipe-dto-expanded-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-EXIT-RECIPE-DTO-EXPANDED-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate=ExitRecipeDtoSnapshotV1
shape_scope=expanded ProgramJSON Layer4 Exit Recipe DTO rows
covered_rows=4
deferred_rows=
decision=RetireCandidateScoped
root_exit_parity_gate=green
loop_body_exit_parity_gate=green
programjson_runtime_parity_green=1
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
