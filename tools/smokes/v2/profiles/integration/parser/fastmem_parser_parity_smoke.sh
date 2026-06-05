#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"

BIN="${NYASH_BIN:-$NYASH_ROOT/target/release/hakorune}"
if [ ! -x "$BIN" ]; then
  BIN="$NYASH_ROOT/target/release/nyash"
fi

if [ ! -x "$BIN" ]; then
  log_error "nyash/hakorune binary not found: $BIN"
  exit 2
fi

FEATURES="${FASTMEM_PARSER_FEATURES:-stage3,rune}"

TMPDIR="$(mktemp -d /tmp/fastmem_parser_parity.XXXXXX)"
cleanup() {
  rm -rf "$TMPDIR"
}
trap cleanup EXIT

RUST_SRC="$TMPDIR/rust_baseline.hako"
RUST_AST="$TMPDIR/rust_baseline.ast.json"
RUST_LOG="$TMPDIR/rust_baseline.log"
HAKO_DRIVER="$TMPDIR/hako_parser_baseline.hako"
HAKO_LOG="$TMPDIR/hako_parser_baseline.log"
HAKO_JSON="$TMPDIR/hako_parser_baseline.program.json"

cat >"$RUST_SRC" <<'HK'
static box Main {
  main() {
    local x = 1 + 2
    local y = (1 << 3) & 7 | (8 >> 1) ^ 2
    return x
  }
}
HK

if ! NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$RUST_AST" "$RUST_SRC" \
  >"$RUST_LOG" 2>&1; then
  log_error "Rust parser baseline AST emit failed"
  tail -n 120 "$RUST_LOG" >&2 || true
  exit 1
fi

python3 - "$RUST_AST" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as f:
    data = json.load(f)

def walk(node):
    if isinstance(node, dict):
        yield node
        for value in node.values():
            yield from walk(value)
    elif isinstance(node, list):
        for item in node:
            yield from walk(item)

ops = {node.get("op") for node in walk(data) if node.get("kind") == "BinaryOp"}
expected = {"+", "<<", ">>", "&", "|", "^"}
missing = expected - ops
if missing:
    print(f"missing Rust parser BinaryOp ops: {sorted(missing)}", file=sys.stderr)
    sys.exit(1)
PY

cat >"$HAKO_DRIVER" <<'HK'
static box Main {
  main() {
    local x = 1 + 2
    local y = (1 << 3) & 7 | (8 >> 1) ^ 2
    return x
  }
}
HK

if ! NYASH_FEATURES="$FEATURES" \
  bash "$NYASH_ROOT/tools/archive/legacy-selfhost/engineering/program_json_v0_stageb_artifact_probe.sh" \
    --in "$HAKO_DRIVER" --out "$HAKO_JSON" \
  >"$HAKO_LOG" 2>&1; then
  log_error ".hako ParserBox baseline Program(JSON v0) emit failed"
  tail -n 120 "$HAKO_LOG" >&2 || true
  exit 1
fi

if ! grep -Fq '"kind":"Program"' "$HAKO_JSON"; then
  log_error ".hako ParserBox baseline missing Program node"
  cat "$HAKO_JSON" >&2 || true
  exit 1
fi

for op in "+" "<<" ">>" "&" "|" "^"; do
  if ! grep -Fq '"type":"Binary"' "$HAKO_JSON" || ! grep -Fq "\"op\":\"$op\"" "$HAKO_JSON"; then
    log_error ".hako ParserBox baseline missing Binary($op)"
    cat "$HAKO_JSON" >&2 || true
    tail -n 120 "$HAKO_LOG" >&2 || true
    exit 1
  fi
done

if [ ! -s "$HAKO_JSON" ]; then
  log_error ".hako ParserBox baseline produced empty Program(JSON)"
  tail -n 120 "$HAKO_LOG" >&2 || true
  exit 1
fi

log_success "fastmem_parser_parity_smoke"
