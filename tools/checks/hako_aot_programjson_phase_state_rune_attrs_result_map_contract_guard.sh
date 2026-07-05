#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="hako-aot-programjson-phase-state-rune-attrs-result-map-contract-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/hako-aot-programjson-phase-state-rune-attrs-result-map-contract-v0.json"
SCANNER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako"
RUNE_ATTRS_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_rune_attrs_box.hako"
PHASE_STATE_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_phase_state_box.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$SCANNER_IMPL" "$RUNE_ATTRS_IMPL" "$PHASE_STATE_IMPL" "$HAKO_BIN"

rm -rf "$ROOT_DIR/target/hako-cache"

TMP_DIR="$(mktemp -d /tmp/hakorune-phase-state-rune-attrs-contract.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
if [[ "${HAKO_KEEP_TMP:-0}" == "1" ]]; then
  echo "debug_tmp=$TMP_DIR" >&2
else
  trap cleanup EXIT
fi

APP="$TMP_DIR/phase_state_rune_attrs_contract_probe.hako"
MIR_JSON="$TMP_DIR/phase_state_rune_attrs_contract_probe.mir.json"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])

if fixture.get("kind") != "HakoAotProgramJsonPhaseStateRuneAttrsResultMapContractV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "HAKO-AOT-PROGRAMJSON-PHASE-STATE-RUNE-ATTRS-RESULT-MAP-CONTRACT-001":
    raise SystemExit("bad fixture token")

rule = fixture.get("contract_rule") or {}
if rule.get("scanner_first_string_result_helper_return_shape") != "map_handle":
    raise SystemExit("bad first-string result contract")
if rule.get("scanner_array_result_helper_return_shape") != "map_handle":
    raise SystemExit("bad array result contract")
if rule.get("rune_attrs_result_helper_return_shape") != "map_handle":
    raise SystemExit("bad rune attrs result contract")
if rule.get("phase_state_parse_uses_rune_attrs_result_helper") is not True:
    raise SystemExit("bad parse/rune attrs contract")
if rule.get("legacy_nullable_scanner_helpers_not_widened") is not True:
    raise SystemExit("bad legacy nullable policy")

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
    "      \"{\\\"version\\\":0,\\\"kind\\\":\\\"Program\\\",\\\"type\\\":\\\"Program\\\",\\\"body\\\":[]}\",",
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
bash "$HAKO_BIN" --backend mir --verify "$RUNE_ATTRS_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$PHASE_STATE_IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for PhaseState RuneAttrs contract probe"
fi

python3 - "$FIXTURE" "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
data = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
downstream = fixture.get("downstream_route_contract") or {}
if downstream.get("owned_by_later_cards") is not True:
    raise SystemExit("downstream route contract must be delegated")

seen = {
    "parse_calls_rune_attrs_result": 0,
    "rune_attrs_result_map_handle": 0,
    "scanner_first_string_result_map_handle": 0,
    "scanner_array_result_map_handle": 0,
}
forbidden = []

for fn in data.get("functions", []):
    name = fn.get("name")
    for route in (fn.get("metadata") or {}).get("global_call_routes", []):
        symbol = route.get("symbol")
        if name == "ProgramJsonV0PhaseStateBox.parse/2" and symbol == "ProgramJsonV0RuneAttrsBox.read_function_runes_map_result/2":
            seen["parse_calls_rune_attrs_result"] += 1
            if route.get("tier") == "DirectAbi" and route.get("return_shape") == "map_handle":
                seen["rune_attrs_result_map_handle"] += 1
        if symbol == "ProgramJsonV0ScannerBox.read_string_field_first_in_obj_result/3":
            if route.get("tier") == "DirectAbi" and route.get("return_shape") == "map_handle":
                seen["scanner_first_string_result_map_handle"] += 1
        if symbol == "ProgramJsonV0ScannerBox.read_array_field_first_in_obj_result/3":
            if route.get("tier") == "DirectAbi" and route.get("return_shape") == "map_handle":
                seen["scanner_array_result_map_handle"] += 1
        if symbol == "ProgramJsonV0RuneAttrsBox.read_function_runes_map_result/2":
            if route.get("tier") == "Unsupported" or route.get("reason"):
                forbidden.append(f"rune attrs result unsupported in {name}")
        if symbol in {
            "ProgramJsonV0ScannerBox.read_string_field_first_in_obj/3",
            "ProgramJsonV0ScannerBox.read_array_field_first_in_obj/3",
        }:
            if route.get("return_shape") in {"object_handle", "map_handle", "mixed_runtime_i64_or_handle"}:
                forbidden.append(f"legacy scanner helper widened: {symbol}")

for key, count in seen.items():
    if count < 1:
        raise SystemExit(f"missing expected route evidence: {key}")
if forbidden:
    raise SystemExit("; ".join(forbidden))
PY

cat <<'REPORT'
output_contract=hako-aot-programjson-phase-state-rune-attrs-result-map-contract-guard-v0
token=HAKO-AOT-PROGRAMJSON-PHASE-STATE-RUNE-ATTRS-RESULT-MAP-CONTRACT-001
rune_attrs_result_helper_return_shape=map_handle
scanner_first_string_result_helper_return_shape=map_handle
scanner_array_result_helper_return_shape=map_handle
phase_state_parse_uses_rune_attrs_result_helper=1
downstream_route_contract=delegated
phase_state_parse_aot_call_fixed=0
layer4_recipe_dto_parity_green=0
source_selfhost_claim=0
summary=ok
REPORT
