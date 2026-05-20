#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-call-external-api-adapter-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

DESIGN="docs/development/current/main/design/hako-alloc-provider-call-external-api-adapter-closeout-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
INVENTORY_GUARD="tools/checks/k2_wide_hako_alloc_provider_call_external_api_adapter_inventory_guard.sh"
PREFLIGHT_GUARD="tools/checks/k2_wide_hako_alloc_provider_call_external_api_adapter_preflight_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_call_external_api_adapter_closeout_guard.sh"

CARD_400A="docs/development/current/main/phases/phase-293x/293x-1022-MIMAP-400A-PROVIDER-CALL-EXTERNAL-API-ADAPTER-INVENTORY.md"
CARD_402A="docs/development/current/main/phases/phase-293x/293x-1024-MIMAP-402A-PROVIDER-CALL-EXTERNAL-API-ADAPTER-PREFLIGHT.md"
CARD_403A="docs/development/current/main/phases/phase-293x/293x-1025-MIMAP-403A-POST-PROVIDER-CALL-EXTERNAL-API-ADAPTER-PREFLIGHT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1026-MIMAP-404A-PROVIDER-CALL-EXTERNAL-API-ADAPTER-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1027-MIMAP-405A-POST-PROVIDER-CALL-EXTERNAL-API-ADAPTER-CLOSEOUT-ROW-SELECTION.md"

printf '[%s] checking MIMAP-404A provider-call external API adapter closeout\n' "$TAG"

guard_require_files "$TAG" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$INVENTORY_GUARD" "$PREFLIGHT_GUARD" "$SELF_SCRIPT" "$CARD_400A" "$CARD_402A" "$CARD_403A" "$CARD" "$NEXT_CARD"
guard_require_exec_files "$TAG" "$INVENTORY_GUARD" "$PREFLIGHT_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-404A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-404A guard"

for card in "$CARD_400A" "$CARD_402A" "$CARD_403A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-405A must be selected current"

guard_expect_in_file "$TAG" 'id = "MIMAP-400A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-400A"
guard_expect_in_file "$TAG" 'id = "MIMAP-402A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-402A"
guard_expect_in_file "$TAG" 'closeout_pack = "provider-call-external-api-adapter"' "$PROOF_MANIFEST_INCLUDE" "adapter rows must be assigned to external API adapter closeout pack"

bash "$INVENTORY_GUARD" --level L2
bash "$PREFLIGHT_GUARD" --level L2

if rg -n 'provider-call-external-api-adapter-(inventory|preflight)-proof|ProviderCallExternalApiAdapter(Inventory|Preflight)|providerCallExternalApiAdapter|callProvider|replace_process_allocator|install_hook|global_allocator|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: external provider adapter or replacement/hook/backend matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

printf '[%s] ok\n' "$TAG"
