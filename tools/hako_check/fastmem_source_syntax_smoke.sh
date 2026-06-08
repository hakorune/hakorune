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

run_fastmem_source_manifest_seed() {
  python3 "$ROOT/tools/hako_check/fastmem_source_manifest_runner.py" \
    --manifest "$ROOT/tools/hako_check/manifests/fastmem_source_syntax_smoke.toml"
}

run_fastmem_source_manifest_seed

emit_fastmem_producer_report() {
  local profile="$1"
  local mir_json="$2"
  local out="$3"

  bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
    --profile "$profile" \
    --mir-json "$mir_json" \
    --out "$out"
}

assert_fastmem_report_check_ok() {
  local report="$1"
  local check="$2"

  bash "$ROOT/tools/hako_check.sh" fastmem-check \
    --inventory "$report" \
    --format kv \
    --out "$check"

  grep -q '^summary=ok$' "$check"
  grep -q '^failure_count=0$' "$check"
}

GOOD_SRC="$TMPDIR/good.hako"
GOOD_AST="$TMPDIR/good.ast.json"
GOOD_INV="$TMPDIR/good.inventory.kv"
GOOD_CHECK="$TMPDIR/good.check.kv"
BAD_SRC="$TMPDIR/bad.hako"
BAD_AST="$TMPDIR/bad.ast.json"
BAD_INV="$TMPDIR/bad.inventory.kv"
BAD_CHECK="$TMPDIR/bad.check.kv"
BAD_BRANCH_SRC="$TMPDIR/bad_branch.hako"
BAD_BRANCH_MIR="$TMPDIR/bad_branch.mir.json"
BAD_BRANCH_LOG="$TMPDIR/bad_branch.log"
cat >"$GOOD_SRC" <<'HK'
static box Main {
  main(ptr) {
    local page_table = ptr
    local page_index = 0
    fastmem PageMapV0 {
      local addr = mem.addr(ptr)
      local key = (addr >> 12) & 255
      local page = page_table[page_index]
      local capacity = page.capacity
      page.used = capacity
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
grep -q '^fastmem_memop_table_index_count=1$' "$GOOD_INV"
grep -q '^field_access_required_verified_direct_count=2$' "$GOOD_INV"
grep -q '^field_access_required_verified_direct_miss_count=0$' "$GOOD_INV"
grep -q '^fastmem_verified_field_access_count=2$' "$GOOD_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$GOOD_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$GOOD_INV"
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

cat >"$BAD_BRANCH_SRC" <<'HK'
static box Main {
  main(ptr) {
    fastmem PageMapV0 {
      if true {
        local addr = mem.addr(ptr)
      } else {
        local addr = mem.addr(ptr)
      }
    }
    return 0
  }
}
HK

if NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$BAD_BRANCH_MIR" "$BAD_BRANCH_SRC" >"$BAD_BRANCH_LOG" 2>&1; then
  echo "[TEST/FAIL] fastmem branch CFG was accepted" >&2
  cat "$BAD_BRANCH_LOG" >&2 || true
  exit 1
fi
# The MIRBuilder unit test pins the precise FastMemory rejection tag. The CLI
# path can still mask builder Err with the existing lexical-scope cleanup
# fail-fast; this smoke only requires that unsupported branch CFG does not pass.
grep -Eq '\[freeze:contract\]\[(fastmem/branch_cfg_requires_owner_eq_condition|lexical_scope/unbalanced_pop)\]' "$BAD_BRANCH_LOG"

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
