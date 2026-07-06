#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-block-recipe-next-contract-capability-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-block-recipe-next-contract-capability-selection-v0.json"
EXIT_ALLOWED="$ROOT_DIR/lang/src/compiler/lib/exit_allowed_block_recipe.hako"
NO_EXIT="$ROOT_DIR/lang/src/compiler/lib/no_exit_block_recipe.hako"
IF_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/if_stmt_handler.hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$EXIT_ALLOWED" "$NO_EXIT" "$IF_HANDLER"

python3 - "$FIXTURE" "$EXIT_ALLOWED" "$NO_EXIT" "$IF_HANDLER" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, exit_allowed_path, no_exit_path, if_handler_path = map(Path, sys.argv[1:])
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
exit_allowed = exit_allowed_path.read_text(encoding="utf-8")
no_exit = no_exit_path.read_text(encoding="utf-8")
if_handler = if_handler_path.read_text(encoding="utf-8")

if fixture.get("kind") != "MirBuilderProgramJsonBlockRecipeNextContractCapabilitySelectionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-NEXT-CONTRACT-CAPABILITY-SELECTION-001":
    raise SystemExit("bad fixture token")

for needle in [
    "box ExitAllowedBlockRecipeBox",
    "IfThenReturnNoElse",
    "return \"IfExitOnly\"",
    "return \"ExitIf\"",
]:
    if needle not in exit_allowed:
        raise SystemExit(f"ExitAllowed reducer drift: {needle}")

for needle in [
    "IfThenLocalElsePrint",
    "return \"JoinThenElse\"",
    "LoopExitAllowedBody",
]:
    if needle not in no_exit:
        raise SystemExit(f"NoExit reducer drift: {needle}")

for needle in [
    "RecipeItemBox.exit_item(\"Return\"",
    "RecipeItemBox.if_item(cond_facts, then_item, else_item)",
    "program_json.substring(else_start, else_start + 4) == \"null\"",
]:
    if needle not in if_handler:
        raise SystemExit(f"If handler producer drift: {needle}")

selected = fixture.get("selected_capability") or {}
if selected.get("name") != "ProgramJsonExitAllowedBlockRecipeIfExitOnlySnapshotV1":
    raise SystemExit("bad selected capability")
if selected.get("target_reducer") != "ExitAllowedBlockRecipeBox":
    raise SystemExit("bad selected reducer")
if selected.get("required_projection_tokens") != ["IfThenReturnNoElse"]:
    raise SystemExit("projection token drift")
if selected.get("required_reducer_outputs") != ["IfExitOnly", "ExitIf"]:
    raise SystemExit("reducer output drift")
if selected.get("selected_next_card") != "MIRBUILDER-PROGRAMJSON-EXIT-ALLOWED-BLOCK-RECIPE-IF-EXIT-ONLY-SNAPSHOT-PARITY-001":
    raise SystemExit("bad next card")

candidates = {row.get("capability"): row for row in fixture.get("candidates", [])}
if candidates.get("ProgramJsonExitAllowedBlockRecipeIfExitOnlySnapshotV1", {}).get("status") != "Selected":
    raise SystemExit("ExitAllowed IfExitOnly candidate must be selected")
if candidates.get("ProgramJsonNoExitBlockRecipeJoinThenElseSnapshotV1", {}).get("status") != "Held":
    raise SystemExit("JoinThenElse candidate must be held")
if candidates.get("ProgramJsonNoExitBlockRecipeLoopV0SnapshotV1", {}).get("status") != "Held":
    raise SystemExit("LoopV0 candidate must be held")

for key, value in {**(fixture.get("stop_conditions") or {}), **(fixture.get("claims") or {})}.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectProgramJsonExitAllowedIfExitOnlyBeforeJoinThenElseAndLoopV0":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != selected.get("selected_next_card"):
    raise SystemExit("decision next card mismatch")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-block-recipe-next-contract-capability-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-NEXT-CONTRACT-CAPABILITY-SELECTION-001
selected_capability=ProgramJsonExitAllowedBlockRecipeIfExitOnlySnapshotV1
selected_reducer=ExitAllowedBlockRecipeBox
selected_projection_tokens=IfThenReturnNoElse
selected_reducer_outputs=IfExitOnly,ExitIf
held_join_then_else_capability=ProgramJsonNoExitBlockRecipeJoinThenElseSnapshotV1
held_loop_capability=ProgramJsonNoExitBlockRecipeLoopV0SnapshotV1
join_then_else_held=1
loop_v0_held=1
programjson_exit_allowed_if_exit_only_parity_green=0
recursive_recipe_bodies_materialization=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering=0
runtime_route_switch=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-EXIT-ALLOWED-BLOCK-RECIPE-IF-EXIT-ONLY-SNAPSHOT-PARITY-001
summary=ok
REPORT
