#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-metadata-record-declarations"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

RECORDS="lang/src/hako_alloc/memory/allocator_metadata_records.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
ALIGNED="lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako"
ALIGNED_STORE="lang/src/hako_alloc/memory/aligned_small_meta_store_box.hako"
HUGE="lang/src/hako_alloc/memory/huge_page_model_box.hako"
HUGE_STORE="lang/src/hako_alloc/memory/huge_page_meta_store_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-214-C205A-ALLOCATOR-METADATA-RECORD-DECLARATIONS.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_allocator_metadata_record_declarations_guard.sh"

echo "[$TAG] checking C205a allocator metadata record declarations"

guard_require_files \
  "$TAG" \
  "$RECORDS" \
  "$MODULE" \
  "$ROOT_README" \
  "$MEMORY_README" \
  "$ALIGNED" \
  "$ALIGNED_STORE" \
  "$HUGE" \
  "$HUGE_STORE" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$PHASE_README" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C205a card must be complete"
guard_expect_in_file "$TAG" 'record HakoAllocAlignedSmallMeta' "$RECORDS" "aligned-small metadata record must be declared"
guard_expect_in_file "$TAG" 'record HakoAllocHugePageMeta' "$RECORDS" "huge-page metadata record must be declared"
guard_expect_in_file "$TAG" 'memory.allocator_metadata_records = "memory/allocator_metadata_records.hako"' "$MODULE" "hako module must export C205a record declarations"
guard_expect_in_file "$TAG" 'allocator_metadata_records.hako' "$ROOT_README" "root README must document C205a owner"
guard_expect_in_file "$TAG" 'allocator_metadata_records.hako' "$MEMORY_README" "memory README must document C205a owner"
guard_expect_in_file "$TAG" 'C205a status:' "$PLAN" "mimalloc plan must record C205a status"
guard_expect_in_file "$TAG" '`C205a` is complete as `293x-214`' "$RECORD_SSOT" "record SSOT must mark C205a complete"
guard_expect_in_file "$TAG" '`293x-214`' "$PHASE_README" "phase README must list C205a row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C205a guard"

guard_expect_in_file "$TAG" 'meta_store: HakoAllocAlignedSmallMetaStore' "$ALIGNED" "M178 owner must delegate metadata storage after C205c"
guard_expect_in_file "$TAG" 'ptrs: ArrayBox = new ArrayBox\(\)' "$ALIGNED_STORE" "aligned-small ptr metadata scalar column must remain runtime truth inside store"
guard_expect_in_file "$TAG" 'alignments: ArrayBox = new ArrayBox\(\)' "$ALIGNED_STORE" "aligned-small alignment metadata scalar column must remain runtime truth inside store"
guard_expect_in_file "$TAG" 'padded_sizes: ArrayBox = new ArrayBox\(\)' "$ALIGNED_STORE" "aligned-small padded-size metadata scalar column must remain runtime truth inside store"
guard_expect_in_file "$TAG" 'meta_store: HakoAllocHugePageMetaStore' "$HUGE" "M180 owner must delegate metadata storage after C205d"
guard_expect_in_file "$TAG" 'page_ids: ArrayBox = new ArrayBox\(\)' "$HUGE_STORE" "huge page id metadata scalar column must remain runtime truth inside store"
guard_expect_in_file "$TAG" 'ptrs: ArrayBox = new ArrayBox\(\)' "$HUGE_STORE" "huge ptr metadata scalar column must remain runtime truth inside store"
guard_expect_in_file "$TAG" 'requested_sizes: ArrayBox = new ArrayBox\(\)' "$HUGE_STORE" "huge requested-size metadata scalar column must remain runtime truth inside store"
guard_expect_in_file "$TAG" 'committed_sizes: ArrayBox = new ArrayBox\(\)' "$HUGE_STORE" "huge committed-size metadata scalar column must remain runtime truth inside store"
guard_expect_in_file "$TAG" 'live_flags: ArrayBox = new ArrayBox\(\)' "$HUGE_STORE" "huge live flag metadata scalar column must remain runtime truth inside store"

if rg -n 'ArrayStorage::InlineRecord|inline-record|InlineRecord' \
  lang/src/hako_alloc -g'*.hako' >/tmp/"$TAG".runtime 2>&1; then
  echo "[$TAG] ERROR: allocator metadata records must not enable inline-record storage from hako_alloc" >&2
  cat /tmp/"$TAG".runtime >&2
  rm -f /tmp/"$TAG".runtime
  exit 1
fi
rm -f /tmp/"$TAG".runtime

if rg -n 'new HakoAllocHugePageMeta\(' lang/src/hako_alloc -g'*.hako' \
  | rg -v '^lang/src/hako_alloc/memory/huge_page_meta_store_box\.hako:' \
  >/tmp/"$TAG".huge_runtime 2>&1; then
  echo "[$TAG] ERROR: huge-page metadata record construction must stay in C205d store owner" >&2
  cat /tmp/"$TAG".huge_runtime >&2
  rm -f /tmp/"$TAG".huge_runtime
  exit 1
fi
rm -f /tmp/"$TAG".huge_runtime

if rg -n 'new HakoAllocAlignedSmallMeta\(' lang/src/hako_alloc -g'*.hako' \
  | rg -v '^lang/src/hako_alloc/memory/aligned_small_meta_store_box\.hako:' \
  >/tmp/"$TAG".aligned_runtime 2>&1; then
  echo "[$TAG] ERROR: aligned-small metadata record construction must stay in C205c store owner" >&2
  cat /tmp/"$TAG".aligned_runtime >&2
  rm -f /tmp/"$TAG".aligned_runtime
  exit 1
fi
rm -f /tmp/"$TAG".aligned_runtime

if rg -n 'HakoAllocAlignedSmallMeta|HakoAllocHugePageMeta|allocator_metadata_records' \
  lang/c-abi/shims src/llvm_py/instructions >/tmp/"$TAG".backend 2>&1; then
  echo "[$TAG] ERROR: C205a record declarations leaked into backend lowering surfaces" >&2
  cat /tmp/"$TAG".backend >&2
  rm -f /tmp/"$TAG".backend
  exit 1
fi
rm -f /tmp/"$TAG".backend

tmp_json="$(mktemp /tmp/hakorune_c205a_records.XXXXXX.json)"
trap 'rm -f "$tmp_json"' EXIT
NYASH_DISABLE_PLUGINS=1 cargo run -q --bin hakorune -- --emit-mir-json "$tmp_json" "$RECORDS" >/tmp/"$TAG".out 2>/tmp/"$TAG".err

guard_expect_in_file "$TAG" '"record_decls"' "$tmp_json" "MIR JSON must carry record declarations"
guard_expect_in_file "$TAG" '"record_layout_plans"' "$tmp_json" "MIR JSON must carry record layout plans"
guard_expect_in_file "$TAG" '"array_record_storage_plans"' "$tmp_json" "MIR JSON must carry array record storage plans"
guard_expect_in_file "$TAG" '"name": "HakoAllocAlignedSmallMeta"' "$tmp_json" "MIR JSON must include aligned-small record declaration"
guard_expect_in_file "$TAG" '"name": "HakoAllocHugePageMeta"' "$tmp_json" "MIR JSON must include huge-page record declaration"
guard_expect_in_file "$TAG" '"record_name": "HakoAllocAlignedSmallMeta"' "$tmp_json" "MIR JSON must include aligned-small storage descriptor"
guard_expect_in_file "$TAG" '"record_name": "HakoAllocHugePageMeta"' "$tmp_json" "MIR JSON must include huge-page storage descriptor"

echo "[$TAG] ok"
