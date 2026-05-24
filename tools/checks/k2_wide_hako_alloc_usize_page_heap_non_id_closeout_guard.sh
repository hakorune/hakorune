#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-usize-page-heap-non-id-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

PAGE_HEAP="lang/src/hako_alloc/memory/page_heap_box.hako"
NUMERIC_FIELDS="lang/src/hako_alloc/memory/NUMERIC_FIELDS.md"
CARD="docs/development/current/main/phases/phase-294x/294x-266-HAKO-ALLOC-USIZE-PAGE-HEAP-NON-ID-CLOSEOUT.md"
INDEX="docs/tools/check-scripts-index.md"
MIMALLOC_LITE="apps/mimalloc-lite/test.sh"
ALLOCATOR_STRESS="apps/allocator-stress/test.sh"
BOXTORRENT="apps/boxtorrent-mini/test.sh"
OBJECT_RETURN="apps/mimalloc-object-return-api-proof/test.sh"
RESULT_CONTRACT="apps/mimalloc-result-contract-proof/test.sh"
MIR="${TMPDIR:-/tmp}/hakorune_page_heap_non_id_closeout.mir.json"

echo "[$TAG] checking page_heap exact non-id closeout"

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

for field in requested_size block_size capacity free_top alloc_count free_count reuse_count current_used peak_used requested_bytes; do
  guard_expect_in_file "$TAG" "${field}: usize" "$PAGE_HEAP" "page_heap non-id field must be exact usize: ${field}"
done

guard_expect_in_file "$TAG" 'page_id: i64' "$PAGE_HEAP" "page id storage must stay signed"
guard_expect_in_file "$TAG" 'block_id: i64' "$PAGE_HEAP" "handle block id storage must stay signed"
guard_expect_fixed_in_file "$TAG" '| `page_heap_box.hako` | `HakoAllocHandle` | `requested_size` | `usize` |' "$NUMERIC_FIELDS" "numeric inventory must keep handle requested_size exact"
guard_expect_fixed_in_file "$TAG" '| `page_heap_box.hako` | `HakoAllocPage` | `block_size` | `usize` |' "$NUMERIC_FIELDS" "numeric inventory must keep page block_size exact"
guard_expect_fixed_in_file "$TAG" '| `page_heap_box.hako` | `HakoAllocPage` | `capacity` | `usize` |' "$NUMERIC_FIELDS" "numeric inventory must keep page capacity exact"
guard_expect_fixed_in_file "$TAG" '| `page_heap_box.hako` | `HakoAllocPage` | `free_top` | `usize` |' "$NUMERIC_FIELDS" "numeric inventory must keep page free_top exact"
guard_expect_in_file "$TAG" '294x-266 Hako Alloc Usize Page Heap Non-Id Closeout' "$CARD" "missing closeout card"
guard_expect_in_file "$TAG" 'k2_wide_hako_alloc_usize_page_heap_non_id_closeout_guard.sh' "$INDEX" "check script index must list closeout guard"

if rg -n 'provider|install_hook|hook_install|global_allocator|hako_mem_|externcall|OSVM|OsVm|HugeRelease|huge_release' \
  "$PAGE_HEAP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: page_heap closeout leaked beyond local page-heap model scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

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
handle = plans.get("HakoAllocHandle")
if page is None:
    raise SystemExit("missing typed object plan: HakoAllocPage")
if handle is None:
    raise SystemExit("missing typed object plan: HakoAllocHandle")

page_fields = {field.get("name"): field for field in page.get("fields", [])}
handle_fields = {field.get("name"): field for field in handle.get("fields", [])}

for name in ("block_size", "capacity", "free_top", "alloc_count", "free_count", "reuse_count", "current_used", "peak_used", "requested_bytes"):
    field = page_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"HakoAllocPage.{name} must be exact usize storage: {field}")

for name in ("page_id",):
    field = page_fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"HakoAllocPage.{name} must remain i64 storage: {field}")

field = handle_fields.get("requested_size")
if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
    raise SystemExit(f"HakoAllocHandle.requested_size must be exact usize storage: {field}")

for name in ("page_id", "block_id"):
    field = handle_fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"HakoAllocHandle.{name} must remain i64 storage: {field}")
PY

rm -f /tmp/"$TAG".emit.out /tmp/"$TAG".emit.err

echo "[$TAG] ok"
