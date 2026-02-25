#!/bin/bash
# gate_c_v1_pipe_vm.sh — Gate-C(Core) v1 JSON (pipe) parity smoke (opt-in)

set -euo pipefail

if [ "${SMOKES_ENABLE_GATE_C_V1:-0}" != "1" ]; then
  echo "[SKIP] SMOKES_ENABLE_GATE_C_V1!=1" >&2
  exit 0
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null)"
if [ -z "$ROOT" ]; then
  ROOT="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"
fi
BIN="$ROOT/target/release/nyash"

if [ ! -x "$BIN" ]; then
  (cd "$ROOT" && cargo build --release >/dev/null 2>&1) || {
    echo "[FAIL] build release nyash" >&2
    exit 1
  }
fi

PAYLOAD='{"schema_version":"1.0","functions":[{"name":"main","params":[],"blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"i64","value":5}},{"op":"ret","value":1}]}]}]}'

run_case() {
  local mode="$1"
  local label="gate_c_v1_pipe_vm(${mode})"

  if [ "$mode" = "plugins-off" ]; then
    export NYASH_DISABLE_PLUGINS=1
  else
    unset NYASH_DISABLE_PLUGINS || true
  fi

  export NYASH_QUIET=1
  export HAKO_QUIET=1
  export NYASH_CLI_VERBOSE=0
  export NYASH_NYRT_SILENT_RESULT=1
  export NYASH_NYVM_CORE=1
  export HAKO_NYVM_CORE=1

  output=$(printf '%s' "$PAYLOAD" | $BIN --ny-parser-pipe 2>&1)
  rc=$?
  last=$(printf '%s\n' "$output" | awk '/Result:/{val=$2} END{print val}')

  if [ "$rc" -ne 0 ]; then
    echo "$output" >&2
    echo "[FAIL] $label (rc=$rc)" >&2
    exit 1
  fi

  if [ "$last" != "5" ]; then
    echo "$output" >&2
    echo "[FAIL] $label (expected 5, got '$last')" >&2
    exit 1
  fi

  echo "[PASS] $label" >&2
}

run_case "plugins-off"
run_case "plugins-on"

exit 0
