#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-safety-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh
source tools/checks/lib/phase_card_paths.sh
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-activation-safety-closeout-inventory-ssot.md"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="$(guard_require_phase293x_card "$TAG" "293x-137-M85-ALLOCATOR-PROVIDER-ACTIVATION-SAFETY-CLOSEOUT-INVENTORY.md")"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
RUNTIME_OWNER="src/runtime/allocator_provider_registry.rs"
ACTIVATION_SAFETY_SOURCE="src/runtime/allocator_provider_activation_safety.rs"
INACTIVE_SOURCE="src/runtime/allocator_provider_diagnostic_inactive.rs"
CLI_FILE="src/cli/allocator_provider_activation_safety.rs"

echo "[$TAG] checking M85 allocator provider activation safety closeout inventory"

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
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"
require_file "$RUNTIME_OWNER"
require_file "$ACTIVATION_SAFETY_SOURCE"
require_file "$INACTIVE_SOURCE"
require_file "$CLI_FILE"

required_ssots=(
  "docs/development/current/main/design/allocator-provider-activation-entry-contract-ssot.md"
  "docs/development/current/main/design/allocator-provider-registry-snapshot-ssot.md"
  "docs/development/current/main/design/allocator-provider-selection-decision-ssot.md"
  "docs/development/current/main/design/allocator-provider-proof-bundle-consumption-ssot.md"
  "docs/development/current/main/design/allocator-provider-rollback-preflight-ssot.md"
  "docs/development/current/main/design/allocator-provider-activation-safety-gate-ssot.md"
  "docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-owner-ssot.md"
  "docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-report-ssot.md"
  "docs/development/current/main/design/allocator-provider-activation-safety-cli-surface-ssot.md"
)

required_fixtures=(
  "docs/development/current/main/design/allocator-provider-activation-entry-contract-v0.toml"
  "docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml"
  "docs/development/current/main/design/allocator-provider-selection-decision-v0.toml"
  "docs/development/current/main/design/allocator-provider-proof-bundle-consumption-v0.toml"
  "docs/development/current/main/design/allocator-provider-rollback-preflight-v0.toml"
  "docs/development/current/main/design/allocator-provider-activation-safety-gate-v0.toml"
)

required_card_files=(
  "293x-128-M76-ALLOCATOR-PROVIDER-ACTIVATION-ENTRY-CONTRACT.md"
  "293x-129-M77-ALLOCATOR-PROVIDER-REGISTRY-SNAPSHOT.md"
  "293x-130-M78-ALLOCATOR-PROVIDER-SELECTION-DECISION.md"
  "293x-131-M79-ALLOCATOR-PROVIDER-PROOF-BUNDLE-CONSUMPTION.md"
  "293x-132-M80-ALLOCATOR-PROVIDER-ROLLBACK-PREFLIGHT.md"
  "293x-133-M81-ALLOCATOR-PROVIDER-ACTIVATION-SAFETY-GATE.md"
  "293x-134-M82-ALLOCATOR-PROVIDER-ACTIVATION-SAFETY-DIAGNOSTIC-OWNER.md"
  "293x-135-M83-ALLOCATOR-PROVIDER-ACTIVATION-SAFETY-DIAGNOSTIC-REPORT.md"
  "293x-136-M84-ALLOCATOR-PROVIDER-ACTIVATION-SAFETY-CLI-SURFACE.md"
)
required_cards=()
for card_file in "${required_card_files[@]}"; do
  required_cards+=("$(guard_require_phase293x_card "$TAG" "$card_file")")
done

required_guards=(
  "tools/checks/k2_wide_allocator_provider_activation_entry_contract_guard.sh"
  "tools/checks/k2_wide_allocator_provider_registry_snapshot_guard.sh"
  "tools/checks/k2_wide_allocator_provider_selection_decision_guard.sh"
  "tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_guard.sh"
  "tools/checks/k2_wide_allocator_provider_rollback_preflight_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_safety_gate_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_owner_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_safety_cli_surface_guard.sh"
)

