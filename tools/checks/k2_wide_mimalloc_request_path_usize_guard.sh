#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-request-path-usize"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

ALIGNMENT="lang/src/hako_alloc/memory/alignment_policy_box.hako"
PAGE="lang/src/hako_alloc/memory/page_box.hako"
SMALL_PATH="lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako"
ROUTER="lang/src/hako_alloc/memory/huge_threshold_router_box.hako"
APP="apps/mimalloc-request-path-usize-proof/main.hako"
APP_TEST="apps/mimalloc-request-path-usize-proof/test.sh"
APP_README="apps/mimalloc-request-path-usize-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-199-M188-REQUEST-PATH-USIZE.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_request_path_usize_guard.sh"

echo "[$TAG] checking M188 request-path usize facades"

guard_require_files \
  "$TAG" \
  "$ALIGNMENT" \
  "$PAGE" \
  "$SMALL_PATH" \
  "$ROUTER" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$PLAN" \
  "$INDEX"

guard_expect_in_file "$TAG" 'normalize_alignment_usize\(alignment: usize\)' "$ALIGNMENT" "M188 must add usize alignment normalization"
guard_expect_in_file "$TAG" 'padded_request_size_usize\(size: usize, alignment: usize\)' "$ALIGNMENT" "M188 must add usize padded request policy"
guard_expect_in_file "$TAG" 'aligned_good_size_usize\(size: usize, alignment: usize\)' "$ALIGNMENT" "M188 must add usize aligned good-size policy"
guard_expect_in_file "$TAG" 'accepts_usize\(size: usize, alignment: usize\)' "$ALIGNMENT" "M188 must add usize alignment accept facade"
guard_expect_in_file "$TAG" 'acquire_usize\(requested_size: usize\)' "$PAGE" "M188 must add usize page acquire facade"
guard_expect_in_file "$TAG" 'allocateAlignedSmallUsize\(size: usize, alignment: usize\)' "$SMALL_PATH" "M188 must add usize aligned small-path facade"
guard_expect_in_file "$TAG" 'classifyAlignedRequestUsize\(size: usize, alignment: usize\)' "$ROUTER" "M188 must add usize routing classifier"
guard_expect_in_file "$TAG" 'allocateUsize\(size: usize\)' "$ROUTER" "M188 must add usize plain allocation facade"
guard_expect_in_file "$TAG" 'allocateAlignedUsize\(size: usize, alignment: usize\)' "$ROUTER" "M188 must add usize aligned allocation facade"
guard_expect_in_file "$TAG" 'M188 exact usize for request path` \| Complete' "$PLAN" "plan must mark M188 complete"
guard_expect_in_file "$TAG" '293x-199 M188 Request Path usize' "$CARD" "missing M188 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M188 guard"

if rg -n '^[[:space:]]+[A-Za-z_][A-Za-z0-9_]*:[[:space:]]+usize' "$ALIGNMENT" "$PAGE" "$SMALL_PATH" "$ROUTER" >/tmp/"$TAG".stored_usize 2>&1; then
  echo "[$TAG] ERROR: M188 must not add stored usize fields" >&2
  cat /tmp/"$TAG".stored_usize >&2
  rm -f /tmp/"$TAG".stored_usize
  exit 1
fi
rm -f /tmp/"$TAG".stored_usize

if rg -n 'provider|install_hook|hook_install|hako_mem_|externcall|OSVM|OsVm|unreserve|release_bytes|decommit|aligned_alloc|HugeRelease|huge_release' \
  "$ALIGNMENT" "$PAGE" "$SMALL_PATH" "$ROUTER" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M188 leaked beyond request-path usize facade scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-request-path-usize|allocateAlignedUsize|acquire_usize|request_path_usize' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M188 app/usize facade matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

tmp_dir="$(mktemp -d /tmp/hakorune_m188_request_path_usize.XXXXXX)"
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

rg -F -q 'mimalloc-request-path-usize-proof' "$out"
rg -F -q 'policy=8,31,112,1' "$out"
rg -F -q 'direct=0,1,0' "$out"
rg -F -q 'setup=1,1' "$out"
rg -F -q 'small=1,12000,8' "$out"
rg -F -q 'route=1,2,1,12001,111,0,2' "$out"
rg -F -q 'counters=2,1,1,1,1,1' "$out"
rg -F -q 'summary=ok' "$out"

cat "$out"
echo "[$TAG] ok"
