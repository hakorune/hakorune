#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-page-map-aligned-huge-osvm-facade-integration"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

OWNER="lang/src/hako_alloc/memory/page_map_aligned_huge_osvm_facade_integration_box.hako"
SMALL="lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako"
HUGE_MODEL="lang/src/hako_alloc/memory/huge_page_model_box.hako"
HUGE_RELEASE="lang/src/hako_alloc/memory/huge_release_seam_box.hako"
OSVM_HEAP="lang/src/hako_alloc/memory/osvm_backed_fast_path_heap_box.hako"
PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
PAGE_RELEASER="lang/src/hako_alloc/memory/page_map_release_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APP="apps/hako-alloc-page-map-aligned-huge-osvm-facade-integration-proof/main.hako"
APP_TEST="apps/hako-alloc-page-map-aligned-huge-osvm-facade-integration-proof/test.sh"
APP_README="apps/hako-alloc-page-map-aligned-huge-osvm-facade-integration-proof/README.md"
CARD="docs/development/current/main/phases/phase-296x/296x-644-HAKO-MIMALLOC-PAGE-MAP-ALIGNED-HUGE-OSVM-FACADE-INTEGRATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
WORKSTREAM="docs/development/current/main/workstreams/mimalloc-current.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_INVENTORY="tools/checks/manifests/proof_apps/hako_alloc_inventory.toml"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_page_map_aligned_huge_osvm_facade_integration_guard.sh"

echo "[$TAG] checking page-map aligned huge osvm facade integration"

guard_require_command "$TAG" rg
guard_require_files \
  "$TAG" \
  "$OWNER" \
  "$SMALL" \
  "$HUGE_MODEL" \
  "$HUGE_RELEASE" \
  "$OSVM_HEAP" \
  "$PAGE_MAP" \
  "$PAGE_RELEASER" \
  "$MODULE" \
  "$ROOT_README" \
  "$MEMORY_README" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$TASKBOARD" \
  "$WORKSTREAM" \
  "$INDEX" \
  "$PROOF_INVENTORY" \
  "$0"

guard_expect_in_file "$TAG" 'Status: Active' "$CARD" "integration card must be active"
guard_expect_in_file "$TAG" 'page_map_aligned_huge_osvm_facade_integration' "$TASKBOARD" "taskboard must mention the integration seam"
guard_expect_in_file "$TAG" 'page_map_aligned_huge_osvm_facade_integration' "$WORKSTREAM" "workstream must mention the integration seam"
guard_expect_in_file "$TAG" 'memory.page_map_aligned_huge_osvm_facade_integration_box = "memory/page_map_aligned_huge_osvm_facade_integration_box.hako"' "$MODULE" "hako_alloc module must export the integration owner"
guard_expect_in_file "$TAG" 'box HakoAllocPageMapAlignedHugeOsVmFacadeIntegration' "$OWNER" "integration owner must exist"
guard_expect_in_file "$TAG" 'probeAlignedHugeOsVm' "$OWNER" "integration owner must expose probeAlignedHugeOsVm"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_map_aligned_small_path_box as HakoAllocPageMapAlignedSmallPathBox' "$OWNER" "integration owner must import aligned small path owner"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.huge_page_model_box as HakoAllocHugePageModelBox' "$OWNER" "integration owner must import huge page model owner"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.osvm_backed_fast_path_heap_box as HakoAllocOsVmBackedFastPathHeapBox' "$OWNER" "integration owner must import OSVM heap owner"
guard_expect_in_file "$TAG" 'HakoAllocPageMapAlignedHugeOsVmFacadeIntegration' "$ROOT_README" "root README must document the integration owner"
guard_expect_in_file "$TAG" 'page_map_aligned_huge_osvm_facade_integration_box.hako' "$MEMORY_README" "memory README must document the integration module"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_map_aligned_huge_osvm_facade_integration_box as HakoAllocPageMapAlignedHugeOsVmFacadeIntegration' "$APP" "proof app must import integration owner"
guard_expect_in_file "$TAG" 'check "page-map aligned huge osvm facade integration"' "$APP" "proof app must use labelled check block"
guard_expect_in_file "$TAG" 'tools/checks/k2_wide_hako_alloc_page_map_aligned_huge_osvm_facade_integration_guard.sh' "$INDEX" "check script index must list the integration guard"
guard_expect_in_file "$TAG" 'id = "M217"' "$PROOF_INVENTORY" "proof app inventory must list the integration proof app"

if rg -n 'provider[A-Za-z0-9_]*[[:space:]]*\(|install_hook[[:space:]]*\(|global_allocator[[:space:]]*\(|winner_claim[[:space:]]*\(|replace_process_allocator|LD_PRELOAD' \
  "$OWNER" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: integration seam leaked provider/replacement/winner behavior" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'hako-page-map-aligned-huge-osvm-facade-integration|HakoAllocPageMapAlignedHugeOsVmFacadeIntegration|page_map_aligned_huge_osvm_facade_integration' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: integration app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

ARTIFACT_DIR="$ROOT_DIR/target/checks/$TAG"
OUT="$ARTIFACT_DIR/out"

mkdir -p "$ARTIFACT_DIR"
rm -f "$OUT"

set +e
NYASH_DISABLE_PLUGINS=1 cargo run -q --bin hakorune -- --backend vm "$APP" |& tee "$OUT"
rc=${PIPESTATUS[0]}
set -e

if [[ "$rc" -eq 0 ]]; then
  echo "[$TAG] ERROR: expected runtime fail-fast but cargo run succeeded" >&2
  exit 1
fi

rg -F -q '[ERROR] ❌ [vm] VM error: Invalid instruction: extern function: Unknown: hako_osvm_reserve_bytes_i64' "$OUT"

cat "$OUT"
echo "[$TAG] ok"
