#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-usize-page-heap-free-top"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

PAGE_HEAP="lang/src/hako_alloc/memory/page_heap_box.hako"
NUMERIC_FIELDS="lang/src/hako_alloc/memory/NUMERIC_FIELDS.md"
CARD="docs/development/current/main/phases/phase-294x/294x-264-HAKO-ALLOC-USIZE-PAGE-HEAP-FREE-TOP.md"
INDEX="docs/tools/check-scripts-index.md"
MIMALLOC_LITE="apps/mimalloc-lite/test.sh"
ALLOCATOR_STRESS="apps/allocator-stress/test.sh"
BOXTORRENT="apps/boxtorrent-mini/test.sh"
OBJECT_RETURN="apps/mimalloc-object-return-api-proof/test.sh"
RESULT_CONTRACT="apps/mimalloc-result-contract-proof/test.sh"
MIR="${TMPDIR:-/tmp}/hakorune_page_heap_free_top_usize.mir.json"

echo "[$TAG] checking page_heap free_top exact usize field group"

guard_require_files \
  "$TAG" \
  "$PAGE_HEAP" \
  "$NUMERIC_FIELDS" \
  "$CARD" \
  "$INDEX" \
  "$MIMALLOC_LITE" \
  "$ALLOCATOR_STRESS" \
  "$BOXTORRENT" \
  "$OBJECT_RETURN" \
  "$RESULT_CONTRACT"

guard_expect_in_file "$TAG" 'free_top: usize' "$PAGE_HEAP" "page heap free_top must be exact usize"
guard_expect_in_file "$TAG" 'capacity: usize' "$PAGE_HEAP" "page heap capacity remains exact usize"
guard_expect_fixed_in_file "$TAG" 'if me.free_top == 0 {' "$PAGE_HEAP" "allocation must keep underflow guard before decrement"
guard_expect_fixed_in_file "$TAG" 'me.free_top = me.free_top - 1' "$PAGE_HEAP" "allocation must keep guarded free_top decrement"
guard_expect_fixed_in_file "$TAG" 'me.free_stack.set(me.free_top, handle.block_id)' "$PAGE_HEAP" "release must keep free_stack push at free_top"
guard_expect_fixed_in_file "$TAG" 'me.free_top = me.free_top + 1' "$PAGE_HEAP" "release must keep free_top increment"
guard_expect_fixed_in_file "$TAG" '| `page_heap_box.hako` | `HakoAllocPage` | `free_top` | `usize` |' "$NUMERIC_FIELDS" "numeric inventory must mark free_top exact usize"
guard_expect_in_file "$TAG" 'HAKO-ALLOC-USIZE-FIELD-GROUP-263' "$NUMERIC_FIELDS" "numeric inventory must record field group 263"
guard_expect_in_file "$TAG" '294x-264 Hako Alloc Usize Page Heap Free Top' "$CARD" "missing field-group card"
guard_expect_in_file "$TAG" 'k2_wide_hako_alloc_usize_page_heap_free_top_guard.sh' "$INDEX" "check script index must list free-top guard"

if rg -n 'free_top: i64' "$PAGE_HEAP" >/tmp/"$TAG".stale 2>&1; then
  echo "[$TAG] ERROR: stale signed page heap free_top storage remains in page_heap_box" >&2
  cat /tmp/"$TAG".stale >&2
  rm -f /tmp/"$TAG".stale
  exit 1
fi
rm -f /tmp/"$TAG".stale

bash "$MIMALLOC_LITE"
bash "$ALLOCATOR_STRESS"
bash "$BOXTORRENT"
bash "$OBJECT_RETURN"
bash "$RESULT_CONTRACT"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" \
NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --emit-mir-json "$MIR" "$ROOT_DIR/apps/mimalloc-lite/main.hako" >/tmp/"$TAG".emit.out 2>/tmp/"$TAG".emit.err

python3 - "$MIR" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
page = plans.get("HakoAllocPage")
if page is None:
    raise SystemExit("missing typed object plan: HakoAllocPage")
fields = {field.get("name"): field for field in page.get("fields", [])}

for name in ("block_size", "capacity", "free_top", "alloc_count", "free_count", "reuse_count", "current_used", "peak_used", "requested_bytes"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"HakoAllocPage.{name} must be exact usize storage: {field}")

field = fields.get("page_id")
if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
    raise SystemExit(f"HakoAllocPage.page_id must remain i64 storage: {field}")
PY

rm -f /tmp/"$TAG".emit.out /tmp/"$TAG".emit.err

echo "[$TAG] ok"
