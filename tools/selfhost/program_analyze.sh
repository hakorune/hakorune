#!/usr/bin/env bash
# tools/selfhost/program_analyze.sh - Phase 160-impl-1 Program JSON Analyzer wrapper
# Compatibility / debug helper; explicit compat route, not a day-to-day route.
#
# Usage:
#   ./tools/selfhost/program_analyze.sh /path/to/program.json
#   ./tools/selfhost/program_analyze.sh < program.json   # stdin
#
# This script reads a Program JSON v0 file and passes it to the .hako analyzer
# for selfhost depth-2 verification (Rust outputs IR → .hako reads IR).

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"
HAKO="$ROOT/tools/selfhost/program_analyze.hako"

if [ ! -x "$BIN" ]; then
  echo "[ERROR] hakorune binary not found: $BIN" >&2
  echo "Run: cargo build --release" >&2
  exit 2
fi

if [ ! -f "$HAKO" ]; then
  echo "[ERROR] program_analyze.hako not found: $HAKO" >&2
  exit 2
fi

# Read JSON from file argument or stdin
if [ $# -ge 1 ] && [ -f "$1" ]; then
  JSON_CONTENT="$(cat "$1")"
elif [ ! -t 0 ]; then
  # Read from stdin
  JSON_CONTENT="$(cat)"
else
  echo "Usage: $0 /path/to/program.json" >&2
  echo "   or: cat program.json | $0" >&2
  exit 2
fi

# Run the .hako analyzer with Program JSON in environment via the explicit compat route.
export HAKO_PROGRAM_JSON="$JSON_CONTENT"
export NYASH_FEATURES="${NYASH_FEATURES:-stage3}"
export NYASH_PARSER_ALLOW_SEMICOLON=1
export NYASH_USING_AST=1
export NYASH_QUIET=0
export HAKO_QUIET=0

exec "$BIN" --backend vm "$HAKO"
