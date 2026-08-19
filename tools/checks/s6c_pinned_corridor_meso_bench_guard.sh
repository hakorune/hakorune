#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="s6c-pinned-corridor-meso-bench-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
BENCH="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_bench.c"
REFERENCE="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_reference.c"
VALIDATOR="$ROOT_DIR/tools/perf/s6c_pinned_corridor_meso_bench.py"
SMOKE="$ROOT_DIR/tools/checks/s6c_pinned_corridor_meso_bench_smoke.sh"
COUNTER_GUARD="$ROOT_DIR/tools/checks/s6c_native_hwcounter_guard.sh"
guard_require_command "$TAG" rg
guard_require_files "$TAG" "$BENCH" "$REFERENCE" "$VALIDATOR" "$SMOKE" "$COUNTER_GUARD"
count_fixed() {
  local needle="$1"
  shift
  (rg -F -o -- "$needle" "$@" || true) | wc -l | tr -d '[:space:]'
}
for needle in CLOCK_MONOTONIC_RAW 'warmup < 10' 'sample < 51' 30000000 \
  hako_text_formal_residence_enter_v1 hako_text_formal_residence_finish_or_abort_v1; do
  [[ "$(count_fixed "$needle" "$BENCH")" -ge 1 ]] || guard_fail "$TAG" "benchmark control missing: $needle"
done
for needle in 32 256 4096 1048576 ascii width2 width3 width4 mixed first middle last miss; do
  [[ "$(count_fixed "$needle" "$BENCH")" -ge 1 ]] || guard_fail "$TAG" "meso corpus missing: $needle"
done
for needle in taskset clang-18 -O3 -fno-lto promotion-test-support address_mod_64 body_sha256; do
  [[ "$(count_fixed "$needle" "$SMOKE")" -ge 1 ]] || guard_fail "$TAG" "smoke control missing: $needle"
done
for needle in 1.15 nearest_rank promotion-evidence-only gated_4k_plus_max_p50 alignment-manifest; do
  [[ "$(count_fixed "$needle" "$VALIDATOR")" -ge 1 ]] || guard_fail "$TAG" "validator contract missing: $needle"
done
for negative in missing-case short-arm shape-drift threshold-red foreign-outline foreign-alignment; do
  [[ "$(count_fixed "$negative" "$SMOKE")" -ge 1 ]] || guard_fail "$TAG" "negative missing: $negative"
done
if rg -n 'memcmp|builtin|fallback|retry|hako_text_formal|ny_main' "$REFERENCE"; then
  guard_fail "$TAG" "C reference must be direct ptr/len scan only"
fi
for file in "$BENCH" "$REFERENCE" "$VALIDATOR" "$SMOKE"; do
  lines="$(wc -l <"$file" | tr -d '[:space:]')"
  (( lines < 760 )) || guard_fail "$TAG" "source reached 760-line split trigger: $file=$lines"
done
bash "$COUNTER_GUARD"
echo "[$TAG] ok (fixed 80-case paired meso evidence; Residence outside timed region)"
