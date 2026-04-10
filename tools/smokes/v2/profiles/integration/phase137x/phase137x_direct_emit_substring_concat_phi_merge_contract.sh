#!/bin/bash
# phase-137x direct emit smoke for the current substring-concat loop-carried phi route
#
# Contract:
# 1) the trustworthy direct MIR probe (`emit_mir_route.sh --route direct`) emits strict JSON
#    for `bench_kilo_micro_substring_concat.hako`.
# 2) the loop still carries the string lane through `%21 = phi([4,0], [22,20])`.
# 3) the backedge still hands `%36` into `%22 = phi([36,19])`.
# 4) proof-bearing corridor metadata still lives on `%36`.
# 5) the carried `%21` / `%22` values now keep non-window `publication_sink` /
#    `materialization_sink` / `direct_kernel_entry` candidates, but still do not
#    widen the plan window across the phi route.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase137x_direct_emit_substring_concat_phi_merge_contract"
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
if len(blocks) != 5:
    raise SystemExit(f"unexpected block count: {len(blocks)}")

def find_phi(block, dst):
    for ins in block.get("instructions", []):
        if ins.get("op") == "phi" and ins.get("dst") == dst:
            return ins
    return None

header_phi = find_phi(blocks[1], 21)
if header_phi is None:
    raise SystemExit("missing loop-carried string phi %21 in header block")
if header_phi.get("incoming") != [[4, 0], [22, 20]]:
    raise SystemExit(f"unexpected %21 incoming edges: {header_phi.get('incoming')}")

backedge_phi = find_phi(blocks[3], 22)
if backedge_phi is None:
    raise SystemExit("missing backedge phi %22 in latch block")
if backedge_phi.get("incoming") != [[36, 19]]:
    raise SystemExit(f"unexpected %22 incoming edges: {backedge_phi.get('incoming')}")

def require_kinds(candidate_list, required, label):
    if not isinstance(candidate_list, list) or not candidate_list:
        raise SystemExit(f"missing candidates for {label}")
    kinds = {cand.get('kind') for cand in candidate_list}
    missing = required - kinds
    if missing:
        raise SystemExit(f"{label} missing candidate kinds: {sorted(missing)}")

candidates = main_fn.get("metadata", {}).get("string_corridor_candidates", {})
helper_candidates = candidates.get("36")
if not isinstance(helper_candidates, list) or not helper_candidates:
    raise SystemExit("missing helper-result candidates for %36")

required_helper_kinds = {"publication_sink", "materialization_sink", "direct_kernel_entry"}
require_kinds(helper_candidates, required_helper_kinds, "helper-result %36")
if not any(cand.get("plan") for cand in helper_candidates):
    raise SystemExit("helper-result %36 lost proof-bearing plan metadata")

required_phi_kinds = {"publication_sink", "materialization_sink", "direct_kernel_entry"}
for phi_dst in ("21", "22"):
    phi_candidates = candidates.get(phi_dst)
    require_kinds(phi_candidates, required_phi_kinds, f"phi %{phi_dst}")
    if any(cand.get("plan") is not None for cand in phi_candidates):
        raise SystemExit(f"phi %{phi_dst} unexpectedly widened a plan window: {phi_candidates}")
    if any(cand.get("kind") == "borrowed_corridor_fusion" for cand in phi_candidates):
        raise SystemExit(f"phi %{phi_dst} unexpectedly gained borrow-producing fusion: {phi_candidates}")
PY
then
    echo "[INFO] emitted MIR:"
    sed -n '1,220p' "$OUT_JSON" || true
    test_fail "$SMOKE_NAME: direct MIR probe did not match the current phi-merge contract"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (direct emit route pins the current substring-concat phi-merge carry contract)"
