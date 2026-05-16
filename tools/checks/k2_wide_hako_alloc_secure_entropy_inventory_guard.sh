#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-secure-entropy-inventory"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

OWNER="lang/src/hako_alloc/memory/secure_entropy_inventory_box.hako"
POLICY="lang/src/hako_alloc/memory/secure_free_list_policy_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APP="apps/hako-alloc-secure-entropy-inventory-proof/main.hako"
APP_TEST="apps/hako-alloc-secure-entropy-inventory-proof/test.sh"
APP_README="apps/hako-alloc-secure-entropy-inventory-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-530-MIMAP-049A-SECURE-ENTROPY-SOURCE-INVENTORY.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-531-MIMAP-049B-POST-SECURE-ENTROPY-INVENTORY-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/mimalloc-secure-entropy-inventory-ssot.md"
CAPABILITY="docs/development/current/main/design/mimalloc-hakorune-capability-surface-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_secure_entropy_inventory_guard.sh"

echo "[$TAG] checking MIMAP-049A secure entropy inventory"

guard_require_command "$TAG" rg
guard_require_files \
  "$TAG" \
  "$OWNER" \
  "$POLICY" \
  "$MODULE" \
  "$ROOT_README" \
  "$MEMORY_README" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$NEXT_CARD" \
  "$DESIGN" \
  "$CAPABILITY" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$0"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-049A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-049A design must be accepted"
guard_expect_in_file "$TAG" 'uses random' "$CAPABILITY" "capability surface must keep random as a named capability"
guard_expect_in_file "$TAG" 'memory.secure_entropy_inventory_box = "memory/secure_entropy_inventory_box.hako"' "$MODULE" "hako_alloc module must export MIMAP-049A owner"
guard_expect_in_file "$TAG" 'box HakoAllocSecureEntropyDecision' "$OWNER" "decision box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocSecureEntropyInventory' "$OWNER" "inventory owner must exist"
guard_expect_in_file "$TAG" 'classifySource' "$OWNER" "inventory must expose classifySource"
guard_expect_in_file "$TAG" 'runtime_entropy_available: i64 = 0' "$OWNER" "runtime entropy must stay inactive"
guard_expect_in_file "$TAG" 'would_call_random: i64 = 0' "$OWNER" "random calls must stay inactive"
guard_expect_in_file "$TAG" 'would_call_os_entropy: i64 = 0' "$OWNER" "OS entropy calls must stay inactive"
guard_expect_in_file "$TAG" 'would_change_secure_list_policy: i64 = 0' "$OWNER" "secure-list behavior changes must stay inactive"
guard_expect_in_file "$TAG" 'HakoAllocSecureEntropyInventory' "$ROOT_README" "root README must document MIMAP-049A owner"
guard_expect_in_file "$TAG" 'secure_entropy_inventory_box.hako' "$MEMORY_README" "memory README must document MIMAP-049A module"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.secure_entropy_inventory_box as HakoAllocSecureEntropyInventory' "$APP" "proof app must import MIMAP-049A owner"
guard_expect_in_file "$TAG" 'check "mimap049a secure entropy inventory"' "$APP" "proof app must use labelled check block"
guard_expect_in_file "$TAG" 'id = "MIMAP-049A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-049A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-049A guard"
guard_expect_in_file "$TAG" 'MIMAP-049B' "$NEXT_CARD" "follow-up planning card must exist"

if rg -n 'externcall|random_source|entropy_source|hako_random|hako_entropy|/dev/urandom|OsVmCoreBox|AtomicCoreBox|TlsCore|provider[A-Za-z0-9_]*[[:space:]]*\(|install_hook|global_allocator|hako_mem_|unreserve|release_bytes|encodeNext|decodeNext|secure_free_list_policy_box' \
  "$OWNER" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-049A leaked beyond read-only entropy inventory scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'hako-alloc-secure-entropy-inventory|HakoAllocSecureEntropyInventory|secure_entropy_inventory' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: MIMAP-049A app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

tmp_dir="$(mktemp -d /tmp/hakorune_mimap049a_secure_entropy.XXXXXX)"
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

rg -F -q 'hako-alloc-secure-entropy-inventory-proof' "$out"
rg -F -q 'proof=1,0,37,1' "$out"
rg -F -q 'inactive=0,0,0,0,0' "$out"
rg -F -q 'rejects=1,2,3,4' "$out"
rg -F -q 'counts=5,1,4,1,1,1,1,1,4' "$out"
rg -F -q 'summary=ok' "$out"

cat "$out"
echo "[$TAG] ok"
