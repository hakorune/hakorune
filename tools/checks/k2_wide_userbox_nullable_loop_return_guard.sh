#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-userbox-nullable-loop-return"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/userbox-nullable-loop-return-proof/main.hako"
APP_README="apps/userbox-nullable-loop-return-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-473-MIR-ROW-C-NULLABLE-USERBOX-OBJECT-RETURN.md"
SSOT="docs/development/current/main/design/userbox-nullable-object-return-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
ROUTE_PLAN="src/mir/user_box_method_route_plan"

echo "[$TAG] running MIR-ROW-C nullable user-box object return guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$CARD" "$SSOT" "$INDEX" "$ROUTE_PLAN/return_shape.rs"
guard_require_exec_files "$TAG" "$0"

guard_expect_in_file "$TAG" "MIR-ROW-C" "$CARD" "card must name MIR-ROW-C"
guard_expect_in_file "$TAG" "Status: SSOT" "$SSOT" "nullable object return SSOT missing"
guard_expect_in_file "$TAG" "return_shape: object_handle" "$SSOT" "SSOT must fix object_handle route contract"
guard_expect_in_file "$TAG" "target_result_box_name" "$SSOT" "SSOT must fix concrete result box publication"
guard_expect_in_file "$TAG" "box NullableReturnItem" "$APP" "proof app item box missing"
guard_expect_in_file "$TAG" "selectItem\\(\\): NullableReturnItem" "$APP" "proof app selected-object method missing"
guard_expect_in_file "$TAG" "local selected: NullableReturnItem = null" "$APP" "proof app must start from nullable object local"
guard_expect_in_file "$TAG" "loop\\(i < count\\)" "$APP" "proof app must use loop-carried selection"
guard_expect_in_file "$TAG" "selected\\.id" "$APP" "proof app must read a field from the returned object"
guard_expect_in_file "$TAG" "selected\\.bump\\(5\\)" "$APP" "proof app must call a method on the returned object"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIR-ROW-C guard"

if rg -n 'userbox-nullable-loop-return|NullableReturn' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "MIR-ROW-C matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mir_row_c_nullable_return.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mir_row_c.mir.json"
exe_out="$tmp_dir/mir_row_c.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
main = functions.get("main")
if main is None:
    raise SystemExit("missing main")
if functions.get("NullableReturnQueue.selectItem/0") is None:
    raise SystemExit("missing NullableReturnQueue.selectItem/0")

routes = main.get("metadata", {}).get("lowering_plan", [])
select_routes = [
    route for route in routes
    if route.get("box_name") == "NullableReturnQueue"
    and route.get("method") == "selectItem"
]
if not select_routes:
    raise SystemExit("missing selectItem lowering plan")
for route in select_routes:
    if route.get("reason") is not None:
        raise SystemExit(f"selectItem route rejected: {route.get('reason')}")
    if route.get("return_shape") != "object_handle":
        raise SystemExit(f"selectItem return_shape drift: {route.get('return_shape')}")
    if route.get("value_demand") != "runtime_i64_or_handle":
        raise SystemExit(f"selectItem value_demand drift: {route.get('value_demand')}")
    if route.get("target_result_box_name") != "NullableReturnItem":
        raise SystemExit(
            f"selectItem target_result_box_name drift: {route.get('target_result_box_name')}"
        )

value_types = main.get("metadata", {}).get("value_types", {})
if not any(
    isinstance(value_type, dict)
    and value_type.get("kind") == "handle"
    and value_type.get("box_type") == "NullableReturnItem"
    for value_type in value_types.values()
):
    raise SystemExit("caller value_types never publish NullableReturnItem")

print("[mir-row-c-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'userbox-nullable-loop-return-proof' "$run_log"
rg -F -q 'id=7' "$run_log"
rg -F -q 'bumped=12' "$run_log"
rg -F -q 'shape=2' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
