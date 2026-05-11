#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-proof-bundle-consumption-diagnostic-report"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-proof-bundle-consumption-diagnostic-report-ssot.md"
SHAPE_SSOT="docs/development/current/main/design/allocator-provider-proof-bundle-consumption-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-proof-bundle-consumption-v0.toml"
SOURCE="src/runtime/allocator_provider_registry.rs"
IMPL_SOURCE="src/runtime/allocator_provider_proof_bundle_consumption.rs"
TEST_SOURCE="src/runtime/allocator_provider_registry_facade_tests.rs"
INACTIVE_SOURCE="src/runtime/allocator_provider_diagnostic_inactive.rs"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-153-M98-ALLOCATOR-PROVIDER-PROOF-BUNDLE-CONSUMPTION-DIAGNOSTIC-REPORT.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M98 allocator provider proof bundle consumption diagnostic report"

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
require_file "$SHAPE_SSOT"
require_file "$FIXTURE"
require_file "$SOURCE"
require_file "$IMPL_SOURCE"
require_file "$TEST_SOURCE"
require_file "$INACTIVE_SOURCE"
require_file "$TASK_BREAKDOWN"
require_file "$CARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Proof Bundle Consumption Diagnostic Report (SSOT)"
require_text "$SSOT" "validate_allocator_provider_proof_bundle_consumption_from_text"
require_text "$SSOT" "AllocatorProviderProofBundleConsumptionFacts"
require_text "$SSOT" "AllocatorProviderProofBundleConsumptionReport"
require_text "$SSOT" "AllocatorProviderProofBundleConsumptionStatus"
require_text "$SSOT" "proof_bundle_consumption_status=ready_inactive"
require_text "$SSOT" "diagnostic=[allocator-provider/proof-bundle-consumption-inactive]"
require_text "$SSOT" "proof_bundle_consumed=false"
require_text "$SSOT" "would_consume_proof_bundle=false"
require_text "$SSOT" "M98 does not add a CLI route"
require_text "$SHAPE_SSOT" "Allocator Provider Proof Bundle Consumption (SSOT)"
require_text "$FIXTURE" 'schema_version = "allocator_provider_proof_bundle_consumption_v0"'
require_text "$FIXTURE" 'proof_bundle_consumption = "inactive"'
require_text "$FIXTURE" 'would_consume_proof_bundle = false'
require_text "$SOURCE" "Diagnostic-only allocator provider registry facade"
require_text "$SOURCE" "AllocatorProviderProofBundleConsumptionFacts"
require_text "$SOURCE" "AllocatorProviderProofBundleConsumptionReport"
require_text "$SOURCE" "AllocatorProviderProofBundleConsumptionStatus"
require_text "$IMPL_SOURCE" "validate_allocator_provider_proof_bundle_consumption("
require_text "$SOURCE" "validate_allocator_provider_proof_bundle_consumption_from_text"
require_text "$IMPL_SOURCE" "DIAG_PROVIDER_PROOF_BUNDLE_CONSUMPTION_INACTIVE"
require_text "$IMPL_SOURCE" "proof_bundle_consumption_fact_checks"
require_text "$TEST_SOURCE" "proof_bundle_consumption_malformed_text_reports_parse_error_without_consuming_proof"
require_text "$IMPL_SOURCE" "REGISTRY_SNAPSHOT_INACTIVE_ACTIONS"
require_text "$IMPL_SOURCE" "diagnostic_actions.would_consume_proof"
require_text "$IMPL_SOURCE" "would_consume_proof_bundle"
require_text "$INACTIVE_SOURCE" "would_consume_proof: false"
require_text "$TASK_BREAKDOWN" "M98 | proof bundle consumption diagnostic report"
require_text "$CARD" "293x-153 M98 Allocator Provider Proof Bundle Consumption Diagnostic Report"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_diagnostic_report_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_diagnostic_report_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_diagnostic_report_guard.sh"

cargo test -q proof_bundle_consumption -- --nocapture

allocator_provider_forbid_activation_gate_open "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_proof_consumption "$TAG"

allocator_provider_forbid_rollback_preparation "$TAG"

allocator_provider_forbid_hook_activation "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider proof bundle diagnostics"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
