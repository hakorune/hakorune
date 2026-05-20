#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-host-replacement-preflight-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-422A has no L3/L4 process replacement evidence" >&2
      exit 2
      ;;
  esac
fi

CARD_420A="docs/development/current/main/phases/phase-293x/293x-1042-MIMAP-420A-HOST-REPLACEMENT-EXPLICIT-PREFLIGHT-INVENTORY.md"
CARD_421A="docs/development/current/main/phases/phase-293x/293x-1043-MIMAP-421A-HOST-REPLACEMENT-BLOCKED-STATE-DIAGNOSTICS.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1044-MIMAP-422A-HOST-REPLACEMENT-PREFLIGHT-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1045-MIMAP-423A-HOOK-INSTALL-PREFLIGHT-PLAN.md"
DESIGN="docs/development/current/main/design/hako-alloc-host-replacement-preflight-closeout-ssot.md"
DESIGN_420A="docs/development/current/main/design/hako-alloc-host-replacement-explicit-preflight-inventory-ssot.md"
DESIGN_421A="docs/development/current/main/design/hako-alloc-host-replacement-blocked-state-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
OWNER_420A="lang/src/hako_alloc/memory/host_replacement_explicit_preflight_inventory_box.hako"
OWNER_421A="lang/src/hako_alloc/memory/host_replacement_blocked_state_diagnostic_box.hako"
GUARD_420A="tools/checks/k2_wide_hako_alloc_host_replacement_explicit_preflight_inventory_guard.sh"
GUARD_421A="tools/checks/k2_wide_hako_alloc_host_replacement_blocked_state_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_host_replacement_preflight_closeout_guard.sh"

printf '[%s] checking MIMAP-422A host replacement preflight closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_420A" "$CARD_421A" "$CARD" "$NEXT_CARD" "$DESIGN" "$DESIGN_420A" "$DESIGN_421A" "$INDEX" "$OWNER_420A" "$OWNER_421A" "$GUARD_420A" "$GUARD_421A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_420A" "$GUARD_421A" "$SELF_SCRIPT"

for card in "$CARD_420A" "$CARD_421A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-423A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-422A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_420A" "MIMAP-420A design must remain accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_421A" "MIMAP-421A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-422A guard"
guard_expect_in_file "$TAG" 'host_replacement_executed: 0' "$OWNER_420A" "MIMAP-420A must keep host replacement execution closed"
guard_expect_in_file "$TAG" 'host_replacement_executed: 0' "$OWNER_421A" "MIMAP-421A must keep host replacement execution closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER_420A" "MIMAP-420A must keep hook install closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER_421A" "MIMAP-421A must keep hook install closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER_420A" "MIMAP-420A must keep backend matcher additions closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER_421A" "MIMAP-421A must keep backend matcher additions closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_420A" "MIMAP-420A must keep global allocator install closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_421A" "MIMAP-421A must keep global allocator install closed"

if rg -n 'replace_process_allocator|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER_420A" "$OWNER_421A" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: host replacement preflight owners must keep execution seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'HostReplacementExplicitPreflightInventory|HostReplacementBlockedStateDiagnostic|host-replacement-explicit-preflight-inventory-proof|host-replacement-blocked-state-diagnostics-proof|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: host replacement preflight owner/app matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

bash "$GUARD_420A" --level L2
bash "$GUARD_421A" --level L2

printf '[%s] ok\n' "$TAG"
