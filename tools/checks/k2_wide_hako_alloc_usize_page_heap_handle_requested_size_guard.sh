#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-usize-page-heap-handle-requested-size"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

PAGE_HEAP="lang/src/hako_alloc/memory/page_heap_box.hako"
NUMERIC_FIELDS="lang/src/hako_alloc/memory/NUMERIC_FIELDS.md"
CARD="docs/development/current/main/phases/phase-294x/294x-258-HAKO-ALLOC-USIZE-PAGE-HEAP-HANDLE-REQUESTED-SIZE.md"
INDEX="docs/tools/check-scripts-index.md"
OBJECT_RETURN="apps/mimalloc-object-return-api-proof/test.sh"
RESULT_CONTRACT="apps/mimalloc-result-contract-proof/test.sh"
APP="apps/mimalloc-object-return-api-proof/main.hako"
MIR="${TMPDIR:-/tmp}/hakorune_page_heap_handle_requested_size_usize.mir.json"

echo "[$TAG] checking page_heap handle requested_size exact usize field group"

guard_require_files \
  "$TAG" \
  "$PAGE_HEAP" \
  "$NUMERIC_FIELDS" \
  "$CARD" \
  "$INDEX" \
  "$OBJECT_RETURN" \
  "$RESULT_CONTRACT" \
  "$APP"

guard_expect_in_file "$TAG" 'box HakoAllocHandle' "$PAGE_HEAP" "page heap must keep handle object"
guard_expect_in_file "$TAG" 'page_id: i64' "$PAGE_HEAP" "handle page_id must stay signed"
guard_expect_in_file "$TAG" 'block_id: i64' "$PAGE_HEAP" "handle block_id must stay signed"
guard_expect_in_file "$TAG" 'requested_size: usize' "$PAGE_HEAP" "handle requested_size must be exact usize"
guard_expect_fixed_in_file "$TAG" 'birth(page_id, block_id, requested_size)' "$PAGE_HEAP" "handle birth parameter surface stays current-lane"
guard_expect_in_file "$TAG" 'allocate\(requested_size\)' "$PAGE_HEAP" "allocate parameter surface stays current-lane"
guard_expect_in_file "$TAG" 'resizeInPlace\(handle, requested_size\)' "$PAGE_HEAP" "resize parameter surface stays current-lane"
guard_expect_in_file "$TAG" 'realloc\(handle, requested_size\)' "$PAGE_HEAP" "realloc parameter surface stays current-lane"
guard_expect_in_file "$TAG" 'reallocResult\(handle, requested_size\)' "$PAGE_HEAP" "reallocResult parameter surface stays current-lane"
guard_expect_fixed_in_file "$TAG" 'me.requested_sizes.set(block_id, requested_size)' "$PAGE_HEAP" "requested_sizes array payload semantics stay unchanged"
guard_expect_fixed_in_file "$TAG" '| `page_heap_box.hako` | `HakoAllocHandle` | `requested_size` | `usize` |' "$NUMERIC_FIELDS" "numeric inventory must mark handle requested_size exact usize"
guard_expect_in_file "$TAG" 'HAKO-ALLOC-USIZE-FIELD-GROUP-257' "$NUMERIC_FIELDS" "numeric inventory must record field group 257"
guard_expect_in_file "$TAG" '294x-258 Hako Alloc Usize Page Heap Handle Requested Size' "$CARD" "missing field-group card"
guard_expect_in_file "$TAG" 'k2_wide_hako_alloc_usize_page_heap_handle_requested_size_guard.sh' "$INDEX" "check script index must list handle requested-size guard"

if rg -n 'requested_size: i64' "$PAGE_HEAP" >/tmp/"$TAG".stale 2>&1; then
  echo "[$TAG] ERROR: stale signed handle requested_size storage remains in page_heap_box" >&2
  cat /tmp/"$TAG".stale >&2
  rm -f /tmp/"$TAG".stale
  exit 1
fi
rm -f /tmp/"$TAG".stale

bash "$OBJECT_RETURN"
bash "$RESULT_CONTRACT"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" \
NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --emit-mir-json "$MIR" "$ROOT_DIR/$APP" >/tmp/"$TAG".emit.out 2>/tmp/"$TAG".emit.err

python3 - "$MIR" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
handle = plans.get("HakoAllocHandle")
if handle is None:
    raise SystemExit("missing typed object plan: HakoAllocHandle")
fields = {field.get("name"): field for field in handle.get("fields", [])}

for name in ("page_id", "block_id"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"HakoAllocHandle.{name} must remain i64 storage: {field}")

field = fields.get("requested_size")
if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
    raise SystemExit(f"HakoAllocHandle.requested_size must be exact usize storage: {field}")
PY

rm -f /tmp/"$TAG".emit.out /tmp/"$TAG".emit.err

echo "[$TAG] ok"
