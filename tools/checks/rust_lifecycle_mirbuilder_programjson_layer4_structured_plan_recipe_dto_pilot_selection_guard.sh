#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-structured-plan-recipe-dto-pilot-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-structured-plan-recipe-dto-pilot-selection-v0.json"
PREVIOUS_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_trycatch_shape_retire_rust_astnode_projector_candidate_guard.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PREVIOUS_GUARD"

bash "$PREVIOUS_GUARD" >/dev/null

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
need(
    fixture.get("kind")
    == "MirBuilderProgramJsonLayer4StructuredPlanRecipeDtoPilotSelectionV1",
    "bad kind",
)
need(
    fixture.get("token")
    == "MIRBUILDER-PROGRAMJSON-LAYER4-STRUCTURED-PLAN-RECIPE-DTO-PILOT-SELECTION-001",
    "bad token",
)

state = fixture.get("input_state") or {}
for key in ["previous_guard", "recipe_tree_ssot", "task_order"]:
    path = root / (state.get(key) or "")
    need(path.exists(), f"missing input path: {key}")

policy = fixture.get("selection_policy") or {}
need(policy.get("stop_layer1_expansion_without_layer4_need") is True, "bad layer1 stop policy")
need(policy.get("source_code_line_cap") == 800, "bad source line cap")
need("Recipe DTO" in (policy.get("layer4_scope") or ""), "bad layer4 scope")
for forbidden in ["MIR mutation", "backend lowering", "ID allocation", "route selection"]:
    need(forbidden in (policy.get("layer4_not_scope") or []), f"missing not-scope: {forbidden}")

candidates = fixture.get("candidates") or []
selected = [c for c in candidates if c.get("decision") == "selected"]
need(len(selected) == 1, "expected exactly one selected candidate")
need(selected[0].get("name") == "ProgramJsonLoopRecipeDtoPilotV1", "bad selected candidate")
owners = selected[0].get("existing_hako_owners") or []
need(len(owners) >= 5, "missing existing hako owners")
for owner in owners:
    path = root / owner
    need(path.exists(), f"missing owner: {owner}")
    lines = path.read_text(encoding="utf-8").count("\n") + 1
    need(lines <= 800, f"source line cap exceeded: {owner} has {lines} lines")

pilot = fixture.get("selected_pilot") or {}
need(pilot.get("name") == "ProgramJsonLoopRecipeDtoPilotV1", "bad pilot")
need(pilot.get("input") == "ProgramJSON v0", "bad input")
need("RecipeItemBox" in (pilot.get("output") or ""), "bad output")
need("RecipeVerifierBox" in (pilot.get("target_existing_route") or ""), "verifier route missing")
need(len(pilot.get("minimum_rows") or []) >= 4, "minimum rows too small")
need(
    pilot.get("next_card") == "MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-PARITY-001",
    "bad next card",
)

acceptance = fixture.get("acceptance") or {}
for key in [
    "must_consume_programjson_structure",
    "must_construct_structured_recipe_dto",
    "must_use_recipe_verifier",
    "must_include_aot_or_exe_gate",
    "implementation_card_required",
    "parity_gate_required",
]:
    need(acceptance.get(key) == 1, f"missing acceptance: {key}")
need(acceptance.get("minimum_parity_row_count") >= 4, "row budget too small")
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
    "unverified_recipe_lowered",
    "unsupported_shape_silently_ignored",
]:
    need(stops.get(key) == 1, f"missing stop condition: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectLayer4StructuredRecipeDtoPilot", "bad decision kind")
need(
    decision.get("selected_next_card")
    == "MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-PARITY-001",
    "bad selected next card",
)

claims = fixture.get("claims") or {}
for key, value in claims.items():
    need(value == 0, f"forbidden claim drift: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-structured-plan-recipe-dto-pilot-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-STRUCTURED-PLAN-RECIPE-DTO-PILOT-SELECTION-001
selected_pilot=ProgramJsonLoopRecipeDtoPilotV1
selected_next_card=MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-PARITY-001
layer4_scope=programjson_to_structured_recipe_dto
must_construct_structured_recipe_dto=1
must_use_recipe_verifier=1
token_snapshot_only=0
string_only_facade=0
source_code_line_cap=800
implementation_done=0
parity_gate_green=0
source_selfhost_claim=0
summary=ok
REPORT
