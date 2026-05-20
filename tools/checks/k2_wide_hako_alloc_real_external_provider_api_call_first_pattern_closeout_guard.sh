#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-real-external-provider-api-call-first-pattern-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

DESIGN="docs/development/current/main/design/hako-alloc-real-external-provider-api-call-first-pattern-closeout-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
PILOT_GUARD="tools/checks/k2_wide_hako_alloc_real_external_provider_api_call_first_pattern_pilot_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_real_external_provider_api_call_first_pattern_closeout_guard.sh"

CARD_415A="docs/development/current/main/phases/phase-293x/293x-1037-MIMAP-415A-REAL-EXTERNAL-PROVIDER-API-CALL-FIRST-PATTERN-PILOT.md"
CARD_416A="docs/development/current/main/phases/phase-293x/293x-1038-MIMAP-416A-POST-REAL-EXTERNAL-PROVIDER-API-CALL-FIRST-PATTERN-PILOT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1039-MIMAP-417A-REAL-EXTERNAL-PROVIDER-API-CALL-FIRST-PATTERN-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1040-MIMAP-418A-POST-REAL-EXTERNAL-PROVIDER-API-CALL-FIRST-PATTERN-CLOSEOUT-ROW-SELECTION.md"

printf '[%s] checking MIMAP-417A real external provider API call first-pattern closeout\n' "$TAG"

guard_require_files "$TAG" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$PILOT_GUARD" "$SELF_SCRIPT" "$CARD_415A" "$CARD_416A" "$CARD" "$NEXT_CARD"
guard_require_exec_files "$TAG" "$PILOT_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-417A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-417A guard"

for card in "$CARD_415A" "$CARD_416A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-418A must be selected current"

guard_expect_in_file "$TAG" 'id = "MIMAP-415A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-415A"
guard_expect_in_file "$TAG" 'closeout_pack = "real-external-provider-api-call"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-415A must be assigned to real external provider API call closeout pack"
guard_expect_in_file "$TAG" 'exe = "required"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-415A must retain L3 first-pattern evidence"

bash "$PILOT_GUARD" --level L3

if rg -n 'real-external-provider-api-call-first-pattern-pilot-proof|RealExternalProviderApiCallFirstPatternPilot|realExternalProviderApiCallFirstPatternPilot|replace_process_allocator|install_hook|global_allocator|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: real external provider API call pilot or replacement/hook/backend matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

printf '[%s] ok\n' "$TAG"
