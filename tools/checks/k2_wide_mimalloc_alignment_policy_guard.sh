#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-alignment-policy"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

APP="apps/mimalloc-alignment-policy-proof/main.hako"
APP_README="apps/mimalloc-alignment-policy-proof/README.md"
APP_TEST="apps/mimalloc-alignment-policy-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-187-M177-ALIGNMENT-POLICY-OBJECT.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
ALIGNMENT="lang/src/hako_alloc/memory/alignment_policy_box.hako"
SIZE_CLASS="lang/src/hako_alloc/memory/size_class_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_alignment_policy_guard.sh"

echo "[$TAG] checking M177 alignment policy object"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$PLAN" \
  "$ALIGNMENT" \
  "$SIZE_CLASS" \
  "$MODULE" \
  "$ROOT_README" \
  "$MEMORY_README" \
  "$INDEX"

guard_expect_in_file "$TAG" 'static box HakoAllocAlignmentPolicy' "$ALIGNMENT" "missing HakoAllocAlignmentPolicy owner"
guard_expect_in_file "$TAG" 'normalize_alignment' "$ALIGNMENT" "missing alignment normalization policy"
guard_expect_in_file "$TAG" 'is_power_of_two' "$ALIGNMENT" "missing power-of-two validation policy"
guard_expect_in_file "$TAG" 'padded_request_size' "$ALIGNMENT" "missing padded-size policy"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.size_class_box as SizeClassBox' "$ALIGNMENT" "alignment policy must consume the existing size-class owner"
guard_expect_in_file "$TAG" 'memory.alignment_policy_box = "memory/alignment_policy_box.hako"' "$MODULE" "alignment policy must be exported"
guard_expect_in_file "$TAG" 'HakoAllocAlignmentPolicy' "$ROOT_README" "root README must document the M177 alignment owner"
guard_expect_in_file "$TAG" 'alignment_policy_box.hako' "$MEMORY_README" "memory README must document the M177 alignment module"
guard_expect_in_file "$TAG" 'M177 alignment policy object' "$PLAN" "plan must retain the M177 row"
guard_expect_in_file "$TAG" '293x-187 M177 Alignment Policy Object' "$CARD" "missing M177 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M177 guard"

if rg -n 'init[[:space:]]*\{' "$ALIGNMENT" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M177 alignment owner must use direct declarations, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n 'page_map|releasePtr|unregister|releaseLocal|aligned_alloc|memcpy|copy_bytes|huge|secure|provider|hook|hako_mem_|externcall' \
  "$ALIGNMENT" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M177 leaked out of alignment-policy-only scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-alignment-policy|HakoAllocAlignmentPolicy|alignment_policy_box' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: alignment policy matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

tmp_dir="$(mktemp -d /tmp/hakorune_m177_alignment.XXXXXX)"
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

rg -F -q 'mimalloc-alignment-policy-proof' "$out"
rg -F -q 'norm=-1,8,8,8,16,64,-1,-1' "$out"
rg -F -q 'padded=-1,-1,31,31,39,111' "$out"
rg -F -q 'good=32,40,112,-1' "$out"
rg -F -q 'accepts=1,1,0,0' "$out"
rg -F -q 'checks=4' "$out"
rg -F -q 'summary=ok' "$out"

cat "$out"
echo "[$TAG] ok"
