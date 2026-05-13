#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-huge-page-metadata-record-store"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

STORE="lang/src/hako_alloc/memory/huge_page_meta_store_box.hako"
MODEL="lang/src/hako_alloc/memory/huge_page_model_box.hako"
RECORDS="lang/src/hako_alloc/memory/allocator_metadata_records.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APP="apps/mimalloc-huge-page-model-proof/main.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-217-C205D-HUGE-PAGE-METADATA-RECORD-STORE.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_huge_page_metadata_record_store_guard.sh"

echo "[$TAG] checking C205d huge-page metadata record store"

guard_require_files \
  "$TAG" \
  "$STORE" \
  "$MODEL" \
  "$RECORDS" \
  "$MODULE" \
  "$ROOT_README" \
  "$MEMORY_README" \
  "$APP" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$PHASE_README" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C205d card must be complete"
guard_expect_in_file "$TAG" 'C205d status:' "$PLAN" "mimalloc plan must record C205d status"
guard_expect_in_file "$TAG" '`C205d` is complete as `293x-217`' "$RECORD_SSOT" "record SSOT must mark C205d complete"
guard_expect_in_file "$TAG" '`293x-217`' "$PHASE_README" "phase README must list C205d row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C205d guard"

guard_expect_in_file "$TAG" 'memory.huge_page_meta_store_box = "memory/huge_page_meta_store_box.hako"' "$MODULE" "hako module must export C205d store"
guard_expect_in_file "$TAG" 'box HakoAllocHugePageMetaStore' "$STORE" "C205d store owner must exist"
guard_expect_in_file "$TAG" 'new HakoAllocHugePageMeta' "$STORE" "C205d store must construct huge-page metadata records"
guard_expect_in_file "$TAG" 'me\.page_ids\.push\(meta\.page_id\)' "$STORE" "C205d store must read record page_id locally"
guard_expect_in_file "$TAG" 'me\.ptrs\.push\(meta\.ptr\)' "$STORE" "C205d store must read record ptr locally"
guard_expect_in_file "$TAG" 'me\.requested_sizes\.push\(meta\.requested_size\)' "$STORE" "C205d store must read record requested size locally"
guard_expect_in_file "$TAG" 'me\.committed_sizes\.push\(meta\.committed_size\)' "$STORE" "C205d store must read record committed size locally"
guard_expect_in_file "$TAG" 'me\.live_flags\.push\(meta\.live\)' "$STORE" "C205d store must read record live flag locally"
guard_expect_in_file "$TAG" 'meta_store: HakoAllocHugePageMetaStore' "$MODEL" "M180 owner must delegate metadata storage"
guard_expect_in_file "$TAG" 'me\.meta_store\.append\(page_id, ptr, requested_size, committed_size\)' "$MODEL" "M180 owner must append through C205d store"
guard_expect_in_file "$TAG" 'return me\.meta_store\.findIndex\(ptr\)' "$MODEL" "M180 owner must delegate findIndex through C205d store"
guard_expect_in_file "$TAG" 'return me\.meta_store\.pageIdAt\(index\)' "$MODEL" "M180 owner must read page id through C206e indexed store seam"
guard_expect_in_file "$TAG" 'return me\.meta_store\.requestedSizeAt\(index\)' "$MODEL" "M180 owner must read requested size through C206e indexed store seam"
guard_expect_in_file "$TAG" 'return me\.meta_store\.committedSizeAt\(index\)' "$MODEL" "M180 owner must read committed size through C206e indexed store seam"
guard_expect_in_file "$TAG" 'me\.meta_store\.markReleasedAt\(index\)' "$MODEL" "M180 owner must release through C206e indexed store seam"
guard_expect_in_file "$TAG" 'huge_page_meta_store_box.hako' "$ROOT_README" "root README must document C205d store"
guard_expect_in_file "$TAG" 'huge_page_meta_store_box.hako' "$MEMORY_README" "memory README must document C205d store"

