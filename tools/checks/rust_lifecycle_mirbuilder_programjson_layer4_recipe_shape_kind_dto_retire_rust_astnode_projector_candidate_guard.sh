#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-recipe-shape-kind-dto-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-recipe-shape-kind-dto-retire-rust-astnode-projector-candidate-v0.json"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_recipe_shape_kind_dto_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PARITY_GATE"

PARITY_OUT="$(guard_cached_run "$TAG" bash "$PARITY_GATE")"
if ! grep -q '^runtime_parity_green=1$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "Recipe shape-kind DTO runtime parity is not green"
fi
if ! grep -q '^mir_json_route_green=1$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "Recipe shape-kind DTO MIR JSON route is not green"
fi
if ! grep -q '^shape_kind_selection=1$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "shape_kind selection must be covered"
fi
if ! grep -q '^route_selection=0$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "route selection must stay unclaimed"
fi

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if fixture.get("kind") != "MirBuilderProgramJsonLayer4RecipeShapeKindDtoRustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001":
    raise SystemExit("bad fixture token")

expected_hashes = {
    "hako_snapshot_source_hash": "sha256:02e7dc303f65412daf36022732bf694dde4b7c742f845ab2721c6eb957f7085b",
    "recipe_seq_source_hash": "sha256:c8f19d7f99a7b84b2ec0197f8964a66c3e5ac3beb44c64fdb9d5dea7e8957ec6",
    "parity_fixture_hash": "sha256:ca828c2bafdd30e74d47cac98465b26f79ab857447c9af983f6d83556cd175bb",
    "parity_gate_hash": "sha256:2dc67225dbbb6b58e126b453c5001405a7859c47e95649395852333e356cc285",
}
evidence = fixture.get("evidence") or {}
for key, expected in expected_hashes.items():
    if evidence.get(key) != expected:
        raise SystemExit(f"hash drift: {key}")

scope = fixture.get("retire_candidate_scope") or {}
if scope.get("retire_candidate") != "RecipeShapeKindDtoSnapshotV1":
    raise SystemExit("bad retire candidate")
if scope.get("shape_scope") != "covered ProgramJSON Layer4 Recipe shape-kind DTO rows":
    raise SystemExit("bad shape scope")
if scope.get("programjson_snapshot_owner") != "ProgramJsonRecipeShapeKindDtoSnapshotV1":
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
    "shape_kind_selection": 1,
    "route_selection": 0,
    "programjson_route_uses_string_only_facade": 0,
    "covered_row_count": 7,
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
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-recipe-shape-kind-dto-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate=RecipeShapeKindDtoSnapshotV1
shape_scope=covered ProgramJSON Layer4 Recipe shape-kind DTO rows
covered_rows=7
deferred_rows=
decision=RetireCandidateScoped
parity_gate=green
programjson_runtime_parity_green=1
shape_kind_selection=1
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
