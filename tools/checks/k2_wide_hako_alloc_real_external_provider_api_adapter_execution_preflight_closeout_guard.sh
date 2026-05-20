#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-real-external-provider-api-adapter-execution-preflight-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

DESIGN="docs/development/current/main/design/hako-alloc-real-external-provider-api-adapter-execution-preflight-closeout-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
PREFLIGHT_GUARD="tools/checks/k2_wide_hako_alloc_real_external_provider_api_adapter_execution_preflight_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_real_external_provider_api_adapter_execution_preflight_closeout_guard.sh"

CARD_410A="docs/development/current/main/phases/phase-293x/293x-1032-MIMAP-410A-REAL-EXTERNAL-PROVIDER-API-ADAPTER-EXECUTION-PREFLIGHT.md"
CARD_411A="docs/development/current/main/phases/phase-293x/293x-1033-MIMAP-411A-POST-REAL-EXTERNAL-PROVIDER-API-ADAPTER-EXECUTION-PREFLIGHT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1034-MIMAP-412A-REAL-EXTERNAL-PROVIDER-API-ADAPTER-EXECUTION-PREFLIGHT-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1035-MIMAP-413A-POST-REAL-EXTERNAL-PROVIDER-API-ADAPTER-EXECUTION-PREFLIGHT-CLOSEOUT-ROW-SELECTION.md"

printf '[%s] checking MIMAP-412A real external provider API adapter execution preflight closeout\n' "$TAG"

guard_require_files "$TAG" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$PREFLIGHT_GUARD" "$SELF_SCRIPT" "$CARD_410A" "$CARD_411A" "$CARD" "$NEXT_CARD"
guard_require_exec_files "$TAG" "$PREFLIGHT_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-412A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-412A guard"

for card in "$CARD_410A" "$CARD_411A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-413A must be selected current"

guard_expect_in_file "$TAG" 'id = "MIMAP-410A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-410A"
guard_expect_in_file "$TAG" 'closeout_pack = "real-external-provider-api-adapter-execution"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-410A must be assigned to the real external provider API adapter execution closeout pack"

bash "$PREFLIGHT_GUARD" --level L2

if rg -n 'real-external-provider-api-adapter-execution-preflight-proof|RealExternalProviderApiAdapterExecutionPreflight|realExternalProviderApiAdapterExecutionPreflight|callProvider|replace_process_allocator|install_hook|global_allocator|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: real external provider API preflight or replacement/hook/backend matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

printf '[%s] ok\n' "$TAG"
