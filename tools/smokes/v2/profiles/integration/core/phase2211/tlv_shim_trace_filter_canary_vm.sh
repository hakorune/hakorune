#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env >/dev/null || exit 2

(
  cd "$NYASH_ROOT" && cargo build -q --release -p nyash-rust --features tlv-shim
)

# Prefer extern route to avoid plugin dependency
CODE='
static box Main {
  main() {
    env.console.log("hello")
    return 0
  }
}
'

out=$(HAKO_CALL_TRACE=1 "$NYASH_ROOT/tools/dev/hako_debug_run.sh" --raw --no-compiler -c "$CODE" 2>&1)
if echo "$out" | grep -q "\[call:env.console.log\]"; then
  echo "[PASS] tlv_shim_trace_filter_canary_vm"
  exit 0
fi
if echo "$out" | grep -qx "hello"; then
  echo "[PASS] tlv_shim_trace_filter_canary_vm (no trace, output ok)"
  exit 0
fi
echo "[FAIL] tlv_shim_trace_filter_canary_vm" >&2
echo "$out" >&2
exit 1
