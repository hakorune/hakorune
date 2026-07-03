#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "usage: tools/hako-check/hako-check.sh <file.hako>"
}

if [[ $# -lt 1 ]]; then
  usage >&2
  exit 1
fi

case "${1:-}" in
  -h|--help|help)
    usage
    exit 0
    ;;
esac

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
# Allow alias: HAKO_BIN overrides binary path. Otherwise prefer 'hako' alias,
# then the Hakorune binary. Legacy nyash is a compatibility fallback only.
BIN="${HAKO_BIN:-}"
if [[ -z "${BIN}" ]]; then
  if [[ -x "$ROOT_DIR/tools/bin/hako" ]]; then
    BIN="$ROOT_DIR/tools/bin/hako"
  elif [[ -x "$ROOT_DIR/tools/bin/hakorune" ]]; then
    BIN="$ROOT_DIR/tools/bin/hakorune"
  elif [[ -x "$ROOT_DIR/target/release/hakorune" ]]; then
    BIN="$ROOT_DIR/target/release/hakorune"
  else
    BIN="$ROOT_DIR/target/release/nyash"
  fi
fi
FILE="$1"

if [[ ! -x "$BIN" ]]; then
  echo "[info] building Hakorune (release) ..." >&2
  cargo build --release -q
fi

if [[ ! -f "$FILE" ]]; then
  echo "error: file not found: $FILE" >&2
  exit 2
fi

# Parse → MIR build → verify (no execute)
"$BIN" --backend mir --verify "$FILE"
echo "OK: $FILE"
