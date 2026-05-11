#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-decision-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-activation-decision-closeout-inventory-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-144-M91-ALLOCATOR-PROVIDER-ACTIVATION-DECISION-CLOSEOUT-INVENTORY.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
RUNTIME_OWNER="src/runtime/allocator_provider_activation_decision.rs"
INACTIVE_SOURCE="src/runtime/allocator_provider_diagnostic_inactive.rs"
CLI_FILE="src/cli/allocator_provider_activation_decision.rs"

echo "[$TAG] checking M91 allocator provider activation decision closeout inventory"

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
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"
require_file "$RUNTIME_OWNER"
require_file "$INACTIVE_SOURCE"
require_file "$CLI_FILE"

required_ssots=(
  "docs/development/current/main/design/allocator-provider-activation-decision-surface-proposal-ssot.md"
  "docs/development/current/main/design/allocator-provider-activation-decision-diagnostic-owner-ssot.md"
  "docs/development/current/main/design/allocator-provider-activation-decision-diagnostic-report-ssot.md"
  "docs/development/current/main/design/allocator-provider-activation-decision-cli-surface-ssot.md"
)

required_fixtures=(
  "docs/development/current/main/design/allocator-provider-activation-decision-v0.toml"
)

required_cards=(
  "docs/development/current/main/phases/phase-293x/293x-138-M86-ALLOCATOR-PROVIDER-ACTIVATION-DECISION-SURFACE-PROPOSAL.md"
  "docs/development/current/main/phases/phase-293x/293x-140-M87-ALLOCATOR-PROVIDER-ACTIVATION-DECISION-FIXTURE-CONTRACT.md"
  "docs/development/current/main/phases/phase-293x/293x-141-M88-ALLOCATOR-PROVIDER-ACTIVATION-DECISION-DIAGNOSTIC-OWNER.md"
  "docs/development/current/main/phases/phase-293x/293x-142-M89-ALLOCATOR-PROVIDER-ACTIVATION-DECISION-DIAGNOSTIC-REPORT.md"
  "docs/development/current/main/phases/phase-293x/293x-143-M90-ALLOCATOR-PROVIDER-ACTIVATION-DECISION-CLI-SURFACE.md"
)

required_guards=(
  "tools/checks/k2_wide_allocator_provider_activation_decision_surface_proposal_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_decision_fixture_contract_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_owner_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_report_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_decision_cli_surface_guard.sh"
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

if rg -n 'latest_card|latest_card_path' "${required_guards[@]}" 2>/dev/null \
  | rg -v 'past_guard_pin|must not pin CURRENT_STATE latest-card pointers' \
    >/tmp/"$TAG".past_guard_pins; then
  cat /tmp/"$TAG".past_guard_pins >&2
  rm -f /tmp/"$TAG".past_guard_pins
  fail "past activation decision guards must not pin CURRENT_STATE latest-card pointers"
fi
rm -f /tmp/"$TAG".past_guard_pins

require_text "$SSOT" "Allocator Provider Activation Decision Closeout Inventory (SSOT)"
require_text "$SSOT" "M86-M90 activation decision diagnostic ladder"
require_text "$SSOT" "hakorune --allocator-provider-activation-decision <ACTIVATION_DECISION_TOML>"
require_text "$SSOT" "activation_decision_allowed=false"
require_text "$SSOT" "would_select_provider=false"
require_text "$SSOT" "would_consume_proof=false"
require_text "$SSOT" "would_prepare_rollback=false"
require_text "$SSOT" "would_open_activation_gate=false"
require_text "$SSOT" "would_install_hook=false"
require_text "$SSOT" "would_replace_process_allocator=false"
require_text "$SSOT" "would_activate=false"
require_text "$RUNTIME_OWNER" "AllocatorProviderActivationDecisionReport"
require_text "$RUNTIME_OWNER" "activation_decision_allowed: false"
require_text "$RUNTIME_OWNER" "DIAGNOSTIC_INACTIVE_ACTIONS"
require_text "$RUNTIME_OWNER" "inactive.would_select_provider"
require_text "$RUNTIME_OWNER" "inactive.would_consume_proof"
require_text "$RUNTIME_OWNER" "inactive.would_prepare_rollback"
require_text "$RUNTIME_OWNER" "inactive.would_open_activation_gate"
require_text "$RUNTIME_OWNER" "inactive.would_install_hook"
require_text "$RUNTIME_OWNER" "inactive.would_replace_process_allocator"
require_text "$RUNTIME_OWNER" "inactive.would_activate"
require_text "$INACTIVE_SOURCE" "DIAGNOSTIC_INACTIVE_ACTIONS"
require_text "$INACTIVE_SOURCE" "would_select_provider: false"
require_text "$INACTIVE_SOURCE" "would_consume_proof: false"
require_text "$INACTIVE_SOURCE" "would_prepare_rollback: false"
require_text "$INACTIVE_SOURCE" "would_open_activation_gate: false"
require_text "$INACTIVE_SOURCE" "would_install_hook: false"
require_text "$INACTIVE_SOURCE" "would_replace_process_allocator: false"
require_text "$INACTIVE_SOURCE" "would_activate: false"
require_text "$CLI_FILE" "maybe_run_allocator_provider_activation_decision_diagnostic"
require_text "$CLI_FILE" "allocator-provider/activation-decision-cli-read-error"
require_text "$CARD" "293x-144 M91 Allocator Provider Activation Decision Closeout Inventory"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_activation_decision_closeout_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_activation_decision_closeout_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_activation_decision_closeout_guard.sh"

if rg -n 'std::env|set_var|var_os|env_bool|env_string|NYASH_ALLOCATOR_PROVIDER|HAKO_ALLOCATOR_PROVIDER|ALLOCATOR_PROVIDER_' "$CLI_FILE" >/tmp/"$TAG".env 2>&1; then
  cat /tmp/"$TAG".env >&2
  rm -f /tmp/"$TAG".env
  fail "activation decision closeout must not add hidden environment toggles"
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
  fail "runner must not own allocator provider activation decision closeout behavior"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
