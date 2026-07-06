#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-post-verifier-boundary-design-stop-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-post-verifier-boundary-design-stop-v0.json"
PREV_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-verifier-boundary-retire-rust-astnode-projector-candidate-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3217-MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-DESIGN-STOP-001.md"
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

token = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-DESIGN-STOP-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-CONSULTATION-001"

if fixture.get("kind") != "MirBuilderProgramJsonRecipeBodiesPostVerifierBoundaryDesignStopV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
if prev.get("claims", {}).get("retire_candidate_recorded") != 1:
    raise SystemExit("previous retire candidate not recorded")

boundary = fixture.get("boundary") or {}
if boundary.get("kind") != "DesignStop":
    raise SystemExit("boundary must be DesignStop")
if boundary.get("implementation_allowed_now") != 0:
    raise SystemExit("implementation must remain stopped")
if "RecipeMatcher" not in boundary.get("next_policy_boundary", ""):
    raise SystemExit("next policy boundary must name RecipeMatcher")

consultation = fixture.get("consultation") or {}
if consultation.get("selected_next_card") != next_card:
    raise SystemExit("bad selected next consultation card")
if not consultation.get("question"):
    raise SystemExit("consultation question missing")

states = {row.get("id"): row.get("state") for row in fixture.get("candidate_next_seams") or []}
if states.get("A_MORE_DTO_COVERAGE_ROWS") != "AllowedAfterConsultation":
    raise SystemExit("DTO coverage option state drift")
if states.get("B_RUNTIME_RECIPEBODIES_PUBLICATION_BRIDGE") != "RequiresConsultation":
    raise SystemExit("runtime publication option must require consultation")
if states.get("C_FULL_RECIPEMATCHER_EXECUTION") != "RequiresConsultation":
    raise SystemExit("RecipeMatcher option must require consultation")

for key, value in (fixture.get("forbidden_without_new_decision") or {}).items():
    if value != 1:
        raise SystemExit(f"forbidden flag drift: {key}")

claims = fixture.get("claims") or {}
if claims.get("design_stop_recorded") != 1:
    raise SystemExit("design stop claim missing")
for key, value in claims.items():
    if key == "design_stop_recorded":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [token, next_card, "implementation_allowed_now=0"]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
if "3217 records the post-verifier RecipeBodies design stop" not in task_order:
    raise SystemExit("task-order 3217 marker missing")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-post-verifier-boundary-design-stop-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-DESIGN-STOP-001
boundary=RecipeBodiesPostVerifierBoundaryDesignStop
implementation_allowed_now=0
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-CONSULTATION-001
option_a_more_dto_rows=AllowedAfterConsultation
option_b_runtime_publication_bridge=RequiresConsultation
option_c_full_recipematcher_execution=RequiresConsultation
recipe_bodies_materialization=0
runtime_recipe_bodies_arena=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
runtime_route_switch=0
source_selfhost_claim=0
summary=ok
REPORT
