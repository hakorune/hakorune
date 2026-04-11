#!/bin/bash
# phase-163x direct emit contract for the recursive Counter.step_chain known-receiver lane
#
# Contract:
# 1) the trustworthy direct MIR probe (`emit_mir_route.sh --route direct`) emits
#    canonical known-user-box method calls for `bench_kilo_micro_userbox_counter_step_chain.hako`
# 2) `main` carries:
#      - `user_box_field_set.inline_scalar` for `Counter.value`
#      - two `user_box_method.known_receiver` rows for `Counter.step_chain`
# 3) `Counter.step_chain/0` forwards to canonical `Counter.step/0`
# 4) `Counter.step/0` keeps the typed `field_get Counter.value : IntegerBox` body

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase163x_direct_emit_user_box_counter_step_chain_contract"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
INPUT="$NYASH_ROOT/benchmarks/bench_kilo_micro_userbox_counter_step_chain.hako"
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
chain_fn = functions.get("Counter.step_chain/0")
leaf_fn = functions.get("Counter.step/0")
if main_fn is None:
    raise SystemExit("missing main function")
if chain_fn is None:
    raise SystemExit("missing Counter.step_chain/0 function")
if leaf_fn is None:
    raise SystemExit("missing Counter.step/0 function")

if len(main_fn.get("blocks", [])) != 5:
    raise SystemExit(f"unexpected main block count: {len(main_fn.get('blocks', []))}")

selections = main_fn.get("metadata", {}).get("thin_entry_selections", [])

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

require_row("user_box_field_set", "Counter.value", "user_box_field_set.inline_scalar")

known_rows = [
    row for row in selections
    if row.get("manifest_row") == "user_box_method.known_receiver"
    and row.get("surface") == "user_box_method"
    and row.get("subject") == "Counter.step_chain"
    and row.get("selected_entry") == "thin_internal_entry"
]
if len(known_rows) != 2:
    raise SystemExit(f"unexpected Counter.step_chain known_receiver rows: {known_rows}")

def iter_insts(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            yield inst

def check_method_call(inst, box_name, method_name):
    if inst.get("op") != "mir_call":
        raise SystemExit(f"expected mir_call, got {inst}")
    mc = inst.get("mir_call", {})
    cal = mc.get("callee", {})
    if cal.get("type") != "Method":
        raise SystemExit(f"expected Method callee: {cal}")
    if cal.get("box_name") != box_name or cal.get("certainty") != "Known" or cal.get("name") != method_name:
        raise SystemExit(f"unexpected callee shape: {cal}")
    if mc.get("args") != []:
        raise SystemExit(f"unexpected call args: {mc.get('args')}")

def check_forward_call(inst, receiver):
    if inst.get("op") != "mir_call":
        raise SystemExit(f"expected mir_call, got {inst}")
    mc = inst.get("mir_call", {})
    cal = mc.get("callee", {})
    if cal.get("type") == "Method":
        if cal.get("box_name") != "Counter" or cal.get("certainty") != "Known" or cal.get("name") != "step":
            raise SystemExit(f"unexpected method callee: {cal}")
        if cal.get("receiver") != receiver:
            raise SystemExit(f"unexpected method receiver: {cal}")
        if mc.get("args") != []:
            raise SystemExit(f"unexpected method call args: {mc.get('args')}")
    elif cal.get("type") == "Global":
        if cal.get("name") != "Counter.step/0":
            raise SystemExit(f"unexpected global callee: {cal}")
        if mc.get("args") != [receiver]:
            raise SystemExit(f"unexpected global call args: {mc.get('args')}")
    else:
        raise SystemExit(f"unexpected chain callee: {cal}")

main_calls = [inst for inst in iter_insts(main_fn)
              if inst.get("op") == "mir_call"
              and inst.get("mir_call", {}).get("callee", {}).get("box_name") == "Counter"
              and inst.get("mir_call", {}).get("callee", {}).get("name") == "step_chain"]
if len(main_calls) != 2:
    raise SystemExit(f"unexpected main Counter.step_chain call count: {len(main_calls)}")
for inst in main_calls:
    check_method_call(inst, "Counter", "step_chain")

chain_insts = list(iter_insts(chain_fn))
if len(chain_fn.get("blocks", [])) != 1:
    raise SystemExit(f"unexpected Counter.step_chain/0 block count: {len(chain_fn.get('blocks', []))}")

chain_copies = [ins for ins in chain_insts if ins.get("op") == "copy"]
chain_call = next((ins for ins in chain_insts if ins.get("op") == "mir_call"), None)
chain_ret = next((ins for ins in chain_insts if ins.get("op") == "ret"), None)
if len(chain_copies) != 2 or chain_call is None or chain_ret is None:
    raise SystemExit(f"unexpected Counter.step_chain/0 forwarding shape: {chain_insts}")
if chain_copies[0].get("src") != 0:
    raise SystemExit(f"unexpected Counter.step_chain/0 copy shape: {chain_copies[0]}")
if chain_copies[1].get("src") != chain_copies[0].get("dst"):
    raise SystemExit(f"unexpected Counter.step_chain/0 copy chain: {chain_copies}")
check_forward_call(chain_call, chain_copies[1].get("dst"))
if chain_ret.get("value") != chain_call.get("dst"):
    raise SystemExit(f"unexpected Counter.step_chain/0 return value: {chain_ret}")

leaf_blocks = leaf_fn.get("blocks", [])
if len(leaf_fn.get("blocks", [])) != 1:
    raise SystemExit(f"unexpected Counter.step/0 block count: {len(leaf_blocks)}")
if leaf_fn.get("params", []) != [0]:
    raise SystemExit(f"unexpected Counter.step/0 params: {leaf_fn.get('params', [])}")
leaf_insts = list(iter_insts(leaf_fn))
field_get = next((ins for ins in leaf_insts if ins.get("op") == "field_get"), None)
const = next((ins for ins in leaf_insts if ins.get("op") == "const"), None)
binop = next((ins for ins in leaf_insts if ins.get("op") == "binop"), None)
ret = next((ins for ins in leaf_insts if ins.get("op") == "ret"), None)
if field_get is None or const is None or binop is None or ret is None:
    raise SystemExit(f"unexpected Counter.step/0 leaf shape: {leaf_insts}")
declared = field_get.get("declared_type", {})
copy_from_recv = next((ins for ins in leaf_insts if ins.get("op") == "copy" and ins.get("src") == 0), None)
if copy_from_recv is None:
    raise SystemExit(f"missing Counter.step/0 receiver copy: {leaf_insts}")
if field_get.get("field") != "value" or field_get.get("box") != copy_from_recv.get("dst"):
    raise SystemExit(f"unexpected Counter.step/0 field_get shape: {field_get}")
if declared.get("box_type") != "IntegerBox":
    raise SystemExit(f"unexpected Counter.step/0 declared type: {declared}")
if const.get("value", {}).get("value") != 2:
    raise SystemExit(f"unexpected Counter.step/0 const shape: {const}")
if binop.get("operation") != "+":
    raise SystemExit(f"unexpected Counter.step/0 binop shape: {binop}")
if ret.get("value") != binop.get("dst"):
    raise SystemExit(f"unexpected Counter.step/0 return shape: {ret}")
PY
then
    echo "[INFO] emitted MIR:"
    sed -n '1,220p' "$OUT_JSON" || true
    test_fail "$SMOKE_NAME: direct MIR probe did not match the recursive Counter.step_chain known-receiver contract"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (direct emit route pins recursive Counter.step_chain known-receiver contract)"
