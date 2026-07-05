#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="hako-aot-programjson-phase-state-consume-stmt-non-control-result-contract-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/hako-aot-programjson-phase-state-consume-stmt-non-control-result-contract-v0.json"
CONSUMER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_phase_state_consumer_box.hako"
PHASE_STATE_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_phase_state_box.hako"
RECIPE_FACTS_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_facts_box.hako"
SAME_MODULE_BODY_SHAPE="$ROOT_DIR/src/mir/same_module_body_shape.rs"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CONSUMER_IMPL" "$PHASE_STATE_IMPL" "$RECIPE_FACTS_IMPL" "$SAME_MODULE_BODY_SHAPE" "$HAKO_BIN"

rm -rf "$ROOT_DIR/target/hako-cache"

TMP_DIR="$(mktemp -d /tmp/hakorune-consume-stmt-non-control-contract.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
if [[ "${HAKO_KEEP_TMP:-0}" == "1" ]]; then
  echo "debug_tmp=$TMP_DIR" >&2
else
  trap cleanup EXIT
fi

APP="$TMP_DIR/consume_stmt_non_control_contract_probe.hako"
MIR_JSON="$TMP_DIR/consume_stmt_non_control_contract_probe.mir.json"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])

if fixture.get("kind") != "HakoAotProgramJsonPhaseStateConsumeStmtNonControlResultContractV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "HAKO-AOT-PROGRAMJSON-PHASE-STATE-CONSUME-STMT-NON-CONTROL-RESULT-CONTRACT-001":
    raise SystemExit("bad fixture token")

rule = fixture.get("contract_rule") or {}
required = {
    "stmt_handlers_use_scanner_result_helpers": True,
    "stmt_handlers_legacy_null_sentinel_scanner_removed": True,
    "recipe_facts_from_stmt_return_shape": "map_handle",
    "consumer_non_control_result_return_shape": "map_handle",
    "same_module_array_push_side_effect_allowed": True,
    "remaining_control_try_nullable_helpers": True,
}
for key, expected in required.items():
    if rule.get(key) != expected:
        raise SystemExit(f"bad contract field: {key}")

decision = fixture.get("decision") or {}
if decision.get("selected_next_card") != "HAKO-AOT-PROGRAMJSON-PHASE-STATE-CONSUME-STMT-CONTROL-TRY-RESULT-CONTRACT-001":
    raise SystemExit("bad next-card decision")

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
    "      \"{\\\"version\\\":0,\\\"kind\\\":\\\"Program\\\",\\\"type\\\":\\\"Program\\\",\\\"body\\\":[{\\\"type\\\":\\\"Print\\\",\\\"expr\\\":{\\\"type\\\":\\\"String\\\",\\\"value\\\":\\\"x\\\"}}]}\",",
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

python3 - "$ROOT_DIR" <<'PY'
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
stmt_dir = root / "lang/src/compiler/mirbuilder/stmt_handlers"
legacy_scanner = re.compile(
    r"read_node_type_at\(|read_int_field_in_obj\(|read_string_field_(?:first|last)_in_obj\("
)
for path in sorted(stmt_dir.glob("*.hako")):
    text = path.read_text(encoding="utf-8")
    if legacy_scanner.search(text):
        raise SystemExit(f"legacy scanner helper remains in {path}")
    if "== null {" in text:
        raise SystemExit(f"legacy null sentinel check remains in {path}")

consumer = (root / "lang/src/compiler/mirbuilder/program_json_v0_phase_state_consumer_box.hako").read_text(encoding="utf-8")
for forbidden in [
    "_handle_non_control_stmt_or_null",
    "_non_control_handler_out_or_null",
    "_non_control_handler_state_or_null",
    "_after_state_from_non_control_result_or_null",
]:
    if forbidden in consumer:
        raise SystemExit(f"legacy non-control nullable helper remains: {forbidden}")
for required in [
    "_handle_non_control_stmt_result",
    "_non_control_handler_out_result",
    "_non_control_handler_state_result",
    "_after_state_from_non_control_result",
]:
    if required not in consumer:
        raise SystemExit(f"missing non-control result helper: {required}")

facts = (root / "lang/src/compiler/mirbuilder/recipe/recipe_facts_box.hako").read_text(encoding="utf-8")
for required in [
    "from_stmt(stmt_kind, state_before, state_after, tag): MapBox",
    "_push_name(arr, name): i64",
    "arr.push(name)",
]:
    if required not in facts:
        raise SystemExit(f"missing RecipeFacts result-map contract surface: {required}")
if 'facts.set("local_names", me._push_name' in facts:
    raise SystemExit("RecipeFacts still treats _push_name as a returned array")

shape = (root / "src/mir/same_module_body_shape.rs").read_text(encoding="utf-8")
for required in [
    "known_collection_push_method_call",
    '"ArrayBox" | "RuntimeDataBox"',
    "same_module_body_accepts_array_push_side_effect_call",
]:
    if required not in shape:
        raise SystemExit(f"missing same-module push proof surface: {required}")
PY

bash "$HAKO_BIN" --backend mir --verify "$CONSUMER_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$PHASE_STATE_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$RECIPE_FACTS_IMPL" >/dev/null
for handler in "$ROOT_DIR"/lang/src/compiler/mirbuilder/stmt_handlers/*.hako; do
  bash "$HAKO_BIN" --backend mir --verify "$handler" >/dev/null
done

if ! bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for consume-stmt non-control contract probe"
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

for callee in fixture.get("expected_handler_routes") or []:
    matches = [r for r in routes if r.get("symbol") == callee]
    if not matches:
        raise SystemExit(f"missing handler route: {callee}")
    if not any(
        r.get("tier") == "DirectAbi"
        and r.get("return_shape") == "map_handle"
        and not r.get("reason")
        for r in matches
    ):
        raise SystemExit(f"handler route is not DirectAbi map_handle: {callee} -> {matches}")

for expected in fixture.get("expected_remaining_blockers") or []:
    matches = find_route(expected["caller"], expected["callee"])
    if not matches:
        raise SystemExit(f"missing remaining blocker route: {expected}")
    if not any(
        r.get("tier") == "Unsupported"
        and r.get("reason") == expected.get("reason")
        for r in matches
    ):
        raise SystemExit(f"remaining blocker route changed: {expected} -> {matches}")

legacy_scanner_symbols = {
    "ProgramJsonV0ScannerBox.read_node_type_at/2",
    "ProgramJsonV0ScannerBox.read_int_field_in_obj/3",
    "ProgramJsonV0ScannerBox.read_string_field_first_in_obj/3",
    "ProgramJsonV0ScannerBox.read_string_field_last_in_obj/3",
}
for r in routes:
    caller = r.get("caller") or ""
    if caller.endswith("StmtHandler.handle/5") and r.get("symbol") in legacy_scanner_symbols:
        raise SystemExit(f"stmt handler still routes through legacy scanner helper: {caller} -> {r.get('symbol')}")
PY

cat <<'REPORT'
output_contract=hako-aot-programjson-phase-state-consume-stmt-non-control-result-contract-guard-v0
token=HAKO-AOT-PROGRAMJSON-PHASE-STATE-CONSUME-STMT-NON-CONTROL-RESULT-CONTRACT-001
stmt_handlers_use_scanner_result_helpers=1
recipe_facts_from_stmt_return_shape=map_handle
consumer_non_control_result_return_shape=map_handle
remaining_blocker=control_try_nullable_helpers
phase_state_parse_aot_call_fixed=0
consume_stmt_full_aot_call_fixed=0
layer4_recipe_dto_parity_green=0
source_selfhost_claim=0
summary=ok
REPORT
