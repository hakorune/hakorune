#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_nyrt_plugin_host_off.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"
EXE="$TMP_DIR/ret0-plugin-host-off.exe"
RUN_OUT="$TMP_DIR/run.out"
RUN_ERR="$TMP_DIR/run.err"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-nyrt-plugin-host-off] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

ROOT_DIR="$ROOT_DIR" bash -c '
  set -euo pipefail
  source "$ROOT_DIR/tools/perf/lib/aot_helpers.sh"
  HAKO_AOT_LDFLAGS="-static-libgcc" \
  NYASH_LLVM_LINK_SYSTEM_LIBS=minimal \
  NYASH_LLVM_LINK_WHOLE_ARCHIVE=0 \
  NYASH_LLVM_LINK_GC_SECTIONS=1 \
    perf_build_ret0_aot_exe "$ROOT_DIR" "$ROOT_DIR/target/release/hakorune" "$1" >/dev/null
' _ "$EXE"

NYASH_CLI_VERBOSE=1 \
HAKO_NYRT_PLUGIN_HOST=off \
  "$EXE" >"$RUN_OUT" 2>"$RUN_ERR"

if ! grep -q 'plugin host init skipped' "$RUN_OUT"; then
  echo "[perf-nyrt-plugin-host-off] expected verbose plugin-host skip line" >&2
  cat "$RUN_OUT" >&2
  cat "$RUN_ERR" >&2
  exit 1
fi

{
  echo "output_contract=perf-nyrt-plugin-host-off-probe-v0"
  echo "input_contract=perf-aot-minimal-system-libs-probe-v0"
  echo "measurement_scope=exact_aot_nyrt_plugin_host_off_probe"
  echo "nyrt_plugin_host_mode=off"
  echo "ret0_exact_aot_build_status=ok"
  echo "ret0_exact_aot_run_status=ok"
  echo "plugin_host_init_skipped=1"
  echo "default_plugin_host_mode_changed=0"
  echo "summary=ok"
} >"$REPORT"

require_line "$REPORT" "output_contract=perf-nyrt-plugin-host-off-probe-v0"
require_line "$REPORT" "nyrt_plugin_host_mode=off"
require_line "$REPORT" "ret0_exact_aot_build_status=ok"
require_line "$REPORT" "ret0_exact_aot_run_status=ok"
require_line "$REPORT" "plugin_host_init_skipped=1"
require_line "$REPORT" "default_plugin_host_mode_changed=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
