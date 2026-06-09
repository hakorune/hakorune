#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-secure-entropy-backed-free-list"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

OWNER="lang/src/hako_alloc/memory/secure_entropy_backed_free_list_box.hako"
INVENTORY="lang/src/hako_alloc/memory/secure_entropy_inventory_box.hako"
POLICY="lang/src/hako_alloc/memory/secure_free_list_policy_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APP="apps/hako-alloc-secure-entropy-backed-free-list-proof/main.hako"
APP_TEST="apps/hako-alloc-secure-entropy-backed-free-list-proof/test.sh"
APP_README="apps/hako-alloc-secure-entropy-backed-free-list-proof/README.md"
CARD="docs/development/current/main/phases/phase-296x/296x-643-HAKO-MIMALLOC-SECURE-ENTROPY-BACKED-FREE-LIST-INTEGRATION.md"
PLAN="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
WORKSTREAM="docs/development/current/main/workstreams/mimalloc-current.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/manifests/proof_apps/hako_alloc_inventory.toml"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_secure_entropy_backed_free_list_guard.sh"

echo "[$TAG] checking secure entropy backed free-list selector"

guard_require_command "$TAG" rg
guard_require_files \
  "$TAG" \
  "$OWNER" \
  "$INVENTORY" \
  "$POLICY" \
  "$MODULE" \
  "$ROOT_README" \
  "$MEMORY_README" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$PLAN" \
  "$WORKSTREAM" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$0"

guard_expect_in_file "$TAG" 'Status: Active' "$CARD" "integration card must be active"
guard_expect_in_file "$TAG" 'selected_feature=secure_entropy_backed_free_list' "$CARD" "card must select secure entropy backed free list"
guard_expect_in_file "$TAG" 'secure_entropy_backed_free_list' "$PLAN" "taskboard must mention secure entropy backed free list"
guard_expect_in_file "$TAG" 'secure_entropy_backed_free_list' "$WORKSTREAM" "workstream must mention secure entropy backed free list"
guard_expect_in_file "$TAG" 'memory.secure_entropy_backed_free_list_box = "memory/secure_entropy_backed_free_list_box.hako"' "$MODULE" "hako_alloc module must export the selector owner"
guard_expect_in_file "$TAG" 'box HakoAllocSecureEntropyBackedFreeListDecision' "$OWNER" "decision box must exist"
guard_expect_in_file "$TAG" 'classify\(' "$OWNER" "selector must expose classify"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.secure_entropy_inventory_box as HakoAllocSecureEntropyInventory' "$OWNER" "selector must import entropy inventory"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.secure_free_list_policy_box as HakoAllocSecureFreeListPolicy' "$OWNER" "selector must import secure free-list policy"
guard_expect_in_file "$TAG" 'HakoAllocSecureEntropyBackedFreeList' "$ROOT_README" "root README must document the selector owner"
guard_expect_in_file "$TAG" 'secure_entropy_backed_free_list_box.hako' "$MEMORY_README" "memory README must document the selector module"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.secure_entropy_backed_free_list_box as HakoAllocSecureEntropyBackedFreeList' "$APP" "proof app must import selector owner"
guard_expect_in_file "$TAG" 'check "secure entropy backed free list"' "$APP" "proof app must use labelled check block"
guard_expect_in_file "$TAG" 'id = "M216"' "$PROOF_MANIFEST" "proof app manifest must list M216"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list the selector guard"

if rg -n 'random_source|entropy_source|hako_random|hako_entropy|/dev/urandom|OsVmCoreBox|AtomicCoreBox|TlsCore|install_hook|global_allocator|hako_mem_|unreserve|release_bytes|decommit|aligned_alloc|HugeRelease|huge_release|page_source|winner' \
  "$OWNER" "$INVENTORY" "$POLICY" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: secure entropy backed free-list selector leaked beyond read-only scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'hako-alloc-secure-entropy-backed-free-list|HakoAllocSecureEntropyBackedFreeList|secure_entropy_backed_free_list' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: selector app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

tmp_dir="$(mktemp -d /tmp/hakorune_secure_entropy_backed_free_list.XXXXXX)"
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

rg -F -q 'hako-alloc-secure-entropy-backed-free-list-proof' "$out"
rg -F -q 'proof=1,0,1,1,262181,3' "$out"
rg -F -q 'rejects=1,2,3,4,5,5' "$out"
rg -F -q 'inactive=0,0,0' "$out"
rg -F -q 'summary=ok' "$out"

cat "$out"
echo "[$TAG] ok"
