#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-decision-diagnostic-report"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-activation-decision-diagnostic-report-ssot.md"
OWNER_SSOT="docs/development/current/main/design/allocator-provider-activation-decision-diagnostic-owner-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-activation-decision-v0.toml"
SOURCE="src/runtime/allocator_provider_activation_decision.rs"
RUNTIME_MOD="src/runtime/mod.rs"
CARD="docs/development/current/main/phases/phase-293x/293x-142-M89-ALLOCATOR-PROVIDER-ACTIVATION-DECISION-DIAGNOSTIC-REPORT.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
M86_GUARD="tools/checks/k2_wide_allocator_provider_activation_decision_surface_proposal_guard.sh"
M87_GUARD="tools/checks/k2_wide_allocator_provider_activation_decision_fixture_contract_guard.sh"

echo "[$TAG] checking M89 allocator provider activation decision diagnostic report"

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
require_file "$OWNER_SSOT"
require_file "$FIXTURE"
require_file "$SOURCE"
require_file "$RUNTIME_MOD"
require_file "$CARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"
require_file "$M86_GUARD"
require_file "$M87_GUARD"

require_text "$SSOT" "Allocator Provider Activation Decision Diagnostic Report (SSOT)"
require_text "$SSOT" "validate_allocator_provider_activation_decision_from_text"
require_text "$SSOT" "ReadyBlocked"
require_text "$SSOT" "activation_decision_allowed = false"
require_text "$SSOT" "would_prepare_rollback = false"
require_text "$OWNER_SSOT" "src/runtime/allocator_provider_activation_decision.rs"
require_text "$FIXTURE" 'decision_surface_owner = "src/runtime/allocator_provider_activation_decision.rs"'
require_text "$SOURCE" "Diagnostic-only allocator provider activation decision reports"
require_text "$SOURCE" "AllocatorProviderActivationDecisionFacts"
require_text "$SOURCE" "AllocatorProviderActivationDecisionReport"
require_text "$SOURCE" "AllocatorProviderActivationDecisionStatus"
require_text "$SOURCE" "validate_allocator_provider_activation_decision("
require_text "$SOURCE" "validate_allocator_provider_activation_decision_from_text"
require_text "$SOURCE" "parse_error: Option<String>"
require_text "$SOURCE" "activation_decision_fact_checks"
require_text "$SOURCE" "activation_decision_malformed_text_reports_parse_error_without_activation"
require_text "$SOURCE" "DIAG_PROVIDER_ACTIVATION_DECISION_BLOCKED"
require_text "$SOURCE" "activation_decision_allowed: false"
require_text "$SOURCE" "would_select_provider: false"
require_text "$SOURCE" "would_consume_proof: false"
require_text "$SOURCE" "would_prepare_rollback: false"
require_text "$SOURCE" "would_open_activation_gate: false"
require_text "$SOURCE" "would_install_hook: false"
require_text "$SOURCE" "would_replace_process_allocator: false"
require_text "$SOURCE" "would_activate: false"
require_text "$SOURCE" "allocator-provider-activation-decision-v0.toml"
require_text "$RUNTIME_MOD" "pub mod allocator_provider_activation_decision;"
require_text "$CARD" "293x-142 M89 Allocator Provider Activation Decision Diagnostic Report"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_report_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_report_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_report_guard.sh"

cargo test -q activation_decision -- --nocapture

if rg -n 'must not add activation decision runtime or CLI code|no_runtime_decision_parser' "$M86_GUARD" "$M87_GUARD" >/tmp/"$TAG".past_guard_pin 2>&1; then
  cat /tmp/"$TAG".past_guard_pin >&2
  rm -f /tmp/"$TAG".past_guard_pin
  fail "M86/M87 guards must not block future activation decision diagnostic owner/type names"
fi
rm -f /tmp/"$TAG".past_guard_pin

allocator_provider_forbid_activation_gate_open "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_proof_consumption "$TAG"

allocator_provider_forbid_rollback_preparation "$TAG"

allocator_provider_forbid_hook_activation "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider activation decision diagnostics"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
