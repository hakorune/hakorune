#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-activation-unsupported-outcome-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

DESIGN="docs/development/current/main/design/hako-alloc-provider-activation-unsupported-outcome-closeout-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
LEDGER_GUARD="tools/checks/k2_wide_hako_alloc_provider_activation_unsupported_outcome_ledger_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_activation_unsupported_outcome_closeout_guard.sh"

CARD_368A="docs/development/current/main/phases/phase-293x/293x-984-MIMAP-368A-PROVIDER-ACTIVATION-FIRST-PATTERN-PLAN.md"
CARD_370A="docs/development/current/main/phases/phase-293x/293x-986-MIMAP-370A-PROVIDER-ACTIVATION-UNSUPPORTED-OUTCOME-LEDGER.md"
CARD_371A="docs/development/current/main/phases/phase-293x/293x-987-MIMAP-371A-POST-PROVIDER-ACTIVATION-UNSUPPORTED-OUTCOME-LEDGER-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-988-MIMAP-372A-PROVIDER-ACTIVATION-UNSUPPORTED-OUTCOME-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-989-MIMAP-373A-POST-PROVIDER-ACTIVATION-UNSUPPORTED-OUTCOME-CLOSEOUT-ROW-SELECTION.md"

printf '[%s] checking MIMAP-372A provider activation unsupported outcome closeout\n' "$TAG"

guard_require_files "$TAG" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$LEDGER_GUARD" "$SELF_SCRIPT" "$CARD_368A" "$CARD_370A" "$CARD_371A" "$CARD" "$NEXT_CARD"
guard_require_exec_files "$TAG" "$LEDGER_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-372A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-372A guard"

for card in "$CARD_368A" "$CARD_370A" "$CARD_371A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-373A must be selected current"

guard_expect_in_file "$TAG" 'id = "MIMAP-370A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-370A"
guard_expect_in_file "$TAG" 'closeout_pack = "provider-activation-unsupported-outcome"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-370A must be assigned to unsupported outcome closeout pack"

bash "$LEDGER_GUARD" --level L2

if rg -n 'providerActivate|callProvider|replace_process_allocator|install_hook|global_allocator|ProviderActivation|GlobalAllocator|HookInstaller|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: provider activation/replacement/hook/backend matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

printf '[%s] ok\n' "$TAG"
