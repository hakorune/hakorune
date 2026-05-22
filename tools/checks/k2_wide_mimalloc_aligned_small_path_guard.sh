#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-aligned-small-path"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

PATH_BOX="lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako"
META_STORE="lang/src/hako_alloc/memory/aligned_small_meta_store_box.hako"
ALIGNMENT="lang/src/hako_alloc/memory/alignment_policy_box.hako"
PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
PAGE_RELEASE="lang/src/hako_alloc/memory/page_map_release_box.hako"
PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APP="apps/mimalloc-aligned-small-path-proof/main.hako"
APP_TEST="apps/mimalloc-aligned-small-path-proof/test.sh"
APP_README="apps/mimalloc-aligned-small-path-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-188-M178-ALIGNED-ALLOCATION-SMALL-PATH.md"
USIZE_CARD="docs/development/current/main/phases/phase-294x/294x-26-HAKO-ALLOC-USIZE-ALIGNED-SMALL-PATH-COUNTERS.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_aligned_small_path_guard.sh"

echo "[$TAG] checking M178 aligned allocation small path"

guard_require_files \
  "$TAG" \
  "$PATH_BOX" \
  "$META_STORE" \
  "$ALIGNMENT" \
  "$PAGE_MAP" \
  "$PAGE_RELEASE" \
  "$PAGE_BOX" \
  "$MODULE" \
  "$ROOT_README" \
  "$MEMORY_README" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$USIZE_CARD" \
  "$PLAN" \
  "$INDEX"

guard_expect_in_file "$TAG" 'box HakoAllocPageMapAlignedSmallPath' "$PATH_BOX" "missing M178 aligned small-path owner"
guard_expect_in_file "$TAG" 'allocateAlignedSmall\(size, alignment\)' "$PATH_BOX" "missing aligned small allocation entry"
guard_expect_in_file "$TAG" 'HakoAllocAlignmentPolicy\.normalize_alignment' "$PATH_BOX" "M178 must use the M177 alignment policy"
guard_expect_in_file "$TAG" 'me\.page_map\.register\(ptr, page\.page_id, block_id\)' "$PATH_BOX" "M178 must publish aligned small handles through page_map.register"
guard_expect_in_file "$TAG" 'meta_store: HakoAllocAlignedSmallMetaStore' "$PATH_BOX" "M178 must delegate live alignment metadata to the C205c store"
guard_expect_in_file "$TAG" 'meta_count: i64 = 0' "$PATH_BOX" "M178 meta_count mirror must remain i64 until metadata store count migrates"
guard_expect_in_file "$TAG" 'next_ptr: i64 = 12000' "$PATH_BOX" "M178 next_ptr must remain i64 pointer-shaped state"
guard_expect_in_file "$TAG" 'alloc_count: usize = 0' "$PATH_BOX" "M178 alloc counter must be exact usize"
guard_expect_in_file "$TAG" 'invalid_alignment_count: usize = 0' "$PATH_BOX" "M178 invalid-alignment counter must be exact usize"
guard_expect_in_file "$TAG" 'oversized_count: usize = 0' "$PATH_BOX" "M178 oversized counter must be exact usize"
guard_expect_in_file "$TAG" 'alloc_fail_count: usize = 0' "$PATH_BOX" "M178 alloc-fail counter must be exact usize"
guard_expect_in_file "$TAG" 'register_fail_count: usize = 0' "$PATH_BOX" "M178 register-fail counter must be exact usize"
guard_expect_in_file "$TAG" 'reject_count: usize = 0' "$PATH_BOX" "M178 reject counter must be exact usize"
guard_expect_in_file "$TAG" 'last_result_ptr: i64 = 0' "$PATH_BOX" "M178 result pointer observer must remain i64"
guard_expect_in_file "$TAG" 'last_alignment: i64 = 0' "$PATH_BOX" "M178 alignment observer must remain i64"
guard_expect_in_file "$TAG" 'last_padded_size: i64 = 0' "$PATH_BOX" "M178 padded-size observer must remain i64"
guard_expect_in_file "$TAG" 'box HakoAllocAlignedSmallMetaStore' "$META_STORE" "C205c aligned-small metadata store must exist"
guard_expect_in_file "$TAG" 'new HakoAllocAlignedSmallMeta' "$META_STORE" "C205c store must use the aligned-small metadata record seam"
guard_expect_in_file "$TAG" 'alignments: ArrayBox = new ArrayBox\(\)' "$META_STORE" "C205c store must keep live alignment scalar storage"
guard_expect_in_file "$TAG" 'alignmentFor\(ptr\): i64' "$PATH_BOX" "M178 must expose scalar alignment metadata for live ptrs"
guard_expect_in_file "$TAG" 'paddedSizeFor\(ptr\): i64' "$PATH_BOX" "M178 must expose scalar padded-size metadata for live ptrs"
guard_expect_in_file "$TAG" 'alignmentFor\(ptr\): i64' "$META_STORE" "C205c store alignmentFor must expose scalar return contract"
guard_expect_in_file "$TAG" 'alignmentAt\(index\): i64' "$META_STORE" "C205c store alignmentAt must expose scalar return contract"
guard_expect_in_file "$TAG" 'paddedSizeFor\(ptr\): i64' "$META_STORE" "C205c store paddedSizeFor must expose scalar return contract"
guard_expect_in_file "$TAG" 'paddedSizeAt\(index\): i64' "$META_STORE" "C205c store paddedSizeAt must expose scalar return contract"
guard_expect_in_file "$TAG" 'memory.page_map_aligned_small_path_box = "memory/page_map_aligned_small_path_box.hako"' "$MODULE" "hako module must export the M178 aligned small-path owner"
guard_expect_in_file "$TAG" 'HakoAllocPageMapAlignedSmallPath' "$ROOT_README" "root README must document the M178 owner"
guard_expect_in_file "$TAG" 'page_map_aligned_small_path_box.hako' "$MEMORY_README" "memory README must document the M178 module"
guard_expect_in_file "$TAG" 'M178 aligned allocation small path' "$PLAN" "plan must retain the M178 row"
guard_expect_in_file "$TAG" '293x-188 M178 Aligned Allocation Small Path' "$CARD" "missing M178 card"
guard_expect_in_file "$TAG" '294x-26 Hako Alloc Usize Aligned-Small Path Counters' "$USIZE_CARD" "missing aligned-small path counter usize card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M178 guard"

