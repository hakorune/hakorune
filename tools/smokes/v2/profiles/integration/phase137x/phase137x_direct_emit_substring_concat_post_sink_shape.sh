#!/bin/bash
# phase-137x direct emit shape smoke for the current substring-concat post-sink body
#
# Contract:
# 1) the trustworthy direct MIR probe (`emit_mir_route.sh --route direct`) emits strict JSON
#    for `bench_kilo_micro_substring_concat.hako`.
# 2) the loop body keeps the current live post-sink non-copy shape:
#      - 17 interesting ops
#      - `nyash.string.substring_len_hii` at positions 6 and 9
#      - `nyash.string.substring_concat3_hhhii` at position 15

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

body = blocks[2].get("instructions", [])
interesting = [ins for ins in body if ins.get("op") != "copy"]
if len(interesting) != 17:
    raise SystemExit(f"unexpected interesting_n={len(interesting)}")

def callee_name(ins):
    mc = ins.get("mir_call", {})
    cal = mc.get("callee", {})
    return cal.get("method") or cal.get("name")

if interesting[6].get("op") != "mir_call" or callee_name(interesting[6]) != "nyash.string.substring_len_hii":
    raise SystemExit("expected substring_len_hii at interesting[6]")

if interesting[9].get("op") != "mir_call" or callee_name(interesting[9]) != "nyash.string.substring_len_hii":
    raise SystemExit("expected substring_len_hii at interesting[9]")

if interesting[15].get("op") != "mir_call" or callee_name(interesting[15]) != "nyash.string.substring_concat3_hhhii":
    raise SystemExit("expected substring_concat3_hhhii at interesting[15]")
PY
then
    echo "[INFO] emitted MIR:"
    sed -n '1,120p' "$OUT_JSON" || true
    test_fail "$SMOKE_NAME: direct MIR probe did not match the current post-sink body shape"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (direct emit route pins current post-sink substring-concat body shape)"
