#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-activation-explicit-input-contract"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

DESIGN="docs/development/current/main/design/hako-alloc-provider-activation-explicit-input-contract-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
UNSUPPORTED_CLOSEOUT_GUARD="tools/checks/k2_wide_hako_alloc_provider_activation_unsupported_outcome_closeout_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_activation_explicit_input_contract_guard.sh"

CARD_372A="docs/development/current/main/phases/phase-293x/293x-988-MIMAP-372A-PROVIDER-ACTIVATION-UNSUPPORTED-OUTCOME-CLOSEOUT.md"
CARD_373A="docs/development/current/main/phases/phase-293x/293x-989-MIMAP-373A-POST-PROVIDER-ACTIVATION-UNSUPPORTED-OUTCOME-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-990-MIMAP-374A-PROVIDER-ACTIVATION-EXPLICIT-INPUT-CONTRACT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-991-MIMAP-375A-POST-PROVIDER-ACTIVATION-EXPLICIT-INPUT-CONTRACT-ROW-SELECTION.md"

printf '[%s] checking MIMAP-374A provider activation explicit-input contract\n' "$TAG"

guard_require_files "$TAG" "$DESIGN" "$INDEX" "$UNSUPPORTED_CLOSEOUT_GUARD" "$SELF_SCRIPT" "$CARD_372A" "$CARD_373A" "$CARD" "$NEXT_CARD"
guard_require_exec_files "$TAG" "$UNSUPPORTED_CLOSEOUT_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-374A design must be accepted"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_372A" "MIMAP-372A unsupported outcome closeout must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_373A" "MIMAP-373A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-374A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-375A must be selected current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-374A guard"

guard_expect_in_file "$TAG" 'Activation inputs must be explicit and row-owned' "$DESIGN" "activation input must be explicit"
guard_expect_in_file "$TAG" 'selected provider candidate token' "$DESIGN" "input bundle must include selected provider token"
guard_expect_in_file "$TAG" 'unsupported-outcome closeout evidence' "$DESIGN" "input bundle must include unsupported closeout evidence"
guard_expect_in_file "$TAG" 'No hidden env, implicit discovery, or process-global activation config.' "$DESIGN" "hidden activation inputs must remain forbidden"
guard_expect_in_file "$TAG" 'No provider activation or provider calls.' "$DESIGN" "provider activation must remain closed"
guard_expect_fixed_in_file "$TAG" 'No hooks or `#[global_allocator]`.' "$DESIGN" "hooks/global allocator must remain closed"

bash "$UNSUPPORTED_CLOSEOUT_GUARD"

if rg -n 'providerActivate|callProvider|replace_process_allocator|install_hook|global_allocator|ProviderActivation|GlobalAllocator|HookInstaller|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: provider activation/replacement/hook/backend matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

printf '[%s] ok\n' "$TAG"
