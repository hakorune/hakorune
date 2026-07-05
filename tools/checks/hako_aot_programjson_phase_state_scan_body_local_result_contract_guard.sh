#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="hako-aot-programjson-phase-state-scan-body-local-result-contract-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/hako-aot-programjson-phase-state-scan-body-local-result-contract-v0.json"
SCANNER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako"
CONSUMER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_phase_state_consumer_box.hako"
PHASE_STATE_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_phase_state_box.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$SCANNER_IMPL" "$CONSUMER_IMPL" "$PHASE_STATE_IMPL" "$HAKO_BIN"

rm -rf "$ROOT_DIR/target/hako-cache"

TMP_DIR="$(mktemp -d /tmp/hakorune-phase-state-scan-body-local-contract.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
if [[ "${HAKO_KEEP_TMP:-0}" == "1" ]]; then
  echo "debug_tmp=$TMP_DIR" >&2
else
  trap cleanup EXIT
fi

APP="$TMP_DIR/phase_state_scan_body_local_contract_probe.hako"
MIR_JSON="$TMP_DIR/phase_state_scan_body_local_contract_probe.mir.json"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])

if fixture.get("kind") != "HakoAotProgramJsonPhaseStateScanBodyLocalResultContractV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "HAKO-AOT-PROGRAMJSON-PHASE-STATE-SCAN-BODY-LOCAL-RESULT-CONTRACT-001":
    raise SystemExit("bad fixture token")

rule = fixture.get("contract_rule") or {}
required = {
    "scanner_read_node_type_result_return_shape": "map_handle",
    "phase_state_append_recipe_item_result_return_shape": "map_handle",
    "phase_state_append_recipe_children_result_return_shape": "map_handle",
}
for key, expected in required.items():
    if rule.get(key) != expected:
        raise SystemExit(f"bad contract field: {key}")
if rule.get("legacy_nullable_append_helpers_removed") is not True:
    raise SystemExit("bad append legacy policy")
if rule.get("scan_body_uses_node_type_result_helper") is not True:
    raise SystemExit("bad node type result policy")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

source = "\n".join([
    "using selfhost.shared.common.box_helpers as BoxHelpers",
    "using lang.compiler.mirbuilder.program_json_v0_phase_state_box as ProgramJsonV0PhaseStateBox",
    "",
    "static box Main {",
    "  main() {",
    "    local out = ProgramJsonV0PhaseStateBox.parse(",
    "      \"{\\\"version\\\":0,\\\"kind\\\":\\\"Program\\\",\\\"type\\\":\\\"Program\\\",\\\"body\\\":[{\\\"type\\\":\\\"If\\\",\\\"cond\\\":{\\\"type\\\":\\\"Bool\\\",\\\"value\\\":1},\\\"then\\\":[{\\\"type\\\":\\\"Return\\\",\\\"value\\\":{\\\"type\\\":\\\"Int\\\",\\\"value\\\":1}}],\\\"else\\\":null}]}\",",
    "      \"[test]\"",
    "    )",
    "    print(\"err=\" + (\"\" + BoxHelpers.map_get(out, \"err\")))",
    "    return 0",
    "  }",
    "}",
    "",
])
app.write_text(source, encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$SCANNER_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$CONSUMER_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$PHASE_STATE_IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for PhaseState scan-body local contract probe"
fi

python3 - "$FIXTURE" "$MIR_JSON" "$PHASE_STATE_IMPL" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
data = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
phase_state_src = Path(sys.argv[3]).read_text(encoding="utf-8")
expected_consume_route = fixture.get("expected_scan_body_consume_stmt_route") or {}

if "_append_recipe_item_or_error" in phase_state_src:
    raise SystemExit("legacy nullable append item helper still present")
if "_append_recipe_children_rec" in phase_state_src:
    raise SystemExit("legacy nullable append children helper still present")
if "read_node_type_at(program_json, idx)" in phase_state_src:
    raise SystemExit("scan body still uses legacy nullable node type helper")

seen = {
    "read_node_type_result_map": 0,
    "append_item_result_map": 0,
    "append_children_result_map": 0,
    "scan_body_consume_stmt_route_observed": 0,
}
forbidden = []

for fn in data.get("functions", []):
    name = fn.get("name")
    for route in (fn.get("metadata") or {}).get("global_call_routes", []):
        symbol = route.get("symbol")
        if symbol == "ProgramJsonV0ScannerBox.read_node_type_at_result/2":
            if route.get("tier") == "DirectAbi" and route.get("return_shape") == "map_handle":
                seen["read_node_type_result_map"] += 1
        if symbol == "ProgramJsonV0PhaseStateBox._append_recipe_item_result/5":
            if route.get("tier") == "DirectAbi" and route.get("return_shape") == "map_handle":
                seen["append_item_result_map"] += 1
        if symbol == "ProgramJsonV0PhaseStateBox._append_recipe_children_result/6":
            if route.get("tier") == "DirectAbi" and route.get("return_shape") == "map_handle":
                seen["append_children_result_map"] += 1
        if name == "ProgramJsonV0PhaseStateBox._scan_body_rec/8" and symbol == expected_consume_route.get("callee_symbol"):
            if (
                route.get("tier") == expected_consume_route.get("tier")
                and route.get("return_shape") == expected_consume_route.get("return_shape")
            ):
                seen["scan_body_consume_stmt_route_observed"] += 1
        if symbol in {
            "ProgramJsonV0PhaseStateBox._append_recipe_item_result/5",
            "ProgramJsonV0PhaseStateBox._append_recipe_children_result/6",
            "ProgramJsonV0ScannerBox.read_node_type_at_result/2",
        }:
            if route.get("tier") == "Unsupported" or route.get("reason"):
                forbidden.append(f"result helper unsupported: {symbol} in {name}")

for key, count in seen.items():
    if count < 1:
        raise SystemExit(f"missing expected route evidence: {key}")
if forbidden:
    raise SystemExit("; ".join(forbidden))
PY

cat <<'REPORT'
output_contract=hako-aot-programjson-phase-state-scan-body-local-result-contract-guard-v0
token=HAKO-AOT-PROGRAMJSON-PHASE-STATE-SCAN-BODY-LOCAL-RESULT-CONTRACT-001
scanner_read_node_type_result_return_shape=map_handle
phase_state_append_recipe_item_result_return_shape=map_handle
phase_state_append_recipe_children_result_return_shape=map_handle
scan_body_consume_stmt_return_shape=map_handle
phase_state_parse_aot_call_fixed=0
layer4_recipe_dto_parity_green=0
source_selfhost_claim=0
summary=ok
REPORT
