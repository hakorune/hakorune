#!/bin/bash
# phase-137x direct emit shape smoke for the current substring-concat post-sink route
#
# Contract:
# 1) direct MIR still carries the substring producers and concat-triplet helper
#    that feed the selected string kernel plan.
# 2) exported `string_kernel_plans` keeps the backend-consumable
#    concat-triplet plan with `[slice, const, slice]` parts and loop payload.
# 3) exact backend route metadata selects that plan by key.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase137x_direct_emit_substring_concat_post_sink_shape"
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

def callee_name(ins):
    call = ins.get("mir_call") or {}
    callee = call.get("callee") or {}
    return callee.get("name") or callee.get("method")

substring_calls = 0
concat3_calls = 0
for block in main_fn.get("blocks", []):
    for ins in block.get("instructions", []):
        if ins.get("op") != "mir_call":
            continue
        name = callee_name(ins)
        if name == "substring":
            substring_calls += 1
        if name == "nyash.string.substring_concat3_hhhii":
            concat3_calls += 1

if substring_calls < 2:
    raise SystemExit(f"expected at least two substring producers, got {substring_calls}")
if concat3_calls != 1:
    raise SystemExit(f"expected one substring_concat3 helper, got {concat3_calls}")

metadata = main_fn.get("metadata") or {}
backend = metadata.get("exact_seed_backend_route")
if not isinstance(backend, dict):
    raise SystemExit("missing exact_seed_backend_route")
if backend.get("tag") != "substring_concat_loop_ascii":
    raise SystemExit(f"wrong exact backend route tag: {backend}")
if backend.get("source_route") != "string_kernel_plans.loop_payload":
    raise SystemExit(f"wrong exact backend route source: {backend}")

selected = backend.get("selected_value")
plans = metadata.get("string_kernel_plans") or {}
plan = plans.get(str(selected))
if not isinstance(selected, int) or not isinstance(plan, dict):
    raise SystemExit(f"selected StringKernelPlan key is not materialized: {backend}")

if plan.get("family") != "concat_triplet_window":
    raise SystemExit(f"unexpected selected plan family: {plan}")
if plan.get("consumer") != "direct_kernel_entry":
    raise SystemExit(f"unexpected selected plan consumer: {plan}")
if plan.get("known_length") != 2:
    raise SystemExit(f"unexpected selected plan known_length: {plan}")
if plan.get("retained_form") != "borrowed_text":
    raise SystemExit(f"unexpected selected plan retained_form: {plan}")

parts = plan.get("parts")
if not isinstance(parts, list) or [part.get("kind") for part in parts] != ["slice", "const", "slice"]:
    raise SystemExit(f"unexpected selected plan parts: {plan}")
if parts[1].get("literal") != "xx":
    raise SystemExit(f"unexpected selected plan middle literal: {plan}")

loop_payload = plan.get("loop_payload")
if not isinstance(loop_payload, dict):
    raise SystemExit(f"missing selected plan loop payload: {plan}")
if loop_payload.get("seed_literal") != "line-seed-abcdef":
    raise SystemExit(f"unexpected seed literal: {loop_payload}")
if loop_payload.get("seed_length") != 16 or loop_payload.get("split_length") != 8:
    raise SystemExit(f"unexpected seed/split payload: {loop_payload}")
if loop_payload.get("loop_bound") != 300000:
    raise SystemExit(f"unexpected loop bound: {loop_payload}")
PY
then
    echo "[INFO] emitted MIR:"
    sed -n '1,220p' "$OUT_JSON" || true
    test_fail "$SMOKE_NAME: direct MIR probe did not match the current post-sink metadata shape"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (direct emit route pins current substring-concat post-sink metadata)"
