#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-runtime-route-shadow-switch-design-stop-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-runtime-route-shadow-switch-design-stop-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3230-MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-ROUTE-SHADOW-SWITCH-DESIGN-STOP-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
SHADOW_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_shadow_parity_gate.sh"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$SHADOW_GATE" "$CURRENT_STATE"

SHADOW_OUT="$(guard_cached_run "$TAG" bash "$SHADOW_GATE")"
if ! grep -q '^recipe_matcher_shadow_parity=1$' <<<"$SHADOW_OUT"; then
  printf '%s\n' "$SHADOW_OUT" >&2
  guard_fail "$TAG" "RecipeMatcher shadow parity prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-ROUTE-SHADOW-SWITCH-DESIGN-STOP-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeBodiesRuntimeRouteShadowSwitchDesignStopV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

boundary = fixture.get("observed_boundary") or {}
if boundary.get("kind") != "RuntimeRouteSwitchAuthoritySelectionRequired":
    raise SystemExit("bad boundary kind")
if boundary.get("programjson_shadow_parity_green") is not True:
    raise SystemExit("shadow parity must be green")
if boundary.get("programjson_runtime_route_authority") is not False:
    raise SystemExit("ProgramJSON runtime authority must remain false")
if boundary.get("rust_astnode_route_remains_authority") is not True:
    raise SystemExit("Rust ASTNode route must remain authority")

states = {row.get("id"): row for row in fixture.get("candidate_decisions") or []}
if states.get("A_SHADOW_ONLY_DUAL_RUN_GUARD", {}).get("state") != "RecommendedDefault":
    raise SystemExit("A must be recommended")
if states.get("B_DIRECT_RUNTIME_ROUTE_SWITCH", {}).get("state") != "RejectedForNow":
    raise SystemExit("B must be rejected for now")
if states.get("C_MORE_DTO_OR_MATCHER_ROWS_BEFORE_SWITCH", {}).get("state") != "ConsultationAlternative":
    raise SystemExit("C must remain consultation alternative")

question = fixture.get("consultation_question") or {}
if question.get("recommended_default") != "A_SHADOW_ONLY_DUAL_RUN_GUARD":
    raise SystemExit("wrong recommended default")

decision = fixture.get("decision") or {}
if decision.get("kind") != "ConsultationRequired":
    raise SystemExit("decision must require consultation")
if decision.get("selected_next_card") != "CONSULTATION_REQUIRED":
    raise SystemExit("selected next must be consultation")

claims = fixture.get("claims") or {}
for key in ["design_stop", "programjson_shadow_parity_green"]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {"design_stop", "programjson_shadow_parity_green"}:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    token,
    "A_SHADOW_ONLY_DUAL_RUN_GUARD",
    "B_DIRECT_RUNTIME_ROUTE_SWITCH",
    "CONSULTATION_REQUIRED",
    "runtime_route_switch=0",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, "CONSULTATION_REQUIRED", "A_SHADOW_ONLY_DUAL_RUN_GUARD"]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
if f'latest_card = "{token}"' not in current_state:
    raise SystemExit("CURRENT_STATE latest card drift")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-runtime-route-shadow-switch-design-stop-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-ROUTE-SHADOW-SWITCH-DESIGN-STOP-001
design_stop=1
programjson_shadow_parity_green=1
recommended_default=A_SHADOW_ONLY_DUAL_RUN_GUARD
selected_next_card=CONSULTATION_REQUIRED
runtime_route_switch=0
programjson_runtime_route_authority=0
recipe_matcher_input_authority=0
full_recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
