#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-recipe-stmt-seq-dto-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-recipe-stmt-seq-dto-retire-rust-astnode-projector-candidate-v0.json"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_recipe_stmt_seq_dto_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PARITY_GATE"

PARITY_OUT="$(guard_cached_run "$TAG" bash "$PARITY_GATE")"
if ! grep -q '^runtime_parity_green=1$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "Recipe stmt-seq DTO runtime parity is not green"
fi
if ! grep -q '^mir_json_route_green=1$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "Recipe stmt-seq DTO MIR JSON route is not green"
fi
if ! grep -q '^recipe_verifier_used=1$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "RecipeVerifier was not used"
fi
if ! grep -q '^recipe_stmt_seq_scanner_used=1$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "Recipe stmt-seq scanner was not used"
fi
if ! grep -q '^shape_kind_selection=0$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "shape_kind selection must stay unclaimed"
fi

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if fixture.get("kind") != "MirBuilderProgramJsonLayer4RecipeStmtSeqDtoRustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-STMT-SEQ-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001":
    raise SystemExit("bad fixture token")

expected_hashes = {
    "hako_snapshot_source_hash": "sha256:d624d4c53af92c417ff725dc122cdce675044929247c1df6da71291cc2e41b34",
    "recipe_seq_source_hash": "sha256:565e273c30397c64226796ec744f5f88dc73e28e5fb60828da80c99b991d324d",
    "parity_fixture_hash": "sha256:436beb8a3dc0f27ece85195165bc9ae2e08a2b1e4e91dbfe0cf77601f4ab4c66",
    "parity_gate_hash": "sha256:ad39f16ef20de81b2536ee90435563fb9421c31f3b8fb7d57fd715ec44bfe4cd",
}
evidence = fixture.get("evidence") or {}
for key, expected in expected_hashes.items():
    if evidence.get(key) != expected:
        raise SystemExit(f"hash drift: {key}")

scope = fixture.get("retire_candidate_scope") or {}
if scope.get("retire_candidate") != "RecipeStmtSeqDtoSnapshotV1":
    raise SystemExit("bad retire candidate")
if scope.get("shape_scope") != "covered ProgramJSON Layer4 Recipe stmt-seq DTO rows":
    raise SystemExit("bad shape scope")
if scope.get("programjson_snapshot_owner") != "ProgramJsonRecipeStmtSeqDtoSnapshotV1":
    raise SystemExit("bad snapshot owner")
expected_rows = [
    "return_int",
    "return_new_box",
    "local_return_var",
    "local_assignment_int_return_var",
    "local_assignment_add_return_var",
    "local_print_var_return_int",
    "local_print_binary_return_int",
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
    "programjson_snapshot_parity_gate": "Green",
    "programjson_runtime_parity_green": 1,
    "programjson_route_traverses_programjson": 1,
    "structured_recipe_dto_constructed": 1,
    "recipe_verifier_used": 1,
    "recipe_stmt_seq_scanner_used": 1,
    "shape_kind_selection": 0,
    "programjson_route_uses_string_only_facade": 0,
    "covered_row_count": 7,
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
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-recipe-stmt-seq-dto-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-STMT-SEQ-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate=RecipeStmtSeqDtoSnapshotV1
shape_scope=covered ProgramJSON Layer4 Recipe stmt-seq DTO rows
covered_rows=7
deferred_rows=
decision=RetireCandidateScoped
parity_gate=green
programjson_runtime_parity_green=1
recipe_verifier_used=1
recipe_stmt_seq_scanner_used=1
shape_kind_selection=0
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
