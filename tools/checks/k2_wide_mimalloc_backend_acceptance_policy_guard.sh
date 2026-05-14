#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-backend-acceptance-policy"
cd "$ROOT_DIR"

source tools/checks/lib/guard_common.sh

POLICY="docs/development/current/main/design/mimalloc-backend-acceptance-policy-ssot.md"
LIMITS="docs/development/current/main/design/vm-known-limitations-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-344-MIMAP-BACKEND-ACCEPTANCE-POLICY.md"
COMMON="tools/checks/lib/guard_common.sh"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_backend_acceptance_policy_guard.sh"
MIMAP_VM_GUARDS=(
  tools/checks/k2_wide_mimalloc_page_free_list_pilot_guard.sh
  tools/checks/k2_wide_mimalloc_lifecycle_integration_pilot_guard.sh
  tools/checks/k2_wide_mimalloc_page_queue_lifecycle_selection_guard.sh
)

for path in "$POLICY" "$LIMITS" "$CARD" "$COMMON" "$INDEX" "${MIMAP_VM_GUARDS[@]}"; do
  [[ -f "$path" ]] || guard_fail "$TAG" "missing required file: $path"
done

guard_expect_in_file "$TAG" 'Decision: accepted' "$POLICY" "policy must be accepted"
guard_expect_in_file "$TAG" 'LLVM/EXE' "$POLICY" "policy must name LLVM/EXE primary acceptance"
guard_expect_in_file "$TAG" 'MIMAP-011+' "$POLICY" "policy must apply to MIMAP-011+"
guard_expect_in_file "$TAG" 'Timeout is never a silent pass' "$POLICY" "policy must reject silent timeout pass"
guard_expect_in_file "$TAG" 'VM-LIM-001 object-heavy page queue/facade route' "$LIMITS" "VM limitation must be recorded"
guard_expect_in_file "$TAG" 'Retire when:' "$LIMITS" "VM limitation must have retirement condition"
guard_expect_in_file "$TAG" 'guard_timeout_run' "$COMMON" "guard common must expose timeout helper"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list backend policy guard"

for guard in "${MIMAP_VM_GUARDS[@]}"; do
  guard_expect_in_file "$TAG" 'guard_timeout_run' "$guard" "MIMAP VM guard must use timeout helper: $guard"
  guard_expect_in_file "$TAG" 'MIMAP_VM_TIMEOUT' "$guard" "MIMAP VM guard must expose timeout override: $guard"
done

echo "[$TAG] ok"
