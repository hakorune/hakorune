#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-phase-state-aot-call-readiness-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-phase-state-aot-call-blocker-v0.json"
PHASE_STATE_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_phase_state_box.hako"
BOX_TYPE_IMPL="$ROOT_DIR/lang/src/shared/common/box_type_inspector_box.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$PHASE_STATE_IMPL" "$BOX_TYPE_IMPL" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-programjson-phase-state-aot-readiness.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/phase_state_parse_probe.hako"
MIR_JSON="$TMP_DIR/phase_state_parse_probe.json"

python3 - "$FIXTURE" "$APP" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])

if fixture.get("kind") != "MirBuilderProgramJsonLayer4PhaseStateAotCallReadinessV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "HAKO-AOT-PROGRAMJSON-PHASE-STATE-PARSE-AOT-CALL-READINESS-001":
    raise SystemExit("bad fixture token")

contract = fixture.get("readiness_contract") or {}
if contract.get("callee_symbol") != "ProgramJsonV0PhaseStateBox.parse/2":
    raise SystemExit("bad readiness callee")
if contract.get("required_tier") != "DirectAbi":
    raise SystemExit("bad readiness tier")
if contract.get("required_return_shape") != "map_handle":
    raise SystemExit("bad readiness return shape")
if contract.get("old_missing_multi_function_emitter_blocker_closed") is not True:
    raise SystemExit("old blocker must be marked closed")
if contract.get("full_aot_executable_green") != 0:
    raise SystemExit("full AOT executable must stay unclaimed")

decision = fixture.get("decision") or {}
if decision.get("selected_next_card") != "MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-PARITY-001":
    raise SystemExit("bad selected next card")

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

bash "$HAKO_BIN" --backend mir --verify "$PHASE_STATE_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$BOX_TYPE_IMPL" >/dev/null
timeout 90s bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >/dev/null

python3 - "$FIXTURE" "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
mir = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
contract = fixture["readiness_contract"]
callee = contract["callee_symbol"]

functions = mir.get("functions") or []
main = next((fn for fn in functions if fn.get("name") == "main"), None)
callee_fn = next((fn for fn in functions if fn.get("name") == callee), None)
if main is None:
    raise SystemExit("main function missing from emitted MIR JSON")
if callee_fn is None:
    raise SystemExit(f"{callee} function missing from emitted MIR JSON")

def route_rows(fn):
    metadata = fn.get("metadata") or {}
    rows = []
    for key in ("global_call_routes", "lowering_plan"):
        value = metadata.get(key) or []
        if isinstance(value, list):
            rows.extend((key, row) for row in value if isinstance(row, dict))
    return rows

matching_routes = [
    (source, row)
    for source, row in route_rows(main)
    if row.get("symbol") == callee or row.get("target_symbol") == callee
]
if not matching_routes:
    raise SystemExit(f"missing route metadata for {callee}")

def route_ok(row):
    return (
        row.get("tier") == contract["required_tier"]
        and row.get("return_shape") == contract["required_return_shape"]
        and row.get("target_result_box_name") == contract["required_result_box"]
        and row.get("target_return_type") == contract["required_target_return_type"]
        and row.get("emit_kind") == "direct_function_call"
        and row.get("target_exists") is True
    )

if not any(route_ok(row) for _, row in matching_routes):
    details = [row for _, row in matching_routes]
    raise SystemExit(f"route metadata does not satisfy readiness contract: {details}")

mir_call_found = False
for block in main.get("blocks") or []:
    for inst in block.get("instructions") or []:
        call = inst.get("mir_call") if isinstance(inst, dict) else None
        call_callee = (call or {}).get("callee") or {}
        if call_callee.get("name") == callee:
            mir_call_found = True
            break
    if mir_call_found:
        break
if not mir_call_found:
    raise SystemExit(f"main has no mir_call instruction for {callee}")

value_types = (main.get("metadata") or {}).get("value_types") or {}
result_values = [
    str(row.get("result_value"))
    for _, row in matching_routes
    if row.get("result_value") is not None
]
if not any(
    isinstance(value_types.get(value_id), dict)
    and value_types[value_id].get("kind") == "handle"
    and value_types[value_id].get("box_type") == contract["required_result_box"]
    for value_id in result_values
):
    raise SystemExit("main value_types does not publish MapBox handle for parse result")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-phase-state-aot-call-readiness-guard-v1
token=HAKO-AOT-PROGRAMJSON-PHASE-STATE-PARSE-AOT-CALL-READINESS-001
callee=ProgramJsonV0PhaseStateBox.parse/2
phase_state_parse_route=DirectAbi
phase_state_parse_return_shape=map_handle
phase_state_parse_result_box=MapBox
old_missing_multi_function_emitter_blocker=closed
mir_json_readiness=green
full_aot_executable_green=0
layer4_recipe_dto_parity_green=0
source_selfhost_claim=0
summary=ok
REPORT
