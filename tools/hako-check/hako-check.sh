#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "usage: tools/hako-check/hako-check.sh <file.hako>" >&2
  exit 1
fi

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
# Allow alias: HAKO_BIN overrides binary path. Otherwise prefer 'hako' alias, then 'nyash'.
BIN="${HAKO_BIN:-}"
if [[ -z "${BIN}" ]]; then
  if [[ -x "$ROOT_DIR/tools/bin/hako" ]]; then
    BIN="$ROOT_DIR/tools/bin/hako"
  elif [[ -x "$ROOT_DIR/tools/bin/hakorune" ]]; then
    BIN="$ROOT_DIR/tools/bin/hakorune"
  else
    BIN="$ROOT_DIR/target/release/nyash"
  fi
fi
FILE="$1"

if [[ ! -x "$BIN" ]]; then
  echo "[info] building nyash (release) ..." >&2
  cargo build --release -q
fi

if [[ ! -f "$FILE" ]]; then
  echo "error: file not found: $FILE" >&2
  exit 2
fi

# Parse → MIR build → verify (no execute)
"$BIN" --backend mir --verify "$FILE"
echo "OK: $FILE"
