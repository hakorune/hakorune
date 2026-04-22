#!/bin/bash
# phase-163x direct emit contract for the current Point.sum known-receiver lane

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase163x_direct_emit_user_box_point_sum_contract"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
INPUT="$NYASH_ROOT/benchmarks/bench_kilo_micro_userbox_point_sum.hako"
OUT_JSON="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.json")"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"

cleanup() {
    rm -f "$OUT_JSON"
}
trap cleanup EXIT

require_smoke_path "$SMOKE_NAME" "emit route helper" "$EMIT_ROUTE" executable || exit 1
require_smoke_path "$SMOKE_NAME" "benchmark input" "$INPUT" || exit 1

if ! timeout "$RUN_TIMEOUT_SECS" "$EMIT_ROUTE" \
    --route direct \
    --timeout-secs "$RUN_TIMEOUT_SECS" \
    --out "$OUT_JSON" \
    --input "$INPUT"; then
    test_fail "$SMOKE_NAME: direct emit route failed"
    exit 1
fi

require_smoke_path "$SMOKE_NAME" "output json" "$OUT_JSON" || exit 1

if ! python3 - <<'PY' "$OUT_JSON"
import json
import sys

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as f:
    data = json.load(f)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
main_fn = functions.get("main")
method_fn = functions.get("Point.sum/0")
if main_fn is None:
    raise SystemExit("missing main function")
if method_fn is None:
    raise SystemExit("missing Point.sum/0 function")

selections = main_fn.get("metadata", {}).get("thin_entry_selections", [])
route = main_fn.get("metadata", {}).get("userbox_known_receiver_method_seed_route")
exact = main_fn.get("metadata", {}).get("exact_seed_backend_route")
if route is None:
    raise SystemExit("missing userbox_known_receiver_method_seed_route")
if route.get("kind") != "point_sum_micro" or route.get("proof") != "userbox_point_sum_micro_seed":
    raise SystemExit(f"unexpected Point.sum route: {route}")
if route.get("ops") != 2000000 or route.get("x_i64") != 1 or route.get("y_i64") != 2 or route.get("sum_i64") != 3:
    raise SystemExit(f"unexpected Point.sum route payload: {route}")
if route.get("known_receiver_count") != 2 or route.get("field_set_count") != 2:
    raise SystemExit(f"unexpected Point.sum route counts: {route}")
if exact != {
    "tag": "userbox_known_receiver_method_seed",
    "source_route": "userbox_known_receiver_method_seed_route",
    "proof": "userbox_point_sum_micro_seed",
    "selected_value": None,
}:
    raise SystemExit(f"unexpected exact route tag: {exact}")

def require_row(surface, subject, manifest_row):
    rows = [
        row for row in selections
        if row.get("surface") == surface
        and row.get("subject") == subject
        and row.get("manifest_row") == manifest_row
        and row.get("selected_entry") == "thin_internal_entry"
    ]
    if len(rows) != 1:
        raise SystemExit(f"expected one row for {surface}/{subject}/{manifest_row}, got {rows}")

require_row("user_box_field_set", "Point.x", "user_box_field_set.inline_scalar")
require_row("user_box_field_set", "Point.y", "user_box_field_set.inline_scalar")

known_rows = [
    row for row in selections
    if row.get("manifest_row") == "user_box_method.known_receiver"
    and row.get("surface") == "user_box_method"
    and row.get("subject") == "Point.sum"
    and row.get("selected_entry") == "thin_internal_entry"
]
if len(known_rows) != 2:
    raise SystemExit(f"unexpected Point.sum known_receiver rows: {known_rows}")

def iter_insts(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            yield inst

main_calls = []
for inst in iter_insts(main_fn):
    if inst.get("op") != "mir_call":
        continue
    mc = inst.get("mir_call", {})
    cal = mc.get("callee", {})
    if cal.get("type") == "Method" and cal.get("box_name") == "Point" and cal.get("name") == "sum":
        if cal.get("certainty") != "Known" or mc.get("args") != [] or cal.get("receiver") is None:
            raise SystemExit(f"unexpected Point.sum call shape: {inst}")
        main_calls.append(inst)
if len(main_calls) != 2:
    raise SystemExit(f"unexpected Point.sum call count: {main_calls}")
if sorted(row.get("value") for row in known_rows) != sorted(inst.get("dst") for inst in main_calls):
    raise SystemExit(f"known_receiver rows do not match call dsts: rows={known_rows} calls={main_calls}")

step_blocks = method_fn.get("blocks", [])
if len(step_blocks) != 1:
    raise SystemExit(f"unexpected Point.sum/0 block count: {len(step_blocks)}")
step_params = method_fn.get("params", [])
if step_params != [0]:
    raise SystemExit(f"unexpected Point.sum/0 params: {step_params}")
step_insts = step_blocks[0].get("instructions", [])

field_gets = [ins for ins in step_insts if ins.get("op") == "field_get"]
if len(field_gets) != 2:
    raise SystemExit(f"unexpected Point.sum/0 field_get count: {field_gets}")
fields = {(ins.get("field"), ins.get("box"), ins.get("declared_type", {}).get("box_type")) for ins in field_gets}
if fields != {("x", 1, "IntegerBox"), ("y", 1, "IntegerBox")}:
    raise SystemExit(f"unexpected Point.sum/0 field_get shape: {field_gets}")

binops = [ins for ins in step_insts if ins.get("op") == "binop"]
if len(binops) != 1 or binops[0].get("operation") != "+":
    raise SystemExit(f"unexpected Point.sum/0 binop shape: {binops}")
if step_insts[-1].get("op") != "ret" or step_insts[-1].get("value") != binops[0].get("dst"):
    raise SystemExit(f"unexpected Point.sum/0 return shape: {step_insts[-1]}")
PY
then
    echo "[INFO] emitted MIR:"
    sed -n '1,220p' "$OUT_JSON" || true
    test_fail "$SMOKE_NAME: direct MIR probe did not match the current Point.sum known-receiver contract"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (direct emit route pins current Point.sum known-receiver contract)"
