#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-selected-provider-precondition"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-selected-provider-precondition-ssot.md"
POST_LADDER="docs/development/current/main/design/allocator-provider-post-m101-implementation-ladder-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-158-M102-ALLOCATOR-PROVIDER-SELECTED-PROVIDER-PRECONDITION.md"
SOURCE="src/runtime/allocator_provider_activation.rs"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M102 allocator provider selected-provider precondition"

fail() {
  echo "[$TAG] ERROR: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing file: $path"
}

require_text() {
  local file="$1"
  local needle="$2"
  rg -F -q "$needle" "$file" || fail "missing text in $file: $needle"
}

require_file "$SSOT"
require_file "$POST_LADDER"
require_file "$CARD"
require_file "$SOURCE"
require_file "$TASK_BREAKDOWN"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Selected-Provider Precondition (SSOT)"
require_text "$SSOT" "allocator_provider_selected_provider_precondition_attempt"
require_text "$SSOT" "BlockedSelectedProviderMismatch"
require_text "$SSOT" "ReadySelectedProviderPrecondition"
require_text "$SSOT" "[allocator-provider/proof-bundle-consumption-selected-provider-mismatch]"
require_text "$SSOT" "[allocator-provider/proof-bundle-consumption-selected-provider-ready]"
require_text "$POST_LADDER" "M102 | selected-provider precondition"
require_text "$POST_LADDER" "M103 | proof validation for selected provider"
require_text "$CARD" "293x-158 M102 Allocator Provider Selected-Provider Precondition"
require_text "$SOURCE" "pub fn allocator_provider_selected_provider_precondition_attempt"
require_text "$SOURCE" "BlockedSelectedProviderMismatch"
require_text "$SOURCE" "ReadySelectedProviderPrecondition"
require_text "$SOURCE" "DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_MISMATCH"
require_text "$SOURCE" "DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_READY"
require_text "$SOURCE" "proof_bundle_consumed: false"
require_text "$SOURCE" "selected_provider_precondition_accepts_matching_provider_without_consuming"
require_text "$SOURCE" "selected_provider_precondition_blocks_absent_caller_provider_without_consuming"
require_text "$SOURCE" "selected_provider_precondition_blocks_provider_mismatch_without_consuming"
require_text "$TASK_BREAKDOWN" "M102 | selected-provider precondition"
require_text "$TASK_BREAKDOWN" "M103 | proof validation for selected provider"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_selected_provider_precondition_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_selected_provider_precondition_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_selected_provider_precondition_guard.sh"

cargo test -q selected_provider_precondition -- --nocapture

allocator_provider_forbid_activation_gate_open "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_proof_consumption "$TAG"

allocator_provider_forbid_rollback_preparation "$TAG"

allocator_provider_forbid_hook_activation "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
