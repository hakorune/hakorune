#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-huge-release-seam"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

RELEASE_SEAM="lang/src/hako_alloc/memory/huge_release_seam_box.hako"
HUGE_MODEL="lang/src/hako_alloc/memory/huge_page_model_box.hako"
HUGE_STORE="lang/src/hako_alloc/memory/huge_page_meta_store_box.hako"
PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APP="apps/mimalloc-huge-release-seam-proof/main.hako"
APP_TEST="apps/mimalloc-huge-release-seam-proof/test.sh"
APP_README="apps/mimalloc-huge-release-seam-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-192-M181-HUGE-RELEASE-SEAM.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_huge_release_seam_guard.sh"

echo "[$TAG] checking M181 huge release seam"

guard_require_files \
  "$TAG" \
  "$RELEASE_SEAM" \
  "$HUGE_MODEL" \
  "$HUGE_STORE" \
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

guard_expect_in_file "$TAG" 'memory.huge_release_seam_box = "memory/huge_release_seam_box.hako"' "$MODULE" "hako module must export the M181 huge release seam"
guard_expect_in_file "$TAG" 'box HakoAllocHugeReleaseSeam' "$RELEASE_SEAM" "missing M181 huge release seam owner"
guard_expect_in_file "$TAG" 'birth\(huge_model\)' "$RELEASE_SEAM" "M181 must take the huge model explicitly"
guard_expect_in_file "$TAG" 'releaseHugePtr\(ptr\)' "$RELEASE_SEAM" "M181 must expose a huge release entry"
guard_expect_in_file "$TAG" 'me\.page_map\.lookup\(ptr\)' "$RELEASE_SEAM" "M181 must start from page-map lookup"
guard_expect_in_file "$TAG" 'me\.huge_model\.isLiveHugePtr\(ptr\)' "$RELEASE_SEAM" "M181 must verify the ptr belongs to the huge model"
guard_expect_in_file "$TAG" 'me\.huge_model\.markReleased\(ptr\)' "$RELEASE_SEAM" "M181 must retire the huge model state before unregister"
guard_expect_in_file "$TAG" 'me\.page_map\.unregister\(ptr\)' "$RELEASE_SEAM" "M181 must unregister page-map ownership after model release"
guard_expect_in_file "$TAG" 'markReleased\(ptr\)' "$HUGE_MODEL" "M181 must add huge model release support"
guard_expect_in_file "$TAG" 'me\.meta_store\.markReleased\(ptr\)' "$HUGE_MODEL" "M181 model must delegate huge live-state clearing through C205d store"
guard_expect_in_file "$TAG" 'live_flags\.set\(index, 0\)' "$HUGE_STORE" "C205d store must clear huge live state explicitly"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.huge_release_seam_box as HakoAllocHugeReleaseSeamBox' "$APP" "proof app must import the M181 owner"
guard_expect_in_file "$TAG" 'HakoAllocHugeReleaseSeam' "$ROOT_README" "root README must document the M181 owner"
guard_expect_in_file "$TAG" 'huge_release_seam_box.hako' "$MEMORY_README" "memory README must document the M181 module"
guard_expect_in_file "$TAG" 'M181 huge release seam' "$PLAN" "plan must retain the M181 row"
guard_expect_in_file "$TAG" '293x-192 M181 Huge Release Seam' "$CARD" "missing M181 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M181 guard"

if rg -n 'init[[:space:]]*\{' "$RELEASE_SEAM" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M181 seam must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n 'releaseLocal\(|unreserve|release_bytes|decommit|OSVM|OsVm|secure|provider|hook|hako_mem_|externcall|memcpy|copy_bytes|aligned_alloc' \
  "$RELEASE_SEAM" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M181 leaked beyond huge release seam scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-huge-release-seam|HakoAllocHugeReleaseSeam|huge_release_seam' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M181 app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

tmp_dir="$(mktemp -d /tmp/hakorune_m181_huge_release.XXXXXX)"
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

rg -F -q 'mimalloc-huge-release-seam-proof' "$out"
rg -F -q 'setup=1,1,1,70000,70001' "$out"
rg -F -q 'release=1,1000,4194305,4194305,0' "$out"
rg -F -q 'reject=0,1,0,1,0,2' "$out"
rg -F -q 'huge=2,1,2,1,0,0,1' "$out"
rg -F -q 'seam=1,1,2,1,0,3' "$out"
rg -F -q 'map=3,2,3,4,2,1,0' "$out"
rg -F -q 'summary=ok' "$out"

cat "$out"
echo "[$TAG] ok"
