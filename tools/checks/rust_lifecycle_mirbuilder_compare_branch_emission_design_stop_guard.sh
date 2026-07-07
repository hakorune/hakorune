#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-branch-emission-design-stop-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-branch-emission-design-stop-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3318-MIRBUILDER-COMPARE-BRANCH-EMISSION-DESIGN-STOP-001.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_mir_compare_emission_bridge_gate.sh"
BRANCH_RS="$ROOT_DIR/src/mir/builder/emission/branch.rs"
FINALIZE_RS="$ROOT_DIR/src/mir/builder/ssa/local/finalize.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" \
  "$PREV_GATE" "$BRANCH_RS" "$FINALIZE_RS"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^mir_compare_emission=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "MIR Compare emission bridge prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" "$BRANCH_RS" "$FINALIZE_RS" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
current_state = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
branch_rs = Path(sys.argv[5]).read_text(encoding="utf-8")
finalize_rs = Path(sys.argv[6]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderCompareBranchEmissionDesignStopV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-BRANCH-EMISSION-DESIGN-STOP-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-COMPARE-MIR-COMPARE-EMISSION-BRIDGE-001", "bad prerequisite")

candidates = {row.get("name"): row for row in fixture.get("candidates") or []}
need(candidates["BranchEmissionBridgeFirst"].get("selected") is True, "Branch bridge must be selected")
need(
    candidates["BranchEmissionBridgeFirst"].get("selected_next_card")
    == "MIRBUILDER-COMPARE-BRANCH-EMISSION-BRIDGE-001",
    "bad selected next",
)
for name in [
    "CombineBranchEmissionWithRouteSelection",
    "SkipFinalizeBranchCondForCompareResult",
    "OpenProgramJsonRuntimeAuthorityNow",
]:
    need(candidates[name].get("selected") is False, f"{name} must not be selected")

claims = fixture.get("claims") or {}
for key in [
    "compare_branch_emission_design_stop",
    "branch_emission_bridge_selected",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "branch_emission_execution",
    "branch_condition_consumption",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need("pub fn finalize_branch_cond" in finalize_rs, "Rust branch condition finalization owner missing")
need("pub fn emit_conditional" in branch_rs, "Rust conditional branch emission owner missing")
need("set_branch" in branch_rs and "MirInstruction::Branch" in branch_rs, "branch emission implementation missing")

allowed_latest = [
    'latest_card = "MIRBUILDER-COMPARE-MIR-COMPARE-EMISSION-BRIDGE-001"',
    'latest_card = "MIRBUILDER-COMPARE-BRANCH-EMISSION-DESIGN-STOP-001"',
    'latest_card = "MIRBUILDER-COMPARE-BRANCH-EMISSION-BRIDGE-001"',
]
need(any(entry in current_state for entry in allowed_latest), "CURRENT_STATE latest card must point to prerequisite or 3318")
need("MIRBUILDER-COMPARE-BRANCH-EMISSION-BRIDGE-001" in task_order, "task-order must name selected Branch bridge")
need("Branch emission execution: `0`" in card, "card must keep Branch emission unclaimed")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-branch-emission-design-stop-v0
token=MIRBUILDER-COMPARE-BRANCH-EMISSION-DESIGN-STOP-001
decision=SelectBranchEmissionBridgeFirst
compare_branch_emission_design_stop=1
branch_emission_bridge_selected=1
branch_emission_execution=0
branch_condition_consumption=0
route_selection=0
runtime_route_switch=0
programjson_runtime_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-BRANCH-EMISSION-BRIDGE-001
summary=ok
REPORT
