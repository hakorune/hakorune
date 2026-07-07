#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-boolrecipe-to-mir-compare-branch-closeout-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-boolrecipe-to-mir-compare-branch-closeout-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3320-MIRBUILDER-COMPARE-BOOLRECIPE-TO-MIR-COMPARE-BRANCH-CLOSEOUT-001.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_branch_emission_bridge_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" "$PREV_GATE"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^compare_branch_emission_bridge=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "Branch emission bridge prerequisite is not green"
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

token = "MIRBUILDER-COMPARE-BOOLRECIPE-TO-MIR-COMPARE-BRANCH-CLOSEOUT-001"
next_card = "MIRBUILDER-COMPARE-RUNTIME-ROUTE-AUTHORITY-DESIGN-STOP-001"

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderCompareBoolRecipeToMirCompareBranchCloseoutV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-COMPARE-BRANCH-EMISSION-BRIDGE-001", "bad prerequisite")
need(fixture.get("decision", {}).get("selected_next_card") == next_card, "bad selected next")

closed_chain = fixture.get("closed_chain") or []
for required in [
    "MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-REQUEST-RESPONSE-ABI-001",
    "MIRBUILDER-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001",
    "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001",
    "MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-BRIDGE-001",
    "MIRBUILDER-COMPARE-MIR-COMPARE-EMISSION-BRIDGE-001",
    "MIRBUILDER-COMPARE-BRANCH-EMISSION-BRIDGE-001",
]:
    need(required in closed_chain, f"closed chain missing {required}")

claims = fixture.get("claims") or {}
for key in [
    "boolrecipe_to_mir_compare_branch_closeout",
    "rhs_valueid_resolution_abi_green",
    "literal_i64_rhs_resolution_bridge_green",
    "symbolref_rhs_lookup_bridge_green",
    "localssa_finalize_compare_bridge_green",
    "mir_compare_emission_bridge_green",
    "branch_emission_bridge_green",
    "compare_branch_lowering_bridge_chain_green",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need("route selection / runtime route switch: `0`" in card, "card must keep route/runtime unclaimed")
need("Source Selfhost: `0`" in card, "card must keep Source Selfhost unclaimed")
allowed_latest = [
    f'latest_card = "{token}"',
    'latest_card = "MIRBUILDER-COMPARE-RUNTIME-ROUTE-AUTHORITY-DESIGN-STOP-001"',
]
need(any(entry in current_state for entry in allowed_latest), "CURRENT_STATE latest card drift")
allowed_next_tasks = [
    f"next_documented_task =\n  {next_card}",
    "next_documented_task =\n  MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001",
]
need(any(entry in task_order for entry in allowed_next_tasks), "task-order next task drift")
allowed_next_chains = [
    f"{next_card} -> MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001 -> SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
]
need(any(entry in task_order for entry in allowed_next_chains), "task-order next chain drift")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-boolrecipe-to-mir-compare-branch-closeout-v0
token=MIRBUILDER-COMPARE-BOOLRECIPE-TO-MIR-COMPARE-BRANCH-CLOSEOUT-001
boolrecipe_to_mir_compare_branch_closeout=1
rhs_valueid_resolution_abi_green=1
literal_i64_rhs_resolution_bridge_green=1
symbolref_rhs_lookup_bridge_green=1
localssa_finalize_compare_bridge_green=1
mir_compare_emission_bridge_green=1
branch_emission_bridge_green=1
compare_branch_lowering_bridge_chain_green=1
route_selection=0
runtime_route_switch=0
programjson_runtime_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-RUNTIME-ROUTE-AUTHORITY-DESIGN-STOP-001
summary=ok
REPORT
