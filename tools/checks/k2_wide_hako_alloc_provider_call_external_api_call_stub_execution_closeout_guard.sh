#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-call-external-api-call-stub-execution-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

DESIGN="docs/development/current/main/design/hako-alloc-provider-call-external-api-call-stub-execution-closeout-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
PILOT_GUARD="tools/checks/k2_wide_hako_alloc_provider_call_external_api_call_stub_execution_pilot_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_call_external_api_call_stub_execution_closeout_guard.sh"

CARD_406A="docs/development/current/main/phases/phase-293x/293x-1028-MIMAP-406A-PROVIDER-CALL-EXTERNAL-API-CALL-STUB-EXECUTION-PILOT.md"
CARD_407A="docs/development/current/main/phases/phase-293x/293x-1029-MIMAP-407A-POST-PROVIDER-CALL-EXTERNAL-API-CALL-STUB-EXECUTION-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1030-MIMAP-408A-EXTERNAL-PROVIDER-API-CALL-STUB-EXECUTION-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1031-MIMAP-409A-POST-EXTERNAL-PROVIDER-API-CALL-STUB-EXECUTION-CLOSEOUT-ROW-SELECTION.md"

printf '[%s] checking MIMAP-408A external provider API call stub execution closeout\n' "$TAG"

guard_require_files "$TAG" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$PILOT_GUARD" "$SELF_SCRIPT" "$CARD_406A" "$CARD_407A" "$CARD" "$NEXT_CARD"
guard_require_exec_files "$TAG" "$PILOT_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-408A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-408A guard"

for card in "$CARD_406A" "$CARD_407A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-409A must be selected current"

guard_expect_in_file "$TAG" 'id = "MIMAP-406A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-406A"
guard_expect_in_file "$TAG" 'closeout_pack = "provider-call-external-api-call-stub-execution"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-406A must be assigned to external API call stub execution closeout pack"

bash "$PILOT_GUARD" --level L2

if rg -n 'provider-call-external-api-call-stub-execution-pilot-proof|ProviderCallExternalApiCallStubExecutionPilot|providerCallExternalApiCallStubExecution|callProvider|replace_process_allocator|install_hook|global_allocator|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: external provider API call stub execution or replacement/hook/backend matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

printf '[%s] ok\n' "$TAG"
