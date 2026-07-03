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
HAKO_ALIAS_BIN="$ROOT_DIR/tools/bin/hako"
HAKORUNE_ALIAS_BIN="$ROOT_DIR/tools/bin/hakorune"
HAKORUNE_RELEASE_BIN="$ROOT_DIR/target/release/hakorune"
LEGACY_NYASH_BIN="$ROOT_DIR/target/release/nyash"
BIN="${HAKO_BIN:-}"
if [[ -z "${BIN}" ]]; then
  if [[ -x "$HAKO_ALIAS_BIN" ]]; then
    BIN="$HAKO_ALIAS_BIN"
  elif [[ -x "$HAKORUNE_ALIAS_BIN" ]]; then
    BIN="$HAKORUNE_ALIAS_BIN"
  elif [[ -x "$HAKORUNE_RELEASE_BIN" ]]; then
    BIN="$HAKORUNE_RELEASE_BIN"
  else
    BIN="$LEGACY_NYASH_BIN"
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
