#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-secure-list-policy"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

POLICY="lang/src/hako_alloc/memory/secure_free_list_policy_box.hako"
DIAG="lang/src/hako_alloc/memory/secure_free_list_diagnostics_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APP="apps/mimalloc-secure-list-encode-decode-proof/main.hako"
APP_TEST="apps/mimalloc-secure-list-encode-decode-proof/test.sh"
APP_README="apps/mimalloc-secure-list-encode-decode-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-195-M184-SECURE-LIST-POLICY.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_secure_list_policy_guard.sh"

echo "[$TAG] checking M184 secure-list encoded-next policy"

guard_require_files \
  "$TAG" \
  "$POLICY" \
  "$DIAG" \
  "$MODULE" \
  "$ROOT_README" \
  "$MEMORY_README" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$PLAN" \
  "$INDEX"

guard_expect_in_file "$TAG" 'memory.secure_free_list_policy_box = "memory/secure_free_list_policy_box.hako"' "$MODULE" "hako module must export the M184 secure-list policy owner"
guard_expect_in_file "$TAG" 'box HakoAllocSecureFreeListPolicy' "$POLICY" "missing M184 secure-list policy owner"
guard_expect_in_file "$TAG" 'encodeNext\(next_block, cookie\)' "$POLICY" "M184 must expose encoded-next creation"
guard_expect_in_file "$TAG" 'decodeNext\(encoded_next, cookie\)' "$POLICY" "M184 must expose encoded-next decoding"
guard_expect_in_file "$TAG" 'validateDecodedIndex\(decoded_next, capacity\)' "$POLICY" "M184 must validate decoded next indices"
guard_expect_in_file "$TAG" 'encodeNextForCapacity\(next_block, cookie, capacity\)' "$POLICY" "M184 must expose capacity-checked encoding"
guard_expect_in_file "$TAG" 'decodeNextForCapacity\(encoded_next, cookie, capacity\)' "$POLICY" "M184 must expose capacity-checked decoding"
guard_expect_in_file "$TAG" 'return -2' "$POLICY" "M184 must keep invalid result distinct from end-of-list -1"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.secure_free_list_policy_box as HakoAllocSecureFreeListPolicy' "$APP" "proof app must import the M184 owner"
guard_expect_in_file "$TAG" 'HakoAllocSecureFreeListPolicy' "$ROOT_README" "root README must document the M184 owner"
guard_expect_in_file "$TAG" 'secure_free_list_policy_box.hako' "$MEMORY_README" "memory README must document the M184 module"
guard_expect_in_file "$TAG" 'M184 secure-list encode/decode small path' "$PLAN" "plan must retain the M184 row"
guard_expect_in_file "$TAG" '293x-195 M184 Secure-List Policy' "$CARD" "missing M184 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M184 guard"

if rg -n 'init[[:space:]]*\{' "$POLICY" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M184 policy must use Unified Members/static helpers, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n 'random_source|entropy_source|crypt|provider|hook|hako_mem_|externcall|OSVM|OsVm|unreserve|release_bytes|decommit|aligned_alloc|HugeRelease|huge_release|releaseLocal\(|page_map|register\(|unregister\(' \
  "$POLICY" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M184 leaked beyond standalone encoded-next policy scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-secure-list-encode-decode|HakoAllocSecureFreeListPolicy|secure_free_list_policy' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M184 app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

tmp_dir="$(mktemp -d /tmp/hakorune_m184_secure_list_policy.XXXXXX)"
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

rg -F -q 'mimalloc-secure-list-encode-decode-proof' "$out"
rg -F -q 'encoded=37,65573,262181' "$out"
rg -F -q 'decoded=-1,0,3' "$out"
rg -F -q 'reject=-2,-2,-2,-2' "$out"
rg -F -q 'capacity=-1,3,-2,-2' "$out"
rg -F -q 'valid=1,1,0,0' "$out"
rg -F -q 'summary=ok' "$out"

cat "$out"
echo "[$TAG] ok"
