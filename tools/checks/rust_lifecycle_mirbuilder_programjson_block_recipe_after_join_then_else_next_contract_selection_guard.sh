#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-block-recipe-after-join-then-else-next-contract-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-block-recipe-after-join-then-else-next-contract-selection-v0.json"
EXIT_ALLOWED="$ROOT_DIR/lang/src/compiler/lib/exit_allowed_block_recipe.hako"
IF_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/if_stmt_handler.hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$EXIT_ALLOWED" "$IF_HANDLER"

python3 - "$FIXTURE" "$EXIT_ALLOWED" "$IF_HANDLER" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, exit_allowed_path, if_handler_path = map(Path, sys.argv[1:])
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
exit_allowed = exit_allowed_path.read_text(encoding="utf-8")
if_handler = if_handler_path.read_text(encoding="utf-8")

if fixture.get("kind") != "MirBuilderProgramJsonBlockRecipeAfterJoinThenElseNextContractSelectionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-AFTER-JOIN-THEN-ELSE-NEXT-CONTRACT-SELECTION-001":
    raise SystemExit("bad fixture token")

for needle in [
    "IfThenLocalElseReturn",
    "return \"IfExitAllowed\"",
    "return \"ElseOnlyExit\"",
]:
    if needle not in exit_allowed:
        raise SystemExit(f"ExitAllowed reducer drift: {needle}")

for needle in [
    "_then_local_int_item",
    "_read_else_print_var_item",
    "_read_else_assignment_int",
]:
    if needle not in if_handler:
        raise SystemExit(f"If handler baseline drift: {needle}")

selected = fixture.get("selected_capability") or {}
if selected.get("name") != "ProgramJsonExitAllowedBlockRecipeElseOnlyExitSnapshotV1":
    raise SystemExit("bad selected capability")
if selected.get("target_reducer") != "ExitAllowedBlockRecipeBox":
    raise SystemExit("bad selected reducer")
if selected.get("required_programjson_producer") != "IfStmtHandler then-local/else-return recipe_root projection":
    raise SystemExit("producer boundary drift")
if selected.get("required_projection_tokens") != ["IfThenLocalElseReturn"]:
    raise SystemExit("projection token drift")
if selected.get("required_reducer_outputs") != ["IfExitAllowed", "ElseOnlyExit"]:
    raise SystemExit("reducer output drift")
if selected.get("selected_next_card") != "MIRBUILDER-PROGRAMJSON-EXIT-ALLOWED-BLOCK-RECIPE-ELSE-ONLY-EXIT-SNAPSHOT-PARITY-001":
    raise SystemExit("bad next card")

candidates = {row.get("capability"): row for row in fixture.get("candidates", [])}
if candidates.get("ProgramJsonExitAllowedBlockRecipeElseOnlyExitSnapshotV1", {}).get("status") != "Selected":
    raise SystemExit("ElseOnlyExit candidate must be selected")
if candidates.get("ProgramJsonExitAllowedBlockRecipeThenOnlyExitSnapshotV1", {}).get("status") != "Held":
    raise SystemExit("ThenOnlyExit candidate must be held")
if candidates.get("ProgramJsonExitAllowedBlockRecipeExitAllSnapshotV1", {}).get("status") != "Held":
    raise SystemExit("ExitAll candidate must be held")
if candidates.get("ProgramJsonRecipeBodiesMaterializationV1", {}).get("status") != "DesignStop":
    raise SystemExit("RecipeBodies must remain a design stop")

for key, value in {**(fixture.get("stop_conditions") or {}), **(fixture.get("claims") or {})}.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectProgramJsonExitAllowedElseOnlyExitAfterJoinThenElse":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != selected.get("selected_next_card"):
    raise SystemExit("decision next card mismatch")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-block-recipe-after-join-then-else-next-contract-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-AFTER-JOIN-THEN-ELSE-NEXT-CONTRACT-SELECTION-001
selected_capability=ProgramJsonExitAllowedBlockRecipeElseOnlyExitSnapshotV1
selected_reducer=ExitAllowedBlockRecipeBox
selected_projection_tokens=IfThenLocalElseReturn
selected_reducer_outputs=IfExitAllowed,ElseOnlyExit
held_then_only_exit_capability=ProgramJsonExitAllowedBlockRecipeThenOnlyExitSnapshotV1
held_exit_all_capability=ProgramJsonExitAllowedBlockRecipeExitAllSnapshotV1
recipe_bodies_design_stop=1
programjson_else_only_exit_parity_green=0
recursive_recipe_bodies_materialization=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering=0
runtime_route_switch=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-EXIT-ALLOWED-BLOCK-RECIPE-ELSE-ONLY-EXIT-SNAPSHOT-PARITY-001
summary=ok
REPORT
