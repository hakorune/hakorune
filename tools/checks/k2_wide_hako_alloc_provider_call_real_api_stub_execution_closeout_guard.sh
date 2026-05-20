#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-call-real-api-stub-execution-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

DESIGN="docs/development/current/main/design/hako-alloc-provider-call-real-api-stub-execution-closeout-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
PILOT_GUARD="tools/checks/k2_wide_hako_alloc_provider_call_real_api_stub_execution_pilot_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_call_real_api_stub_execution_closeout_guard.sh"

CARD_396A="docs/development/current/main/phases/phase-293x/293x-1018-MIMAP-396A-PROVIDER-CALL-REAL-API-STUB-EXECUTION-PILOT.md"
CARD_397A="docs/development/current/main/phases/phase-293x/293x-1019-MIMAP-397A-POST-PROVIDER-CALL-REAL-API-STUB-EXECUTION-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1020-MIMAP-398A-PROVIDER-CALL-REAL-API-STUB-EXECUTION-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1021-MIMAP-399A-POST-PROVIDER-CALL-REAL-API-STUB-EXECUTION-CLOSEOUT-ROW-SELECTION.md"

printf '[%s] checking MIMAP-398A provider-call real API stub execution closeout\n' "$TAG"

guard_require_files "$TAG" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$PILOT_GUARD" "$SELF_SCRIPT" "$CARD_396A" "$CARD_397A" "$CARD" "$NEXT_CARD"
guard_require_exec_files "$TAG" "$PILOT_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-398A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-398A guard"

for card in "$CARD_396A" "$CARD_397A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-399A must be selected current"

guard_expect_in_file "$TAG" 'id = "MIMAP-396A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-396A"
guard_expect_in_file "$TAG" 'closeout_pack = "provider-call-real-api-stub-execution"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-396A must be assigned to stub execution closeout pack"

bash "$PILOT_GUARD" --level L2

if rg -n 'provider-call-real-api-stub-execution-pilot-proof|ProviderCallRealApiStubExecutionPilot|providerCallRealApiStubExecution|callProvider|replace_process_allocator|install_hook|global_allocator|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: provider-call stub execution or replacement/hook/backend matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

printf '[%s] ok\n' "$TAG"
