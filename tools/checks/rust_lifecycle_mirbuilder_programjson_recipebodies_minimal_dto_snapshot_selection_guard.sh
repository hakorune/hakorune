#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-minimal-dto-snapshot-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-minimal-dto-snapshot-selection-v0.json"
DECISION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-minimal-basis-decision-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3196-MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-SNAPSHOT-SELECTION-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$DECISION" "$CARD" "$TASK_ORDER"

python3 - "$FIXTURE" "$DECISION" "$CARD" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, decision_path, card_path, task_order_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
decision_fixture = json.loads(Path(decision_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-SNAPSHOT-SELECTION-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-SNAPSHOT-PARITY-001"
owner = "ProgramJsonRecipeBodiesMinimalDtoSnapshotBox"
owner_path = "lang/src/compiler/mirbuilder/program_json_recipebodies_minimal_dto_snapshot.hako"

if fixture.get("kind") != "MirBuilderProgramJsonRecipeBodiesMinimalDtoSnapshotSelectionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
if decision_fixture.get("decision", {}).get("selected_first_slice") != token:
    raise SystemExit("selection must follow 3195 decision")

selected = fixture.get("selected_owner") or {}
if selected.get("name") != owner:
    raise SystemExit("bad selected owner")
if selected.get("path") != owner_path:
    raise SystemExit("bad selected owner path")
if "StmtOnly" not in selected.get("responsibility", ""):
    raise SystemExit("selected responsibility must stay StmtOnly")
if "StmtOnlyBlockRecipeBox" not in selected.get("does_not_reuse", []):
    raise SystemExit("must not reuse token reducer")

rows = fixture.get("selected_rows") or []
if [row.get("row_id") for row in rows] != [
    "empty_stmt_only_body",
    "single_local_stmt_body",
    "local_then_print_stmt_body",
]:
    raise SystemExit("selected rows drift")

contract = fixture.get("output_contract") or {}
if contract.get("snapshot_kind") != "ProgramJsonRecipeBodiesMinimalDtoV1":
    raise SystemExit("bad snapshot kind")
for key in ["body_id_contract", "stmt_ref_contract"]:
    if "snapshot-local token only" not in contract.get(key, ""):
        raise SystemExit(f"{key} must be snapshot-local")
for field in [
    "snapshot_kind",
    "err",
    "root_body_id",
    "body_count",
    "body0_item_count",
    "body0_items",
    "refs",
    "non_claims",
]:
    if field not in contract.get("required_fields", []):
        raise SystemExit(f"required field missing: {field}")

next_ = fixture.get("next_card") or {}
if next_.get("selected_next_card") != next_card:
    raise SystemExit("bad next card")
if next_.get("card_type") != "implementation + parity gate":
    raise SystemExit("bad next card type")

forbidden = fixture.get("forbidden_in_next_card") or {}
for key, value in forbidden.items():
    if value != 1:
        raise SystemExit(f"forbidden flag drift: {key}")

claims = fixture.get("claims") or {}
if claims.get("owner_selected") != 1:
    raise SystemExit("owner_selected claim missing")
for key, value in claims.items():
    if key == "owner_selected":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [token, owner, owner_path, next_card]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
if "3196 selects ProgramJsonRecipeBodiesMinimalDtoSnapshotBox" not in task_order:
    raise SystemExit("task-order 3196 marker missing")
if "ProgramJsonRecipeBodiesMinimalDtoV1" not in task_order:
    raise SystemExit("task-order RecipeBodies minimal DTO follow-up missing")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-minimal-dto-snapshot-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-SNAPSHOT-SELECTION-001
selected_owner=ProgramJsonRecipeBodiesMinimalDtoSnapshotBox
selected_owner_path=lang/src/compiler/mirbuilder/program_json_recipebodies_minimal_dto_snapshot.hako
snapshot_kind=ProgramJsonRecipeBodiesMinimalDtoV1
selected_rows=empty_stmt_only_body,single_local_stmt_body,local_then_print_stmt_body
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-SNAPSHOT-PARITY-001
recipe_bodies_materialization=0
runtime_recipe_bodies_arena=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering=0
runtime_route_switch=0
source_selfhost_claim=0
summary=ok
REPORT
