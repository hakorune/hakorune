#!/bin/bash
# phase-137x direct emit shape smoke for the current substring-concat post-sink body
#
# Contract:
# 1) the trustworthy direct MIR probe (`emit_mir_route.sh --route direct`) emits strict JSON
#    for `bench_kilo_micro_substring_concat.hako`.
# 2) the preheader still computes the seed length via `StringBox.length()`,
#    and the exit still returns `length() + ...`.
# 3) the loop body keeps the current live post-sink non-copy shape:
#      - 17 interesting ops
#      - shared-source substring producers at positions 3 and 4
#      - direct-kernel scalar-consumer candidates at positions 6 and 9
#      - `nyash.string.substring_concat3_hhhii` at position 15
# 4) the helper result keeps its live proof-bearing corridor metadata:
#      - `%36` carries `publication_sink` and `direct_kernel_entry`
#      - both plans keep `source_root=21` and outer window `%71..%72`

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
if len(interesting) != 17:
    raise SystemExit(f"unexpected interesting_n={len(interesting)}")

def callee_name(ins):
    mc = ins.get("mir_call", {})
    cal = mc.get("callee", {})
    return cal.get("method") or cal.get("name")

candidates = main_fn.get("metadata", {}).get("string_corridor_candidates", {})

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

if interesting[15].get("op") != "mir_call" or callee_name(interesting[15]) != "nyash.string.substring_concat3_hhhii":
    raise SystemExit("expected substring_concat3_hhhii at interesting[15]")

if interesting[6].get("op") != "mir_call" or not has_candidate(interesting[6].get("dst"), "direct_kernel_entry"):
    raise SystemExit("expected direct_kernel_entry candidate at interesting[6]")

if interesting[9].get("op") != "mir_call" or not has_candidate(interesting[9].get("dst"), "direct_kernel_entry"):
    raise SystemExit("expected direct_kernel_entry candidate at interesting[9]")

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
