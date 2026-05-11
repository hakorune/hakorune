#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-selection-decision-diagnostic-report"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-selection-decision-diagnostic-report-ssot.md"
SHAPE_SSOT="docs/development/current/main/design/allocator-provider-selection-decision-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-selection-decision-v0.toml"
SOURCE="src/runtime/allocator_provider_registry.rs"
INACTIVE_SOURCE="src/runtime/allocator_provider_diagnostic_inactive.rs"
RUNTIME_MOD="src/runtime/mod.rs"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-150-M96-ALLOCATOR-PROVIDER-SELECTION-DECISION-DIAGNOSTIC-REPORT.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M96 allocator provider selection decision diagnostic report"

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
require_file "$INACTIVE_SOURCE"
require_file "$RUNTIME_MOD"
require_file "$TASK_BREAKDOWN"
require_file "$CARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

past_guards=(
  "tools/checks/k2_wide_allocator_provider_activation_implementation_entry_contract_guard.sh"
  "tools/checks/k2_wide_allocator_provider_registry_snapshot_diagnostic_report_guard.sh"
  "tools/checks/k2_wide_allocator_provider_diagnostic_inactive_actions_guard.sh"
  "tools/checks/k2_wide_allocator_provider_registry_snapshot_cli_surface_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_diagnostic_closeout_guard.sh"
)

for guard in "${past_guards[@]}"; do
  require_file "$guard"
done

if rg -n 'latest_card[[:space:]]*=|latest_card_path[[:space:]]*=' "${past_guards[@]}" >/tmp/"$TAG".past_guard_pins 2>&1; then
  cat /tmp/"$TAG".past_guard_pins >&2
  rm -f /tmp/"$TAG".past_guard_pins
  fail "past activation diagnostic guards must not pin CURRENT_STATE latest-card pointers"
fi
rm -f /tmp/"$TAG".past_guard_pins

require_text "$SSOT" "Allocator Provider Selection Decision Diagnostic Report (SSOT)"
require_text "$SSOT" "validate_allocator_provider_selection_decision_from_text"
require_text "$SSOT" "AllocatorProviderSelectionDecisionFacts"
require_text "$SSOT" "AllocatorProviderSelectionDecisionReport"
require_text "$SSOT" "AllocatorProviderSelectionDecisionStatus"
require_text "$SSOT" "selection_decision_status=ready_inactive"
require_text "$SSOT" "diagnostic=[allocator-provider/selection-decision-inactive]"
require_text "$SSOT" "selected_provider_id=none_reserved"
require_text "$SSOT" "selected_provider_id_absent=true"
require_text "$SSOT" "active_registry_built=false"
require_text "$SSOT" "would_build_registry=false"
require_text "$SSOT" "would_select_provider=false"
require_text "$SSOT" "would_consume_proof=false"
require_text "$SSOT" "would_prepare_rollback=false"
require_text "$SSOT" "would_open_activation_gate=false"
require_text "$SSOT" "would_install_hook=false"
require_text "$SSOT" "would_replace_process_allocator=false"
require_text "$SSOT" "would_activate=false"
require_text "$SSOT" "M97 may expose this report through an explicit CLI diagnostic surface"
require_text "$SHAPE_SSOT" "Allocator Provider Selection Decision (SSOT)"
require_text "$FIXTURE" 'schema_version = "allocator_provider_selection_decision_v0"'
require_text "$FIXTURE" 'selection_owner = "src/runtime/allocator_provider_registry.rs"'
require_text "$FIXTURE" 'selected_provider_id = "none_reserved"'
require_text "$FIXTURE" 'would_select_provider = false'
require_text "$SOURCE" "Diagnostic-only allocator provider registry"
require_text "$SOURCE" "AllocatorProviderSelectionDecisionFacts"
require_text "$SOURCE" "AllocatorProviderSelectionDecisionReport"
require_text "$SOURCE" "AllocatorProviderSelectionDecisionStatus"
require_text "$SOURCE" "validate_allocator_provider_selection_decision("
require_text "$SOURCE" "validate_allocator_provider_selection_decision_from_text"
require_text "$SOURCE" "parse_error: Option<String>"
require_text "$SOURCE" "selection_decision_fact_checks"
require_text "$SOURCE" "selection_decision_malformed_text_reports_parse_error_without_selection"
require_text "$SOURCE" "DIAG_PROVIDER_SELECTION_DECISION_INACTIVE"
require_text "$SOURCE" "DIAG_PROVIDER_SELECTION_UNSUPPORTED_PROVIDER"
require_text "$SOURCE" "REGISTRY_SNAPSHOT_INACTIVE_ACTIONS"
require_text "$SOURCE" "diagnostic_actions.would_select_provider"
require_text "$SOURCE" "diagnostic_actions.would_consume_proof"
require_text "$SOURCE" "diagnostic_actions.would_prepare_rollback"
require_text "$SOURCE" "diagnostic_actions.would_open_activation_gate"
require_text "$SOURCE" "diagnostic_actions.would_install_hook"
require_text "$SOURCE" "diagnostic_actions.would_replace_process_allocator"
require_text "$SOURCE" "diagnostic_actions.would_activate"
require_text "$SOURCE" "inactive.active_registry_built"
require_text "$SOURCE" "inactive.would_build_registry"
require_text "$INACTIVE_SOURCE" "REGISTRY_SNAPSHOT_INACTIVE_ACTIONS"
require_text "$INACTIVE_SOURCE" "DIAGNOSTIC_INACTIVE_ACTIONS"
require_text "$RUNTIME_MOD" "pub mod allocator_provider_registry;"
require_text "$TASK_BREAKDOWN" "M96 | selection decision diagnostic report"
require_text "$TASK_BREAKDOWN" "M97 selection decision CLI surface"
require_text "$CARD" "293x-150 M96 Allocator Provider Selection Decision Diagnostic Report"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_selection_decision_diagnostic_report_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_selection_decision_diagnostic_report_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_selection_decision_diagnostic_report_guard.sh"

cargo test -q selection_decision -- --nocapture

allocator_provider_forbid_activation_gate_open "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_proof_consumption "$TAG"

allocator_provider_forbid_rollback_preparation "$TAG"

allocator_provider_forbid_hook_activation "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider selection decision diagnostics"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
