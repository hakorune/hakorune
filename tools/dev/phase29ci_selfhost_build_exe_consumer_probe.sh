#!/usr/bin/env bash
set -euo pipefail

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
export NYASH_ROOT="$ROOT"
export NYASH_BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"

if [ ! -x "$NYASH_BIN" ]; then
  echo "[phase29ci/probe] missing hakorune binary: $NYASH_BIN" >&2
  exit 2
fi

TMP_JSON="$(mktemp --suffix .phase29ci.program.json)"
TMP_MIR="/tmp/phase29ci_selfhost_build_consumer_probe.mir.json"
TMP_EXE="/tmp/phase29ci_selfhost_build_consumer_probe.exe"

cleanup() {
  rm -f "$TMP_JSON" "$TMP_MIR" "$TMP_EXE" 2>/dev/null || true
}
trap cleanup EXIT

BIN="$NYASH_BIN"
JSON_OUT=""
MIR_OUT=""
KEEP_TMP=0

source "$ROOT/tools/selfhost/lib/selfhost_build_exe.sh"

cat > "$TMP_JSON" <<'JSON'
{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":0}}]}
JSON

emit_mir_json_from_program_json_v0 "$TMP_JSON" "$TMP_MIR"
if [ ! -s "$TMP_MIR" ] || ! head -n1 "$TMP_MIR" | grep -q '"functions"'; then
  echo "[phase29ci/probe] MIR output missing or malformed: $TMP_MIR" >&2
  exit 1
fi

emit_exe_from_program_json_v0 "$TMP_JSON" "$TMP_EXE"
if [ ! -x "$TMP_EXE" ]; then
  echo "[phase29ci/probe] EXE output missing: $TMP_EXE" >&2
  exit 1
fi

echo "[phase29ci/probe] PASS"
