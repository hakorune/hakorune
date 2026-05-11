#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-diagnostic-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-activation-diagnostic-closeout-inventory-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-149-M95-ALLOCATOR-PROVIDER-ACTIVATION-DIAGNOSTIC-CLOSEOUT-INVENTORY.md"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
M92_SSOT="docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-ssot.md"
M92_FIXTURE="docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-v0.toml"
M93_SSOT="docs/development/current/main/design/allocator-provider-registry-snapshot-diagnostic-report-ssot.md"
M93B_SSOT="docs/development/current/main/design/allocator-provider-diagnostic-inactive-actions-ssot.md"
M94_SSOT="docs/development/current/main/design/allocator-provider-registry-snapshot-cli-surface-ssot.md"
RUNTIME_REGISTRY="src/runtime/allocator_provider_registry.rs"
RUNTIME_REGISTRY_SNAPSHOT="src/runtime/allocator_provider_registry_snapshot.rs"
INACTIVE_SOURCE="src/runtime/allocator_provider_diagnostic_inactive.rs"
M94_CLI="src/cli/allocator_provider_registry_snapshot.rs"

echo "[$TAG] checking M95 allocator provider activation diagnostic closeout inventory"

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
require_file "$TASK_BREAKDOWN"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"
require_file "$M92_SSOT"
require_file "$M92_FIXTURE"
require_file "$M93_SSOT"
require_file "$M93B_SSOT"
require_file "$M94_SSOT"
require_file "$RUNTIME_REGISTRY"
require_file "$RUNTIME_REGISTRY_SNAPSHOT"
require_file "$INACTIVE_SOURCE"
require_file "$M94_CLI"

required_cards=(
  "docs/development/current/main/phases/phase-293x/293x-145-M92-ALLOCATOR-PROVIDER-ACTIVATION-IMPLEMENTATION-ENTRY-CONTRACT.md"
  "docs/development/current/main/phases/phase-293x/293x-146-M93-ALLOCATOR-PROVIDER-REGISTRY-SNAPSHOT-DIAGNOSTIC-REPORT.md"
  "docs/development/current/main/phases/phase-293x/293x-147-M93B-ALLOCATOR-PROVIDER-DIAGNOSTIC-INACTIVE-ACTIONS.md"
  "docs/development/current/main/phases/phase-293x/293x-148-M94-ALLOCATOR-PROVIDER-REGISTRY-SNAPSHOT-CLI-SURFACE.md"
)

required_guards=(
  "tools/checks/k2_wide_allocator_provider_activation_implementation_entry_contract_guard.sh"
  "tools/checks/k2_wide_allocator_provider_registry_snapshot_diagnostic_report_guard.sh"
  "tools/checks/k2_wide_allocator_provider_diagnostic_inactive_actions_guard.sh"
  "tools/checks/k2_wide_allocator_provider_registry_snapshot_cli_surface_guard.sh"
)

for file in "${required_cards[@]}"; do
  require_file "$file"
done

for guard in "${required_guards[@]}"; do
  require_file "$guard"
  require_text "$INDEX" "$guard"
  require_text "$DEV_GATE" "$guard"
  require_text "$ALLOCATOR_GROUP" "$guard"
done

if rg -n 'latest_card[[:space:]]*=|latest_card_path[[:space:]]*=' "${required_guards[@]}" >/tmp/"$TAG".past_guard_pins 2>&1; then
  cat /tmp/"$TAG".past_guard_pins >&2
  rm -f /tmp/"$TAG".past_guard_pins
  fail "past activation diagnostic guards must not pin CURRENT_STATE latest-card pointers"
fi
rm -f /tmp/"$TAG".past_guard_pins

require_text "$SSOT" "Allocator Provider Activation Diagnostic Closeout Inventory (SSOT)"
require_text "$SSOT" "M92 through"
require_text "$SSOT" "M93B inactive-action cleanup"
require_text "$SSOT" "coverage-only"
require_text "$SSOT" "hakorune --allocator-provider-registry-snapshot <REGISTRY_SNAPSHOT_TOML>"
require_text "$SSOT" "src/runtime/allocator_provider_diagnostic_inactive.rs"
require_text "$SSOT" "active_registry_built=false"
require_text "$SSOT" "would_build_registry=false"
require_text "$SSOT" "would_select_provider=false"
require_text "$SSOT" "would_consume_proof=false"
require_text "$SSOT" "would_prepare_rollback=false"
require_text "$SSOT" "would_open_activation_gate=false"
require_text "$SSOT" "would_install_hook=false"
require_text "$SSOT" "would_replace_process_allocator=false"
require_text "$SSOT" "would_activate=false"
require_text "$SSOT" "M96 selection decision diagnostic report"
require_text "$CARD" "293x-149 M95 Allocator Provider Activation Diagnostic Closeout Inventory"
require_text "$TASK_BREAKDOWN" "| M95 | activation diagnostic closeout inventory fixing coverage across M92-M94/M93B without activation | complete |"
require_text "$TASK_BREAKDOWN" "| M96 | selection decision diagnostic report | runtime report over caller-provided selection decision TOML text | provider selection |"
require_text "$M92_SSOT" "| M95 | activation diagnostic closeout inventory"
require_text "$M92_SSOT" "| M96 | selection decision diagnostic report"
require_text "$M92_FIXTURE" '"M95 activation diagnostic closeout inventory"'
require_text "$M92_FIXTURE" '"M96 selection decision diagnostic report"'
require_text "$M93_SSOT" "M94 may expose this report through an explicit CLI diagnostic surface"
require_text "$M93B_SSOT" "src/runtime/allocator_provider_diagnostic_inactive.rs"
require_text "$M94_SSOT" "Allocator Provider Registry Snapshot CLI Surface (SSOT)"
require_text "$RUNTIME_REGISTRY" "AllocatorProviderRegistrySnapshotReport"
require_text "$RUNTIME_REGISTRY" "validate_allocator_provider_registry_snapshot_from_text"
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
require_text "$INACTIVE_SOURCE" "would_open_activation_gate: false"
require_text "$INACTIVE_SOURCE" "would_install_hook: false"
require_text "$INACTIVE_SOURCE" "would_replace_process_allocator: false"
require_text "$INACTIVE_SOURCE" "would_activate: false"
require_text "$M94_CLI" "maybe_run_allocator_provider_registry_snapshot_diagnostic"
require_text "$M94_CLI" "allocator-provider/registry-snapshot-cli-read-error"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_activation_diagnostic_closeout_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_activation_diagnostic_closeout_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_activation_diagnostic_closeout_guard.sh"

allocator_provider_forbid_activation_gate_open "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_proof_consumption "$TAG"

allocator_provider_forbid_rollback_preparation "$TAG"

allocator_provider_forbid_hook_activation "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider activation diagnostic closeout behavior"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
