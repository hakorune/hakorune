#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL="$ROOT_DIR/tools/allocator/userbox_startup_loader_owner_split.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_nyrt_runtime_hooks_off.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-nyrt-runtime-hooks-off] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_not_line() {
  local file="$1"
  local unexpected="$2"
  if grep -q "^${unexpected}$" "$file"; then
    echo "[perf-nyrt-runtime-hooks-off] unexpected line in ${file#$ROOT_DIR/}: $unexpected" >&2
    exit 1
  fi
}

HAKO_NYRT_PLUGIN_HOST=off \
NYASH_NYRT_RUNTIME_HOOKS=off \
NYASH_NYRT_SILENT_RESULT=1 \
"$TOOL" --out "$REPORT" --startup-runs 10 --lane-warmup 0 --lane-repeat 1 --lane-kernel-inner-runs 3 >/dev/null

require_line "$REPORT" "output_contract=perf-userbox-startup-loader-owner-split-v0"
require_line "$REPORT" "runtime_hooks_mode=off"
require_line "$REPORT" "runtime_hooks_init_skipped=1"
require_line "$REPORT" "attribution_startup_loader_attribution_report=1"
require_line "$REPORT" "attribution_measurement_harness_failure_count=0"
require_not_line "$REPORT" "startup_loader_top_0_symbol=nyash_rust::runtime::global_hooks::set_from_runtime"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
