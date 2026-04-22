#!/bin/bash
# phase-137x direct emit smoke for the current substring-concat phi metadata route
#
# Contract:
# 1) direct MIR for `bench_kilo_micro_substring_concat.hako` still has a
#    loop-carried string phi lane.
# 2) relation metadata still distinguishes the preserving latch phi from the
#    merged stop-at-merge phi.
# 3) the exact backend route points at a selected StringKernelPlan key instead
#    of relying on hard-coded value ids in this smoke.

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

string_phis = []
for block in main_fn.get("blocks", []):
    for ins in block.get("instructions", []):
        if ins.get("op") != "phi":
            continue
        dst_type = ins.get("dst_type")
        if isinstance(dst_type, dict) and dst_type.get("kind") == "string":
            string_phis.append(ins)

if not any(len(phi.get("incoming") or []) == 2 for phi in string_phis):
    raise SystemExit(f"missing merged string phi lane: {string_phis}")
if not any(len(phi.get("incoming") or []) == 1 for phi in string_phis):
    raise SystemExit(f"missing preserving latch string phi lane: {string_phis}")

metadata = main_fn.get("metadata") or {}
backend = metadata.get("exact_seed_backend_route")
if not isinstance(backend, dict) or backend.get("tag") != "substring_concat_loop_ascii":
    raise SystemExit(f"missing substring-concat exact backend route: {backend}")

selected = backend.get("selected_value")
plans = metadata.get("string_kernel_plans") or {}
plan = plans.get(str(selected))
if not isinstance(selected, int) or not isinstance(plan, dict):
    raise SystemExit(f"selected StringKernelPlan key is not materialized: {backend}")

if plan.get("family") != "concat_triplet_window":
    raise SystemExit(f"wrong selected plan family: {plan}")
if plan.get("consumer") != "direct_kernel_entry":
    raise SystemExit(f"wrong selected plan consumer: {plan}")
if not isinstance(plan.get("loop_payload"), dict):
    raise SystemExit(f"selected plan lost loop payload: {plan}")

corridor_root = plan.get("corridor_root")
relations = metadata.get("string_corridor_relations") or {}
flat_relations = [
    rel
    for rels in relations.values()
    if isinstance(rels, list)
    for rel in rels
    if isinstance(rel, dict)
]

if not any(
    rel.get("kind") == "phi_carry_base"
    and rel.get("base_value") == corridor_root
    and rel.get("window_contract") == "preserve_plan_window"
    for rel in flat_relations
):
    raise SystemExit(f"missing preserve_plan_window relation for %{corridor_root}: {relations}")

if not any(
    rel.get("kind") == "phi_carry_base"
    and rel.get("base_value") == corridor_root
    and rel.get("window_contract") == "stop_at_merge"
    for rel in flat_relations
):
    raise SystemExit(f"missing stop_at_merge relation for %{corridor_root}: {relations}")
PY
then
    echo "[INFO] emitted MIR:"
    sed -n '1,220p' "$OUT_JSON" || true
    test_fail "$SMOKE_NAME: direct MIR probe did not match the current phi metadata contract"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (direct emit route pins current substring-concat phi metadata)"
