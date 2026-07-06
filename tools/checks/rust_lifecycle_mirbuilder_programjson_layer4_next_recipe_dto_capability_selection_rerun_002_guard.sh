#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-next-recipe-dto-capability-selection-rerun-002-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-next-recipe-dto-capability-selection-rerun-002-v0.json"
PREVIOUS_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_seq_recipe_dto_loop_root_retire_rust_astnode_projector_candidate_guard.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PREVIOUS_GUARD"

PREVIOUS_OUT="$(bash "$PREVIOUS_GUARD")"
for required in \
  '^summary=ok$' \
  '^retire_candidate=SeqRecipeDtoSnapshotV1$' \
  '^covered_rows=6$' \
  '^loop_root_children_supported=1$' \
  '^shape_kind_included=1$' \
  '^route_selection=0$'
do
  if ! grep -q "$required" <<<"$PREVIOUS_OUT"; then
    printf '%s\n' "$PREVIOUS_OUT" >&2
    guard_fail "$TAG" "previous Seq loop-root retire-candidate drift: $required"
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
need(fixture.get("kind") == "MirBuilderProgramJsonLayer4NextRecipeDtoCapabilitySelectionRerun002V1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-LAYER4-NEXT-RECIPE-DTO-CAPABILITY-SELECTION-RERUN-002", "bad token")

state = fixture.get("input_state") or {}
for key in ["previous_guard", "task_order"]:
    path = root / (state.get(key) or "")
    need(path.exists(), f"missing input path: {key}")

policy = fixture.get("selection_policy") or {}
need(policy.get("capability_batch_required") is True, "capability batch policy missing")
need(policy.get("source_code_line_cap") == 800, "bad line cap")
for forbidden in ["MIR mutation", "backend lowering", "ID allocation", "route selection", "full RecipeMatcher execution"]:
    need(forbidden in (policy.get("layer4_not_scope") or []), f"missing forbidden scope: {forbidden}")

candidates = fixture.get("candidates") or []
selected = [row for row in candidates if row.get("decision") == "selected"]
need(len(selected) == 1, "expected exactly one selected candidate")
need(selected[0].get("name") == "ProgramJsonRecipePortSigLoopRootV1", "bad selected candidate")
owner = root / (selected[0].get("implementation_owner") or "")
need(owner.exists(), "missing implementation owner")
need(owner.read_text(encoding="utf-8").count("\n") + 1 <= 800, "source line cap exceeded")

cap = fixture.get("selected_capability") or {}
need(cap.get("name") == "ProgramJsonRecipePortSigLoopRootV1", "bad selected capability")
need(cap.get("expected_port_sig") == "def_count=1;update_count=2", "bad expected port sig")
need(len(cap.get("source_rows") or []) == 6, "source row count drift")
need((root / cap.get("source_fixture", "")).exists(), "missing source fixture")
need(cap.get("next_card") == "MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-PORT-SIG-DTO-LOOP-ROOT-PARITY-001", "bad next card")

acceptance = fixture.get("acceptance") or {}
for key in [
    "must_consume_programjson_structure",
    "must_construct_structured_recipe_dto",
    "must_use_recipe_verifier",
    "must_use_recipe_port_sig_snapshot",
    "parity_gate_required",
]:
    need(acceptance.get(key) == 1, f"missing acceptance: {key}")
need(acceptance.get("minimum_parity_row_count") == 6, "bad row count")
need(acceptance.get("implementation_card_required") == 0, "implementation card should not be required")
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
    decision.get("selected_next_card") == "MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-PORT-SIG-DTO-LOOP-ROOT-PARITY-001",
    "bad selected next card",
)

claims = fixture.get("claims") or {}
for key, value in claims.items():
    need(value == 0, f"forbidden claim drift: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-next-recipe-dto-capability-selection-rerun-002-guard-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-NEXT-RECIPE-DTO-CAPABILITY-SELECTION-RERUN-002
selected_capability=ProgramJsonRecipePortSigLoopRootV1
selected_next_card=MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-PORT-SIG-DTO-LOOP-ROOT-PARITY-001
source_rows=6
expected_port_sig=def_count=1;update_count=2
must_construct_structured_recipe_dto=1
must_use_recipe_verifier=1
must_use_recipe_port_sig_snapshot=1
token_snapshot_only=0
string_only_facade=0
source_code_line_cap=800
implementation_done=0
parity_gate_green=0
source_selfhost_claim=0
summary=ok
REPORT
