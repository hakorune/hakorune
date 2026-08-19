#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="s6c-native-hwcounter-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
BENCH="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_bench.c"
REFERENCE="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_reference.c"
COLLECTOR="$ROOT_DIR/tools/perf/s6c_native_hwcounter_collect.py"
BUILDER="$ROOT_DIR/tools/perf/build_s6c_native_hwcounter_fixture.sh"
SMOKE="$ROOT_DIR/tools/checks/s6c_native_hwcounter_smoke.sh"
guard_require_command "$TAG" rg
guard_require_files "$TAG" "$BENCH" "$REFERENCE" "$COLLECTOR" "$BUILDER" "$SMOKE"

expect() {
  local file="$1" needle="$2"
  rg -F -q -- "$needle" "$file" || guard_fail "$TAG" "missing contract '$needle' in $file"
}
for needle in '--arm' '--case' '--iterations' 'PERF_FORMAT_GROUP' 'PERF_FORMAT_ID' \
  'PERF_FORMAT_TOTAL_TIME_ENABLED' 'PERF_FORMAT_TOTAL_TIME_RUNNING' 'exclude_kernel = 1' \
  'exclude_hv = 1' 'PERF_COUNT_SW_CONTEXT_SWITCHES' 'PERF_COUNT_SW_CPU_MIGRATIONS'; do
  expect "$BENCH" "$needle"
done
for event in cycles:u instructions:u branches:u branch-misses:u stalled-cycles-frontend:u \
  L1-icache-load-misses:u iTLB-load-misses:u; do
  expect "$BENCH" "$event"
  expect "$COLLECTOR" "$event"
done
for needle in 'PAIR_COUNT = 51' 'RUN_COUNT = 3' 'mixed/4096/first' \
  'paired-log-ratio-t95' 'physical instruction schedule candidate' 'branch layout candidate' \
  'frontend placement candidate' 'os.replace' 'NoSafeSlice'; do
  expect "$COLLECTOR" "$needle"
done
for negative in 'wrong arm' 'wrong case' 'iteration drift' 'result mismatch' 'event ID drift' \
  'missing event' 'multiplex/time scaling' 'hypervisor negative' 'migration/context-switch' \
  'partial report publication'; do
  expect "$COLLECTOR" "$negative"
done
if rg -n 'PERF_TYPE_RAW|raw_event_fallback[^\n]*True' "$BENCH" "$COLLECTOR"; then
  guard_fail "$TAG" "raw PMU event/fallback is forbidden"
fi
[[ "$(sha256sum "$REFERENCE" | awk '{print $1}')" == \
  35f043f24430a3dc904b7d7363e82b54c64420185b52e954416b63bf8413c49f ]] || \
  guard_fail "$TAG" "C reference changed"
for file in "$BENCH" "$COLLECTOR" "$BUILDER" "$SMOKE"; do
  lines="$(wc -l <"$file" | tr -d '[:space:]')"
  (( lines < 760 )) || guard_fail "$TAG" "source reached 760-line split trigger: $file=$lines"
  (( lines < 800 )) || guard_fail "$TAG" "source reached 800-line hard stop: $file=$lines"
done
expect "$BUILDER" 'CARGO_BUILD_JOBS=4'
expect "$BUILDER" '--profile quick'
python3 "$COLLECTOR" --self-test
echo "[$TAG] ok (native-only separate-process fixed-event contract)"
