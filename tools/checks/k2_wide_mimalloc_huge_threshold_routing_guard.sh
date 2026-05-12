#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-huge-threshold-routing"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

ROUTER="lang/src/hako_alloc/memory/huge_threshold_router_box.hako"
ALIGNED_SMALL="lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako"
ALIGNMENT="lang/src/hako_alloc/memory/alignment_policy_box.hako"
SIZE_CLASS="lang/src/hako_alloc/memory/size_class_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APP="apps/mimalloc-huge-threshold-routing-proof/main.hako"
APP_TEST="apps/mimalloc-huge-threshold-routing-proof/test.sh"
APP_README="apps/mimalloc-huge-threshold-routing-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-190-M179-HUGE-THRESHOLD-ROUTING.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_huge_threshold_routing_guard.sh"

echo "[$TAG] checking M179 huge threshold/routing"

guard_require_files \
  "$TAG" \
  "$ROUTER" \
  "$ALIGNED_SMALL" \
  "$ALIGNMENT" \
  "$SIZE_CLASS" \
  "$MODULE" \
  "$ROOT_README" \
  "$MEMORY_README" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$PLAN" \
  "$INDEX"

guard_expect_in_file "$TAG" 'memory.huge_threshold_router_box = "memory/huge_threshold_router_box.hako"' "$MODULE" "hako module must export the M179 router"
guard_expect_in_file "$TAG" 'box HakoAllocHugeThresholdRouter' "$ROUTER" "missing M179 huge threshold router owner"
guard_expect_in_file "$TAG" 'hugeThreshold\(\)' "$ROUTER" "M179 must expose the huge threshold observer"
guard_expect_in_file "$TAG" 'classifyPaddedSize\(padded_size\)' "$ROUTER" "M179 must classify padded sizes before routing"
guard_expect_in_file "$TAG" 'allocateAligned\(size, alignment\)' "$ROUTER" "M179 must expose aligned routing entry"
guard_expect_in_file "$TAG" 'SizeClassBox\.size_to_bin\(padded_size\)' "$ROUTER" "M179 must classify through the size-class policy owner"
guard_expect_in_file "$TAG" 'me\.small_path\.allocateAlignedSmall\(size, alignment\)' "$ROUTER" "M179 must delegate small requests to the M178 owner"
guard_expect_in_file "$TAG" 'me\.huge_reject_count = me\.huge_reject_count \+ 1' "$ROUTER" "M179 must fail fast for huge unsupported requests"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.huge_threshold_router_box as HakoAllocHugeThresholdRouterBox' "$APP" "proof app must import the M179 router"
guard_expect_in_file "$TAG" 'HakoAllocHugeThresholdRouter' "$ROOT_README" "root README must document the M179 owner"
guard_expect_in_file "$TAG" 'huge_threshold_router_box.hako' "$MEMORY_README" "memory README must document the M179 module"
guard_expect_in_file "$TAG" 'M179 huge threshold and routing' "$PLAN" "plan must retain the M179 row"
guard_expect_in_file "$TAG" '293x-190 M179 Huge Threshold Routing' "$CARD" "missing M179 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M179 guard"

if rg -n 'init[[:space:]]*\{' "$ROUTER" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M179 router must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n 'HugePage|huge_page|page_model_huge|unreserve|release_bytes|decommit|OSVM|OsVm|secure|provider|hook|hako_mem_|externcall|memcpy|copy_bytes|aligned_alloc' \
  "$ROUTER" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M179 leaked beyond threshold/routing scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-huge-threshold-routing|HakoAllocHugeThresholdRouter|huge_threshold_router' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M179 app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

tmp_dir="$(mktemp -d /tmp/hakorune_m179_huge_threshold.XXXXXX)"
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

rg -F -q 'mimalloc-huge-threshold-routing-proof' "$out"
rg -F -q 'setup=1,1' "$out"
rg -F -q 'threshold=4194304,1,2' "$out"
rg -F -q 'alloc=1,12000,31,1,12001,111' "$out"
rg -F -q 'reject=0,2,0,-1,0,-2' "$out"
rg -F -q 'router=2,2,0,1,1,1,1,3' "$out"
rg -F -q 'small_path=2,0,0,0,0,0,2' "$out"
rg -F -q 'page=1,0,0,1,0,0,2,2' "$out"
rg -F -q 'summary=ok' "$out"

cat "$out"
echo "[$TAG] ok"
