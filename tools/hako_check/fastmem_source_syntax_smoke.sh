#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"
if [ ! -x "$BIN" ]; then
  BIN="$ROOT/target/release/nyash"
fi

if [ ! -x "$BIN" ]; then
  echo "[TEST/FAIL] hakorune/nyash binary not found: $BIN" >&2
  exit 2
fi

FEATURES="${FASTMEM_SOURCE_FEATURES:-stage3,rune}"
TMPDIR="$(mktemp -d "${TMPDIR:-/tmp}/hako_fastmem_source.XXXXXX")"
cleanup() {
  rm -rf "$TMPDIR"
}
trap cleanup EXIT

GOOD_SRC="$TMPDIR/good.hako"
GOOD_AST="$TMPDIR/good.ast.json"
GOOD_INV="$TMPDIR/good.inventory.kv"
GOOD_CHECK="$TMPDIR/good.check.kv"
BAD_SRC="$TMPDIR/bad.hako"
BAD_AST="$TMPDIR/bad.ast.json"
BAD_INV="$TMPDIR/bad.inventory.kv"
BAD_CHECK="$TMPDIR/bad.check.kv"

cat >"$GOOD_SRC" <<'HK'
static box Main {
  main(ptr) {
    fastmem PageMapV0 {
      local addr = mem.addr(ptr)
      local key = (addr >> 12) & 255
    }
    return 0
  }
}
HK

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$GOOD_AST" "$GOOD_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$GOOD_AST" \
  --out "$GOOD_INV"

grep -q '^input_kind=ast_json$' "$GOOD_INV"
grep -q '^measured_hot_path_owner=hako_source$' "$GOOD_INV"
grep -q '^fastmem_region_count=1$' "$GOOD_INV"
grep -q '^fastmem_contract_count=1$' "$GOOD_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$GOOD_INV"
grep -q '^fastmem_contract_family=allocator.page_map$' "$GOOD_INV"
grep -q '^fastmem_memop_region_begin_count=1$' "$GOOD_INV"
grep -q '^fastmem_memop_region_end_count=1$' "$GOOD_INV"
grep -q '^fastmem_memop_unbalanced_region_count=0$' "$GOOD_INV"
grep -q '^fastmem_memop_addr_of_count=1$' "$GOOD_INV"
grep -q '^fastmem_memop_logical_shr_count=1$' "$GOOD_INV"
grep -q '^fastmem_memop_and_count=1$' "$GOOD_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$GOOD_INV"
grep -q '^fastmem_type_abi_hot_lookup_count=0$' "$GOOD_INV"
grep -q '^fastmem_provider_abi_crossing_count=0$' "$GOOD_INV"
grep -q '^summary=ok$' "$GOOD_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --ast-json "$GOOD_AST" \
  --format kv \
  --out "$GOOD_CHECK"
grep -q '^summary=ok$' "$GOOD_CHECK"
grep -q '^failure_count=0$' "$GOOD_CHECK"

cat >"$BAD_SRC" <<'HK'
static box Main {
  main(ptr) {
    fastmem PageMapV0 {
      local addr = mem.addr(ptr)
      local bad = arbitrary(ptr)
    }
    return 0
  }
}
HK

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$BAD_AST" "$BAD_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$BAD_AST" \
  --out "$BAD_INV"
grep -q '^fastmem_forbidden_call_count=1$' "$BAD_INV"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --ast-json "$BAD_AST" \
  --format kv \
  --out "$BAD_CHECK"; then
  echo "[TEST/FAIL] fastmem-check accepted forbidden call inside fastmem" >&2
  cat "$BAD_CHECK" >&2 || true
  exit 1
fi
grep -q '^summary=failed$' "$BAD_CHECK"
grep -q '^failure_0_reason=fastmem_forbidden_call_count$' "$BAD_CHECK"

echo "[TEST/OK] fastmem_source_syntax"
