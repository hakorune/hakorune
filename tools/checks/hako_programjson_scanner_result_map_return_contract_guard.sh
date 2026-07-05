#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="hako-programjson-scanner-result-map-return-contract-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/hako-programjson-scanner-result-map-return-contract-v0.json"
SCANNER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$SCANNER_IMPL" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-scanner-result-map-contract.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
if [[ "${HAKO_KEEP_TMP:-0}" == "1" ]]; then
  echo "debug_tmp=$TMP_DIR" >&2
else
  trap cleanup EXIT
fi

APP="$TMP_DIR/scanner_result_map_contract.hako"
EXPECTED="$TMP_DIR/expected.txt"
RUN_LOG="$TMP_DIR/run.log"
EXE="$TMP_DIR/scanner_result_map_contract.exe"
MIR_JSON="$TMP_DIR/scanner_result_map_contract.mir.json"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

if fixture.get("kind") != "HakoProgramJsonScannerResultMapReturnContractV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "HAKO-PROGRAMJSON-SCANNER-RESULT-MAP-RETURN-CONTRACT-001":
    raise SystemExit("bad fixture token")

rule = fixture.get("contract_rule") or {}
for key in [
    "generic_void_object_return_reject_remains",
    "body_proof_alone_cannot_publish_object_return",
    "legacy_null_sentinel_helpers_are_no_new_consumer",
    "new_scanner_helpers_return_total_result_map",
]:
    if rule.get(key) is not True:
        raise SystemExit(f"contract rule drift: {key}")

policy = fixture.get("aot_publication_policy") or {}
decision = fixture.get("decision") or {}
if decision.get("selected_approach") != "B":
    raise SystemExit("scanner result-map contract must select approach B")
if decision.get("approach_a_body_proof_void_object_widening") != "Avoid":
    raise SystemExit("approach A must remain forbidden")
if decision.get("approach_c_nullable_out_map_bridge") != "TemporaryBridgeOnlyWithRemovalCard":
    raise SystemExit("approach C bridge policy drift")
if policy.get("void_signature_object_return") != "Reject":
    raise SystemExit("void signature object return policy drift")
if policy.get("map_handle_result_map") != "PublishMapHandle":
    raise SystemExit("map handle publication policy drift")
if policy.get("mixed_runtime_i64_or_handle_for_scanner_out_map") != "Forbidden":
    raise SystemExit("scanner out-map mixed runtime policy drift")
if policy.get("nullable_out_map_bridge_requires_remove_after") is not True:
    raise SystemExit("nullable bridge must require remove_after")
if policy.get("nullable_out_map_bridge_new_consumers_allowed") is not False:
    raise SystemExit("nullable bridge must not allow new consumers")

helpers = {h.get("helper_id"): h for h in fixture.get("result_map_helpers") or []}
for helper_id in [
    "ProgramJsonV0ScannerBox.read_int_field_in_obj_result/3",
    "ProgramJsonV0ScannerBox.read_string_field_last_in_obj_result/3",
]:
    helper = helpers.get(helper_id)
    if not helper:
        raise SystemExit(f"missing result helper contract: {helper_id}")
    if helper.get("return_contract") != "TotalResultMapReturn":
        raise SystemExit(f"bad result helper contract: {helper_id}")
    if helper.get("return_shape") != "map_handle":
        raise SystemExit(f"bad result helper return shape: {helper_id}")
    if helper.get("aot_directabi_allowed") is not True:
        raise SystemExit(f"bad direct abi flag: {helper_id}")

legacy = {h.get("helper_id"): h for h in fixture.get("legacy_helpers") or []}
for helper_id in [
    "ProgramJsonV0ScannerBox.read_int_field_in_obj/3",
    "ProgramJsonV0ScannerBox.read_string_field_last_in_obj/3",
]:
    helper = legacy.get(helper_id)
    if not helper:
        raise SystemExit(f"missing legacy helper contract: {helper_id}")
    if helper.get("new_consumers_allowed") is not False:
        raise SystemExit(f"legacy helper allows new consumers: {helper_id}")
    if helper.get("aot_directabi_expansion_allowed") is not False:
        raise SystemExit(f"legacy helper allows direct abi expansion: {helper_id}")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "mir_mutation",
    "id_allocation",
    "backend_lowering_claim",
    "new_backend_route",
    "new_abi",
    "programjson_layer4_parity_green",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

