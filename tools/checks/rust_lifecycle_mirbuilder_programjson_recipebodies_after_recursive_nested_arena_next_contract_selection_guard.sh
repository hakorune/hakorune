#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-after-recursive-nested-arena-next-contract-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-after-recursive-nested-arena-next-contract-selection-v0.json"
PREV_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-recursive-nested-body-arena-retire-rust-astnode-projector-candidate-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3214-MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-RECURSIVE-NESTED-ARENA-NEXT-CONTRACT-SELECTION-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PREV_FIXTURE" "$CARD" "$TASK_ORDER"

python3 - "$FIXTURE" "$PREV_FIXTURE" "$CARD" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, prev_path, card_path, task_order_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
prev = json.loads(Path(prev_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-RECURSIVE-NESTED-ARENA-NEXT-CONTRACT-SELECTION-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-PARITY-001"

if fixture.get("kind") != "MirBuilderProgramJsonRecipeBodiesAfterRecursiveNestedArenaNextContractSelectionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
if fixture.get("input_state", {}).get("previous_snapshot_kind") != "ProgramJsonRecipeBodiesRecursiveNestedArenaBuilderV1":
    raise SystemExit("previous snapshot drift")
if prev.get("claims", {}).get("retire_candidate_recorded") != 1:
    raise SystemExit("previous retire-candidate not recorded")

states = {row.get("id"): row.get("state") for row in fixture.get("candidate_contracts") or []}
if states.get("A_RECIPEBODIES_VERIFIER_BOUNDARY_PARITY") != "SelectedNext":
    raise SystemExit("RecipeBodies verifier boundary must be selected")
if states.get("B_RUNTIME_RECIPEBODIES_PUBLICATION") != "Forbidden":
    raise SystemExit("runtime RecipeBodies publication must stay forbidden")
if states.get("C_FULL_RECIPEMATCHER_EXECUTION") != "Forbidden":
    raise SystemExit("full RecipeMatcher execution must stay forbidden")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectRecipeBodiesVerifierBoundaryParity":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != next_card:
    raise SystemExit("bad selected next card")
if "RecipeVerifierBox.verify/2" not in decision.get("boundary_scope", ""):
    raise SystemExit("boundary scope must name RecipeVerifierBox.verify/2")

acceptance = fixture.get("acceptance_for_next_card") or {}
for key in [
    "must_consume_recursive_nested_arena_dto",
    "must_call_existing_recipe_verifier_boundary",
    "must_prove_result_map_output",
    "must_keep_programjson_builder_policy_free",
    "must_keep_runtime_recipe_bodies_publication_zero",
]:
    if acceptance.get(key) != 1:
        raise SystemExit(f"acceptance missing: {key}")
if acceptance.get("minimum_rows") != ["local_loop_body_if_branch_return"]:
    raise SystemExit("minimum rows drift")

for key, value in (fixture.get("forbidden_in_next_card") or {}).items():
    if value != 1:
        raise SystemExit(f"forbidden flag drift: {key}")

claims = fixture.get("claims") or {}
if claims.get("next_contract_selected") != 1:
    raise SystemExit("next contract claim missing")
for key, value in claims.items():
    if key == "next_contract_selected":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [token, next_card, "A_RECIPEBODIES_VERIFIER_BOUNDARY_PARITY"]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
if "3214 selects RecipeBodies verifier boundary parity" not in task_order:
    raise SystemExit("task-order 3214 marker missing")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-after-recursive-nested-arena-next-contract-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-RECURSIVE-NESTED-ARENA-NEXT-CONTRACT-SELECTION-001
selected_option=A_RECIPEBODIES_VERIFIER_BOUNDARY_PARITY
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-PARITY-001
minimum_rows=local_loop_body_if_branch_return
recipe_bodies_verifier_boundary_implemented=0
recipe_bodies_materialization=0
runtime_recipe_bodies_arena=0
full_recipe_matcher_execution=0
verifier_policy_reimplementation=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
runtime_route_switch=0
source_selfhost_claim=0
summary=ok
REPORT