if rg -n 'page_ids: ArrayBox|ptrs: ArrayBox|requested_sizes: ArrayBox|committed_sizes: ArrayBox|live_flags: ArrayBox' \
  "$MODEL" >/tmp/"$TAG".model_columns 2>&1; then
  echo "[$TAG] ERROR: M180 owner still owns direct huge-page metadata columns" >&2
  cat /tmp/"$TAG".model_columns >&2
  rm -f /tmp/"$TAG".model_columns
  exit 1
fi
rm -f /tmp/"$TAG".model_columns

if rg -n 'ArrayStorage::InlineRecord|InlineRecord|provider|hook|hako_mem_|externcall|memcpy|copy_bytes|aligned_alloc|releaseLocal|OSVM|OsVm' \
  "$STORE" "$MODEL" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: C205d leaked beyond huge metadata store scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'new HakoAllocHugePageMeta\(' lang/src/hako_alloc -g'*.hako' \
  | rg -v '^lang/src/hako_alloc/memory/huge_page_meta_store_box\.hako:' \
  >/tmp/"$TAG".hako_alloc 2>&1; then
  echo "[$TAG] ERROR: huge-page metadata record construction must stay in the C205d store owner" >&2
  cat /tmp/"$TAG".hako_alloc >&2
  rm -f /tmp/"$TAG".hako_alloc
  exit 1
fi
rm -f /tmp/"$TAG".hako_alloc

if rg -n 'HakoAllocHugePageMeta|HakoAllocHugePageMetaStore|huge_page_meta_store|InlineRecord' \
  lang/c-abi/shims src/llvm_py/instructions >/tmp/"$TAG".backend 2>&1; then
  echo "[$TAG] ERROR: C205d leaked into backend lowering surfaces" >&2
  cat /tmp/"$TAG".backend >&2
  rm -f /tmp/"$TAG".backend
  exit 1
fi
rm -f /tmp/"$TAG".backend

tmp_dir="$(mktemp -d /tmp/hakorune_c205d_huge_meta.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir="$tmp_dir/m180.mir.json"
out="$tmp_dir/vm.out"
err="$tmp_dir/vm.err"

NYASH_DISABLE_PLUGINS=1 cargo run -q --bin hakorune -- --emit-mir-json "$mir" "$APP" \
  >"$tmp_dir/emit.out" 2>"$tmp_dir/emit.err"

python3 - "$mir" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1]))
targets = [
    f for f in data.get("functions", [])
    if f.get("name") == "HakoAllocHugePageMetaStore.append/4"
]
if not targets:
    raise SystemExit("HakoAllocHugePageMetaStore.append/4 not found")

def walk(value):
    if isinstance(value, dict):
        yield value
        for child in value.values():
            yield from walk(child)
    elif isinstance(value, list):
        for child in value:
            yield from walk(child)

nodes = []
for fn in targets:
    nodes.extend(walk(fn))

if any(node.get("op") == "newbox" and node.get("type") == "HakoAllocHugePageMeta" for node in nodes):
    raise SystemExit("huge metadata record leaked to NewBox in store append")
if any(node.get("op") == "field_get" and node.get("field") in {"page_id", "ptr", "requested_size", "committed_size", "live"} for node in nodes):
    raise SystemExit("huge metadata record field read leaked to FieldGet in store append")
PY

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm "$APP" >"$out" 2>"$err"

rg -F -q 'mimalloc-huge-page-model-proof' "$out"
rg -F -q 'alloc0=1,70000,1000,4194305,4194305,1,1' "$out"
rg -F -q 'alloc1=1,70001,1001,8388608,8388608' "$out"
rg -F -q 'reject=0,1,0,2' "$out"
rg -F -q 'missing=0,-1,0' "$out"
rg -F -q 'huge=2,2,2,1,1,0,2' "$out"
rg -F -q 'map=2,2,2,1,0,0,0' "$out"
rg -F -q 'summary=ok' "$out"

echo "[$TAG] ok"
