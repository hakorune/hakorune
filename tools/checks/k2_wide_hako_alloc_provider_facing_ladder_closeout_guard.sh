#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-facing-ladder-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

DESIGN="docs/development/current/main/design/hako-alloc-provider-facing-ladder-closeout-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
SELECTION_GUARD="tools/checks/k2_wide_hako_alloc_provider_selection_inventory_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_facing_ladder_closeout_guard.sh"

CARD_358A="docs/development/current/main/phases/phase-293x/293x-974-MIMAP-358A-PROVIDER-FACING-LADDER-CLOSED-PLAN.md"
CARD_360A="docs/development/current/main/phases/phase-293x/293x-976-MIMAP-360A-PROVIDER-BOUNDARY-DIAGNOSTIC-VOCABULARY.md"
CARD_362A="docs/development/current/main/phases/phase-293x/293x-978-MIMAP-362A-PROVIDER-READINESS-PREFLIGHT.md"
CARD_364A="docs/development/current/main/phases/phase-293x/293x-980-MIMAP-364A-PROVIDER-SELECTION-INVENTORY.md"
CARD_365A="docs/development/current/main/phases/phase-293x/293x-981-MIMAP-365A-POST-PROVIDER-SELECTION-INVENTORY-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-982-MIMAP-366A-PROVIDER-FACING-LADDER-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-983-MIMAP-367A-POST-PROVIDER-FACING-LADDER-CLOSEOUT-ROW-SELECTION.md"

printf '[%s] checking MIMAP-366A provider-facing ladder closeout\n' "$TAG"

guard_require_files "$TAG" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$SELECTION_GUARD" "$SELF_SCRIPT" "$CARD_358A" "$CARD_360A" "$CARD_362A" "$CARD_364A" "$CARD_365A" "$CARD" "$NEXT_CARD"
guard_require_exec_files "$TAG" "$SELECTION_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-366A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-366A guard"

for card in "$CARD_358A" "$CARD_360A" "$CARD_362A" "$CARD_364A" "$CARD_365A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-367A must be selected current"

for row in MIMAP-360A MIMAP-362A MIMAP-364A; do
  guard_expect_in_file "$TAG" "id = \"$row\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must list $row"
done

bash "$SELECTION_GUARD" --level L2

if rg -n 'providerActivate|replace_process_allocator|install_hook|global_allocator|ProviderActivation|GlobalAllocator|HookInstaller|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: provider-facing activation/replacement/hook/backend matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

printf '[%s] ok\n' "$TAG"
