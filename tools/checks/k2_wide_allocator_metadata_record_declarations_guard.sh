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
HUGE="lang/src/hako_alloc/memory/huge_page_model_box.hako"
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
  "$HUGE" \
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

guard_expect_in_file "$TAG" 'meta_ptrs: ArrayBox = new ArrayBox\(\)' "$ALIGNED" "M178 ptr metadata scalar column must remain runtime truth"
guard_expect_in_file "$TAG" 'meta_alignments: ArrayBox = new ArrayBox\(\)' "$ALIGNED" "M178 alignment metadata scalar column must remain runtime truth"
guard_expect_in_file "$TAG" 'meta_padded_sizes: ArrayBox = new ArrayBox\(\)' "$ALIGNED" "M178 padded-size metadata scalar column must remain runtime truth"
guard_expect_in_file "$TAG" 'page_ids: ArrayBox = new ArrayBox\(\)' "$HUGE" "M180 page id scalar column must remain runtime truth"
guard_expect_in_file "$TAG" 'ptrs: ArrayBox = new ArrayBox\(\)' "$HUGE" "M180 ptr scalar column must remain runtime truth"
guard_expect_in_file "$TAG" 'requested_sizes: ArrayBox = new ArrayBox\(\)' "$HUGE" "M180 requested-size scalar column must remain runtime truth"
guard_expect_in_file "$TAG" 'committed_sizes: ArrayBox = new ArrayBox\(\)' "$HUGE" "M180 committed-size scalar column must remain runtime truth"
guard_expect_in_file "$TAG" 'live_flags: ArrayBox = new ArrayBox\(\)' "$HUGE" "M180 live flag scalar column must remain runtime truth"

if rg -n 'new HakoAllocAlignedSmallMeta|new HakoAllocHugePageMeta|ArrayStorage::InlineRecord|inline-record|InlineRecord' \
  lang/src/hako_alloc -g'*.hako' >/tmp/"$TAG".runtime 2>&1; then
  echo "[$TAG] ERROR: C205a must not construct records or enable inline-record storage from hako_alloc" >&2
  cat /tmp/"$TAG".runtime >&2
  rm -f /tmp/"$TAG".runtime
  exit 1
fi
rm -f /tmp/"$TAG".runtime

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
