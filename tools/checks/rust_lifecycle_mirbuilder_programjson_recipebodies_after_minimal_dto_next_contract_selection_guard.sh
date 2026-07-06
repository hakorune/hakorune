#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-after-minimal-dto-next-contract-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-after-minimal-dto-next-contract-selection-v0.json"
PREV_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-minimal-dto-retire-rust-astnode-projector-candidate-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3199-MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-MINIMAL-DTO-NEXT-CONTRACT-SELECTION-001.md"
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

token = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-MINIMAL-DTO-NEXT-CONTRACT-SELECTION-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-ONE-SHAPE-ARENA-BUILDER-PARITY-001"
rows = [
    "empty_stmt_only_body",
    "single_local_stmt_body",
    "local_then_print_stmt_body",
]

if fixture.get("kind") != "MirBuilderProgramJsonRecipeBodiesAfterMinimalDtoNextContractSelectionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
if fixture.get("input_state", {}).get("minimal_dto_snapshot_kind") != "ProgramJsonRecipeBodiesMinimalDtoV1":
    raise SystemExit("minimal DTO snapshot kind drift")
if fixture.get("input_state", {}).get("covered_rows") != rows:
    raise SystemExit("input covered rows drift")
if prev.get("retire_candidate", {}).get("covered_rows") != rows:
    raise SystemExit("previous retire-candidate rows drift")
if prev.get("claims", {}).get("retire_candidate_recorded") != 1:
    raise SystemExit("previous retire-candidate not recorded")

states = {row.get("id"): row.get("state") for row in fixture.get("candidate_contracts") or []}
if states.get("A_DTO_ONLY_STMT_ONLY_BODYID_STMTREF_SNAPSHOT") != "CompletedAsRetireCandidate":
    raise SystemExit("A state drift")
if states.get("B_ONE_SHAPE_ARENA_BUILDER_PARITY") != "SelectedNext":
    raise SystemExit("B must be selected next")
if states.get("C_VERIFIER_FIRST_BOUNDARY") != "Deferred":
    raise SystemExit("C must remain deferred")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectOneShapeArenaBuilderParity":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != next_card:
    raise SystemExit("bad selected next card")
if decision.get("selected_shape") != "StmtOnlyLocalOrPrintMinimal":
    raise SystemExit("bad selected shape")
if "snapshot-local BodyId=0" not in decision.get("arena_scope", ""):
    raise SystemExit("arena scope must remain snapshot-local")

acceptance = fixture.get("acceptance_for_next_card") or {}
for key in [
    "must_consume_programjson",
    "must_build_structured_result_map",
    "must_expose_root_body_id",
    "must_expose_body_item_refs",
    "must_compare_against_minimal_dto_oracle",
    "must_keep_runtime_route_switch_zero",
]:
    if acceptance.get(key) != 1:
        raise SystemExit(f"acceptance missing: {key}")
if acceptance.get("minimum_rows") != rows:
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

for needle in [token, next_card, "B_ONE_SHAPE_ARENA_BUILDER_PARITY"]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
if "3199 selects one-shape RecipeBodies arena-builder parity" not in task_order:
    raise SystemExit("task-order 3199 marker missing")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-after-minimal-dto-next-contract-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-MINIMAL-DTO-NEXT-CONTRACT-SELECTION-001
selected_option=B_ONE_SHAPE_ARENA_BUILDER_PARITY
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-ONE-SHAPE-ARENA-BUILDER-PARITY-001
covered_rows=empty_stmt_only_body,single_local_stmt_body,local_then_print_stmt_body
one_shape_arena_builder_implemented=0
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
