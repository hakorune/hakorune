#!/bin/bash
# phase-163x direct emit contract for the current Counter.step known-receiver lane
#
# Contract:
# 1) the trustworthy direct MIR probe (`emit_mir_route.sh --route direct`) emits
#    canonical known-user-box method calls for `bench_kilo_micro_userbox_counter_step.hako`
# 2) `main` carries:
#      - `user_box_field_set.inline_scalar` for `Counter.value`
#      - two `user_box_method.known_receiver` rows for `Counter.step`
# 3) the loop and exit callsites stay on canonical `Counter.step` method calls,
#    not `RuntimeDataBox` union dispatch or `Global Counter.step/0`
# 4) `Counter.step/0` keeps the typed `field_get Counter.value : IntegerBox` body

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase163x_direct_emit_user_box_counter_step_contract"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
INPUT="$NYASH_ROOT/benchmarks/bench_kilo_micro_userbox_counter_step.hako"
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
method_fn = functions.get("Counter.step/0")
if main_fn is None:
    raise SystemExit("missing main function")
if method_fn is None:
    raise SystemExit("missing Counter.step/0 function")

selections = main_fn.get("metadata", {}).get("thin_entry_selections", [])
route = main_fn.get("metadata", {}).get("userbox_known_receiver_method_seed_route")
exact = main_fn.get("metadata", {}).get("exact_seed_backend_route")
if route is None:
    raise SystemExit("missing userbox_known_receiver_method_seed_route")
if route.get("kind") != "counter_step_micro" or route.get("proof") != "userbox_counter_step_micro_seed":
    raise SystemExit(f"unexpected Counter.step route: {route}")
if route.get("ops") != 2000000 or route.get("base_i64") != 41 or route.get("delta_i64") != 2 or route.get("step_i64") != 43:
    raise SystemExit(f"unexpected Counter.step route payload: {route}")
if route.get("known_receiver_count") != 2 or route.get("field_set_count") != 1:
    raise SystemExit(f"unexpected Counter.step route counts: {route}")
if exact != {
    "tag": "userbox_known_receiver_method_seed",
    "source_route": "userbox_known_receiver_method_seed_route",
    "proof": "userbox_counter_step_micro_seed",
    "selected_value": None,
}:
    raise SystemExit(f"unexpected exact route tag: {exact}")
field_rows = [
    row for row in selections
    if row.get("manifest_row") == "user_box_field_set.inline_scalar"
    and row.get("surface") == "user_box_field_set"
    and row.get("subject") == "Counter.value"
    and row.get("selected_entry") == "thin_internal_entry"
]
if len(field_rows) != 1:
    raise SystemExit(f"expected one Counter.value field-set row, got {len(field_rows)}")

known_rows = [
    row for row in selections
    if row.get("manifest_row") == "user_box_method.known_receiver"
    and row.get("surface") == "user_box_method"
    and row.get("subject") == "Counter.step"
    and row.get("selected_entry") == "thin_internal_entry"
]
if len(known_rows) != 2:
    raise SystemExit(f"unexpected Counter.step known_receiver rows: {known_rows}")

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
    if cal.get("type") == "Method" and cal.get("box_name") == "Counter" and cal.get("name") == "step":
        if cal.get("certainty") != "Known" or mc.get("args") != [] or cal.get("receiver") is None:
            raise SystemExit(f"unexpected Counter.step call shape: {inst}")
        main_calls.append(inst)
if len(main_calls) != 2:
    raise SystemExit(f"unexpected Counter.step call count: {main_calls}")
if sorted(row.get("value") for row in known_rows) != sorted(inst.get("dst") for inst in main_calls):
    raise SystemExit(f"known_receiver rows do not match call dsts: rows={known_rows} calls={main_calls}")

step_blocks = method_fn.get("blocks", [])
if len(step_blocks) != 1:
    raise SystemExit(f"unexpected Counter.step/0 block count: {len(step_blocks)}")
step_params = method_fn.get("params", [])
if step_params != [0]:
    raise SystemExit(f"unexpected Counter.step/0 params: {step_params}")
step_insts = step_blocks[0].get("instructions", [])
copies = [ins for ins in step_insts if ins.get("op") == "copy"]
if not copies or copies[0].get("src") != 0 or copies[0].get("dst") != 1:
    raise SystemExit(f"unexpected Counter.step/0 receiver copy shape: {copies}")
field_get = next((ins for ins in step_insts if ins.get("op") == "field_get"), None)
if field_get is None:
    raise SystemExit("missing field_get in Counter.step/0")
declared = field_get.get("declared_type", {})
if field_get.get("field") != "value" or field_get.get("box") != 1:
    raise SystemExit(f"unexpected Counter.step/0 field_get shape: {field_get}")
if declared.get("box_type") != "IntegerBox":
    raise SystemExit(f"unexpected Counter.step/0 declared type: {declared}")
PY
then
    echo "[INFO] emitted MIR:"
    sed -n '1,180p' "$OUT_JSON" || true
    test_fail "$SMOKE_NAME: direct MIR probe did not match the current Counter.step known-receiver contract"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (direct emit route pins current Counter.step known-receiver contract)"
