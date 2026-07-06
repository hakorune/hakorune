#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-next-recipe-dto-capability-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-next-recipe-dto-capability-selection-v0.json"
PREVIOUS_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_recipe_stmt_seq_dto_loop_root_retire_rust_astnode_projector_candidate_guard.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PREVIOUS_GUARD"

PREVIOUS_OUT="$(bash "$PREVIOUS_GUARD")"
for required in \
  '^summary=ok$' \
  '^retire_candidate=RecipeStmtSeqDtoSnapshotV1$' \
  '^covered_rows=6$' \
  '^recipe_root_seq_scanner_used=1$' \
  '^loop_root_children_supported=1$'
do
  if ! grep -q "$required" <<<"$PREVIOUS_OUT"; then
    printf '%s\n' "$PREVIOUS_OUT" >&2
    guard_fail "$TAG" "previous loop-root stmt-seq retire-candidate drift: $required"
  fi
done

python3 - "$FIXTURE" "$ROOT_DIR" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
root = Path(sys.argv[2])


def need(condition, message):
    if not condition:
        raise SystemExit(message)


need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonLayer4NextRecipeDtoCapabilitySelectionV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-LAYER4-NEXT-RECIPE-DTO-CAPABILITY-SELECTION-001", "bad token")

state = fixture.get("input_state") or {}
for key in ["previous_guard", "task_order"]:
    path = root / (state.get(key) or "")
    need(path.exists(), f"missing input path: {key}")

policy = fixture.get("selection_policy") or {}
need(policy.get("capability_batch_required") is True, "capability batch policy missing")
need(policy.get("source_code_line_cap") == 800, "bad line cap")
need("ProgramJSON" in (policy.get("layer4_scope") or ""), "bad scope")
for forbidden in ["MIR mutation", "backend lowering", "ID allocation", "route selection", "full RecipeMatcher execution"]:
    need(forbidden in (policy.get("layer4_not_scope") or []), f"missing forbidden scope: {forbidden}")

candidates = fixture.get("candidates") or []
selected = [row for row in candidates if row.get("decision") == "selected"]
need(len(selected) == 1, "expected exactly one selected candidate")
need(selected[0].get("name") == "ProgramJsonRecipeShapeKindDtoLoopRootV1", "bad selected candidate")
for key in ["implementation_owner", "support_owner"]:
    path = root / (selected[0].get(key) or "")
    need(path.exists(), f"missing owner path: {key}")
    lines = path.read_text(encoding="utf-8").count("\n") + 1
    need(lines <= 800, f"source line cap exceeded: {path} has {lines} lines")

cap = fixture.get("selected_capability") or {}
need(cap.get("name") == "ProgramJsonRecipeShapeKindDtoLoopRootV1", "bad selected capability")
need(cap.get("input") == "ProgramJSON v0", "bad input")
need(cap.get("expected_shape_kind") == "phase21_local_loop_if_varltint_then_return_int_body_inc_return_var_or_int", "bad expected shape_kind")
need(len(cap.get("source_rows") or []) == 6, "source row count drift")
need((root / cap.get("source_fixture", "")).exists(), "missing source fixture")
need(cap.get("next_card") == "MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-LOOP-ROOT-PARITY-001", "bad next card")

acceptance = fixture.get("acceptance") or {}
for key in [
    "must_consume_programjson_structure",
    "must_construct_structured_recipe_dto",
    "must_use_recipe_verifier",
    "must_use_recipe_root_sequence_scanner",
    "must_select_shape_kind",
    "implementation_card_required",
    "parity_gate_required",
]:
    need(acceptance.get(key) == 1, f"missing acceptance: {key}")
need(acceptance.get("minimum_parity_row_count") == 6, "bad row count")
need(acceptance.get("token_snapshot_only") == 0, "token snapshot only forbidden")
need(acceptance.get("string_only_facade") == 0, "string-only facade forbidden")

stops = fixture.get("stop_conditions") or {}
for key in [
    "prebuilt_token_snapshot_input",
    "source_contains_or_regex_proof",
    "rust_astnode_projector_used_as_target_input",
    "mir_mutation_or_lowering_added",
    "route_selection_added",
    "recipe_matcher_execution_added",
    "unsupported_shape_silently_ignored",
]:
    need(stops.get(key) == 1, f"missing stop condition: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectLayer4RecipeDtoCapability", "bad decision kind")
need(
    decision.get("selected_next_card") == "MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-LOOP-ROOT-PARITY-001",
    "bad selected next card",
)

claims = fixture.get("claims") or {}
for key, value in claims.items():
    need(value == 0, f"forbidden claim drift: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-next-recipe-dto-capability-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-NEXT-RECIPE-DTO-CAPABILITY-SELECTION-001
selected_capability=ProgramJsonRecipeShapeKindDtoLoopRootV1
selected_next_card=MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-LOOP-ROOT-PARITY-001
source_rows=6
expected_shape_kind=phase21_local_loop_if_varltint_then_return_int_body_inc_return_var_or_int
must_construct_structured_recipe_dto=1
must_use_recipe_verifier=1
must_use_recipe_root_sequence_scanner=1
must_select_shape_kind=1
token_snapshot_only=0
string_only_facade=0
source_code_line_cap=800
implementation_done=0
parity_gate_green=0
source_selfhost_claim=0
summary=ok
REPORT
