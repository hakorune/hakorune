#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-usize-page-heap-requested-bytes"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

PAGE_HEAP="lang/src/hako_alloc/memory/page_heap_box.hako"
NUMERIC_FIELDS="lang/src/hako_alloc/memory/NUMERIC_FIELDS.md"
CARD="docs/development/current/main/phases/phase-294x/294x-256-HAKO-ALLOC-USIZE-PAGE-HEAP-REQUESTED-BYTES.md"
INDEX="docs/tools/check-scripts-index.md"
MIMALLOC_LITE="apps/mimalloc-lite/test.sh"
ALLOCATOR_STRESS="apps/allocator-stress/test.sh"
BOXTORRENT="apps/boxtorrent-mini/test.sh"
MIR="${TMPDIR:-/tmp}/hakorune_page_heap_requested_bytes_usize.mir.json"

echo "[$TAG] checking page_heap requested_bytes exact usize field group"

guard_require_files \
  "$TAG" \
  "$PAGE_HEAP" \
  "$NUMERIC_FIELDS" \
  "$CARD" \
  "$INDEX" \
  "$MIMALLOC_LITE" \
  "$ALLOCATOR_STRESS" \
  "$BOXTORRENT"

guard_expect_in_file "$TAG" 'requested_bytes: usize = 0' "$PAGE_HEAP" "page heap requested_bytes must be exact usize"
guard_expect_in_file "$TAG" 'requestedBytes()' "$PAGE_HEAP" "page heap must keep requestedBytes observer"
guard_expect_fixed_in_file "$TAG" 'me.requested_bytes = me.requested_bytes + requested_size' "$PAGE_HEAP" "page heap must keep accepted request byte accumulation"
guard_expect_in_file "$TAG" 'requested_size: usize' "$PAGE_HEAP" "handle requested_size is exact usize after handle-payload row"
guard_expect_in_file "$TAG" 'block_size: usize' "$PAGE_HEAP" "page heap block_size is exact usize after size-class row"
guard_expect_in_file "$TAG" 'capacity: usize' "$PAGE_HEAP" "page heap capacity is exact usize after capacity row"
guard_expect_in_file "$TAG" 'free_top: usize' "$PAGE_HEAP" "page heap free_top is exact usize after stack-top row"
guard_expect_in_file "$TAG" 'HakoAllocPage` | `requested_bytes` | `usize`' "$NUMERIC_FIELDS" "numeric inventory must mark requested_bytes exact usize"
guard_expect_in_file "$TAG" 'HAKO-ALLOC-USIZE-FIELD-GROUP-255' "$NUMERIC_FIELDS" "numeric inventory must record field group 255"
guard_expect_in_file "$TAG" '294x-256 Hako Alloc Usize Page Heap Requested Bytes' "$CARD" "missing field-group card"
guard_expect_in_file "$TAG" 'k2_wide_hako_alloc_usize_page_heap_requested_bytes_guard.sh' "$INDEX" "check script index must list requested-bytes guard"

if rg -n 'requested_bytes: i64 = 0' "$PAGE_HEAP" >/tmp/"$TAG".stale 2>&1; then
  echo "[$TAG] ERROR: stale signed requested_bytes storage remains in page_heap_box" >&2
  cat /tmp/"$TAG".stale >&2
  rm -f /tmp/"$TAG".stale
  exit 1
fi
rm -f /tmp/"$TAG".stale

bash "$MIMALLOC_LITE"
bash "$ALLOCATOR_STRESS"
bash "$BOXTORRENT"

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

field = fields.get("requested_bytes")
if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
    raise SystemExit(f"HakoAllocPage.requested_bytes must be exact usize storage: {field}")

field = fields.get("block_size")
if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
    raise SystemExit(f"HakoAllocPage.block_size must be exact usize storage after block-size row: {field}")

field = fields.get("capacity")
if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
    raise SystemExit(f"HakoAllocPage.capacity must be exact usize storage after capacity row: {field}")

field = fields.get("free_top")
if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
    raise SystemExit(f"HakoAllocPage.free_top must be exact usize storage after stack-top row: {field}")

for name in ("page_id",):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"HakoAllocPage.{name} must remain i64 storage: {field}")
PY

rm -f /tmp/"$TAG".emit.out /tmp/"$TAG".emit.err

echo "[$TAG] ok"