obj = json.dumps({"value": 42, "name": "x"}, separators=(",", ":"), ensure_ascii=False)
source = f'''using lang.compiler.mirbuilder.program_json_v0_scanner_box as ProgramJsonV0ScannerBox

static box Main {{
  main() {{
    local obj = {json.dumps(obj)}
    local i = ProgramJsonV0ScannerBox.read_int_field_in_obj_result(obj, "value", 0)
    local im = ProgramJsonV0ScannerBox.read_int_field_in_obj_result(obj, "missing", 0)
    local s = ProgramJsonV0ScannerBox.read_string_field_last_in_obj_result(obj, "name", 0)
    local sm = ProgramJsonV0ScannerBox.read_string_field_last_in_obj_result(obj, "missing", 0)
    print("scanner_result_map:"
      + "int_ok=" + i.get("ok")
      + ";int_missing=" + im.get("ok")
      + ";str_ok=" + s.get("ok")
      + ";str_missing=" + sm.get("ok"))
    return 0
  }}
}}
'''

app.write_text(source, encoding="utf-8")
expected.write_text(
    "scanner_result_map:int_ok=1;int_missing=0;str_ok=1;str_missing=0\n",
    encoding="utf-8",
)
PY

bash "$HAKO_BIN" --backend mir --verify "$SCANNER_IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for scanner result-map contract"
fi

python3 - "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
expected_result_helpers = {
    "ProgramJsonV0ScannerBox.read_int_field_in_obj_result/3",
    "ProgramJsonV0ScannerBox.read_string_field_last_in_obj_result/3",
}
legacy_helpers = {
    "ProgramJsonV0ScannerBox.read_int_field_in_obj/3",
    "ProgramJsonV0ScannerBox.read_string_field_last_in_obj/3",
}
seen = {helper: 0 for helper in expected_result_helpers}
legacy_widened = []

for fn in data.get("functions", []):
    for route in (fn.get("metadata") or {}).get("global_call_routes", []):
        symbol = route.get("symbol")
        if symbol in expected_result_helpers:
            if route.get("tier") != "DirectAbi":
                raise SystemExit(f"result helper is not DirectAbi: {symbol}")
            if route.get("return_shape") != "map_handle":
                raise SystemExit(f"result helper return_shape drift: {symbol}")
            if route.get("target_result_box_name") != "MapBox":
                raise SystemExit(f"result helper target box drift: {symbol}")
            if route.get("target_return_type") != "box<MapBox>":
                raise SystemExit(f"result helper target return type drift: {symbol}")
            seen[symbol] += 1
        if symbol in legacy_helpers:
            if route.get("return_shape") == "mixed_runtime_i64_or_handle":
                legacy_widened.append(symbol)
            if route.get("tier") == "DirectAbi" and route.get("return_shape") in {
                "object_handle",
                "map_handle",
                "mixed_runtime_i64_or_handle",
            }:
                legacy_widened.append(symbol)

for symbol, count in seen.items():
    if count < 1:
        raise SystemExit(f"missing result helper call route: {symbol}")
if legacy_widened:
    raise SystemExit("legacy scanner helper widened: " + ", ".join(sorted(set(legacy_widened))))
PY

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit AOT scanner result-map executable"
fi

if ! "$EXE" >"$RUN_LOG" 2>&1; then
  tail -n 160 "$RUN_LOG" || true
  guard_fail "$TAG" "failed to run AOT scanner result-map executable"
fi

python3 - "$EXPECTED" "$RUN_LOG" <<'PY'
import sys
from pathlib import Path

expected = Path(sys.argv[1]).read_text(encoding="utf-8").strip()
lines = [
    line.strip()
    for line in Path(sys.argv[2]).read_text(encoding="utf-8").splitlines()
    if line.strip()
    and not line.startswith("Result:")
    and not line.startswith("[freeze:contract]")
]
if lines != [expected]:
    print(f"expected: {expected}", file=sys.stderr)
    print(f"actual: {lines}", file=sys.stderr)
    raise SystemExit(1)
PY

cargo test -q mir::global_call_route_plan::tests::same_module_sum_handles --lib >/dev/null

cat <<'REPORT'
output_contract=hako-programjson-scanner-result-map-return-contract-guard-v0
fixture=hako-programjson-scanner-result-map-return-contract-v0.json
result_helpers_return_shape=map_handle
result_helpers_directabi=green
legacy_null_sentinel_helpers_new_consumers_allowed=false
generic_void_object_return_reject_remains=true
mixed_runtime_i64_or_handle_for_scanner_out_map=forbidden
runtime_rows=green
source_selfhost_claim=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
new_backend_route=0
new_abi=0
programjson_layer4_parity_green=0
summary=ok
REPORT
