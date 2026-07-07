#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-runtime-route-authority-design-stop-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-runtime-route-authority-design-stop-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3321-MIRBUILDER-COMPARE-RUNTIME-ROUTE-AUTHORITY-DESIGN-STOP-001.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_boolrecipe_to_mir_compare_branch_closeout_guard.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" "$PREV_GUARD"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GUARD")"
if ! grep -q '^compare_branch_lowering_bridge_chain_green=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "Compare/Branch closeout prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
current_state = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-COMPARE-RUNTIME-ROUTE-AUTHORITY-DESIGN-STOP-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001"

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderCompareRuntimeRouteAuthorityDesignStopV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-COMPARE-BOOLRECIPE-TO-MIR-COMPARE-BRANCH-CLOSEOUT-001", "bad prerequisite")
need(fixture.get("decision", {}).get("selected_next_card") == next_card, "bad selected next")

context = fixture.get("decision_context") or {}
need(context.get("compare_branch_bridge_chain_green") is True, "bridge chain must be green")
need(context.get("runtime_authority") == "Rust ASTNode route", "runtime authority drift")
need(context.get("programjson_route") == "shadow_only", "ProgramJSON route drift")
need(context.get("runtime_route_switch_requested") is False, "runtime switch must not be requested")

candidates = {row.get("name"): row for row in fixture.get("candidates") or []}
need(candidates["RuntimeAdjacentShadowGuardFirst"].get("selected") is True, "shadow guard must be selected")
need(candidates["RuntimeAdjacentShadowGuardFirst"].get("selected_next_card") == next_card, "bad selected candidate next")
for rejected in ["DirectRuntimeRouteAuthoritySwitchNow", "SourceSelfhostWiderRouteSelectionNow"]:
    need(candidates[rejected].get("selected") is False, f"{rejected} must not be selected")

claims = fixture.get("claims") or {}
for key in [
    "compare_runtime_route_authority_design_stop",
    "runtime_adjacent_shadow_guard_selected",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "direct_runtime_route_authority_switch",
    "programjson_runtime_route_authority",
    "runtime_route_switch",
    "recipe_matcher_input_authority",
    "route_selection",
    "mir_lowering_authority",
    "mir_mutation_authority",
    "id_allocation",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "Do not switch runtime route authority from Rust to ProgramJSON here.",
    "direct runtime route authority switch: `0`",
    "Source Selfhost: `0`",
    next_card,
]:
    need(needle in card, f"card missing {needle}")
need(f'latest_card = "{token}"' in current_state, "CURRENT_STATE latest card drift")
need(f"next_documented_task =\n  {next_card}" in task_order, "task-order next task drift")
need(f"{next_card} -> SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001" in task_order, "task-order next chain drift")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-runtime-route-authority-design-stop-v0
token=MIRBUILDER-COMPARE-RUNTIME-ROUTE-AUTHORITY-DESIGN-STOP-001
decision=SelectRuntimeAdjacentShadowGuardBeforeAuthoritySwitch
compare_runtime_route_authority_design_stop=1
runtime_adjacent_shadow_guard_selected=1
direct_runtime_route_authority_switch=0
programjson_runtime_route_authority=0
runtime_route_switch=0
recipe_matcher_input_authority=0
route_selection=0
mir_lowering_authority=0
mir_mutation_authority=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001
summary=ok
REPORT
