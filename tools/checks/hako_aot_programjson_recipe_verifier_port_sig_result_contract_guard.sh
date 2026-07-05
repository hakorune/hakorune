#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="hako-aot-programjson-recipe-verifier-port-sig-result-contract-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/hako-aot-programjson-recipe-verifier-port-sig-result-contract-v0.json"
RECIPE_VERIFIER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_verifier_box.hako"
RECIPE_PORT_SIG_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_port_sig_box.hako"
CONSUMER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_phase_state_consumer_box.hako"
PHASE_STATE_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_phase_state_box.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$RECIPE_VERIFIER_IMPL" "$RECIPE_PORT_SIG_IMPL" "$CONSUMER_IMPL" "$PHASE_STATE_IMPL" "$HAKO_BIN"

rm -rf "$ROOT_DIR/target/hako-cache"

TMP_DIR="$(mktemp -d /tmp/hakorune-recipe-verifier-port-sig-contract.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
if [[ "${HAKO_KEEP_TMP:-0}" == "1" ]]; then
  echo "debug_tmp=$TMP_DIR" >&2
else
  trap cleanup EXIT
fi

APP="$TMP_DIR/recipe_verifier_port_sig_contract_probe.hako"
MIR_JSON="$TMP_DIR/recipe_verifier_port_sig_contract_probe.mir.json"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])

if fixture.get("kind") != "HakoAotProgramJsonRecipeVerifierPortSigResultContractV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "HAKO-AOT-PROGRAMJSON-RECIPE-VERIFIER-PORT-SIG-RESULT-CONTRACT-001":
    raise SystemExit("bad fixture token")

rule = fixture.get("contract_rule") or {}
required = {
    "recipe_verifier_helpers_return_total_result_maps": True,
    "recipe_port_sig_helpers_return_total_result_maps": True,
    "recipe_port_sig_representation": "CountOnlyPortSigV1",
    "recipe_port_sig_dynamic_object_handle_republication": False,
    "recipe_verifier_boxhelpers_array_len_route_removed": True,
}
for key, expected in required.items():
    if rule.get(key) != expected:
        raise SystemExit(f"bad contract field: {key}")

decision = fixture.get("decision") or {}
if decision.get("selected_next_card") != "HAKO-AOT-PROGRAMJSON-PHASE-STATE-SCAN-BODY-REC-RESULT-CONTRACT-001":
    raise SystemExit("bad next-card decision")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    if key == "recipe_verifier_port_sig_aot_call_fixed":
        if value != 1:
            raise SystemExit("recipe verifier/port sig claim must be fixed")
    elif value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

source = "\n".join([
    "using selfhost.shared.common.box_helpers as BoxHelpers",
    "using lang.compiler.mirbuilder.program_json_v0_phase_state_box as ProgramJsonV0PhaseStateBox",
    "",
    "static box Main {",
    "  main() {",
    "    local out = ProgramJsonV0PhaseStateBox.parse(",
    "      \"{\\\"version\\\":0,\\\"kind\\\":\\\"Program\\\",\\\"type\\\":\\\"Program\\\",\\\"body\\\":[{\\\"type\\\":\\\"If\\\",\\\"cond\\\":{\\\"type\\\":\\\"Bool\\\",\\\"value\\\":1},\\\"then\\\":[],\\\"else\\\":null}]}\",",
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

python3 - "$RECIPE_VERIFIER_IMPL" "$RECIPE_PORT_SIG_IMPL" <<'PY'
import sys
from pathlib import Path

verifier = Path(sys.argv[1]).read_text(encoding="utf-8")
port_sig = Path(sys.argv[2]).read_text(encoding="utf-8")

for required in [
    "verify(recipe_root, tag): MapBox",
    "_verify_item(item, tag, depth): MapBox",
    "_apply_names(sig, names, mode): MapBox",
    "_apply_facts(sig, facts): MapBox",
]:
    if required not in verifier:
        raise SystemExit(f"missing verifier contract: {required}")
if "BoxHelpers.array_len(items)" in verifier or "BoxHelpers.array_len(names)" in verifier:
    raise SystemExit("verifier still routes array length through BoxHelpers")

for required in [
    "create(): MapBox",
    "_ensure_sig(sig): MapBox",
    "mark_def(sig, name): MapBox",
    "mark_update(sig, name): MapBox",
    "merge(lhs, rhs): MapBox",
    "snapshot(sig): MapBox",
    '"def_count" => 0',
    '"update_count" => 0',
]:
    if required not in port_sig:
        raise SystemExit(f"missing port sig contract: {required}")
if '"defs" =>' in port_sig or '"updates" =>' in port_sig:
    raise SystemExit("PortSig must stay count-only for this AOT contract")
PY

bash "$HAKO_BIN" --backend mir --verify "$RECIPE_PORT_SIG_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$RECIPE_VERIFIER_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$CONSUMER_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$PHASE_STATE_IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for RecipeVerifier/PortSig contract probe"
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
    raise SystemExit("forbidden recipe unsupported routes remain: " + "; ".join(forbidden))

next_families = set(fixture.get("expected_next_blocker_family") or [])
seen_next = set()
for r in routes:
    sym = r.get("symbol") or ""
    if r.get("tier") == "Unsupported" or r.get("reason"):
        for family in next_families:
            if sym.startswith(family + "."):
                seen_next.add(family)
if seen_next != next_families:
    raise SystemExit(f"missing expected next blocker families: seen={sorted(seen_next)} expected={sorted(next_families)}")
PY

cat <<'REPORT'
output_contract=hako-aot-programjson-recipe-verifier-port-sig-result-contract-guard-v0
token=HAKO-AOT-PROGRAMJSON-RECIPE-VERIFIER-PORT-SIG-RESULT-CONTRACT-001
recipe_verifier_unsupported_routes=0
recipe_port_sig_unsupported_routes=0
recipe_port_sig_representation=CountOnlyPortSigV1
remaining_blocker_family=none
recipe_verifier_port_sig_aot_call_fixed=1
phase_state_parse_aot_call_fixed=0
layer4_recipe_dto_parity_green=0
source_selfhost_claim=0
summary=ok
REPORT
