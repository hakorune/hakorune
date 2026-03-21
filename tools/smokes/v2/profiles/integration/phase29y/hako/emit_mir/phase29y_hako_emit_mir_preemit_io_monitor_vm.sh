#!/bin/bash
# phase29y_hako_emit_mir_preemit_io_monitor_vm.sh
# Monitor-only diagnostic pin (non-gating):
# - Observe pre-emit TOML/openat behavior for `--hako-emit-mir-json`.
# - Keep day-to-day run non-gating; use `--strict` only for failure-driven regression triage.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

STRICT=0
if [ "${1:-}" = "--strict" ]; then
  STRICT=1
  shift
fi

INPUT="${1:-$NYASH_ROOT/apps/tests/phase29y_continue_assignment_in_continue_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/phase29y_hako_emit_mir_preemit.XXXXXX.json)"
TRACE_BASE_COLD="/tmp/phase29y_hako_emit_mir_preemit_cold.$$"
TRACE_BASE_HOT="/tmp/phase29y_hako_emit_mir_preemit_hot.$$"
CACHE_FILE="${NYASH_STAGE1_MODULES_CACHE:-$NYASH_ROOT/target/.cache/stage1_module_env.json}"

cleanup() {
  rm -f "$TMP_MIR"
  rm -f "${TRACE_BASE_COLD}".* "${TRACE_BASE_HOT}".*
}
trap cleanup EXIT

if ! command -v strace >/dev/null 2>&1; then
  test_skip "phase29y_hako_emit_mir_preemit_io_monitor_vm" "strace not available"
  exit 0
fi

if [ ! -f "$INPUT" ]; then
  test_fail "phase29y_hako_emit_mir_preemit_io_monitor_vm: fixture missing: $INPUT"
  exit 2
fi

count_matches() {
  local pattern="$1"
  local file="$2"
  local n
  n=$(rg -c "$pattern" "$file" 2>/dev/null || true)
  if [ -z "$n" ]; then
    echo 0
  else
    echo "$n"
  fi
}

run_trace() {
  local label="$1"
  local trace_base="$2"
  local output rc

  rm -f "$TMP_MIR"
  rm -f "${trace_base}".*

  set +e
  output=$(timeout "$RUN_TIMEOUT_SECS" env \
    NYASH_DISABLE_PLUGINS=1 \
    NYASH_VM_USE_FALLBACK=0 \
    NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
    NYASH_JOINIR_DEV=0 \
    NYASH_JOINIR_STRICT=0 \
    HAKO_JOINIR_STRICT=0 \
    HAKO_JOINIR_PLANNER_REQUIRED=0 \
    strace -ff -o "$trace_base" -e trace=openat \
    "$NYASH_BIN" --hako-emit-mir-json "$TMP_MIR" "$INPUT" 2>&1)
  rc=$?
  set -e

  if [ "$rc" -eq 124 ]; then
    test_fail "phase29y_hako_emit_mir_preemit_io_monitor_vm($label): outer timeout triggered"
    exit 1
  fi

  if [ "$rc" -ne 0 ]; then
    if ! printf '%s\n' "$output" | rg -q '\[stage1-cli\] emit-mir: stage1 stub timed out after [0-9]+ ms'; then
      echo "$output" | tail -n 80 || true
      test_fail "phase29y_hako_emit_mir_preemit_io_monitor_vm($label): unexpected failure contract (rc=$rc)"
      exit 1
    fi
  fi

  local trace_files=("${trace_base}".*)
  if [ ! -e "${trace_files[0]}" ]; then
    test_fail "phase29y_hako_emit_mir_preemit_io_monitor_vm($label): trace output missing"
    exit 1
  fi
}

collect_counts() {
  local trace_base="$1"
  local parent_module=0 child_module=0
  local parent_hako=0 child_hako=0
  local parent_nyash=0 child_nyash=0
  local f c_module c_hako c_nyash

  for f in "${trace_base}".*; do
    [ -f "$f" ] || continue
    c_module=$(count_matches "hako_module\\.toml" "$f")
    c_hako=$(count_matches "hako\\.toml" "$f")
    c_nyash=$(count_matches "nyash\\.toml" "$f")
    if rg -q "lang/src/runner/stage1_cli\\.hako" "$f"; then
      child_module=$((child_module + c_module))
      child_hako=$((child_hako + c_hako))
      child_nyash=$((child_nyash + c_nyash))
    else
      parent_module=$((parent_module + c_module))
      parent_hako=$((parent_hako + c_hako))
      parent_nyash=$((parent_nyash + c_nyash))
    fi
  done

  echo "$parent_module $child_module $parent_hako $child_hako $parent_nyash $child_nyash"
}

# cold run (clear cache snapshot)
rm -f "$CACHE_FILE"
run_trace "cold" "$TRACE_BASE_COLD"
read -r COLD_PARENT_MODULE COLD_CHILD_MODULE COLD_PARENT_HAKO COLD_CHILD_HAKO COLD_PARENT_NYASH COLD_CHILD_NYASH < <(collect_counts "$TRACE_BASE_COLD")

# hot run (reuse cache snapshot)
run_trace "hot" "$TRACE_BASE_HOT"
read -r HOT_PARENT_MODULE HOT_CHILD_MODULE HOT_PARENT_HAKO HOT_CHILD_HAKO HOT_PARENT_NYASH HOT_CHILD_NYASH < <(collect_counts "$TRACE_BASE_HOT")

echo "[monitor] cold hako_module.toml parent=${COLD_PARENT_MODULE} child=${COLD_CHILD_MODULE}"
echo "[monitor] hot  hako_module.toml parent=${HOT_PARENT_MODULE} child=${HOT_CHILD_MODULE}"
echo "[monitor] cold hako.toml        parent=${COLD_PARENT_HAKO} child=${COLD_CHILD_HAKO}"
echo "[monitor] hot  hako.toml        parent=${HOT_PARENT_HAKO} child=${HOT_CHILD_HAKO}"

if [ "$STRICT" -eq 1 ]; then
  if [ "$COLD_CHILD_MODULE" -ne 0 ]; then
    test_fail "phase29y_hako_emit_mir_preemit_io_monitor_vm(strict): cold child hako_module.toml must be 0 (actual=${COLD_CHILD_MODULE})"
    exit 1
  fi
  if [ "$HOT_PARENT_MODULE" -ne 0 ] || [ "$HOT_CHILD_MODULE" -ne 0 ]; then
    test_fail "phase29y_hako_emit_mir_preemit_io_monitor_vm(strict): hot hako_module.toml must be 0/0 (actual=${HOT_PARENT_MODULE}/${HOT_CHILD_MODULE})"
    exit 1
  fi
else
  if [ "$COLD_CHILD_MODULE" -ne 0 ]; then
    log_warn "monitor drift: cold child hako_module.toml expected 0 (actual=${COLD_CHILD_MODULE})"
  fi
  if [ "$HOT_PARENT_MODULE" -ne 0 ] || [ "$HOT_CHILD_MODULE" -ne 0 ]; then
    log_warn "monitor drift: hot hako_module.toml expected 0/0 (actual=${HOT_PARENT_MODULE}/${HOT_CHILD_MODULE})"
  fi
fi

test_pass "phase29y_hako_emit_mir_preemit_io_monitor_vm: PASS (monitor-only; strict=${STRICT})"
