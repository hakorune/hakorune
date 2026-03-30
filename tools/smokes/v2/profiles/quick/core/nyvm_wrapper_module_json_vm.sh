#!/bin/bash
# nyvm_wrapper_module_json_vm.sh — Ny wrapper bridge module-json canary (opt-in)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"
fi
BIN="$ROOT/target/release/hakorune"
if [ ! -x "$BIN" ]; then
  BIN="$ROOT/target/release/nyash"
fi

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

# Run via direct MIR intake; this canary is not a Program(JSON v0) compat route.
set +e
out=$("$BIN" --mir-json-file "$JSON_FILE" 2>&1)
rc=$?
set -e
if [ "$rc" -eq 7 ] && [ -z "$out" ]; then
  pass "nyvm_wrapper_module_json_vm"
else
  echo "$out" >&2
  fail "nyvm_wrapper_module_json_vm (expected rc=7 and no stdout, got rc=$rc)"
fi
