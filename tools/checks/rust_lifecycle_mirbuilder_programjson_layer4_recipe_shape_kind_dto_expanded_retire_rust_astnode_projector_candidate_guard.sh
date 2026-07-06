#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-recipe-shape-kind-dto-expanded-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-recipe-shape-kind-dto-expanded-retire-rust-astnode-projector-candidate-v0.json"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_recipe_shape_kind_dto_expanded_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PARITY_GATE"

PARITY_OUT="$(guard_cached_run "$TAG" bash "$PARITY_GATE")"
if ! grep -q '^runtime_parity_green=1$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "expanded Recipe shape-kind DTO runtime parity is not green"
fi
if ! grep -q '^mir_json_route_green=1$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "expanded Recipe shape-kind DTO MIR JSON route is not green"
fi
if ! grep -q '^expanded_rows=6$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "expanded row count must stay 6"
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
if fixture.get("kind") != "MirBuilderProgramJsonLayer4RecipeShapeKindDtoExpandedRustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-EXPANDED-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001":
    raise SystemExit("bad fixture token")

expected_hashes = {
    "hako_snapshot_source_hash": "sha256:5fb9fb790689fba3b4d70f5ae6ab190c7c425bfcbc41a9c70cb242d6f6903ad5",
    "return_handler_source_hash": "sha256:b88a19b5d016d28cac5e958b7d7b0afaa26f9c94fc3a12896cf4ed0c20bc54cb",
    "parity_fixture_hash": "sha256:5db1f4e8d405ec64aa2738168688bc0f39096c7fc3d5fadd00f555edb9822497",
    "parity_gate_hash": "sha256:224a18a3dac65efbd43d80b266a8716171e9b3d5856eedb0c4281329040a6669",
}
evidence = fixture.get("evidence") or {}
for key, expected in expected_hashes.items():
    if evidence.get(key) != expected:
        raise SystemExit(f"hash drift: {key}")

scope = fixture.get("retire_candidate_scope") or {}
if scope.get("retire_candidate") != "RecipeShapeKindDtoSnapshotV1":
    raise SystemExit("bad retire candidate")
if scope.get("shape_scope") != "expanded ProgramJSON Layer4 Recipe shape-kind DTO Return-only rows":
    raise SystemExit("bad shape scope")
if scope.get("programjson_snapshot_owner") != "ProgramJsonRecipeShapeKindDtoSnapshotV1":
    raise SystemExit("bad snapshot owner")
expected_rows = [
    "return_new_stringbox_abc",
    "return_call_id0",
    "return_call_id1_int9",
    "return_call_id1_int7",
    "return_method_stringbox_length_abc",
    "return_method_stringbox_indexof_b_abc",
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
    "return_handler_dynamic_token_stable": 1,
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
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-recipe-shape-kind-dto-expanded-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-EXPANDED-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate=RecipeShapeKindDtoSnapshotV1
shape_scope=expanded ProgramJSON Layer4 Recipe shape-kind DTO Return-only rows
covered_rows=6
deferred_rows=
decision=RetireCandidateScoped
parity_gate=green
programjson_runtime_parity_green=1
shape_kind_selection=1
route_selection=0
return_handler_dynamic_token_stable=1
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
