#!/usr/bin/env bash
set -euo pipefail

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"
IN=""
OUT=""
RAW_LOG=""

usage() {
  cat >&2 <<'USAGE'
Usage: tools/dev/phase29cv_stageb_artifact_probe.sh --in <source.hako> [--out <program.json>] [--raw-log <log>]

Explicit Program(JSON v0) diagnostic probe. This replaces
selfhost_build.sh --keep-tmp / NYASH_SELFHOST_KEEP_RAW=1.
USAGE
}

while [ $# -gt 0 ]; do
  case "$1" in
    --in) IN="$2"; shift 2;;
    --out) OUT="$2"; shift 2;;
    --raw-log) RAW_LOG="$2"; shift 2;;
    -h|--help) usage; exit 0;;
    *) echo "[phase29cv/stageb-artifact] unknown arg: $1" >&2; usage; exit 2;;
  esac
done

if [ -z "$IN" ]; then
  echo "[phase29cv/stageb-artifact] --in <source.hako> is required" >&2
  usage
  exit 2
fi
if [ ! -f "$IN" ]; then
  echo "[phase29cv/stageb-artifact] input not found: $IN" >&2
  exit 2
fi
if [ ! -x "$BIN" ]; then
  echo "[phase29cv/stageb-artifact] hakorune binary not found: $BIN" >&2
  echo "                                  set NYASH_BIN or build target/release/hakorune" >&2
  exit 2
fi

if [ -z "$OUT" ]; then
  OUT="$(mktemp --suffix .program-json-v0.json)"
fi

source "$ROOT/tools/lib/program_json_v0_compat.sh"

tmp_log="$(mktemp)"
cleanup() {
  rm -f "$tmp_log" 2>/dev/null || true
}
trap cleanup EXIT

set +e
program_json_v0_compat_emit_to_file "$BIN" "$OUT" "$IN" >"$tmp_log" 2>&1
rc=$?
set -e

if [ -n "$RAW_LOG" ]; then
  cp "$tmp_log" "$RAW_LOG" 2>/dev/null || true
fi

if [ "$rc" -ne 0 ] || [ ! -s "$OUT" ]; then
  echo "[phase29cv/stageb-artifact] Program(JSON v0) emit failed (rc=$rc)" >&2
  tail -n 120 "$tmp_log" >&2 || true
  exit 1
fi

printf '%s\n' "$OUT"
