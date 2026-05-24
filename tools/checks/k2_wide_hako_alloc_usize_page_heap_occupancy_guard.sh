#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-usize-page-heap-occupancy"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

PAGE_HEAP="lang/src/hako_alloc/memory/page_heap_box.hako"
NUMERIC_FIELDS="lang/src/hako_alloc/memory/NUMERIC_FIELDS.md"
CARD="docs/development/current/main/phases/phase-294x/294x-254-HAKO-ALLOC-USIZE-PAGE-HEAP-OCCUPANCY.md"
INDEX="docs/tools/check-scripts-index.md"
MIMALLOC_LITE="apps/mimalloc-lite/test.sh"
ALLOCATOR_STRESS="apps/allocator-stress/test.sh"
MIR="${TMPDIR:-/tmp}/hakorune_page_heap_occupancy_usize.mir.json"

echo "[$TAG] checking page_heap occupancy exact usize field group"

guard_require_files \
  "$TAG" \
  "$PAGE_HEAP" \
  "$NUMERIC_FIELDS" \
  "$CARD" \
  "$INDEX" \
  "$MIMALLOC_LITE" \
  "$ALLOCATOR_STRESS"

guard_expect_in_file "$TAG" 'current_used: usize = 0' "$PAGE_HEAP" "page heap current_used must be exact usize"
guard_expect_in_file "$TAG" 'peak_used: usize = 0' "$PAGE_HEAP" "page heap peak_used must be exact usize"
guard_expect_in_file "$TAG" 'free_top: i64' "$PAGE_HEAP" "page heap free_top stays signed"
guard_expect_in_file "$TAG" 'requested_bytes: usize = 0' "$PAGE_HEAP" "page heap requested_bytes is exact usize after byte-sum row"
guard_expect_in_file "$TAG" 'HakoAllocPage` | `current_used` | `usize`' "$NUMERIC_FIELDS" "numeric inventory must mark current_used exact usize"
guard_expect_in_file "$TAG" 'HakoAllocPage` | `peak_used` | `usize`' "$NUMERIC_FIELDS" "numeric inventory must mark peak_used exact usize"
guard_expect_in_file "$TAG" 'HAKO-ALLOC-USIZE-FIELD-GROUP-253' "$NUMERIC_FIELDS" "numeric inventory must record field group 253"
guard_expect_in_file "$TAG" '294x-254 Hako Alloc Usize Page Heap Occupancy' "$CARD" "missing field-group card"
guard_expect_in_file "$TAG" 'k2_wide_hako_alloc_usize_page_heap_occupancy_guard.sh' "$INDEX" "check script index must list occupancy guard"

if rg -n 'current_used: i64 = 0|peak_used: i64 = 0' "$PAGE_HEAP" >/tmp/"$TAG".stale 2>&1; then
  echo "[$TAG] ERROR: stale signed occupancy storage remains in page_heap_box" >&2
  cat /tmp/"$TAG".stale >&2
  rm -f /tmp/"$TAG".stale
  exit 1
fi
rm -f /tmp/"$TAG".stale

bash "$MIMALLOC_LITE"
bash "$ALLOCATOR_STRESS"

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

for name in ("current_used", "peak_used"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"HakoAllocPage.{name} must be exact usize storage: {field}")

for name in ("requested_bytes",):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"HakoAllocPage.{name} must be exact usize storage after requested-bytes row: {field}")

for name in ("page_id", "block_size", "capacity", "free_top"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"HakoAllocPage.{name} must remain i64 storage: {field}")
PY

rm -f /tmp/"$TAG".emit.out /tmp/"$TAG".emit.err

echo "[$TAG] ok"
