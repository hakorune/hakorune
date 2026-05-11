#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-size-class-policy"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

APP="apps/mimalloc-size-class-policy-proof/main.hako"
APP_README="apps/mimalloc-size-class-policy-proof/README.md"
APP_TEST="apps/mimalloc-size-class-policy-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-163-M163-MIMALLOC-SIZE-CLASS-POLICY-OWNER.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
SIZE_CLASS="lang/src/hako_alloc/memory/size_class_box.hako"
LAYOUT="lang/src/hako_alloc/memory/layout_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
INDEX="docs/tools/check-scripts-index.md"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_size_class_policy_guard.sh"

echo "[$TAG] checking M163 mimalloc size-class policy owner"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$PLAN" \
  "$SIZE_CLASS" \
  "$LAYOUT" \
  "$MODULE" \
  "$INDEX" \
  "$ALLOCATOR_GROUP"

guard_expect_in_file "$TAG" 'static box SizeClassBox' "$SIZE_CLASS" "missing SizeClassBox owner"
guard_expect_in_file "$TAG" 'size_to_bin' "$SIZE_CLASS" "missing size_to_bin policy"
guard_expect_in_file "$TAG" 'bin_size' "$SIZE_CLASS" "missing bin_size policy"
guard_expect_in_file "$TAG" 'huge_bin' "$SIZE_CLASS" "missing huge bin sentinel"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.size_class_box as SizeClassBox' "$LAYOUT" "LayoutBox must delegate to SizeClassBox"
guard_expect_in_file "$TAG" 'memory.size_class_box = "memory/size_class_box.hako"' "$MODULE" "SizeClassBox must be exported"
guard_expect_in_file "$TAG" 'M163 mimalloc size-class policy owner' "$PLAN" "plan must retain M163 row"
guard_expect_in_file "$TAG" '293x-163 M163 Mimalloc Size-Class Policy Owner' "$CARD" "missing M163 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M163 guard"

if rg -F -q "$SELF_SCRIPT" "$ALLOCATOR_GROUP"; then
  guard_fail "$TAG" "M163 focused guard must not be registered as another wide allocator gate step"
fi

if rg -n 'mimalloc-size-class-policy|SizeClassBox|MI_SIZE_CLASS|MI_CLASS_CAP' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: size-class policy matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

tmp_dir="$(mktemp -d /tmp/hakorune_m163_size_class.XXXXXX)"
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

rg -F -q 'mimalloc-size-class-policy-proof' "$out"
rg -F -q 'bins=1,1,2,4,6,8,9,10,24,48,72,73' "$out"
rg -F -q 'sizes=8,32,64,80,96,128,160,1024,65536,524288,4194304' "$out"
rg -F -q 'good=8,40,80,96,655360,-1' "$out"
rg -F -q 'layout=0,1,-1,-1,32,64,1,0' "$out"
rg -F -q 'checks=5,4' "$out"
rg -F -q 'summary=ok' "$out"

cat "$out"
echo "[$TAG] ok"
