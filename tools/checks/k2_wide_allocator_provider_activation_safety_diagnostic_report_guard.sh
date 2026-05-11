#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-safety-diagnostic-report"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-report-ssot.md"
OWNER_SSOT="docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-owner-ssot.md"
GATE_SSOT="docs/development/current/main/design/allocator-provider-activation-safety-gate-ssot.md"
GATE_FIXTURE="docs/development/current/main/design/allocator-provider-activation-safety-gate-v0.toml"
SOURCE="src/runtime/allocator_provider_registry.rs"
IMPL_SOURCE="src/runtime/allocator_provider_activation_safety.rs"
INACTIVE_SOURCE="src/runtime/allocator_provider_diagnostic_inactive.rs"
RUNTIME_MOD="src/runtime/mod.rs"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-135-M83-ALLOCATOR-PROVIDER-ACTIVATION-SAFETY-DIAGNOSTIC-REPORT.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
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
require_file "$OWNER_SSOT"
require_file "$GATE_SSOT"
require_file "$GATE_FIXTURE"
require_file "$SOURCE"
require_file "$IMPL_SOURCE"
require_file "$INACTIVE_SOURCE"
require_file "$RUNTIME_MOD"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"
require_file "$M82_GUARD"

require_text "$SSOT" "Allocator Provider Activation Safety Diagnostic Report (SSOT)"
require_text "$SSOT" "validate_allocator_provider_activation_safety_gate_from_text"
require_text "$OWNER_SSOT" "src/runtime/allocator_provider_registry.rs"
require_text "$GATE_SSOT" "activation_safety_gate = \"inactive\""
require_text "$GATE_FIXTURE" 'activation_gate_open = false'
require_text "$SOURCE" "Diagnostic-only allocator provider registry"
require_text "$SOURCE" "AllocatorProviderActivationSafetyFacts"
require_text "$SOURCE" "AllocatorProviderActivationSafetyReport"
require_text "$SOURCE" "AllocatorProviderActivationSafetyStatus"
require_text "$IMPL_SOURCE" "validate_allocator_provider_activation_safety_gate("
require_text "$SOURCE" "validate_allocator_provider_activation_safety_gate_from_text"
require_text "$IMPL_SOURCE" "parse_error: Option<String>"
require_text "$IMPL_SOURCE" "activation_safety_fact_checks"
require_text "$IMPL_SOURCE" "activation_safety_diagnostic_checks"
require_text "$SOURCE" "activation_safety_malformed_text_reports_parse_error_without_activation"
require_text "$IMPL_SOURCE" "DIAG_PROVIDER_ACTIVATION_SAFETY_BLOCKED"
require_text "$IMPL_SOURCE" "SAFETY_GATE_INACTIVE_ACTIONS"
require_text "$IMPL_SOURCE" "inactive.activation_gate_open"
require_text "$IMPL_SOURCE" "inactive.would_open_activation_gate"
require_text "$IMPL_SOURCE" "inactive.would_activate_hook"
require_text "$IMPL_SOURCE" "inactive.would_activate"
require_text "$INACTIVE_SOURCE" "SAFETY_GATE_INACTIVE_ACTIONS"
require_text "$INACTIVE_SOURCE" "activation_gate_open: false"
require_text "$INACTIVE_SOURCE" "would_open_activation_gate: false"
require_text "$INACTIVE_SOURCE" "would_activate_hook: false"
require_text "$INACTIVE_SOURCE" "would_activate: false"
require_text "$SOURCE" "allocator-provider-activation-safety-gate-v0.toml"
require_text "$SSOT" "parse_error = Some(...)"
require_text "$RUNTIME_MOD" "pub mod allocator_provider_registry;"
require_text "$TASK_BREAKDOWN" "M83 | activation safety diagnostic report"
require_text "$TASKBOARD" '| `M83 allocator provider activation safety diagnostic report` | `live-narrow` |'
require_text "$TASKBOARD" '106. `M83 allocator provider activation safety diagnostic report`'
require_text "$CARD" "293x-135 M83 Allocator Provider Activation Safety Diagnostic Report"
require_text "$PHASE_README" '`293x-135`'
require_text "$REAL_APP_TASKBOARD" '[x] `293x-135` M83 allocator provider activation safety diagnostic report'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh"

cargo test -q activation_safety -- --nocapture

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
