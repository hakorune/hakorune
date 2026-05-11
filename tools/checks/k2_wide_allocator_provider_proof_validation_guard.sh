#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-proof-validation"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-selected-provider-proof-validation-ssot.md"
POST_LADDER="docs/development/current/main/design/allocator-provider-post-m101-implementation-ladder-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-159-M103-ALLOCATOR-PROVIDER-SELECTED-PROVIDER-PROOF-VALIDATION.md"
ACTIVATION_SOURCE="src/runtime/allocator_provider_activation.rs"
VALIDATION_SOURCE="src/runtime/allocator_provider_proof_validation.rs"
RUNTIME_MOD="src/runtime/mod.rs"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_allocator_provider_proof_validation_guard.sh"

echo "[$TAG] checking M103 allocator provider selected-provider proof validation"

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
require_file "$ACTIVATION_SOURCE"
require_file "$VALIDATION_SOURCE"
require_file "$RUNTIME_MOD"
require_file "$TASK_BREAKDOWN"
require_file "$INDEX"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Selected-Provider Proof Validation (SSOT)"
require_text "$SSOT" "allocator_provider_selected_provider_proof_validation_attempt"
require_text "$SSOT" "BlockedSelectedProviderProofIncomplete"
require_text "$SSOT" "ReadySelectedProviderProofValidated"
require_text "$SSOT" "[allocator-provider/proof-bundle-consumption-selected-provider-proof-ready]"
require_text "$POST_LADDER" "M103 | proof validation for selected provider"
require_text "$POST_LADDER" "M104 ALLOCATOR-PROVIDER-PROOF-BUNDLE-CONSUMPTION-TOKEN"
require_text "$CARD" "293x-159 M103 Allocator Provider Selected-Provider Proof Validation"
require_text "$ACTIVATION_SOURCE" "pub fn allocator_provider_selected_provider_proof_validation_attempt"
require_text "$ACTIVATION_SOURCE" "BlockedSelectedProviderProofMissing"
require_text "$ACTIVATION_SOURCE" "BlockedSelectedProviderProofIncomplete"
require_text "$ACTIVATION_SOURCE" "ReadySelectedProviderProofValidated"
require_text "$ACTIVATION_SOURCE" "DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_PROOF_READY"
require_text "$ACTIVATION_SOURCE" "selected_provider_proof_validated"
require_text "$ACTIVATION_SOURCE" "proof_bundle_consumed: false"
require_text "$ACTIVATION_SOURCE" "selected_provider_proof_validation_accepts_matching_provider_without_consuming"
require_text "$ACTIVATION_SOURCE" "selected_provider_proof_validation_blocks_incomplete_operations_without_consuming"
require_text "$VALIDATION_SOURCE" "pub(crate) fn validate_selected_provider_proof"
require_text "$VALIDATION_SOURCE" "AllocatorProviderSelectedProviderProofValidationFacts"
require_text "$VALIDATION_SOURCE" "selected_provider_proof_operations_cover_request"
require_text "$VALIDATION_SOURCE" "selected_provider_proof_validation_accepts_ready_report_for_selected_provider"
require_text "$RUNTIME_MOD" "pub(crate) mod allocator_provider_proof_validation;"
require_text "$TASK_BREAKDOWN" "M103 | selected-provider proof validation"
require_text "$TASK_BREAKDOWN" "M104 | proof bundle consumption token"
require_text "$INDEX" "$SELF_SCRIPT"

if rg -F -q "$SELF_SCRIPT" "$ALLOCATOR_GROUP"; then
  fail "M103 focused guard must not be registered as another per-row wide allocator gate step"
fi

cargo test -q selected_provider_proof_validation -- --nocapture

allocator_provider_forbid_activation_gate_open "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_proof_consumption "$TAG"

allocator_provider_forbid_rollback_preparation "$TAG"

allocator_provider_forbid_hook_activation "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
