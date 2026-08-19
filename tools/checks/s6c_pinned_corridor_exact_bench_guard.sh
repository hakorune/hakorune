#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="s6c-pinned-corridor-exact-bench-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

LOWERING="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pinned_text_selected_lowering.inc"
DRIVER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_exact_leaf_driver.c"
BENCH="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_exact_leaf_bench.c"
REFERENCE="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_exact_leaf_reference.c"
CHECKER="$ROOT_DIR/tools/perf/s6c_pinned_corridor_exact_bench.py"
SMOKE="$ROOT_DIR/tools/checks/s6c_pinned_corridor_exact_bench_smoke.sh"
guard_require_command "$TAG" rg
guard_require_files "$TAG" "$LOWERING" "$DRIVER" "$BENCH" "$REFERENCE" "$CHECKER" "$SMOKE"

count_fixed() { (rg -F -o -- "$1" "${@:2}" || true) | wc -l | tr -d '[:space:]'; }

[[ "$(count_fixed 'HAKO_LLVMC_PTFC_EXACT_LEAF_EVIDENCE_PLAN_V1' "$LOWERING" "$DRIVER")" == 4 ]] || \
  guard_fail "$TAG" "one default-off real-plan capture hook and one test override are required"
[[ "$(count_fixed 'hako_llvmc_ptfc_emit_selected_leaf(' "$LOWERING" "$DRIVER")" == 3 ]] || \
  guard_fail "$TAG" "evidence callable must reuse the sole production leaf emitter"
[[ "$(count_fixed 'hako_llvmc_compile_json_pure_first(' "$DRIVER")" == 1 ]] || \
  guard_fail "$TAG" "driver must consume one real selected candidate"
if rg -n 'ny_main|memcmp|hako_text_formal_residence_enter|hako_text_formal_residence_finish' "$DRIVER"; then
  guard_fail "$TAG" "leaf projection must not benchmark whole-call or lifecycle work"
fi
for needle in CLOCK_MONOTONIC_RAW 'warmup < 10' 'sample < 51' 20000000 CASES hako_text_formal_residence_enter_v1 hako_text_formal_residence_finish_or_abort_v1; do
  [[ "$(count_fixed "$needle" "$BENCH")" -ge 1 ]] || guard_fail "$TAG" "benchmark control missing: $needle"
done
for needle in taskset clang-18 -O3 -fno-lto promotion-test-support; do
  [[ "$(count_fixed "$needle" "$SMOKE")" -ge 1 ]] || guard_fail "$TAG" "smoke control missing: $needle"
done
for needle in 1.10 1.15 1.30 nearest_rank promotion-evidence-only; do
  [[ "$(count_fixed "$needle" "$CHECKER")" -ge 1 ]] || guard_fail "$TAG" "evidence contract missing: $needle"
done
for negative in missing-case short-sample zero-denominator threshold-red eager-load; do
  [[ "$(count_fixed "$negative" "$SMOKE")" -ge 1 ]] || guard_fail "$TAG" "negative missing: $negative"
done
if rg -n 'thresholds.*=.*1\.[4-9]|inline.*nowait|fallback|retry' "$CHECKER" "$BENCH" "$REFERENCE"; then
  guard_fail "$TAG" "threshold relaxation or fallback/retry is forbidden"
fi
for file in "$LOWERING" "$DRIVER" "$BENCH" "$REFERENCE" "$CHECKER" "$SMOKE"; do
  lines="$(wc -l <"$file" | tr -d '[:space:]')"
  (( lines < 760 )) || guard_fail "$TAG" "source reached the 760-line split trigger: ${file#"$ROOT_DIR/"}=$lines"
done

echo "[$TAG] ok (real-plan production emitter + paired fixed-control evidence; promotion authority unchanged)"
