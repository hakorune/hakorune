#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-seq-recipe-dto-loop-root-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-seq-recipe-dto-loop-root-retire-rust-astnode-projector-candidate-v0.json"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_seq_recipe_dto_loop_root_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PARITY_GATE"

PARITY_OUT="$(guard_cached_run "$TAG" bash "$PARITY_GATE")"
for required in \
  '^runtime_parity_green=1$' \
  '^mir_json_route_green=1$' \
  '^expanded_rows=6$' \
  '^root_sequence_scanner_used=1$' \
  '^loop_root_children_supported=1$' \
  '^shape_kind_included=1$' \
  '^route_selection=0$'
do
  if ! grep -q "$required" <<<"$PARITY_OUT"; then
    printf '%s\n' "$PARITY_OUT" >&2
    guard_fail "$TAG" "Seq Recipe loop-root prerequisite drift: $required"
  fi
done

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if fixture.get("kind") != "MirBuilderProgramJsonLayer4SeqRecipeDtoLoopRootRustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-LAYER4-SEQ-RECIPE-DTO-LOOP-ROOT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001":
    raise SystemExit("bad fixture token")

expected_hashes = {
    "hako_snapshot_source_hash": "sha256:9229f9d5b29a64fec56b849c39931d041141d93c49834ff65d0f175f0313bc8a",
    "parity_fixture_hash": "sha256:1a2f9b6367194f4c0217fdefd5ec1c5e724482a2a8c90ba1edd268d5c1376314",
    "parity_gate_hash": "sha256:43e062d076132d15923723ad4af5d040df260786fad4cc54665d09849cd0b048",
}
evidence = fixture.get("evidence") or {}
for key, expected in expected_hashes.items():
    if evidence.get(key) != expected:
        raise SystemExit(f"hash drift: {key}")

scope = fixture.get("retire_candidate_scope") or {}
if scope.get("retire_candidate") != "SeqRecipeDtoSnapshotV1":
    raise SystemExit("bad retire candidate")
if scope.get("shape_scope") != "covered ProgramJSON Layer4 Seq Recipe DTO loop-root rows":
    raise SystemExit("bad shape scope")
if scope.get("programjson_snapshot_owner") != "ProgramJsonSeqRecipeDtoSnapshotV1":
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
    "seq_recipe_loop_root_parity_gate": "Green",
    "programjson_runtime_parity_green": 1,
    "programjson_route_traverses_programjson": 1,
    "structured_recipe_dto_constructed": 1,
    "root_sequence_scanner_used": 1,
    "loop_root_children_supported": 1,
    "shape_kind_included": 1,
    "route_selection": 0,
    "programjson_route_uses_string_only_facade": 0,
    "covered_row_count": 6,
    "full_recipe_matcher_execution": 0,
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
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-seq-recipe-dto-loop-root-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-SEQ-RECIPE-DTO-LOOP-ROOT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate=SeqRecipeDtoSnapshotV1
shape_scope=covered ProgramJSON Layer4 Seq Recipe DTO loop-root rows
covered_rows=6
deferred_rows=
decision=RetireCandidateScoped
seq_recipe_loop_root_parity_gate=green
programjson_runtime_parity_green=1
root_sequence_scanner_used=1
loop_root_children_supported=1
shape_kind_included=1
route_selection=0
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
full_recipe_matcher_execution=0
mir_mutation=0
id_allocation=0
backend_lowering=0
source_selfhost_claim=0
hako_adopted_decision=0
programjson_full_parser_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-LAYER4-NEXT-RECIPE-DTO-CAPABILITY-SELECTION-001
summary=ok
REPORT
