#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-usize-page-heap-capacity"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

PAGE_HEAP="lang/src/hako_alloc/memory/page_heap_box.hako"
NUMERIC_FIELDS="lang/src/hako_alloc/memory/NUMERIC_FIELDS.md"
CARD="docs/development/current/main/phases/phase-294x/294x-262-HAKO-ALLOC-USIZE-PAGE-HEAP-CAPACITY.md"
INDEX="docs/tools/check-scripts-index.md"
MIMALLOC_LITE="apps/mimalloc-lite/test.sh"
ALLOCATOR_STRESS="apps/allocator-stress/test.sh"
OBJECT_RETURN="apps/mimalloc-object-return-api-proof/test.sh"
RESULT_CONTRACT="apps/mimalloc-result-contract-proof/test.sh"
MIR="${TMPDIR:-/tmp}/hakorune_page_heap_capacity_usize.mir.json"

echo "[$TAG] checking page_heap capacity exact usize field group"

guard_require_files \
  "$TAG" \
  "$PAGE_HEAP" \
  "$NUMERIC_FIELDS" \
  "$CARD" \
  "$INDEX" \
  "$MIMALLOC_LITE" \
  "$ALLOCATOR_STRESS" \
  "$OBJECT_RETURN" \
  "$RESULT_CONTRACT"

guard_expect_in_file "$TAG" 'capacity: usize' "$PAGE_HEAP" "page heap capacity must be exact usize"
guard_expect_in_file "$TAG" 'block_size: usize' "$PAGE_HEAP" "page heap block_size remains exact usize"
guard_expect_in_file "$TAG" 'free_top: usize' "$PAGE_HEAP" "page heap free_top is exact usize after stack-top row"
guard_expect_fixed_in_file "$TAG" 'me.free_top = capacity' "$PAGE_HEAP" "capacity still seeds free_top"
guard_expect_in_file "$TAG" 'loop\(i < capacity\)' "$PAGE_HEAP" "seed loop must still be bounded by capacity"
guard_expect_in_file "$TAG" 'if handle\.block_id >= me\.capacity' "$PAGE_HEAP" "live-handle bound must still use capacity"
guard_expect_fixed_in_file "$TAG" '| `page_heap_box.hako` | `HakoAllocPage` | `capacity` | `usize` |' "$NUMERIC_FIELDS" "numeric inventory must mark capacity exact usize"
guard_expect_in_file "$TAG" 'HAKO-ALLOC-USIZE-FIELD-GROUP-261' "$NUMERIC_FIELDS" "numeric inventory must record field group 261"
guard_expect_in_file "$TAG" '294x-262 Hako Alloc Usize Page Heap Capacity' "$CARD" "missing field-group card"
guard_expect_in_file "$TAG" 'k2_wide_hako_alloc_usize_page_heap_capacity_guard.sh' "$INDEX" "check script index must list capacity guard"

if rg -n 'capacity: i64' "$PAGE_HEAP" >/tmp/"$TAG".stale 2>&1; then
  echo "[$TAG] ERROR: stale signed page heap capacity storage remains in page_heap_box" >&2
  cat /tmp/"$TAG".stale >&2
  rm -f /tmp/"$TAG".stale
  exit 1
fi
rm -f /tmp/"$TAG".stale

bash "$MIMALLOC_LITE"
bash "$ALLOCATOR_STRESS"
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

for name in ("page_id",):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"HakoAllocPage.{name} must remain i64 storage: {field}")
PY

rm -f /tmp/"$TAG".emit.out /tmp/"$TAG".emit.err

echo "[$TAG] ok"
