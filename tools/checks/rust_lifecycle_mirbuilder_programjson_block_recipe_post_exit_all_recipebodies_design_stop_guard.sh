#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-block-recipe-post-exit-all-recipebodies-design-stop-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-block-recipe-post-exit-all-recipebodies-design-stop-v0.json"
RECIPE_SSOT="$ROOT_DIR/docs/development/current/main/design/recipe-tree-and-parts-ssot.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
HAKO_LIB_DIR="$ROOT_DIR/lang/src/compiler/lib"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$RECIPE_SSOT" "$TASK_ORDER"

python3 - "$FIXTURE" "$RECIPE_SSOT" "$TASK_ORDER" "$HAKO_LIB_DIR" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, ssot_path, task_order_path, hako_lib_dir = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
ssot = Path(ssot_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
hako_text = "\n".join(p.read_text(encoding="utf-8") for p in Path(hako_lib_dir).glob("*block_recipe.hako"))

if fixture.get("kind") != "MirBuilderProgramJsonBlockRecipePostExitAllRecipeBodiesDesignStopV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-POST-EXIT-ALL-RECIPEBODIES-DESIGN-STOP-001":
    raise SystemExit("bad fixture token")

boundary = fixture.get("boundary") or {}
if boundary.get("kind") != "DesignStop":
    raise SystemExit("boundary must be DesignStop")
if boundary.get("implementation_allowed_now") != 0:
    raise SystemExit("implementation must remain stopped")
if "RecipeBlock" not in boundary.get("recipe_tree_public_boundary", ""):
    raise SystemExit("RecipeBlock public boundary missing")
if "RecipeBodies/RecipeBody" not in boundary.get("recipe_bodies_internal_boundary", ""):
    raise SystemExit("RecipeBodies internal boundary missing")

for needle in [
    "RecipeBlock { body_id, items: Vec<RecipeItem> }",
    "Storage: RecipeBody",
    "`RecipeBody` は `BodyId`/`StmtRef`",
    "RecipeBodies::bodies",
]:
    if needle not in ssot:
        raise SystemExit(f"recipe SSOT drift: {needle}")

for row in [
    "IfThenLocalNoElse",
    "IfThenLocalElsePrint",
    "IfThenReturnNoElse",
    "IfThenLocalElseReturn",
    "IfThenReturnElseLocal",
    "IfThenReturnElseBreak",
    "LoopExitAllowedBody",
]:
    if row not in fixture.get("input_state", {}).get("covered_flat_block_rows", []):
        raise SystemExit(f"covered flat row missing: {row}")
    if row not in hako_text:
        raise SystemExit(f"flat row reducer token missing from .hako libs: {row}")

next_consultation = fixture.get("next_consultation") or {}
if next_consultation.get("selected_next_card") != "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-BASIS-CONSULTATION-001":
    raise SystemExit("bad next consultation card")
if next_consultation.get("recommended_first_slice_if_approved") != "ProgramJsonRecipeBodiesMinimalDtoV1":
    raise SystemExit("bad recommended first slice")

allowed = fixture.get("allowed_after_consultation") or {}
for key in [
    "recipebodies_minimal_dto",
    "body_id_stmt_ref_snapshot",
    "one_shape_arena_builder_parity",
    "verifier_contract_fixture",
]:
    if allowed.get(key) != 1:
        raise SystemExit(f"missing allowed post-consultation item: {key}")

forbidden = fixture.get("forbidden_without_new_decision") or {}
for key, value in forbidden.items():
    if value != 1:
        raise SystemExit(f"forbidden flag drift: {key}")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

if "selected next task:\n  MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-BASIS-CONSULTATION-001" not in task_order:
    raise SystemExit("task-order next task is not the RecipeBodies consultation")
if "RecipeBodies design stop reached" not in task_order:
    raise SystemExit("task-order design stop marker missing")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-block-recipe-post-exit-all-recipebodies-design-stop-guard-v0
token=MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-POST-EXIT-ALL-RECIPEBODIES-DESIGN-STOP-001
boundary=RecipeBodiesDesignStop
implementation_allowed_now=0
covered_flat_block_rows=7
next_consultation=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-BASIS-CONSULTATION-001
recommended_first_slice_if_approved=ProgramJsonRecipeBodiesMinimalDtoV1
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
