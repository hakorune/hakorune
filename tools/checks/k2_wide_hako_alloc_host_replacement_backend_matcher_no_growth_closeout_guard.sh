#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-host-replacement-backend-matcher-no-growth-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

CARD_422A="docs/development/current/main/phases/phase-293x/293x-1044-MIMAP-422A-HOST-REPLACEMENT-PREFLIGHT-CLOSEOUT.md"
CARD_423A="docs/development/current/main/phases/phase-293x/293x-1045-MIMAP-423A-HOOK-INSTALL-PREFLIGHT-PLAN.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1046-MIMAP-424A-BACKEND-MATCHER-NO-GROWTH-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1047-MIMAP-425A-OPTIONAL-PROCESS-ALLOCATOR-REPLACEMENT-PROPOSAL.md"
DESIGN="docs/development/current/main/design/hako-alloc-host-replacement-backend-matcher-no-growth-closeout-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_host_replacement_backend_matcher_no_growth_closeout_guard.sh"

printf '[%s] checking MIMAP-424A host replacement backend matcher no-growth closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_422A" "$CARD_423A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

for card in "$CARD_422A" "$CARD_423A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-425A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-424A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-424A guard"
guard_expect_in_file "$TAG" 'No backend `.inc` matcher by app, box, owner, hook, replacement, or row name.' "$DESIGN" "design must define no-growth matcher boundary"

patterns=(
  'hako-alloc-host-replacement-explicit-preflight-inventory-proof'
  'HakoAllocHostReplacementExplicitPreflightInventory'
  'hako-alloc-host-replacement-blocked-state-diagnostics-proof'
  'HakoAllocHostReplacementBlockedStateDiagnostic'
  'host-replacement-preflight-closeout'
  'hook-install-preflight'
  'HookInstallPreflight'
  'process-allocator-replacement'
  'ProcessAllocatorReplacement'
  'HostReplacement'
)

for pattern in "${patterns[@]}"; do
  if rg -n -F "$pattern" lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
    echo "[$TAG] ERROR: host replacement/hook owner or row matcher leaked into .inc: $pattern" >&2
    cat /tmp/"$TAG".inc_leak >&2
    rm -f /tmp/"$TAG".inc_leak
    exit 1
  fi
done
rm -f /tmp/"$TAG".inc_leak

if rg -n 'providerActivate|replace_process_allocator|install_hook|global_allocator|owner-name matcher|by app name|by owner name|hook-name matcher|replacement-name matcher' lang/c-abi/shims >/tmp/"$TAG".backend_leak 2>&1; then
  echo "[$TAG] ERROR: provider/replacement/hook/backend matcher wording leaked into .inc" >&2
  cat /tmp/"$TAG".backend_leak >&2
  rm -f /tmp/"$TAG".backend_leak
  exit 1
fi
rm -f /tmp/"$TAG".backend_leak

printf '[%s] ok\n' "$TAG"
