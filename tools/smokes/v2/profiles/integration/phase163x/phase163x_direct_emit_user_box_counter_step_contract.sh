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
if sorted(row.get("value") for row in known_rows) != [24, 44]:
    raise SystemExit(f"unexpected Counter.step known_receiver values: {known_rows}")

def find_block(fn, block_id):
    for block in fn.get("blocks", []):
        if block.get("id") == block_id:
            return block
    raise SystemExit(f"missing block {block_id}")

def require_method_call(block_id, inst_idx, receiver, dst):
    block = find_block(main_fn, block_id)
    instructions = block.get("instructions", [])
    if inst_idx >= len(instructions):
        raise SystemExit(f"missing instruction {block_id}#{inst_idx}")
    inst = instructions[inst_idx]
    if inst.get("op") != "mir_call":
        raise SystemExit(f"expected mir_call at {block_id}#{inst_idx}")
    mc = inst.get("mir_call", {})
    cal = mc.get("callee", {})
    if cal.get("type") != "Method":
        raise SystemExit(f"expected Method callee at {block_id}#{inst_idx}: {cal}")
    if cal.get("box_name") != "Counter" or cal.get("certainty") != "Known" or cal.get("name") != "step":
        raise SystemExit(f"unexpected method callee at {block_id}#{inst_idx}: {cal}")
    if cal.get("receiver") != receiver or mc.get("args") != [] or inst.get("dst") != dst:
        raise SystemExit(
            f"unexpected receiver/args/dst at {block_id}#{inst_idx}: receiver={cal.get('receiver')} args={mc.get('args')} dst={inst.get('dst')}"
        )

require_method_call(22, 3, 33, 24)
require_method_call(24, 3, 47, 44)

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
