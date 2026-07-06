#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-post-verifier-boundary-consultation-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-post-verifier-boundary-consultation-v0.json"
PREV_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-post-verifier-boundary-design-stop-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3218-MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-CONSULTATION-001.md"
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

token = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-CONSULTATION-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-DECISION-001"

if fixture.get("kind") != "MirBuilderProgramJsonRecipeBodiesPostVerifierBoundaryConsultationV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
if prev.get("claims", {}).get("design_stop_recorded") != 1:
    raise SystemExit("previous design stop not recorded")

consultation = fixture.get("consultation") or {}
if consultation.get("recommended_option") != "A_MORE_DTO_COVERAGE_ROWS":
    raise SystemExit("recommended option drift")
if not consultation.get("question"):
    raise SystemExit("consultation question missing")

options = {row.get("id"): row for row in fixture.get("options") or []}
if options.get("A_MORE_DTO_COVERAGE_ROWS", {}).get("state") != "Recommended":
    raise SystemExit("option A must be recommended")
if options.get("B_RUNTIME_RECIPEBODIES_PUBLICATION_BRIDGE", {}).get("state") != "RequiresDecision":
    raise SystemExit("option B must require decision")
if options.get("C_FULL_RECIPEMATCHER_EXECUTION", {}).get("state") != "RequiresDecision":
    raise SystemExit("option C must require decision")
for required in ["runtime_publication_shape", "bridge_owner", "removal_condition"]:
    if required not in options["B_RUNTIME_RECIPEBODIES_PUBLICATION_BRIDGE"].get("required_contract", []):
        raise SystemExit(f"option B missing required contract: {required}")
for required in ["matched_contract_kind_surface", "failure_freeze_behavior", "route_selection_boundary"]:
    if required not in options["C_FULL_RECIPEMATCHER_EXECUTION"].get("required_contract", []):
        raise SystemExit(f"option C missing required contract: {required}")

decision = fixture.get("decision") or {}
if decision.get("selected_next_card") != next_card:
    raise SystemExit("bad next card")
if decision.get("implementation_selected") != 0:
    raise SystemExit("consultation must not select implementation")

claims = fixture.get("claims") or {}
if claims.get("consultation_prepared") != 1:
    raise SystemExit("consultation_prepared claim missing")
for key, value in claims.items():
    if key == "consultation_prepared":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [token, next_card, "A_MORE_DTO_COVERAGE_ROWS"]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
if "3218 prepares the post-verifier RecipeBodies consultation" not in task_order:
    raise SystemExit("task-order 3218 marker missing")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-post-verifier-boundary-consultation-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-CONSULTATION-001
consultation_prepared=1
recommended_option=A_MORE_DTO_COVERAGE_ROWS
implementation_selected=0
runtime_recipe_bodies_publication=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
runtime_route_switch=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-DECISION-001
summary=ok
REPORT
