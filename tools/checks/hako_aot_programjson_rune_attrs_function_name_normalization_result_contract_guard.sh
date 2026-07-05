#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="hako-aot-programjson-rune-attrs-function-name-normalization-result-contract-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/hako-aot-programjson-rune-attrs-function-name-normalization-result-contract-v0.json"
RUNE_ATTRS_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_rune_attrs_box.hako"
PHASE_STATE_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_phase_state_box.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$RUNE_ATTRS_IMPL" "$PHASE_STATE_IMPL" "$HAKO_BIN"

rm -rf "$ROOT_DIR/target/hako-cache"

TMP_DIR="$(mktemp -d /tmp/hakorune-rune-attrs-normalization-contract.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
if [[ "${HAKO_KEEP_TMP:-0}" == "1" ]]; then
  echo "debug_tmp=$TMP_DIR" >&2
else
  trap cleanup EXIT
fi

APP="$TMP_DIR/rune_attrs_normalization_contract_probe.hako"
MIR_JSON="$TMP_DIR/rune_attrs_normalization_contract_probe.mir.json"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])

if fixture.get("kind") != "HakoAotProgramJsonRuneAttrsFunctionNameNormalizationResultContractV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "HAKO-AOT-PROGRAMJSON-RUNE-ATTRS-FUNCTION-NAME-NORMALIZATION-RESULT-CONTRACT-001":
    raise SystemExit("bad fixture token")

rule = fixture.get("contract_rule") or {}
required = {
    "normalization_helper": "ProgramJsonV0RuneAttrsBox._normalize_function_name_result/4",
    "normalization_return_shape": "map_handle",
    "legacy_string_or_error_normalizer_removed": True,
    "freeze_tag_string_probe_removed_from_caller": True,
}
for key, expected in required.items():
    if rule.get(key) != expected:
        raise SystemExit(f"bad contract field: {key}")

decision = fixture.get("decision") or {}
if decision.get("selected_next_card") != "HAKO-AOT-PROGRAMJSON-PHASE-STATE-PARSE-AOT-CALL-READINESS-001":
    raise SystemExit("bad next-card decision")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    if key == "rune_attrs_normalization_aot_call_fixed":
        if value != 1:
            raise SystemExit("rune attrs normalization claim must be fixed")
    elif value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

source = "\n".join([
    "using selfhost.shared.common.box_helpers as BoxHelpers",
    "using lang.compiler.mirbuilder.program_json_v0_phase_state_box as ProgramJsonV0PhaseStateBox",
    "",
    "static box Main {",
    "  main() {",
    "    local out = ProgramJsonV0PhaseStateBox.parse(",
    "      \"{\\\"version\\\":0,\\\"kind\\\":\\\"Program\\\",\\\"type\\\":\\\"Program\\\",\\\"defs\\\":[{\\\"type\\\":\\\"Function\\\",\\\"box\\\":\\\"Main\\\",\\\"name\\\":\\\"main\\\",\\\"params\\\":[],\\\"attrs\\\":{\\\"runes\\\":[\\\"inline\\\"]}}],\\\"body\\\":[]}\",",
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

python3 - "$RUNE_ATTRS_IMPL" <<'PY'
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text(encoding="utf-8")
if "_normalize_function_name_result(name, box_name, params_json, tag): MapBox" not in text:
    raise SystemExit("missing result-map normalizer")
if "_normalize_function_name_or_error" in text:
    raise SystemExit("legacy string-or-error normalizer remains")
if "is_freeze_tag(function_name)" in text:
    raise SystemExit("caller still probes freeze-tag strings")
for required in [
    "_name_ok(value): MapBox",
    "_name_err(tag, msg): MapBox",
    "local name_result = me._normalize_function_name_result(",
]:
    if required not in text:
        raise SystemExit(f"missing normalization contract surface: {required}")
PY

bash "$HAKO_BIN" --backend mir --verify "$RUNE_ATTRS_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$PHASE_STATE_IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for RuneAttrs normalization contract probe"
fi

python3 - "$FIXTURE" "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
data = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))

routes = []
for fn in data.get("functions", []):
    caller = fn.get("name")
    for route in (fn.get("metadata") or {}).get("global_call_routes", []):
        route = dict(route)
        route["caller"] = caller
        routes.append(route)

def find_route(caller, callee):
    return [
        r for r in routes
        if r.get("caller") == caller and r.get("symbol") == callee
    ]

for expected in fixture.get("expected_direct_routes") or []:
    matches = find_route(expected["caller"], expected["callee"])
    if not matches:
        raise SystemExit(f"missing direct route: {expected}")
    if not any(
        r.get("tier") == "DirectAbi"
        and r.get("return_shape") == expected.get("return_shape")
        and not r.get("reason")
        for r in matches
    ):
        raise SystemExit(f"route is not DirectAbi {expected.get('return_shape')}: {expected} -> {matches}")

forbidden = []
for family in fixture.get("forbidden_unsupported_route_families") or []:
    for r in routes:
        sym = r.get("symbol") or ""
        if sym.startswith(family + ".") and (r.get("tier") == "Unsupported" or r.get("reason")):
            forbidden.append(f"{r.get('caller')} -> {sym}: {r.get('reason')}")
if forbidden:
    raise SystemExit("forbidden RuneAttrs unsupported routes remain: " + "; ".join(forbidden))
PY

cat <<'REPORT'
output_contract=hako-aot-programjson-rune-attrs-function-name-normalization-result-contract-guard-v0
token=HAKO-AOT-PROGRAMJSON-RUNE-ATTRS-FUNCTION-NAME-NORMALIZATION-RESULT-CONTRACT-001
rune_attrs_normalization_return_shape=map_handle
rune_attrs_unsupported_routes=0
rune_attrs_normalization_aot_call_fixed=1
phase_state_parse_aot_call_fixed=0
layer4_recipe_dto_parity_green=0
source_selfhost_claim=0
summary=ok
REPORT
