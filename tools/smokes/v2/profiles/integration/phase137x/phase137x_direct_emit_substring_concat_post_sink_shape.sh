#!/bin/bash
# phase-137x direct emit shape smoke for the current substring-concat post-sink body
#
# Contract:
# 1) the trustworthy direct MIR probe (`emit_mir_route.sh --route direct`) emits strict JSON
#    for `bench_kilo_micro_substring_concat.hako`.
# 2) the preheader still computes the seed length via `StringBox.length()`,
#    and the exit still returns `length() + ...`.
# 3) the loop body keeps the current live post-sink non-copy shape:
#      - 14 interesting ops
#      - shared-source substring producers at positions 3 and 4
#      - collapsed `source_len + const_len` at position 7
#      - `nyash.string.substring_concat3_hhhii` at position 12
#      - no loop `nyash.string.substring_len_hii`
# 4) the helper result keeps its live proof-bearing corridor metadata:
#      - `%36` carries `publication_sink` and `direct_kernel_entry`
#      - both plans keep `source_root=21` and outer window `%71..%72`
# 5) exported `string_kernel_plans` also keeps the backend-consumable concat-triplet plan:
#      - `%36` exports `family=concat_triplet_window`
#      - consumer stays `direct_kernel_entry`
#      - parts stay `[slice, const, slice]`

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase137x_direct_emit_substring_concat_post_sink_shape"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
INPUT="$NYASH_ROOT/benchmarks/bench_kilo_micro_substring_concat.hako"
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

main_fn = next((fn for fn in data.get("functions", []) if fn.get("name") == "main"), None)
if main_fn is None:
    raise SystemExit("missing main function")

blocks = main_fn.get("blocks", [])
if len(blocks) < 3:
    raise SystemExit(f"unexpected block count: {len(blocks)}")

def interesting_ops(block_idx):
    return [ins for ins in blocks[block_idx].get("instructions", []) if ins.get("op") != "copy"]

preheader = interesting_ops(0)
if len(preheader) != 6:
    raise SystemExit(f"unexpected preheader interesting_n={len(preheader)}")

preheader_len = preheader[2]
if preheader_len.get("op") != "mir_call":
    raise SystemExit("expected preheader length mir_call at interesting[2]")
preheader_cal = preheader_len.get("mir_call", {}).get("callee", {})
preheader_box = preheader_cal.get("box_name") or preheader_cal.get("box_type")
preheader_name = preheader_cal.get("method") or preheader_cal.get("name")
preheader_args = preheader_len.get("mir_call", {}).get("args")
if preheader_box != "StringBox" or preheader_name != "length" or not isinstance(preheader_args, list) or len(preheader_args) != 0:
    raise SystemExit(
        f"unexpected preheader length call: box={preheader_box} name={preheader_name} args={preheader_args}"
    )

interesting = interesting_ops(2)
if len(interesting) != 14:
    raise SystemExit(f"unexpected interesting_n={len(interesting)}")

def callee_name(ins):
    mc = ins.get("mir_call", {})
    cal = mc.get("callee", {})
    return cal.get("method") or cal.get("name")

candidates = main_fn.get("metadata", {}).get("string_corridor_candidates", {})
kernel_plans = main_fn.get("metadata", {}).get("string_kernel_plans", {})

def has_candidate(value, kind):
    items = candidates.get(str(value))
    if not isinstance(items, list):
        return False
    return any(cand.get("kind") == kind for cand in items)

def find_candidate(value, kind):
    items = candidates.get(str(value))
    if not isinstance(items, list):
        return None
    for cand in items:
        if cand.get("kind") == kind:
            return cand
    return None

def require_slice(idx, start, end):
    ins = interesting[idx]
    if ins.get("op") != "mir_call" or callee_name(ins) != "substring":
        raise SystemExit(f"expected substring producer at interesting[{idx}]")
    cand = find_candidate(ins.get("dst"), "publication_sink")
    if cand is None:
        raise SystemExit(f"expected publication_sink candidate at interesting[{idx}]")
    plan = cand.get("plan", {})
    if plan.get("source_root") != 21 or plan.get("start") != start or plan.get("end") != end:
        raise SystemExit(
            f"unexpected publication_sink plan on interesting[{idx}]: "
            f"source_root={plan.get('source_root')} start={plan.get('start')} end={plan.get('end')}"
        )

require_slice(3, 46, 47)
require_slice(4, 47, 5)

for inst in interesting:
    if callee_name(inst) == "nyash.string.substring_len_hii":
        raise SystemExit(f"unexpected substring_len_hii in post-sink body: {inst}")

if interesting[6].get("op") != "const" or interesting[6].get("value", {}).get("value") != 2:
    raise SystemExit(f"expected const_len at interesting[6], got {interesting[6]}")

if interesting[7].get("op") != "binop" or interesting[7].get("operation") != "+":
    raise SystemExit(f"expected collapsed source_len + const_len at interesting[7], got {interesting[7]}")
