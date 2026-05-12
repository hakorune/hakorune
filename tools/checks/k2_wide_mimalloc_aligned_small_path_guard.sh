#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-aligned-small-path"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

PATH_BOX="lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako"
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
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_aligned_small_path_guard.sh"

echo "[$TAG] checking M178 aligned allocation small path"

guard_require_files \
  "$TAG" \
  "$PATH_BOX" \
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
  "$PLAN" \
  "$INDEX"

guard_expect_in_file "$TAG" 'box HakoAllocPageMapAlignedSmallPath' "$PATH_BOX" "missing M178 aligned small-path owner"
guard_expect_in_file "$TAG" 'allocateAlignedSmall\(size, alignment\)' "$PATH_BOX" "missing aligned small allocation entry"
guard_expect_in_file "$TAG" 'HakoAllocAlignmentPolicy\.normalize_alignment' "$PATH_BOX" "M178 must use the M177 alignment policy"
guard_expect_in_file "$TAG" 'me\.page_map\.register\(ptr, page\.page_id, block_id\)' "$PATH_BOX" "M178 must publish aligned small handles through page_map.register"
guard_expect_in_file "$TAG" 'meta_alignments: ArrayBox = new ArrayBox\(\)' "$PATH_BOX" "M178 must store live alignment metadata"
guard_expect_in_file "$TAG" 'alignmentFor\(ptr\)' "$PATH_BOX" "M178 must expose alignment metadata for live ptrs"
guard_expect_in_file "$TAG" 'paddedSizeFor\(ptr\)' "$PATH_BOX" "M178 must expose padded-size metadata for live ptrs"
guard_expect_in_file "$TAG" 'memory.page_map_aligned_small_path_box = "memory/page_map_aligned_small_path_box.hako"' "$MODULE" "hako module must export the M178 aligned small-path owner"
guard_expect_in_file "$TAG" 'HakoAllocPageMapAlignedSmallPath' "$ROOT_README" "root README must document the M178 owner"
guard_expect_in_file "$TAG" 'page_map_aligned_small_path_box.hako' "$MEMORY_README" "memory README must document the M178 module"
guard_expect_in_file "$TAG" 'M178 aligned allocation small path' "$PLAN" "plan must retain the M178 row"
guard_expect_in_file "$TAG" '293x-188 M178 Aligned Allocation Small Path' "$CARD" "missing M178 card"
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

cat "$out"
echo "[$TAG] ok"
