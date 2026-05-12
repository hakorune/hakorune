#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-size-class-usize-policy"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SIZE_CLASS="lang/src/hako_alloc/memory/size_class_box.hako"
APP="apps/mimalloc-size-class-usize-policy-proof/main.hako"
APP_TEST="apps/mimalloc-size-class-usize-policy-proof/test.sh"
APP_README="apps/mimalloc-size-class-usize-policy-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-198-M187-SIZE-CLASS-USIZE-POLICY.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_size_class_usize_policy_guard.sh"

echo "[$TAG] checking M187 size-class usize policy facade"

guard_require_files \
  "$TAG" \
  "$SIZE_CLASS" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$PLAN" \
  "$INDEX"

guard_expect_in_file "$TAG" 'size_to_bin_usize\(size: usize\)' "$SIZE_CLASS" "M187 must add usize size-to-bin facade"
guard_expect_in_file "$TAG" 'good_size_usize\(size: usize\)' "$SIZE_CLASS" "M187 must add usize good-size facade"
guard_expect_in_file "$TAG" 'bin_size_usize\(bin: usize\)' "$SIZE_CLASS" "M187 must add usize bin-size facade"
guard_expect_in_file "$TAG" 'accepts_usize\(size: usize\)' "$SIZE_CLASS" "M187 must add usize accepts facade"
guard_expect_in_file "$TAG" 'return me\.good_size\(size\)' "$SIZE_CLASS" "M187 good-size facade must delegate to signed sentinel owner"
guard_expect_in_file "$TAG" 'return me\.bin_size\(bin\)' "$SIZE_CLASS" "M187 bin-size facade must delegate to signed sentinel owner"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.size_class_box as SizeClassBox' "$APP" "proof app must import SizeClassBox"
guard_expect_in_file "$TAG" 'M187 exact usize for size-class policy` \| Complete' "$PLAN" "plan must mark M187 complete"
guard_expect_in_file "$TAG" '293x-198 M187 Size-Class usize Policy' "$CARD" "missing M187 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M187 guard"

if rg -n '^[[:space:]]+[A-Za-z_][A-Za-z0-9_]*:[[:space:]]+usize' "$SIZE_CLASS" >/tmp/"$TAG".stored_usize 2>&1; then
  echo "[$TAG] ERROR: M187 must not add stored usize fields to SizeClassBox" >&2
  cat /tmp/"$TAG".stored_usize >&2
  rm -f /tmp/"$TAG".stored_usize
  exit 1
fi
rm -f /tmp/"$TAG".stored_usize

if rg -n 'provider|install_hook|hook_install|hako_mem_|externcall|OSVM|OsVm|page_map|releaseLocal\(|aligned_alloc|HugeRelease|huge_release' \
  "$SIZE_CLASS" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M187 leaked beyond size-class policy scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-size-class-usize-policy|size_to_bin_usize|good_size_usize|bin_size_usize' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M187 app/usize facade matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

tmp_dir="$(mktemp -d /tmp/hakorune_m187_size_class_usize.XXXXXX)"
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

rg -F -q 'mimalloc-size-class-usize-policy-proof' "$out"
rg -F -q 'bins=1,5,9,72,73' "$out"
rg -F -q 'sizes=8,64,4194304,-1' "$out"
rg -F -q 'good=8,40,655360,-1' "$out"
rg -F -q 'accepts=1,0' "$out"
rg -F -q 'summary=ok' "$out"

cat "$out"
echo "[$TAG] ok"
