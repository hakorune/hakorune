#!/bin/bash
# gate_c_v1_file_vm.sh — Gate-C(Core) v1 JSON (file) parity smoke (opt-in)

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

JSON_FILE="/tmp/gate_c_v1_file_$$.json"
trap 'rm -f "$JSON_FILE"' EXIT
cat > "$JSON_FILE" <<'JSON'
{"schema_version":"1.0","functions":[{"name":"main","params":[],"blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"i64","value":7}},{"op":"ret","value":1}]}]}]}
JSON

run_case() {
  local mode="$1"
  local label="gate_c_v1_file_vm(${mode})"

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

  # Debug stdout for env (optional)
  if [ "${SMOKES_DEBUG:-0}" = "1" ]; then
    echo "[DEBUG] mode=$mode" >&2
    env | grep -E 'NYASH|HAKO' >&2
  fi

  output="$($BIN --json-file "$JSON_FILE" 2>&1)"
  rc=$?
  last=$(printf '%s\n' "$output" | awk '/Result:/{val=$2} END{print val}')

  if [ "$rc" -ne 0 ]; then
    echo "$output" >&2
    echo "[FAIL] $label (rc=$rc)" >&2
    exit 1
  fi

  if [ "$last" != "7" ]; then
    echo "$output" >&2
    echo "[FAIL] $label (expected 7, got '$last')" >&2
    exit 1
  fi

  echo "[PASS] $label" >&2
}

run_case "plugins-off"
run_case "plugins-on"

exit 0
