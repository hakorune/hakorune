#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-aligned-small-metadata-record-store"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

STORE="lang/src/hako_alloc/memory/aligned_small_meta_store_box.hako"
PATH_BOX="lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako"
RECORDS="lang/src/hako_alloc/memory/allocator_metadata_records.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APP="apps/mimalloc-aligned-small-path-proof/main.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-216-C205C-ALIGNED-SMALL-METADATA-RECORD-STORE.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_aligned_small_metadata_record_store_guard.sh"

echo "[$TAG] checking C205c aligned-small metadata record store"

guard_require_files \
  "$TAG" \
  "$STORE" \
  "$PATH_BOX" \
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

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C205c card must be complete"
guard_expect_in_file "$TAG" 'C205c status:' "$PLAN" "mimalloc plan must record C205c status"
guard_expect_in_file "$TAG" '`C205c` is complete as `293x-216`' "$RECORD_SSOT" "record SSOT must mark C205c complete"
guard_expect_in_file "$TAG" '`293x-216`' "$PHASE_README" "phase README must list C205c row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C205c guard"

guard_expect_in_file "$TAG" 'memory.aligned_small_meta_store_box = "memory/aligned_small_meta_store_box.hako"' "$MODULE" "hako module must export C205c store"
guard_expect_in_file "$TAG" 'box HakoAllocAlignedSmallMetaStore' "$STORE" "C205c store owner must exist"
guard_expect_in_file "$TAG" 'new HakoAllocAlignedSmallMeta' "$STORE" "C205c store must construct aligned-small metadata records"
guard_expect_in_file "$TAG" 'me\.ptrs\.push\(meta\.ptr\)' "$STORE" "C205c store must read record ptr locally"
guard_expect_in_file "$TAG" 'me\.alignments\.push\(meta\.alignment\)' "$STORE" "C205c store must read record alignment locally"
guard_expect_in_file "$TAG" 'me\.padded_sizes\.push\(meta\.padded_size\)' "$STORE" "C205c store must read record padded size locally"
guard_expect_in_file "$TAG" 'meta_store: HakoAllocAlignedSmallMetaStore' "$PATH_BOX" "M178 owner must delegate metadata storage"
guard_expect_in_file "$TAG" 'me\.meta_store\.append\(ptr, normalized, padded_size\)' "$PATH_BOX" "M178 owner must append through C205c store"
guard_expect_in_file "$TAG" 'return me\.meta_store\.alignmentFor\(ptr\)' "$PATH_BOX" "M178 owner must read alignment through C205c store"
guard_expect_in_file "$TAG" 'return me\.meta_store\.paddedSizeFor\(ptr\)' "$PATH_BOX" "M178 owner must read padded size through C205c store"
guard_expect_in_file "$TAG" 'aligned_small_meta_store_box.hako' "$ROOT_README" "root README must document C205c store"
guard_expect_in_file "$TAG" 'aligned_small_meta_store_box.hako' "$MEMORY_README" "memory README must document C205c store"

if rg -n 'meta_ptrs|meta_alignments|meta_padded_sizes' "$PATH_BOX" >/tmp/"$TAG".path_columns 2>&1; then
  echo "[$TAG] ERROR: M178 owner still owns direct aligned-small metadata columns" >&2
  cat /tmp/"$TAG".path_columns >&2
  rm -f /tmp/"$TAG".path_columns
  exit 1
fi
rm -f /tmp/"$TAG".path_columns

if rg -n 'ArrayStorage::InlineRecord|InlineRecord' \
  lang/src/hako_alloc -g'*.hako' >/tmp/"$TAG".future 2>&1; then
  echo "[$TAG] ERROR: C205c leaked into packed ArrayBox storage" >&2
  cat /tmp/"$TAG".future >&2
  rm -f /tmp/"$TAG".future
  exit 1
fi
rm -f /tmp/"$TAG".future

if rg -n 'HakoAllocAlignedSmallMeta|HakoAllocAlignedSmallMetaStore|aligned_small_meta_store|InlineRecord' \
  lang/c-abi/shims src/llvm_py/instructions >/tmp/"$TAG".backend 2>&1; then
  echo "[$TAG] ERROR: C205c leaked into backend lowering surfaces" >&2
  cat /tmp/"$TAG".backend >&2
  rm -f /tmp/"$TAG".backend
  exit 1
fi
rm -f /tmp/"$TAG".backend

tmp_dir="$(mktemp -d /tmp/hakorune_c205c_aligned_meta.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir="$tmp_dir/m178.mir.json"
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
    if f.get("name") == "HakoAllocAlignedSmallMetaStore.append/3"
]
if not targets:
    raise SystemExit("HakoAllocAlignedSmallMetaStore.append/3 not found")

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

if any(node.get("op") == "newbox" and node.get("type") == "HakoAllocAlignedSmallMeta" for node in nodes):
    raise SystemExit("aligned metadata record leaked to NewBox in store append")
if any(node.get("op") == "field_get" and node.get("field") in {"ptr", "alignment", "padded_size"} for node in nodes):
    raise SystemExit("aligned metadata record field read leaked to FieldGet in store append")
PY

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm "$APP" >"$out" 2>"$err"

rg -F -q 'mimalloc-aligned-small-path-proof' "$out"
rg -F -q 'alloc=1,12000,8,31,1,12001,64,111' "$out"
rg -F -q 'release=1,0,0' "$out"
rg -F -q 'path=2,2,1,1,0,4,2' "$out"
rg -F -q 'summary=ok' "$out"

echo "[$TAG] ok"
