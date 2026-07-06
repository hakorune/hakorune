#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-block-recipe-recursive-contract-capability-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-block-recipe-recursive-contract-capability-selection-v0.json"
NO_EXIT="$ROOT_DIR/lang/src/compiler/lib/no_exit_block_recipe.hako"
EXIT_ALLOWED="$ROOT_DIR/lang/src/compiler/lib/exit_allowed_block_recipe.hako"
STMT_ONLY_SNAPSHOT="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_stmt_only_block_recipe_snapshot.hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$NO_EXIT" "$EXIT_ALLOWED" "$STMT_ONLY_SNAPSHOT"

python3 - "$FIXTURE" "$NO_EXIT" "$EXIT_ALLOWED" "$STMT_ONLY_SNAPSHOT" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, no_exit_path, exit_allowed_path, snapshot_path = map(Path, sys.argv[1:])
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
no_exit = no_exit_path.read_text(encoding="utf-8")
exit_allowed = exit_allowed_path.read_text(encoding="utf-8")
snapshot = snapshot_path.read_text(encoding="utf-8")

if fixture.get("kind") != "MirBuilderProgramJsonBlockRecipeRecursiveContractCapabilitySelectionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-RECURSIVE-CONTRACT-CAPABILITY-SELECTION-001":
    raise SystemExit("bad fixture token")

required_no_exit = [
    "box NoExitBlockRecipeBox",
    "IfThenLocalNoElse",
    "IfThenLocalElsePrint",
    "LoopExitAllowedBody",
    "return \"IfJoin\"",
    "return \"JoinNoElse\"",
    "return \"JoinThenElse\"",
]
for needle in required_no_exit:
    if needle not in no_exit:
        raise SystemExit(f"NoExit reducer drift: {needle}")

required_exit_allowed = [
    "box ExitAllowedBlockRecipeBox",
    "IfThenReturnElseBreak",
    "IfThenReturnNoElse",
    "IfThenReturnElseLocal",
    "IfThenLocalElseReturn",
    "Break",
    "Continue",
    "Return",
]
for needle in required_exit_allowed:
    if needle not in exit_allowed:
        raise SystemExit(f"ExitAllowed reducer drift: {needle}")

required_snapshot = [
    "box ProgramJsonStmtOnlyBlockRecipeSnapshotBox",
    "StmtOnlyBlockRecipeBox.build_summary",
    "IfNoExit",
    "LoopNoExit",
]
for needle in required_snapshot:
    if needle not in snapshot:
        raise SystemExit(f"StmtOnly snapshot drift: {needle}")

selected = fixture.get("selected_capability") or {}
if selected.get("name") != "ProgramJsonNoExitBlockRecipeIfJoinSnapshotV1":
    raise SystemExit("bad selected capability")
if selected.get("target_reducer") != "NoExitBlockRecipeBox":
    raise SystemExit("bad selected reducer")
if selected.get("selected_next_card") != "MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-IF-JOIN-SNAPSHOT-PARITY-001":
    raise SystemExit("bad next card")
if selected.get("required_projection_tokens") != ["IfThenLocalNoElse"]:
    raise SystemExit("projection token drift")
if selected.get("required_reducer_outputs") != ["IfJoin", "JoinNoElse"]:
    raise SystemExit("reducer output drift")

candidates = {row.get("capability"): row for row in fixture.get("candidates", [])}
if candidates.get("ProgramJsonNoExitBlockRecipeIfJoinSnapshotV1", {}).get("status") != "Selected":
    raise SystemExit("NoExit IfJoin candidate must be selected")
if candidates.get("ProgramJsonNoExitBlockRecipeJoinThenElseSnapshotV1", {}).get("status") != "Held":
    raise SystemExit("NoExit JoinThenElse candidate must be held")
if candidates.get("ProgramJsonExitAllowedBlockRecipeSnapshotV1", {}).get("status") != "Held":
    raise SystemExit("ExitAllowed candidate must be held")
if candidates.get("ProgramJsonNoExitBlockRecipeLoopV0SnapshotV1", {}).get("status") != "Held":
    raise SystemExit("LoopV0 candidate must be held")

stop = fixture.get("stop_conditions") or {}
claims = fixture.get("claims") or {}
for key, value in {**stop, **claims}.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectProgramJsonNoExitIfJoinBeforeExitAllowed":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != selected.get("selected_next_card"):
    raise SystemExit("decision next card mismatch")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-block-recipe-recursive-contract-capability-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-RECURSIVE-CONTRACT-CAPABILITY-SELECTION-001
selected_capability=ProgramJsonNoExitBlockRecipeIfJoinSnapshotV1
selected_reducer=NoExitBlockRecipeBox
held_capability=ProgramJsonExitAllowedBlockRecipeSnapshotV1
held_join_then_else_capability=ProgramJsonNoExitBlockRecipeJoinThenElseSnapshotV1
held_loop_capability=ProgramJsonNoExitBlockRecipeLoopV0SnapshotV1
selected_projection_tokens=IfThenLocalNoElse
selected_reducer_outputs=IfJoin,JoinNoElse
no_exit_if_join_rows=1
exit_allowed_held=1
join_then_else_held=1
loop_v0_held=1
programjson_no_exit_if_join_parity_green=0
recursive_recipe_bodies_materialization=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering=0
runtime_route_switch=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-IF-JOIN-SNAPSHOT-PARITY-001
summary=ok
REPORT
