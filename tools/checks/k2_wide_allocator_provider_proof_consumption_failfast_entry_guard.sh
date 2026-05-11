#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-proof-consumption-failfast-entry"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-proof-consumption-failfast-entry-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-157-M101-ALLOCATOR-PROVIDER-PROOF-CONSUMPTION-FAILFAST-ENTRY.md"
SOURCE="src/runtime/allocator_provider_activation.rs"
RUNTIME_MOD="src/runtime/mod.rs"
M100_GUARD="tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_entry_contract_guard.sh"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M101 allocator provider proof consumption fail-fast entry"

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
require_file "$CARD"
require_file "$SOURCE"
require_file "$RUNTIME_MOD"
require_file "$M100_GUARD"
require_file "$TASK_BREAKDOWN"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Proof Consumption Fail-Fast Entry (SSOT)"
require_text "$SSOT" "allocator_provider_proof_bundle_consumption_attempt"
require_text "$SSOT" "[allocator-provider/proof-bundle-consumption-selected-provider-missing]"
require_text "$SSOT" "BlockedMissingSelectedProvider"
require_text "$CARD" "293x-157 M101 Allocator Provider Proof Consumption Fail-Fast Entry"
require_text "$SOURCE" "pub fn allocator_provider_proof_bundle_consumption_attempt"
require_text "$SOURCE" "AllocatorProviderProofBundleConsumptionAttemptStatus"
require_text "$SOURCE" "BlockedMissingSelectedProvider"
require_text "$SOURCE" "BlockedMissingProofBundleReport"
require_text "$SOURCE" "DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_SELECTED_PROVIDER_MISSING"
require_text "$SOURCE" "DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_REPORT_MISSING"
require_text "$SOURCE" "proof_bundle_consumed: false"
require_text "$SOURCE" "would_consume_proof_bundle: diagnostic_actions.would_consume_proof"
require_text "$SOURCE" "proof_bundle_consumption_attempt_blocks_when_selected_provider_is_absent"
require_text "$SOURCE" "proof_bundle_consumption_attempt_blocks_malformed_report_without_consuming"
require_text "$RUNTIME_MOD" "pub mod allocator_provider_activation;"
require_text "$TASK_BREAKDOWN" "M101 | proof consumption fail-fast entry"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_proof_consumption_failfast_entry_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_proof_consumption_failfast_entry_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_proof_consumption_failfast_entry_guard.sh"

if rg -n 'must not exist before|require_text "\$CURRENT_STATE"|CURRENT_STATE=' \
  "$M100_GUARD" >/tmp/"$TAG".m100_guard_pin 2>&1; then
  cat /tmp/"$TAG".m100_guard_pin >&2
  rm -f /tmp/"$TAG".m100_guard_pin
  fail "M100 guard must be future-compatible after M101"
fi
rm -f /tmp/"$TAG".m100_guard_pin

cargo test -q proof_bundle_consumption_attempt -- --nocapture

allocator_provider_forbid_activation_gate_open "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_proof_consumption "$TAG"

allocator_provider_forbid_rollback_preparation "$TAG"

allocator_provider_forbid_hook_activation "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider proof consumption behavior"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
