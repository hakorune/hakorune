#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-safety-diagnostic-report"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-report-ssot.md"
GATE_SSOT="docs/development/current/main/design/allocator-provider-activation-safety-gate-ssot.md"
GATE_FIXTURE="docs/development/current/main/design/allocator-provider-activation-safety-gate-v0.toml"
RUNTIME_REGISTRY="src/runtime/allocator_provider_registry.rs"
RUNTIME_SAFETY="src/runtime/allocator_provider_activation_safety.rs"
REGISTRY_FACADE_TESTS="src/runtime/allocator_provider_registry_facade_tests.rs"
RUNTIME_REGISTRY_SNAPSHOT="src/runtime/allocator_provider_registry_snapshot.rs"
INACTIVE_SOURCE="src/runtime/allocator_provider_diagnostic_inactive.rs"
RUNTIME_MOD="src/runtime/mod.rs"
CARD="docs/development/current/main/phases/phase-293x/293x-135-M83-ALLOCATOR-PROVIDER-ACTIVATION-SAFETY-DIAGNOSTIC-REPORT.md"
INDEX="docs/tools/check-scripts-index.md"
FAMILY="tools/checks/allocator/families/provider/activation_safety.steps"
M82_GUARD="tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_owner_guard.sh"

echo "[$TAG] checking M83 allocator provider activation safety diagnostic report"

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
require_file "$GATE_SSOT"
require_file "$GATE_FIXTURE"
require_file "$RUNTIME_REGISTRY"
require_file "$RUNTIME_SAFETY"
require_file "$REGISTRY_FACADE_TESTS"
require_file "$RUNTIME_REGISTRY_SNAPSHOT"
require_file "$INACTIVE_SOURCE"
require_file "$RUNTIME_MOD"
require_file "$CARD"
require_file "$INDEX"
require_file "$FAMILY"
require_file "$M82_GUARD"

require_text "$SSOT" "Allocator Provider Activation Safety Diagnostic Report (SSOT)"
require_text "$SSOT" "validate_allocator_provider_activation_safety_gate_from_text"
require_text "$SSOT" "activation_gate_open = false"
require_text "$SSOT" "would_open_activation_gate = false"
require_text "$SSOT" "would_activate_hook = false"
require_text "$SSOT" "would_activate = false"
require_text "$GATE_SSOT" "Allocator Provider Activation Safety Gate (SSOT)"
require_text "$GATE_FIXTURE" 'safety_gate_owner = "src/runtime/allocator_provider_registry.rs"'
require_text "$RUNTIME_SAFETY" "Diagnostic-only allocator provider activation safety report"
require_text "$RUNTIME_SAFETY" "AllocatorProviderActivationSafetyFacts"
require_text "$RUNTIME_SAFETY" "AllocatorProviderActivationSafetyReport"
require_text "$RUNTIME_SAFETY" "AllocatorProviderActivationSafetyStatus"
require_text "$RUNTIME_SAFETY" "validate_allocator_provider_activation_safety_gate("
require_text "$RUNTIME_SAFETY" "validate_allocator_provider_activation_safety_gate_from_text"
require_text "$RUNTIME_SAFETY" "parse_error: Option<String>"
require_text "$RUNTIME_SAFETY" "activation_safety_fact_checks"
require_text "$REGISTRY_FACADE_TESTS" "activation_safety_malformed_text_reports_parse_error_without_activation"
require_text "$RUNTIME_SAFETY" "DIAG_PROVIDER_ACTIVATION_SAFETY_BLOCKED"
require_text "$INACTIVE_SOURCE" "activation_gate_open: false"
require_text "$INACTIVE_SOURCE" "DIAGNOSTIC_INACTIVE_ACTIONS"
require_text "$INACTIVE_SOURCE" "would_open_activation_gate: false"
require_text "$INACTIVE_SOURCE" "would_activate_hook: false"
require_text "$INACTIVE_SOURCE" "would_activate: false"
require_text "$RUNTIME_SAFETY" "inactive.would_open_activation_gate"
require_text "$RUNTIME_SAFETY" "inactive.would_activate_hook"
require_text "$RUNTIME_SAFETY" "inactive.would_activate"
require_text "$RUNTIME_REGISTRY_SNAPSHOT" "REGISTRY_SNAPSHOT_INACTIVE_ACTIONS"
require_text "$RUNTIME_REGISTRY_SNAPSHOT" "inactive.active_registry_built"
require_text "$RUNTIME_REGISTRY_SNAPSHOT" "inactive.would_build_registry"
require_text "$RUNTIME_REGISTRY_SNAPSHOT" "diagnostic_actions.would_select_provider"
require_text "$RUNTIME_REGISTRY_SNAPSHOT" "diagnostic_actions.would_consume_proof"
require_text "$RUNTIME_REGISTRY_SNAPSHOT" "diagnostic_actions.would_prepare_rollback"
require_text "$RUNTIME_REGISTRY_SNAPSHOT" "diagnostic_actions.would_open_activation_gate"
require_text "$RUNTIME_REGISTRY_SNAPSHOT" "diagnostic_actions.would_install_hook"
require_text "$RUNTIME_REGISTRY_SNAPSHOT" "diagnostic_actions.would_replace_process_allocator"
require_text "$RUNTIME_REGISTRY_SNAPSHOT" "diagnostic_actions.would_activate"
require_text "$INACTIVE_SOURCE" "DIAGNOSTIC_INACTIVE_ACTIONS"
require_text "$INACTIVE_SOURCE" "REGISTRY_SNAPSHOT_INACTIVE_ACTIONS"
require_text "$INACTIVE_SOURCE" "active_registry_built: false"
require_text "$INACTIVE_SOURCE" "would_build_registry: false"
require_text "$INACTIVE_SOURCE" "would_select_provider: false"
require_text "$INACTIVE_SOURCE" "would_consume_proof: false"
require_text "$INACTIVE_SOURCE" "would_prepare_rollback: false"
require_text "$INACTIVE_SOURCE" "would_install_hook: false"
require_text "$INACTIVE_SOURCE" "would_replace_process_allocator: false"
require_text "$RUNTIME_MOD" "pub mod allocator_provider_registry;"
require_text "$RUNTIME_MOD" "pub(crate) mod allocator_provider_registry_snapshot;"
require_text "$CARD" "293x-135 M83 Allocator Provider Activation Safety Diagnostic Report"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh"
require_text "$FAMILY" "tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh"

allocator_provider_forbid_activation_gate_open "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_proof_consumption "$TAG"

allocator_provider_forbid_rollback_preparation "$TAG"

allocator_provider_forbid_hook_activation "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider activation safety diagnostics"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
