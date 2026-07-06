#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-after-one-shape-arena-next-contract-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-after-one-shape-arena-next-contract-selection-v0.json"
PREV_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-one-shape-arena-builder-retire-rust-astnode-projector-candidate-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3202-MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-ONE-SHAPE-ARENA-NEXT-CONTRACT-SELECTION-001.md"
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

token = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-ONE-SHAPE-ARENA-NEXT-CONTRACT-SELECTION-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-IF-BRANCH-MULTI-BODY-ARENA-PARITY-001"

if fixture.get("kind") != "MirBuilderProgramJsonRecipeBodiesAfterOneShapeArenaNextContractSelectionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
if fixture.get("input_state", {}).get("previous_snapshot_kind") != "ProgramJsonRecipeBodiesOneShapeArenaBuilderV1":
    raise SystemExit("previous snapshot drift")
if prev.get("claims", {}).get("retire_candidate_recorded") != 1:
    raise SystemExit("previous retire-candidate not recorded")

states = {row.get("id"): row.get("state") for row in fixture.get("candidate_contracts") or []}
if states.get("A_IF_BRANCH_MULTI_BODY_ARENA_PARITY") != "SelectedNext":
    raise SystemExit("If branch multi-body arena must be selected")
if states.get("B_LOOP_BODY_MULTI_BODY_ARENA_PARITY") != "Deferred":
    raise SystemExit("Loop body arena must remain deferred")
if states.get("C_VERIFIER_FIRST_BOUNDARY") != "Deferred":
    raise SystemExit("Verifier-first must remain deferred")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectIfBranchMultiBodyArenaParity":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != next_card:
    raise SystemExit("bad selected next card")
if "then_body_id=1" not in decision.get("arena_scope", ""):
    raise SystemExit("then body id scope missing")
if "else_body_id=2" not in decision.get("arena_scope", ""):
    raise SystemExit("else body id scope missing")

acceptance = fixture.get("acceptance_for_next_card") or {}
for key in [
    "must_consume_programjson",
    "must_build_structured_result_map",
    "must_expose_root_body_id",
    "must_expose_if_then_else_body_ids",
    "must_emit_three_bodies",
    "must_keep_runtime_route_switch_zero",
]:
    if acceptance.get(key) != 1:
        raise SystemExit(f"acceptance missing: {key}")
if acceptance.get("minimum_rows") != ["local_if_then_local_else_print_return"]:
    raise SystemExit("minimum rows drift")

forbidden = fixture.get("forbidden_in_next_card") or {}
for key, value in forbidden.items():
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

for needle in [token, next_card, "A_IF_BRANCH_MULTI_BODY_ARENA_PARITY"]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
if "3202 selects If branch multi-body RecipeBodies arena parity" not in task_order:
    raise SystemExit("task-order 3202 marker missing")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-after-one-shape-arena-next-contract-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-ONE-SHAPE-ARENA-NEXT-CONTRACT-SELECTION-001
selected_option=A_IF_BRANCH_MULTI_BODY_ARENA_PARITY
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-IF-BRANCH-MULTI-BODY-ARENA-PARITY-001
minimum_rows=local_if_then_local_else_print_return
if_branch_multi_body_arena_implemented=0
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
