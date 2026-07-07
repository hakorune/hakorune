#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-mir-compare-emission-bridge-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-mir-compare-emission-bridge-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3317-MIRBUILDER-COMPARE-MIR-COMPARE-EMISSION-BRIDGE-001.md"
IMPL="$ROOT_DIR/src/mir/builder/compare_mir_compare_emission_bridge.rs"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_localssa_finalize_compare_bridge_gate.sh"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$IMPL" "$PREV_GATE"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^localssa_finalize_compare_execution=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "LocalSSA finalize_compare bridge prerequisite is not green"
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
need(fixture.get("kind") == "MirBuilderCompareMirCompareEmissionBridgeV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-MIR-COMPARE-EMISSION-BRIDGE-001", "bad token")
need(fixture.get("owner") == "CompareMirCompareEmissionBridge", "bad owner")
need(fixture.get("prerequisite") == "MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-BRIDGE-001", "bad prerequisite")
need(fixture.get("output_contract") == "CompareMirCompareEmissionResponseV1", "bad output contract")

rows = fixture.get("rows") or []
need([row.get("row_id") for row in rows] == ["finalized_literal_operands_lt_compare"], "row set drift")

claims = fixture.get("claims") or {}
for key in [
    "compare_mir_compare_emission_bridge",
    "mir_compare_emission",
    "compare_result_valueid_allocated",
    "bool_result_type_publication",
    "finalized_literal_operands_lt_compare",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "mir_branch_emission",
    "branch_condition_consumption",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "CompareMirCompareEmissionBridge",
    "CompareMirCompareEmissionResponse",
    "emit_compare_from_finalized_operands",
    "emission::compare::emit_to(builder, dst, op, lhs, rhs)",
    "builder.next_value_id()",
    "mir_compare_emitted: true",
    "bool_result_type_publication: true",
    "mir_branch_emitted: false",
    "runtime_route_switch: false",
    "programjson_runtime_authority: false",
]:
    need(needle in impl, f"implementation missing token: {needle}")
for forbidden in [
    "emission::branch",
    "emit_branch",
    "Branch {",
    "build_comparison_op",
    "route_loop",
]:
    need(forbidden not in impl, f"forbidden implementation token: {forbidden}")

need("MIR Branch emission: `0`" in card, "card must keep Branch emission unclaimed")
need("Branch condition consumption: `0`" in card, "card must keep Branch consumption unclaimed")
PY

cargo test -q --lib compare_mir_compare_emission_bridge_emits_compare_only -- --nocapture

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-mir-compare-emission-bridge-v0
token=MIRBUILDER-COMPARE-MIR-COMPARE-EMISSION-BRIDGE-001
owner=CompareMirCompareEmissionBridge
compare_mir_compare_emission_bridge=1
mir_compare_emission=1
compare_result_valueid_allocated=1
bool_result_type_publication=1
finalized_literal_operands_lt_compare=1
mir_branch_emission=0
branch_condition_consumption=0
route_selection=0
runtime_route_switch=0
programjson_runtime_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-BRANCH-EMISSION-DESIGN-STOP-001
summary=ok
REPORT
