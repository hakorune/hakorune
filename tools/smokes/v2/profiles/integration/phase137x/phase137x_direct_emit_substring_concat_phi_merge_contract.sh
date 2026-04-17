#!/bin/bash
# phase-137x direct emit smoke for the current substring-concat loop-carried phi route
#
# Contract:
# 1) the trustworthy direct MIR probe (`emit_mir_route.sh --route direct`) emits strict JSON
#    for `bench_kilo_micro_substring_concat.hako`.
# 2) the header still keeps the loop contract:
#      - three `phi`s
#      - positive loop bound
#      - compare `<`
#      - branch
# 3) the loop still carries the string lane through `%21 = phi([4,0], [22,20])`.
# 4) the backedge still hands `%36` into `%22 = phi([36,19])`.
# 5) proof-bearing corridor metadata still lives on `%36`.
# 6) the relation metadata now makes the stop line explicit:
#      - `%22` uses `preserve_plan_window`
#      - `%21` uses `stop_at_merge`
#      - `%21` also carries `stable_length_scalar` with witness `%5`
#    and the candidates still reflect that:
#      - `%22` preserves the proof-bearing plan window
#      - `%21` keeps only non-window `publication_sink` /
#        `materialization_sink` / `direct_kernel_entry` continuity.
# 7) the latch still increments the trip counter with `const 1` and `+`.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase137x_direct_emit_substring_concat_phi_merge_contract"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
INPUT="$NYASH_ROOT/benchmarks/bench_kilo_micro_substring_concat.hako"
OUT_DIR="$NYASH_ROOT/target/smokes/phase137x"
OUT_JSON="$OUT_DIR/${SMOKE_NAME}.$$.$RANDOM.json"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"

cleanup() {
    rm -f "$OUT_JSON"
}
trap cleanup EXIT

mkdir -p "$OUT_DIR"

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

header_ops = [ins for ins in blocks[1].get("instructions", []) if ins.get("op") != "copy"]
if len(header_ops) != 6:
    raise SystemExit(f"unexpected header interesting_n={len(header_ops)}")
if [ins.get("op") for ins in header_ops[:3]] != ["phi", "phi", "phi"]:
    raise SystemExit(f"unexpected header phi prefix: {[ins.get('op') for ins in header_ops[:3]]}")
if header_ops[3].get("op") != "const" or header_ops[4].get("op") != "compare" or header_ops[5].get("op") != "branch":
    raise SystemExit(f"unexpected header suffix ops: {[ins.get('op') for ins in header_ops[3:]]}")
if header_ops[3].get("value", {}).get("value", 0) <= 0:
    raise SystemExit(f"unexpected non-positive loop bound: {header_ops[3].get('value')}")
if header_ops[4].get("operation") != "<":
    raise SystemExit(f"unexpected header compare op: {header_ops[4].get('operation')}")

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

latch_ops = [ins for ins in blocks[3].get("instructions", []) if ins.get("op") != "copy"]
if len(latch_ops) != 5:
    raise SystemExit(f"unexpected latch interesting_n={len(latch_ops)}")
if latch_ops[2].get("op") != "const" or latch_ops[2].get("value", {}).get("value") != 1:
    raise SystemExit(f"unexpected latch increment const: {latch_ops[2]}")
if latch_ops[3].get("op") != "binop" or latch_ops[3].get("operation") != "+":
    raise SystemExit(f"unexpected latch increment op: {latch_ops[3]}")

def require_kinds(candidate_list, required, label):
    if not isinstance(candidate_list, list) or not candidate_list:
        raise SystemExit(f"missing candidates for {label}")
    kinds = {cand.get('kind') for cand in candidate_list}
    missing = required - kinds
    if missing:
        raise SystemExit(f"{label} missing candidate kinds: {sorted(missing)}")

candidates = main_fn.get("metadata", {}).get("string_corridor_candidates", {})
relations = main_fn.get("metadata", {}).get("string_corridor_relations", {})
helper_candidates = candidates.get("36")
if not isinstance(helper_candidates, list) or not helper_candidates:
    raise SystemExit("missing helper-result candidates for %36")

required_helper_kinds = {"publication_sink", "materialization_sink", "direct_kernel_entry"}
require_kinds(helper_candidates, required_helper_kinds, "helper-result %36")
if not any(cand.get("plan") for cand in helper_candidates):
    raise SystemExit("helper-result %36 lost proof-bearing plan metadata")

required_phi_kinds = {"publication_sink", "materialization_sink", "direct_kernel_entry"}

phi22_relations = relations.get("22")
if not isinstance(phi22_relations, list) or not phi22_relations:
    raise SystemExit("missing phi %22 relations")
if not any(
    rel.get("kind") == "phi_carry_base"
    and rel.get("base_value") == 36
    and rel.get("window_contract") == "preserve_plan_window"
    for rel in phi22_relations
):
    raise SystemExit(f"phi %22 lost preserve_plan_window relation: {phi22_relations}")

phi22_candidates = candidates.get("22")
require_kinds(phi22_candidates, required_phi_kinds, "phi %22")
if not any(cand.get("plan") is not None for cand in phi22_candidates):
    raise SystemExit(f"phi %22 lost the single-input carried plan window: {phi22_candidates}")
if any(cand.get("kind") == "borrowed_corridor_fusion" for cand in phi22_candidates):
    raise SystemExit(f"phi %22 unexpectedly gained borrow-producing fusion: {phi22_candidates}")

phi21_relations = relations.get("21")
if not isinstance(phi21_relations, list) or not phi21_relations:
    raise SystemExit("missing phi %21 relations")
if not any(
    rel.get("kind") == "phi_carry_base"
    and rel.get("base_value") == 36
    and rel.get("window_contract") == "stop_at_merge"
    for rel in phi21_relations
):
    raise SystemExit(f"phi %21 lost stop_at_merge relation: {phi21_relations}")

if not any(
    rel.get("kind") == "stable_length_scalar"
    and rel.get("base_value") == 36
    and rel.get("witness_value") == 5
    and rel.get("window_contract") == "stop_at_merge"
    for rel in phi21_relations
):
    raise SystemExit(f"phi %21 lost stable_length_scalar witness: {phi21_relations}")

phi21_candidates = candidates.get("21")
require_kinds(phi21_candidates, required_phi_kinds, "phi %21")
if any(cand.get("plan") is not None for cand in phi21_candidates):
    raise SystemExit(f"phi %21 unexpectedly widened a merged plan window: {phi21_candidates}")
if any(cand.get("kind") == "borrowed_corridor_fusion" for cand in phi21_candidates):
    raise SystemExit(f"phi %21 unexpectedly gained borrow-producing fusion: {phi21_candidates}")
PY
then
    echo "[INFO] emitted MIR:"
    sed -n '1,220p' "$OUT_JSON" || true
    test_fail "$SMOKE_NAME: direct MIR probe did not match the current phi-merge contract"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (direct emit route pins the current substring-concat phi-merge carry contract)"
