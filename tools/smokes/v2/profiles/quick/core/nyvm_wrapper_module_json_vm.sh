#!/bin/bash
# nyvm_wrapper_module_json_vm.sh — Ny wrapper bridge module-json canary (opt-in)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"
fi
BIN="$ROOT/target/release/nyash"

warn() { echo -e "[WARN] $*" >&2; }
info() { echo -e "[INFO] $*" >&2; }
pass() { echo -e "[PASS] $*" >&2; }
skip() { echo -e "[SKIP] $*" >&2; exit 0; }
fail() { echo -e "[FAIL] $*" >&2; exit 1; }

# Opt-in guard
if [ "${SMOKES_ENABLE_NYVM_WRAPPER:-0}" != "1" ]; then
  skip "SMOKES_ENABLE_NYVM_WRAPPER!=1"
fi

if [ ! -x "$BIN" ]; then
  (cd "$ROOT" && cargo build --release >/dev/null 2>&1) || fail "build failed"
fi

# Minimal MIR(JSON v0) module (return 7) — module-shaped
JSON_FILE="/tmp/nyvm_wrapper_mod_$$.json"
trap 'rm -f "$JSON_FILE"' EXIT
cat > "$JSON_FILE" <<'J'
{"kind":"MIR","schema_version":"1.0","functions":[{"name":"main","params":[],"blocks":[{"id":0,"instructions":[
 {"op":"const","dst":1,"value":{"type":"i64","value":7}},
 {"op":"ret","value":1}
]}]}]}
J

# If wrapper path is not wired, skip rather than fail
if ! strings "$BIN" 2>/dev/null | grep -q 'NyVmDispatcher'; then
  skip "binary lacks NyVmDispatcher symbols (wrapper likely not wired)"
fi

# Run via Gate-C to Interpreter (control), then (optionally) wrapper would be tested when wired
out=$("$BIN" --json-file "$JSON_FILE" 2>&1 || true)
last=$(printf '%s\n' "$out" | awk '/^(✅|ResultType|Result:)/{next} NF{last=$0} END{print last}')
if [ "$last" = "7" ]; then
  pass "nyvm_wrapper_module_json_vm"
else
  echo "$out" >&2
  fail "nyvm_wrapper_module_json_vm (expected 7, got '$last')"
fi

