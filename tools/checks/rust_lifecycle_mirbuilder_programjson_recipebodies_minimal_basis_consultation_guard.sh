#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-minimal-basis-consultation-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-minimal-basis-consultation-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3194-MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-BASIS-CONSULTATION-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
RECIPE_SSOT="$ROOT_DIR/docs/development/current/main/design/recipe-tree-and-parts-ssot.md"
RUST_BLOCK="$ROOT_DIR/src/mir/builder/control_flow/plan/recipe_tree/block.rs"
HAKO_ITEM="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_item_box.hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$RECIPE_SSOT" "$RUST_BLOCK" "$HAKO_ITEM"

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$RECIPE_SSOT" "$RUST_BLOCK" "$HAKO_ITEM" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, ssot_path, rust_block_path, hako_item_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
ssot = Path(ssot_path).read_text(encoding="utf-8")
rust_block = Path(rust_block_path).read_text(encoding="utf-8")
hako_item = Path(hako_item_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-BASIS-CONSULTATION-001"
recommended = "A_DTO_ONLY_STMT_ONLY_BODYID_STMTREF_SNAPSHOT"

if fixture.get("kind") != "MirBuilderProgramJsonRecipeBodiesMinimalBasisConsultationV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

question = fixture.get("consultation_question") or {}
if question.get("recommended_option_id") != recommended:
    raise SystemExit("recommended option drift")
if question.get("reason_token") != "StmtOnlyDtoProvesReferenceBoundaryWithoutRecursiveArenaOrLowering":
    raise SystemExit("reason token drift")

options = {row.get("id"): row for row in fixture.get("options") or []}
if options.get(recommended, {}).get("status") != "recommended":
    raise SystemExit("recommended option row missing")
if options[recommended].get("recipe_bodies_runtime_materialization") != 0:
    raise SystemExit("A must stay DTO-only")
if options[recommended].get("requires_new_hako_syntax") != 0:
    raise SystemExit("A must not require syntax")
if options[recommended].get("requires_new_hako_library_api") != 0:
    raise SystemExit("A must not require new library API")

slice_ = fixture.get("recommended_first_slice") or {}
if slice_.get("snapshot_kind") != "ProgramJsonRecipeBodiesMinimalDtoV1":
    raise SystemExit("bad snapshot kind")
if "snapshot-local token only" not in slice_.get("body_id_contract", ""):
    raise SystemExit("BodyId contract must be snapshot-local")
if "snapshot-local token only" not in slice_.get("stmt_ref_contract", ""):
    raise SystemExit("StmtRef contract must be snapshot-local")

for needle in [
    "pub(in crate::mir::builder) struct BodyId",
    "pub(in crate::mir::builder) struct RecipeBodies",
    "pub body_id: BodyId",
    "Stmt(StmtRef)",
]:
    if needle not in rust_block:
        raise SystemExit(f"Rust RecipeBodies boundary drift: {needle}")

for needle in [
    "**Storage**: `RecipeBodies/RecipeBody`",
    "`RecipeBody` は `BodyId`/`StmtRef`",
    "`RecipeBodies::bodies` の直接アクセス",
]:
    if needle not in ssot:
        raise SystemExit(f"Recipe SSOT drift: {needle}")

for needle in [
    "seq(items)",
    "if_item(cond_facts, then_item, else_item)",
    "loop_item(cond_facts, body_item)",
    "exit_item(exit_kind, payload)",
]:
    if needle not in hako_item:
        raise SystemExit(f"Hako RecipeItem vocabulary drift: {needle}")

forbidden = fixture.get("forbidden_without_new_decision") or {}
for key, value in forbidden.items():
    if value != 1:
        raise SystemExit(f"forbidden flag drift: {key}")

claims = fixture.get("claims") or {}
if claims.get("consultation_prepared") != 1:
    raise SystemExit("consultation must be prepared")
for key, value in claims.items():
    if key == "consultation_prepared":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

if token not in card:
    raise SystemExit("card token missing")
if recommended not in card:
    raise SystemExit("card recommended option missing")
if "implementation_selected=0" not in card:
    raise SystemExit("card must keep implementation unselected")
if "selected next task:\n  MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-BASIS-DECISION-001" not in task_order:
    raise SystemExit("task-order must wait for RecipeBodies decision")
if "3194 prepares the RecipeBodies minimal-basis consultation" not in task_order:
    raise SystemExit("task-order 3194 marker missing")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-minimal-basis-consultation-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-BASIS-CONSULTATION-001
consultation_prepared=1
recommended_option=A_DTO_ONLY_STMT_ONLY_BODYID_STMTREF_SNAPSHOT
recommended_first_slice=ProgramJsonRecipeBodiesMinimalDtoV1
implementation_selected=0
recipe_bodies_materialization=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering=0
runtime_route_switch=0
source_selfhost_claim=0
summary=ok
REPORT
