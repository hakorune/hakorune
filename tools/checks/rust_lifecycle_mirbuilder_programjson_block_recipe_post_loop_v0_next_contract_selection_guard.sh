#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-block-recipe-post-loop-v0-next-contract-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-block-recipe-post-loop-v0-next-contract-selection-v0.json"
NO_EXIT="$ROOT_DIR/lang/src/compiler/lib/no_exit_block_recipe.hako"
IF_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/if_stmt_handler.hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$NO_EXIT" "$IF_HANDLER"

python3 - "$FIXTURE" "$NO_EXIT" "$IF_HANDLER" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, no_exit_path, if_handler_path = map(Path, sys.argv[1:])
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
no_exit = no_exit_path.read_text(encoding="utf-8")
if_handler = if_handler_path.read_text(encoding="utf-8")

if fixture.get("kind") != "MirBuilderProgramJsonBlockRecipePostLoopV0NextContractSelectionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-POST-LOOP-V0-NEXT-CONTRACT-SELECTION-001":
    raise SystemExit("bad fixture token")

for needle in [
    "IfThenLocalElsePrint",
    "return \"IfJoin\"",
    "return \"JoinThenElse\"",
]:
    if needle not in no_exit:
        raise SystemExit(f"NoExit reducer drift: {needle}")

for needle in [
    "_then_local_int_item",
    "_read_else_assignment_int",
]:
    if needle not in if_handler:
        raise SystemExit(f"If handler producer baseline drift: {needle}")

selected = fixture.get("selected_capability") or {}
if selected.get("name") != "ProgramJsonNoExitBlockRecipeJoinThenElseSnapshotV1":
    raise SystemExit("bad selected capability")
if selected.get("target_reducer") != "NoExitBlockRecipeBox":
    raise SystemExit("bad selected reducer")
if selected.get("required_programjson_producer") != "IfStmtHandler then-local/else-print recipe_root projection":
    raise SystemExit("producer boundary drift")
if selected.get("required_projection_tokens") != ["IfThenLocalElsePrint"]:
    raise SystemExit("projection token drift")
if selected.get("required_reducer_outputs") != ["IfJoin", "JoinThenElse"]:
    raise SystemExit("reducer output drift")
if selected.get("selected_next_card") != "MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-JOIN-THEN-ELSE-SNAPSHOT-PARITY-001":
    raise SystemExit("bad next card")

candidates = {row.get("capability"): row for row in fixture.get("candidates", [])}
if candidates.get("ProgramJsonNoExitBlockRecipeJoinThenElseSnapshotV1", {}).get("status") != "Selected":
    raise SystemExit("JoinThenElse candidate must be selected")
if candidates.get("ProgramJsonExitAllowedBlockRecipeThenElseModeSnapshotV1", {}).get("status") != "Held":
    raise SystemExit("additional ExitAllowed candidate must be held")

for key, value in {**(fixture.get("stop_conditions") or {}), **(fixture.get("claims") or {})}.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectProgramJsonNoExitJoinThenElseAfterLoopV0":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != selected.get("selected_next_card"):
    raise SystemExit("decision next card mismatch")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-block-recipe-post-loop-v0-next-contract-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-POST-LOOP-V0-NEXT-CONTRACT-SELECTION-001
selected_capability=ProgramJsonNoExitBlockRecipeJoinThenElseSnapshotV1
selected_reducer=NoExitBlockRecipeBox
selected_projection_tokens=IfThenLocalElsePrint
selected_reducer_outputs=IfJoin,JoinThenElse
held_additional_exit_allowed_capability=ProgramJsonExitAllowedBlockRecipeThenElseModeSnapshotV1
additional_exit_allowed_if_modes_held=1
programjson_join_then_else_parity_green=0
recursive_recipe_bodies_materialization=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering=0
runtime_route_switch=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-JOIN-THEN-ELSE-SNAPSHOT-PARITY-001
summary=ok
REPORT
