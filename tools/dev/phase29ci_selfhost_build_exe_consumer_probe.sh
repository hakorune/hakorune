#!/usr/bin/env bash
set -euo pipefail

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
export NYASH_ROOT="$ROOT"
export NYASH_BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"
LOG_PREFIX="[phase29ci/bridge-capsule]"

if [ ! -x "$NYASH_BIN" ]; then
  echo "$LOG_PREFIX missing hakorune binary: $NYASH_BIN" >&2
  exit 2
fi

TMP_JSON="$(mktemp --suffix .phase29ci.program.json)"
TMP_MIR="/tmp/phase29ci_selfhost_build_consumer_probe.mir.json"
TMP_EXE="/tmp/phase29ci_selfhost_build_consumer_probe.exe"
NYLL="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}"
NYRT_DIR="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}"

cleanup() {
  rm -f "$TMP_JSON" "$TMP_MIR" "$TMP_EXE" 2>/dev/null || true
}
trap cleanup EXIT

BIN="$NYASH_BIN"

if [ ! -x "$NYLL" ] && [ ! -f "$NYLL" ]; then
  echo "$LOG_PREFIX missing ny-llvmc: $NYLL" >&2
  exit 2
fi

source "$ROOT/tools/selfhost/lib/program_json_mir_bridge.sh"

cat > "$TMP_JSON" <<'JSON'
{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":0}}]}
JSON

program_json_mir_bridge_emit "$BIN" "$TMP_JSON" "$TMP_MIR" "$LOG_PREFIX"
if [ ! -s "$TMP_MIR" ] || ! grep -q '"functions"' "$TMP_MIR"; then
  echo "$LOG_PREFIX MIR output missing or malformed: $TMP_MIR" >&2
  exit 1
fi

"$NYLL" --in "$TMP_MIR" --emit exe --nyrt "$NYRT_DIR" --out "$TMP_EXE"
if [ ! -x "$TMP_EXE" ]; then
  echo "$LOG_PREFIX EXE output missing: $TMP_EXE" >&2
  exit 1
fi

echo "$LOG_PREFIX PASS"