if rg -n 'init[[:space:]]*\{' "$PATH_BOX" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M178 owner must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n 'aligned_alloc|memcpy|copy_bytes|provider|hook|hako_mem_|externcall|Huge|huge|secure|remote_free|unreserve|decommit' \
  "$PATH_BOX" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M178 leaked out of aligned small-path scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-aligned-small-path|HakoAllocPageMapAlignedSmallPath|page_map_aligned_small_path' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M178 app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

tmp_dir="$(mktemp -d /tmp/hakorune_m178_aligned_small.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/out"
err="$tmp_dir/err"
mir="$tmp_dir/aligned_small.mir.json"

if [[ -n "${HAKORUNE_BIN:-}" ]]; then
  HAKO_CMD=("$HAKORUNE_BIN")
else
  HAKO_CMD=(cargo run -q --bin hakorune --)
fi

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "${HAKO_CMD[@]}" --backend vm "$APP" >"$out" 2>"$err"

rg -F -q 'mimalloc-aligned-small-path-proof' "$out"
rg -F -q 'setup=1,1' "$out"
rg -F -q 'alloc=1,12000,8,31,1,12001,64,111' "$out"
rg -F -q 'reject=0,0,0,0' "$out"
rg -F -q 'release=1,0,0' "$out"
rg -F -q 'path=2,2,1,1,0,4,2' "$out"
rg -F -q 'seam=1,1,0,0,0,0' "$out"
rg -F -q 'page=0,1,1,1,0,0,2,1' "$out"
rg -F -q 'summary=ok' "$out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" \
NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "${HAKO_CMD[@]}" --emit-mir-json "$mir" "$APP" >"$tmp_dir/emit.out" 2>"$tmp_dir/emit.err"

python3 - "$mir" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
required = {
    "main",
    "HakoAllocPageMapAlignedSmallPath.birth/1",
    "HakoAllocPageMapAlignedSmallPath.allocateAlignedSmall/2",
    "HakoAllocPageMapAlignedSmallPath.alignmentFor/1",
    "HakoAllocPageMapAlignedSmallPath.paddedSizeFor/1",
    "ProofCheck.expect/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

unsupported = []
for fn in functions.values():
    for plan in fn.get("metadata", {}).get("lowering_plan", []):
        if plan.get("emit_kind") == "unsupported":
            unsupported.append((fn.get("name"), plan.get("site"), plan.get("symbol"), plan.get("reason")))
if unsupported:
    raise SystemExit(f"unsupported lowering plans remain: {unsupported[:5]}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
path_plan = plans.get("HakoAllocPageMapAlignedSmallPath")
if path_plan is None:
    raise SystemExit("missing typed object plan: HakoAllocPageMapAlignedSmallPath")
fields = {field.get("name"): field for field in path_plan.get("fields", [])}
for name in (
    "alloc_count",
    "invalid_alignment_count",
    "oversized_count",
    "alloc_fail_count",
    "register_fail_count",
    "reject_count",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"aligned-small path {name} must be exact usize storage: {field}")
for name in ("meta_count", "next_ptr", "last_result_ptr", "last_alignment", "last_padded_size"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"aligned-small path {name} must remain i64 storage: {field}")
PY

cat "$out"
echo "[$TAG] ok"