for file in "${required_ssots[@]}" "${required_fixtures[@]}" "${required_cards[@]}"; do
  require_file "$file"
done

for guard in "${required_guards[@]}"; do
  require_file "$guard"
  require_text "$INDEX" "$guard"
  require_text "$DEV_GATE" "$guard"
  require_text "$ALLOCATOR_GROUP" "$guard"
done

if rg -n 'latest_card|latest_card_path' "${required_guards[@]}" >/tmp/"$TAG".past_guard_pins 2>&1; then
  cat /tmp/"$TAG".past_guard_pins >&2
  rm -f /tmp/"$TAG".past_guard_pins
  fail "past activation safety guards must not pin CURRENT_STATE latest-card pointers"
fi
rm -f /tmp/"$TAG".past_guard_pins

require_text "$SSOT" "Allocator Provider Activation Safety Closeout Inventory (SSOT)"
require_text "$SSOT" "M76-M84 activation safety diagnostic ladder"
require_text "$SSOT" "hakorune --allocator-provider-activation-safety-gate <ACTIVATION_SAFETY_GATE_TOML>"
require_text "$SSOT" "activation_gate_open=false"
require_text "$SSOT" "would_open_activation_gate=false"
require_text "$SSOT" "would_activate_hook=false"
require_text "$SSOT" "would_activate=false"
require_text "$RUNTIME_OWNER" "AllocatorProviderActivationSafetyReport"
require_text "$ACTIVATION_SAFETY_SOURCE" "SAFETY_GATE_INACTIVE_ACTIONS"
require_text "$ACTIVATION_SAFETY_SOURCE" "inactive.activation_gate_open"
require_text "$ACTIVATION_SAFETY_SOURCE" "inactive.would_open_activation_gate"
require_text "$ACTIVATION_SAFETY_SOURCE" "inactive.would_activate_hook"
require_text "$ACTIVATION_SAFETY_SOURCE" "inactive.would_activate"
require_text "$INACTIVE_SOURCE" "SAFETY_GATE_INACTIVE_ACTIONS"
require_text "$INACTIVE_SOURCE" "activation_gate_open: false"
require_text "$INACTIVE_SOURCE" "would_open_activation_gate: false"
require_text "$INACTIVE_SOURCE" "would_activate_hook: false"
require_text "$INACTIVE_SOURCE" "would_activate: false"
require_text "$CLI_FILE" "maybe_run_allocator_provider_activation_safety_diagnostic"
require_text "$CLI_FILE" "allocator-provider/activation-safety-cli-read-error"
require_text "$TASK_BREAKDOWN" "M85 | activation safety closeout inventory"
require_text "$TASKBOARD" '| `M85 allocator provider activation safety closeout inventory` | `live-narrow` |'
require_text "$TASKBOARD" '108. `M85 allocator provider activation safety closeout inventory`'
require_text "$CARD" "293x-137 M85 Allocator Provider Activation Safety Closeout Inventory"
require_text "$PHASE_README" '`293x-137`'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_activation_safety_closeout_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_activation_safety_closeout_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_activation_safety_closeout_guard.sh"

if rg -n 'std::env|set_var|var_os|env_bool|env_string|NYASH_ALLOCATOR_PROVIDER|HAKO_ALLOCATOR_PROVIDER|ALLOCATOR_PROVIDER_' "$CLI_FILE" >/tmp/"$TAG".env 2>&1; then
  cat /tmp/"$TAG".env >&2
  rm -f /tmp/"$TAG".env
  fail "activation safety closeout must not add hidden environment toggles"
fi
rm -f /tmp/"$TAG".env

allocator_provider_forbid_activation_gate_open "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_proof_consumption "$TAG"

allocator_provider_forbid_rollback_preparation "$TAG"

allocator_provider_forbid_hook_activation "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider activation safety closeout behavior"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
