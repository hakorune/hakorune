#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-branch-emission-bridge-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-branch-emission-bridge-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3319-MIRBUILDER-COMPARE-BRANCH-EMISSION-BRIDGE-001.md"
IMPL="$ROOT_DIR/src/mir/builder/compare_branch_emission_bridge.rs"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_branch_emission_design_stop_guard.sh"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$IMPL" "$PREV_GATE"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^branch_emission_bridge_selected=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "Branch emission design-stop prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$IMPL" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
impl = Path(sys.argv[3]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderCompareBranchEmissionBridgeV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-BRANCH-EMISSION-BRIDGE-001", "bad token")
need(fixture.get("owner") == "CompareBranchEmissionBridge", "bad owner")
need(fixture.get("prerequisite") == "MIRBUILDER-COMPARE-BRANCH-EMISSION-DESIGN-STOP-001", "bad prerequisite")
need(fixture.get("output_contract") == "CompareBranchEmissionResponseV1", "bad output contract")

rows = fixture.get("rows") or []
need([row.get("row_id") for row in rows] == ["compare_result_to_conditional_branch"], "row set drift")

claims = fixture.get("claims") or {}
for key in [
    "compare_branch_emission_bridge",
    "branch_condition_consumption",
    "localssa_finalize_branch_cond_execution",
    "branch_emission_execution",
    "compare_result_to_conditional_branch",
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

for needle in [
    "CompareBranchEmissionBridge",
    "CompareBranchEmissionResponse",
    "emit_branch_from_compare_result",
    "ssa::local::finalize_branch_cond(builder, &mut finalized_condition)",
    "emission::branch::emit_conditional(builder, finalized_condition, then_block, else_block)",
    "branch_condition_consumed: true",
    "branch_emission_executed: true",
    "route_selection: false",
    "runtime_route_switch: false",
    "programjson_runtime_authority: false",
]:
    need(needle in impl, f"implementation missing token: {needle}")
for forbidden in [
    "route_loop",
    "try_route_recipe",
    "programjson_runtime_authority: true",
    "runtime_route_switch: true",
]:
    need(forbidden not in impl, f"forbidden implementation token: {forbidden}")

need("route selection / runtime route switch: `0`" in card, "card must keep route/runtime unclaimed")
PY

cargo test -q --lib compare_branch_emission_bridge_emits_branch_only -- --nocapture

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-branch-emission-bridge-v0
token=MIRBUILDER-COMPARE-BRANCH-EMISSION-BRIDGE-001
owner=CompareBranchEmissionBridge
compare_branch_emission_bridge=1
branch_condition_consumption=1
localssa_finalize_branch_cond_execution=1
branch_emission_execution=1
compare_result_to_conditional_branch=1
route_selection=0
runtime_route_switch=0
programjson_runtime_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-BOOLRECIPE-TO-MIR-COMPARE-BRANCH-CLOSEOUT-001
summary=ok
REPORT
