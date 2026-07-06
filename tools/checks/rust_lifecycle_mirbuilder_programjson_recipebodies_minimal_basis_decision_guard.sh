#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-minimal-basis-decision-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-minimal-basis-decision-v0.json"
CONSULT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-minimal-basis-consultation-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3195-MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-BASIS-DECISION-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CONSULT" "$CARD" "$TASK_ORDER"

python3 - "$FIXTURE" "$CONSULT" "$CARD" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, consult_path, card_path, task_order_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
consult = json.loads(Path(consult_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-BASIS-DECISION-001"
selected = "A_DTO_ONLY_STMT_ONLY_BODYID_STMTREF_SNAPSHOT"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-SNAPSHOT-SELECTION-001"

if fixture.get("kind") != "MirBuilderProgramJsonRecipeBodiesMinimalBasisDecisionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
if fixture.get("input_state", {}).get("selected_option_id") != selected:
    raise SystemExit("selected option drift")
if consult.get("consultation_question", {}).get("recommended_option_id") != selected:
    raise SystemExit("decision must follow 3194 recommendation")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectDtoOnlySnapshotLocalBodyIdStmtRef":
    raise SystemExit("bad decision kind")
if decision.get("selected_first_slice") != next_card:
    raise SystemExit("bad selected first slice")
if decision.get("snapshot_kind") != "ProgramJsonRecipeBodiesMinimalDtoV1":
    raise SystemExit("bad snapshot kind")

contract = fixture.get("selected_contract") or {}
for key in ["body_id_contract", "stmt_ref_contract"]:
    if "snapshot-local token only" not in contract.get(key, ""):
        raise SystemExit(f"{key} must be snapshot-local")
if contract.get("output_is_runtime_arena") != 0:
    raise SystemExit("first slice must not be runtime arena")
if contract.get("requires_new_hako_syntax") != 0:
    raise SystemExit("first slice must not require new syntax")
if contract.get("requires_new_hako_library_api") != 0:
    raise SystemExit("first slice must not require new library API")

acceptance = fixture.get("first_slice_acceptance") or {}
for key in [
    "must_consume_programjson",
    "must_emit_bodyid_stmtref_tokens",
    "must_compare_against_rust_oracle",
    "must_name_non_claims_in_output",
]:
    if acceptance.get(key) != 1:
        raise SystemExit(f"acceptance missing: {key}")
if len(acceptance.get("minimum_rows") or []) < 3:
    raise SystemExit("minimum rows too small")

forbidden = fixture.get("forbidden_in_first_slice") or {}
for key, value in forbidden.items():
    if value != 1:
        raise SystemExit(f"forbidden flag drift: {key}")

claims = fixture.get("claims") or {}
if claims.get("consultation_decision_recorded") != 1:
    raise SystemExit("decision claim missing")
if claims.get("implementation_selected") != 1:
    raise SystemExit("implementation selection claim missing")
for key, value in claims.items():
    if key in {"consultation_decision_recorded", "implementation_selected"}:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [token, selected, next_card, "BodyId = snapshot-local token only"]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
if "selected next task:\n  " + next_card not in task_order:
    raise SystemExit("task-order next task drift")
if "3195 selects DTO-only snapshot-local BodyId/StmtRef" not in task_order:
    raise SystemExit("task-order 3195 marker missing")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-minimal-basis-decision-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-BASIS-DECISION-001
selected_option=A_DTO_ONLY_STMT_ONLY_BODYID_STMTREF_SNAPSHOT
selected_first_slice=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-SNAPSHOT-SELECTION-001
snapshot_kind=ProgramJsonRecipeBodiesMinimalDtoV1
implementation_selected=1
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
