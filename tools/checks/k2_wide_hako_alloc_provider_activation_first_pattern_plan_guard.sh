#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-activation-first-pattern-plan"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

DESIGN="docs/development/current/main/design/hako-alloc-provider-activation-first-pattern-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
LADDER_CLOSEOUT_GUARD="tools/checks/k2_wide_hako_alloc_provider_facing_ladder_closeout_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_activation_first_pattern_plan_guard.sh"

CARD_366A="docs/development/current/main/phases/phase-293x/293x-982-MIMAP-366A-PROVIDER-FACING-LADDER-CLOSEOUT.md"
CARD_367A="docs/development/current/main/phases/phase-293x/293x-983-MIMAP-367A-POST-PROVIDER-FACING-LADDER-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-984-MIMAP-368A-PROVIDER-ACTIVATION-FIRST-PATTERN-PLAN.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-985-MIMAP-369A-POST-PROVIDER-ACTIVATION-FIRST-PATTERN-PLAN-ROW-SELECTION.md"

printf '[%s] checking MIMAP-368A provider activation first-pattern plan\n' "$TAG"

guard_require_files "$TAG" "$DESIGN" "$INDEX" "$LADDER_CLOSEOUT_GUARD" "$SELF_SCRIPT" "$CARD_366A" "$CARD_367A" "$CARD" "$NEXT_CARD"
guard_require_exec_files "$TAG" "$LADDER_CLOSEOUT_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-368A design must be accepted"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_366A" "MIMAP-366A provider-facing ladder closeout must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_367A" "MIMAP-367A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-368A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-369A must be selected current"

guard_expect_in_file "$TAG" 'provider activation unsupported outcome ledger' "$DESIGN" "unsupported activation outcome ledger must be the next behavior row"
guard_expect_in_file "$TAG" 'Actual provider activation requires a later explicit' "$DESIGN" "activation must remain behind a later explicit row"
guard_expect_in_file "$TAG" 'L3 exact-MIR evidence is required' "$DESIGN" "activation first-pattern must require L3 evidence"
guard_expect_in_file "$TAG" 'No provider activation.' "$DESIGN" "provider activation must remain closed"
guard_expect_in_file "$TAG" 'No host allocator replacement.' "$DESIGN" "host allocator replacement must remain closed"
guard_expect_fixed_in_file "$TAG" 'No hooks or `#[global_allocator]`.' "$DESIGN" "hooks/global allocator must remain closed"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-368A guard"

bash "$LADDER_CLOSEOUT_GUARD"

if rg -n 'providerActivate|replace_process_allocator|install_hook|global_allocator|ProviderActivation|GlobalAllocator|HookInstaller|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: provider activation/replacement/hook/backend matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

printf '[%s] ok\n' "$TAG"
