#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-post-verifier-boundary-decision-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-post-verifier-boundary-decision-v0.json"
PREV_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-post-verifier-boundary-consultation-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3219-MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-DECISION-001.md"
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

token = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-DECISION-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-EXPANDED-DTO-COVERAGE-PARITY-001"

if fixture.get("kind") != "MirBuilderProgramJsonRecipeBodiesPostVerifierBoundaryDecisionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
if prev.get("claims", {}).get("consultation_prepared") != 1:
    raise SystemExit("previous consultation not prepared")
if prev.get("consultation", {}).get("recommended_option") != "A_MORE_DTO_COVERAGE_ROWS":
    raise SystemExit("previous recommended option drift")

decision = fixture.get("decision") or {}
if decision.get("selected_option") != "A_MORE_DTO_COVERAGE_ROWS":
    raise SystemExit("bad selected option")
if decision.get("selected_next_card") != next_card:
    raise SystemExit("bad next card")

acceptance = fixture.get("acceptance_for_next_card") or {}
for key, value in acceptance.items():
    if value != 1:
        raise SystemExit(f"acceptance flag drift: {key}")

blocked = fixture.get("still_requires_new_decision") or {}
for key, value in blocked.items():
    if value != 1:
        raise SystemExit(f"new-decision boundary drift: {key}")

claims = fixture.get("claims") or {}
for key in ["decision_recorded", "implementation_selected"]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {"decision_recorded", "implementation_selected"}:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [token, next_card, "A_MORE_DTO_COVERAGE_ROWS"]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
if "3219 selects more DTO coverage rows" not in task_order:
    raise SystemExit("task-order 3219 marker missing")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-post-verifier-boundary-decision-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-DECISION-001
selected_option=A_MORE_DTO_COVERAGE_ROWS
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-EXPANDED-DTO-COVERAGE-PARITY-001
implementation_selected=1
runtime_recipe_bodies_publication=0
full_recipe_matcher_execution=0
route_selection=0
runtime_route_switch=0
source_selfhost_claim=0
summary=ok
REPORT
