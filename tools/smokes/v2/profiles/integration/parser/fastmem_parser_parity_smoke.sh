#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"

HAKORUNE_BIN="${HAKORUNE_BIN:-$NYASH_ROOT/target/release/hakorune}"
LEGACY_NYASH_BIN="$NYASH_ROOT/target/release/nyash"
BIN="${NYASH_BIN:-$HAKORUNE_BIN}"
if [ ! -x "$BIN" ]; then
  BIN="$LEGACY_NYASH_BIN"
fi

if [ ! -x "$BIN" ]; then
  log_error "hakorune binary not found: $BIN"
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
RUST_BAD_FASTMEM="$TMPDIR/rust_bad_fastmem.hako"
RUST_BAD_UNSAFE="$TMPDIR/rust_bad_unsafe.hako"
HAKO_BAD_FASTMEM="$TMPDIR/hako_bad_fastmem.hako"
HAKO_BAD_UNSAFE="$TMPDIR/hako_bad_unsafe.hako"

expect_rust_parse_fail() {
  local src_path="$1"
  local expect="$2"
  local label="$3"
  local out_path="$TMPDIR/${label}.ast.json"
  local log_path="$TMPDIR/${label}.rust.log"

  if NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$out_path" "$src_path" \
    >"$log_path" 2>&1; then
    log_error "Rust parser unexpectedly accepted $label"
    cat "$out_path" >&2 || true
    exit 1
  fi

  if ! grep -Fq "$expect" "$log_path"; then
    log_error "Rust parser missing expected fail-fast tag for $label"
    tail -n 120 "$log_path" >&2 || true
    exit 1
  fi
}

expect_hako_parse_fail() {
  local src_path="$1"
  local expect="$2"
  local label="$3"
  local out_path="$TMPDIR/${label}.program.json"
  local log_path="$TMPDIR/${label}.hako.log"

  if NYASH_FEATURES="$FEATURES" \
    bash "$NYASH_ROOT/tools/archive/legacy-selfhost/engineering/program_json_v0_stageb_artifact_probe.sh" \
      --in "$src_path" --out "$out_path" \
    >"$log_path" 2>&1; then
    log_error ".hako ParserBox unexpectedly accepted $label"
    cat "$out_path" >&2 || true
    exit 1
  fi

  if ! grep -Fq "$expect" "$log_path"; then
    log_error ".hako ParserBox missing expected fail-fast tag for $label"
    tail -n 120 "$log_path" >&2 || true
    exit 1
  fi
}

cat >"$RUST_SRC" <<'HK'
static box Main {
  @rune Inline(prefer)
  @rune FastMemory(PageMapV0)
  main() {
    local x = 1 + 2
    local y = (1 << 3) & 7 | (8 >> 1) ^ 2
    fastmem PageMapV0 {
      local z = (y >> 1) & 3
    }
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

rune_names = {
    node.get("name")
    for node in walk(data)
    if isinstance(node.get("args"), list)
}
for name in ("Inline", "FastMemory"):
    if name not in rune_names:
        print(f"missing Rust parser rune metadata: {name}", file=sys.stderr)
        sys.exit(1)

if not any(
    node.get("kind") == "FastMemRegion" and node.get("contract") == "PageMapV0"
    for node in walk(data)
):
    print("missing Rust parser FastMemRegion(PageMapV0)", file=sys.stderr)
    sys.exit(1)
PY

cat >"$HAKO_DRIVER" <<'HK'
static box Main {
  @rune Inline(prefer)
  @rune FastMemory(PageMapV0)
  main() {
    local x = 1 + 2
    local y = (1 << 3) & 7 | (8 >> 1) ^ 2
    fastmem PageMapV0 {
      local z = (y >> 1) & 3
    }
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

if ! grep -Fq '"type":"FastMemRegion"' "$HAKO_JSON" || ! grep -Fq '"contract":"PageMapV0"' "$HAKO_JSON"; then
  log_error ".hako ParserBox baseline missing FastMemRegion(PageMapV0)"
  cat "$HAKO_JSON" >&2 || true
  tail -n 120 "$HAKO_LOG" >&2 || true
  exit 1
fi

if [ ! -s "$HAKO_JSON" ]; then
  log_error ".hako ParserBox baseline produced empty Program(JSON)"
  tail -n 120 "$HAKO_LOG" >&2 || true
  exit 1
fi

cat >"$RUST_BAD_FASTMEM" <<'HK'
static box Main {
  main() {
    fastmem {
      local x = 1
    }
    return 0
  }
}
HK

cat >"$RUST_BAD_UNSAFE" <<'HK'
static box Main {
  main() {
    unsafe {
      local x = 1
    }
    return 0
  }
}
HK

cp "$RUST_BAD_FASTMEM" "$HAKO_BAD_FASTMEM"
cp "$RUST_BAD_UNSAFE" "$HAKO_BAD_UNSAFE"

expect_rust_parse_fail \
  "$RUST_BAD_FASTMEM" \
  "[freeze:contract][parser/fastmem] contract name after fastmem" \
  "rust_bad_fastmem"

expect_rust_parse_fail \
  "$RUST_BAD_UNSAFE" \
  "[freeze:contract][parser/unsafe] unsafe block is not supported; use fastmem ContractName { ... }" \
  "rust_bad_unsafe"

expect_hako_parse_fail \
  "$HAKO_BAD_FASTMEM" \
  "[freeze:contract][parser/fastmem] contract name after fastmem" \
  "hako_bad_fastmem"

expect_hako_parse_fail \
  "$HAKO_BAD_UNSAFE" \
  "[freeze:contract][parser/unsafe] unsafe block is not supported; use fastmem ContractName { ... }" \
  "hako_bad_unsafe"

log_success "fastmem_parser_parity_smoke"
