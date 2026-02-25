#!/usr/bin/env bash
# ny_selfhost_inline.sh — Ny selfhost inline parser/JSON v0 debug helper
#
# Purpose:
#   - Minimal, shell-based equivalent of the inline Ny selfhost pipeline used in src/runner/selfhost.rs.
#   - Reads a Nyash/Ny source file, runs ParserBox.parse_program2 + EmitterBox.emit_program
#     via a tiny Hako harness, and prints Program(JSON v0) to stdout.
#   - Intended for debugging only; not wired into the main CLI or build scripts.
#
# Usage:
#   tools/ny_selfhost_inline.sh <source.ny> [nyash_binary]
#     - <source.ny>: Nyash/Ny source file to parse.
#     - nyash_binary: optional path to hakorune/nyash binary (default: target/release/hakorune or nyash).
#
# Notes:
#   - Forces Stage-3 parser ON for the child (NYASH_FEATURES=stage3/NYASH_FEATURES=stage3).
#   - Enables using resolver with file-based using (dev profile) so that lang.compiler.* modules can be found.
#   - Does NOT perform Stage-0 prelude text merge or preexpand_at_local; it feeds the raw source to ParserBox.

set -euo pipefail

if [ "$#" -lt 1 ] || [ "$#" -gt 2 ]; then
  echo "Usage: $0 <source.ny> [nyash_binary]" >&2
  exit 2
fi

SRC="$1"
BIN="${2:-}"

if [ ! -f "$SRC" ]; then
  echo "[ny-selfhost-inline] source not found: $SRC" >&2
  exit 1
fi

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [ -z "$BIN" ]; then
  if [ -x "$ROOT/target/release/hakorune" ]; then
    BIN="$ROOT/target/release/hakorune"
  else
    BIN="$ROOT/target/release/nyash"
  fi
fi

if [ ! -x "$BIN" ]; then
  echo "[ny-selfhost-inline] nyash/hakorune binary not found: $BIN" >&2
  exit 1
fi

HARNESS="$(mktemp --suffix=.hako)"
trap 'rm -f "$HARNESS" || true' EXIT

cat >"$HARNESS" <<'HAKO'
using lang.compiler.parser.box as ParserBox
using lang.compiler.stage1.emitter_box as EmitterBox

static box Main {
  method main(args){
    local src = env.get("NYASH_INLINE_SRC")
    if src == null {
      print("[ny-selfhost-inline] NYASH_INLINE_SRC is null")
      return 1
    }
    local s = "" + src
    local p = new ParserBox()
    p.stage3_enable(1)
    print("[ny-selfhost-inline] entry len=" + ("" + s.length()))
    local json = p.parse_program2(s)
    local e = new EmitterBox()
    json = e.emit_program(json, "[]")
    print(json)
    return 0
  }
}
HAKO

# Feed raw source via env to avoid huge string literal escaping on the shell side.
SRC_CONTENT="$(cat "$SRC")"

NYASH_INLINE_SRC="$SRC_CONTENT" \
NYASH_JSON_ONLY=1 \
NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
NYASH_ALLOW_USING_FILE=1 HAKO_ALLOW_USING_FILE=1 \
"$BIN" --backend vm "$HARNESS"