operands = {interesting[7].get("lhs"), interesting[7].get("rhs")}
if 5 not in operands or interesting[6].get("dst") not in operands:
    raise SystemExit(f"collapsed len add should use source_len %5 and const_len %{interesting[6].get('dst')}: {interesting[7]}")

if interesting[12].get("op") != "mir_call" or callee_name(interesting[12]) != "nyash.string.substring_concat3_hhhii":
    raise SystemExit("expected substring_concat3_hhhii at interesting[12]")

helper_candidates = candidates.get("36")
if not isinstance(helper_candidates, list) or not helper_candidates:
    raise SystemExit("missing string_corridor_candidates for helper result %36")

def find_helper_candidate(kind):
    for cand in helper_candidates:
        if cand.get("kind") == kind:
            return cand
    return None

for kind in ("publication_sink", "direct_kernel_entry"):
    cand = find_helper_candidate(kind)
    if cand is None:
        raise SystemExit(f"missing {kind} candidate on helper result %36")
    plan = cand.get("plan", {})
    if plan.get("source_root") != 21 or plan.get("start") != 71 or plan.get("end") != 72:
        raise SystemExit(
            f"unexpected {kind} plan window on helper result %36: "
            f"source_root={plan.get('source_root')} start={plan.get('start')} end={plan.get('end')}"
        )

helper_kernel_plan = kernel_plans.get("36")
if not isinstance(helper_kernel_plan, dict):
    raise SystemExit("missing string_kernel_plans export for helper result %36")
if helper_kernel_plan.get("family") != "concat_triplet_window":
    raise SystemExit(f"unexpected helper kernel plan family: {helper_kernel_plan}")
if helper_kernel_plan.get("consumer") != "direct_kernel_entry":
    raise SystemExit(f"unexpected helper kernel plan consumer: {helper_kernel_plan}")
if helper_kernel_plan.get("source_root") != 21:
    raise SystemExit(f"unexpected helper kernel plan source_root: {helper_kernel_plan}")
parts = helper_kernel_plan.get("parts")
if not isinstance(parts, list) or [part.get("kind") for part in parts] != ["slice", "const", "slice"]:
    raise SystemExit(f"unexpected helper kernel plan parts: {helper_kernel_plan}")
if helper_kernel_plan.get("known_length") != 2:
    raise SystemExit(f"unexpected helper kernel plan known_length: {helper_kernel_plan}")
if helper_kernel_plan.get("retained_form") != "borrowed_text":
    raise SystemExit(f"unexpected helper kernel plan retained_form: {helper_kernel_plan}")
if parts[1].get("literal") != "xx":
    raise SystemExit(f"unexpected helper kernel plan middle literal: {helper_kernel_plan}")
if helper_kernel_plan.get("barriers", {}).get("publication") != "candidate":
    raise SystemExit(f"unexpected helper kernel plan publication barrier: {helper_kernel_plan}")
if helper_kernel_plan.get("direct_kernel_entry", {}).get("state") != "candidate":
    raise SystemExit(f"unexpected helper kernel plan direct entry state: {helper_kernel_plan}")
loop_payload = helper_kernel_plan.get("loop_payload", {})
if loop_payload.get("seed_literal") != "line-seed-abcdef":
    raise SystemExit(f"unexpected helper kernel plan seed literal: {helper_kernel_plan}")
if loop_payload.get("seed_length") != 16 or loop_payload.get("split_length") != 8:
    raise SystemExit(f"unexpected helper kernel plan seed/split payload: {helper_kernel_plan}")
if loop_payload.get("loop_bound") != 300000:
    raise SystemExit(f"unexpected helper kernel plan loop bound: {helper_kernel_plan}")

exit_ops = interesting_ops(4)
if len(exit_ops) != 3:
    raise SystemExit(f"unexpected exit interesting_n={len(exit_ops)}")

exit_len = exit_ops[0]
if exit_len.get("op") != "mir_call":
    raise SystemExit("expected exit length mir_call at interesting[0]")
exit_cal = exit_len.get("mir_call", {}).get("callee", {})
exit_box = exit_cal.get("box_name") or exit_cal.get("box_type")
exit_name = exit_cal.get("method") or exit_cal.get("name")
exit_args = exit_len.get("mir_call", {}).get("args")
if exit_box not in ("RuntimeDataBox", "StringBox") or exit_name != "length" or not isinstance(exit_args, list) or len(exit_args) != 0:
    raise SystemExit(
        f"unexpected exit length call: box={exit_box} name={exit_name} args={exit_args}"
    )

if exit_ops[1].get("op") != "binop" or exit_ops[1].get("operation") != "+":
    raise SystemExit("expected exit addition at interesting[1]")
if exit_ops[2].get("op") != "ret":
    raise SystemExit("expected exit ret at interesting[2]")
PY
then
    echo "[INFO] emitted MIR:"
    sed -n '1,120p' "$OUT_JSON" || true
    test_fail "$SMOKE_NAME: direct MIR probe did not match the current post-sink body shape"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (direct emit route pins current post-sink substring-concat body shape)"
