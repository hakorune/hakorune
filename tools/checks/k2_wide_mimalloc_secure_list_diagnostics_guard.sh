#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-secure-list-diagnostics"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

DIAG="lang/src/hako_alloc/memory/secure_free_list_diagnostics_box.hako"
PAGE_MODEL="lang/src/hako_alloc/memory/page_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APP="apps/mimalloc-secure-list-diagnostics-proof/main.hako"
APP_TEST="apps/mimalloc-secure-list-diagnostics-proof/test.sh"
APP_README="apps/mimalloc-secure-list-diagnostics-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-194-M183-SECURE-LIST-DIAGNOSTICS.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_secure_list_diagnostics_guard.sh"

echo "[$TAG] checking M183 secure-list diagnostics"

guard_require_files \
  "$TAG" \
  "$DIAG" \
  "$PAGE_MODEL" \
  "$MODULE" \
  "$ROOT_README" \
  "$MEMORY_README" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$PLAN" \
  "$INDEX"

guard_expect_in_file "$TAG" 'memory.secure_free_list_diagnostics_box = "memory/secure_free_list_diagnostics_box.hako"' "$MODULE" "hako module must export the M183 secure-list diagnostics owner"
guard_expect_in_file "$TAG" 'box HakoAllocSecureFreeListDiagnostics' "$DIAG" "missing M183 secure-list diagnostics owner"
guard_expect_in_file "$TAG" 'scanPage\(page\)' "$DIAG" "M183 must expose a page diagnostics entry"
guard_expect_in_file "$TAG" 'observeBlock\(page, block_id\)' "$DIAG" "M183 must observe page-local block ids"
guard_expect_in_file "$TAG" 'page\.blockIsLive\(block_id\)' "$DIAG" "M183 must detect live blocks appearing in free lists"
guard_expect_in_file "$TAG" 'out_of_range_free_block_count' "$DIAG" "M183 must count out-of-range free-list entries"
guard_expect_in_file "$TAG" 'duplicate_free_block_count' "$DIAG" "M183 must count duplicate free-list entries"
guard_expect_in_file "$TAG" 'live_block_in_free_list_count' "$DIAG" "M183 must count live blocks in free-list entries"
guard_expect_in_file "$TAG" 'free_count_mismatch_count' "$DIAG" "M183 must count free/local/used total mismatches"
guard_expect_in_file "$TAG" 'local_free_count_mismatch_count' "$DIAG" "M183 must count malformed local-free tops"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.secure_free_list_diagnostics_box as HakoAllocSecureFreeListDiagnosticsBox' "$APP" "proof app must import the M183 owner"
guard_expect_in_file "$TAG" 'HakoAllocSecureFreeListDiagnostics' "$ROOT_README" "root README must document the M183 owner"
guard_expect_in_file "$TAG" 'secure_free_list_diagnostics_box.hako' "$MEMORY_README" "memory README must document the M183 module"
guard_expect_in_file "$TAG" 'M183 secure-list diagnostics-only' "$PLAN" "plan must retain the M183 row"
guard_expect_in_file "$TAG" '293x-194 M183 Secure-List Diagnostics' "$CARD" "missing M183 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M183 guard"

if rg -n 'init[[:space:]]*\{' "$DIAG" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M183 diagnostics must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n 'encodeNext|decodeNext|validateDecodedIndex|cookie_source|random_source|xor|rotate|hash|provider|hook|hako_mem_|externcall|OSVM|OsVm|unreserve|release_bytes|decommit|aligned_alloc|HugeRelease|huge_release' \
  "$DIAG" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M183 leaked beyond diagnostics-only scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-secure-list-diagnostics|HakoAllocSecureFreeListDiagnostics|secure_free_list_diagnostics' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M183 app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

tmp_dir="$(mktemp -d /tmp/hakorune_m183_secure_list_diagnostics.XXXXXX)"
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

rg -F -q 'mimalloc-secure-list-diagnostics-proof' "$out"
rg -F -q 'good=1,1' "$out"
rg -F -q 'bad=0,0,1,1,1,1,0' "$out"
rg -F -q 'local_bad=0,0,0,1,0,1,1' "$out"
rg -F -q 'totals=3,1,2,1,2,1,2,1' "$out"
rg -F -q 'summary=ok' "$out"

cat "$out"
echo "[$TAG] ok"
