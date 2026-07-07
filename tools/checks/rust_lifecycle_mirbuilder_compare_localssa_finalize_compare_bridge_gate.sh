#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-localssa-finalize-compare-bridge-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-localssa-finalize-compare-bridge-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3316-MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-BRIDGE-001.md"
IMPL="$ROOT_DIR/src/mir/builder/compare_localssa_finalize_compare_bridge.rs"
DESIGN_STOP_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_localssa_finalize_compare_design_stop_guard.sh"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$IMPL" "$DESIGN_STOP_GATE"

DESIGN_STOP_OUT="$(guard_cached_run "$TAG" bash "$DESIGN_STOP_GATE")"
if ! grep -q '^localssa_finalize_compare_bridge_selected=1$' <<<"$DESIGN_STOP_OUT"; then
  printf '%s\n' "$DESIGN_STOP_OUT" >&2
  guard_fail "$TAG" "LocalSSA finalize_compare design-stop prerequisite is not green"
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
need(fixture.get("kind") == "MirBuilderCompareLocalSsaFinalizeCompareBridgeV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-BRIDGE-001", "bad token")
need(fixture.get("owner") == "CompareLocalSsaFinalizeCompareBridge", "bad owner")
need(fixture.get("prerequisite") == "MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001", "bad prerequisite")
need(fixture.get("output_contract") == "CompareLocalSsaFinalizeCompareResponseV1", "bad output contract")

rows = fixture.get("rows") or []
need([row.get("row_id") for row in rows] == ["same_block_literal_operands_finalize"], "row set drift")

claims = fixture.get("claims") or {}
for key in [
    "compare_localssa_finalize_compare_bridge",
    "localssa_finalize_compare_execution",
    "lhs_rhs_valueids_finalized",
    "same_block_literal_operands_finalize",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "mir_compare_emission",
    "mir_branch_emission",
    "bool_result_type_publication",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "CompareLocalSsaFinalizeCompareBridge",
    "CompareLocalSsaFinalizeCompareResponse",
    "finalize_operands",
    "ssa::local::finalize_compare(builder, &mut lhs_final, &mut rhs_final)",
    "localssa_finalize_compare_executed: true",
    "mir_compare_emitted: false",
    "mir_branch_emitted: false",
    "bool_result_type_publication: false",
    "runtime_route_switch: false",
    "programjson_runtime_authority: false",
]:
    need(needle in impl, f"implementation missing token: {needle}")
for forbidden in [
    "emission::compare::emit_to",
    "emission::branch",
    "build_comparison_op",
    "CompareOp",
    "MirType::Bool",
    "value_types.insert",
    "route_loop",
]:
    need(forbidden not in impl, f"forbidden implementation token: {forbidden}")

need("MIR Compare emission: `0`" in card, "card must keep Compare emission unclaimed")
need("Bool result type publication: `0`" in card, "card must keep Bool publication unclaimed")
PY

cargo test -q --lib compare_localssa_finalize_compare_bridge_finalizes_operands_without_compare_emission -- --nocapture

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-localssa-finalize-compare-bridge-v0
token=MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-BRIDGE-001
owner=CompareLocalSsaFinalizeCompareBridge
compare_localssa_finalize_compare_bridge=1
localssa_finalize_compare_execution=1
lhs_rhs_valueids_finalized=1
same_block_literal_operands_finalize=1
mir_compare_emission=0
mir_branch_emission=0
bool_result_type_publication=0
route_selection=0
runtime_route_switch=0
programjson_runtime_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-MIR-COMPARE-EMISSION-BRIDGE-001
summary=ok
REPORT
