#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-huge-page-model"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

HUGE_MODEL="lang/src/hako_alloc/memory/huge_page_model_box.hako"
PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APP="apps/mimalloc-huge-page-model-proof/main.hako"
APP_TEST="apps/mimalloc-huge-page-model-proof/test.sh"
APP_README="apps/mimalloc-huge-page-model-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-191-M180-HUGE-PAGE-MODEL.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_huge_page_model_guard.sh"

echo "[$TAG] checking M180 huge page model"

guard_require_files \
  "$TAG" \
  "$HUGE_MODEL" \
  "$PAGE_MAP" \
  "$MODULE" \
  "$ROOT_README" \
  "$MEMORY_README" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$PLAN" \
  "$INDEX"

guard_expect_in_file "$TAG" 'memory.huge_page_model_box = "memory/huge_page_model_box.hako"' "$MODULE" "hako module must export the M180 huge page model"
guard_expect_in_file "$TAG" 'box HakoAllocHugePageModel' "$HUGE_MODEL" "missing M180 huge page model owner"
guard_expect_in_file "$TAG" 'allocateHuge\(requested_size, committed_size\)' "$HUGE_MODEL" "M180 must expose a huge allocation model entry"
guard_expect_in_file "$TAG" 'me\.page_map\.register\(ptr, page_id, 0\)' "$HUGE_MODEL" "M180 must publish huge handles through HakoAllocPageMap"
guard_expect_in_file "$TAG" 'requested_sizes: ArrayBox = new ArrayBox\(\)' "$HUGE_MODEL" "M180 must store requested-size metadata"
guard_expect_in_file "$TAG" 'committed_sizes: ArrayBox = new ArrayBox\(\)' "$HUGE_MODEL" "M180 must store committed-size metadata"
guard_expect_in_file "$TAG" 'live_flags: ArrayBox = new ArrayBox\(\)' "$HUGE_MODEL" "M180 must keep live metadata separate from page-local free lists"
guard_expect_in_file "$TAG" 'isLiveHugePtr\(ptr\)' "$HUGE_MODEL" "M180 must expose live-huge observer"
guard_expect_in_file "$TAG" 'requestedSizeFor\(ptr\)' "$HUGE_MODEL" "M180 must expose requested-size observer"
guard_expect_in_file "$TAG" 'committedSizeFor\(ptr\)' "$HUGE_MODEL" "M180 must expose committed-size observer"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.huge_page_model_box as HakoAllocHugePageModelBox' "$APP" "proof app must import the M180 owner"
guard_expect_in_file "$TAG" 'HakoAllocHugePageModel' "$ROOT_README" "root README must document the M180 owner"
guard_expect_in_file "$TAG" 'huge_page_model_box.hako' "$MEMORY_README" "memory README must document the M180 module"
guard_expect_in_file "$TAG" 'M180 huge page model' "$PLAN" "plan must retain the M180 row"
guard_expect_in_file "$TAG" '293x-191 M180 Huge Page Model' "$CARD" "missing M180 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M180 guard"

if rg -n 'init[[:space:]]*\{' "$HUGE_MODEL" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M180 model must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n '\.unregister\(|releasePtr\(|releaseLocal\(|HugeRelease|huge_release|unreserve|release_bytes|decommit|OSVM|OsVm|secure|provider|hook|hako_mem_|externcall|memcpy|copy_bytes|aligned_alloc' \
  "$HUGE_MODEL" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M180 leaked beyond huge-page model scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-huge-page-model|HakoAllocHugePageModel|huge_page_model' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M180 app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

tmp_dir="$(mktemp -d /tmp/hakorune_m180_huge_page_model.XXXXXX)"
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

rg -F -q 'mimalloc-huge-page-model-proof' "$out"
rg -F -q 'alloc0=1,70000,1000,4194305,4194305,1,1' "$out"
rg -F -q 'alloc1=1,70001,1001,8388608,8388608' "$out"
rg -F -q 'reject=0,1,0,2' "$out"
rg -F -q 'missing=0,-1,0' "$out"
rg -F -q 'huge=2,2,2,1,1,0,2' "$out"
rg -F -q 'map=2,2,2,1,0,0,0' "$out"
rg -F -q 'summary=ok' "$out"

cat "$out"
echo "[$TAG] ok"
