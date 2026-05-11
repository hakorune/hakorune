#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-diagnostic-inactive-actions"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-diagnostic-inactive-actions-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-147-M93B-ALLOCATOR-PROVIDER-DIAGNOSTIC-INACTIVE-ACTIONS.md"
INACTIVE_SOURCE="src/runtime/allocator_provider_diagnostic_inactive.rs"
REGISTRY_SOURCE="src/runtime/allocator_provider_registry.rs"
REGISTRY_SNAPSHOT_SOURCE="src/runtime/allocator_provider_registry_snapshot.rs"
ACTIVATION_SAFETY_SOURCE="src/runtime/allocator_provider_activation_safety.rs"
DECISION_SOURCE="src/runtime/allocator_provider_activation_decision.rs"
RUNTIME_MOD="src/runtime/mod.rs"
M83_GUARD="tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh"
M85_GUARD="tools/checks/k2_wide_allocator_provider_activation_safety_closeout_guard.sh"
M89_GUARD="tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_report_guard.sh"
M91_GUARD="tools/checks/k2_wide_allocator_provider_activation_decision_closeout_guard.sh"
M93_GUARD="tools/checks/k2_wide_allocator_provider_registry_snapshot_diagnostic_report_guard.sh"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M93B allocator provider diagnostic inactive actions"

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
require_file "$INACTIVE_SOURCE"
require_file "$REGISTRY_SOURCE"
require_file "$REGISTRY_SNAPSHOT_SOURCE"
require_file "$ACTIVATION_SAFETY_SOURCE"
require_file "$DECISION_SOURCE"
require_file "$RUNTIME_MOD"
require_file "$M83_GUARD"
require_file "$M85_GUARD"
require_file "$M89_GUARD"
require_file "$M91_GUARD"
require_file "$M93_GUARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Diagnostic Inactive Actions (SSOT)"
require_text "$SSOT" "src/runtime/allocator_provider_diagnostic_inactive.rs"
require_text "$SSOT" "DIAGNOSTIC_INACTIVE_ACTIONS"
require_text "$SSOT" "REGISTRY_SNAPSHOT_INACTIVE_ACTIONS"
require_text "$SSOT" "SAFETY_GATE_INACTIVE_ACTIONS"
require_text "$SSOT" "M93B guard deliberately does not add a new latest-card pin"
require_text "$CARD" "293x-147 M93B Allocator Provider Diagnostic Inactive Actions"

require_text "$INACTIVE_SOURCE" "Shared inactive output shapes for allocator provider diagnostics"
require_text "$INACTIVE_SOURCE" "pub(crate) const DIAGNOSTIC_INACTIVE_ACTIONS"
require_text "$INACTIVE_SOURCE" "pub(crate) const REGISTRY_SNAPSHOT_INACTIVE_ACTIONS"
require_text "$INACTIVE_SOURCE" "pub(crate) const SAFETY_GATE_INACTIVE_ACTIONS"
require_text "$INACTIVE_SOURCE" "would_select_provider: false"
require_text "$INACTIVE_SOURCE" "would_consume_proof: false"
require_text "$INACTIVE_SOURCE" "would_prepare_rollback: false"
require_text "$INACTIVE_SOURCE" "would_open_activation_gate: false"
require_text "$INACTIVE_SOURCE" "would_install_hook: false"
require_text "$INACTIVE_SOURCE" "would_replace_process_allocator: false"
require_text "$INACTIVE_SOURCE" "active_registry_built: false"
require_text "$INACTIVE_SOURCE" "would_build_registry: false"
require_text "$INACTIVE_SOURCE" "would_activate_hook: false"
require_text "$INACTIVE_SOURCE" "allocator_provider_inactive_actions_are_all_false"

require_text "$RUNTIME_MOD" "pub(crate) mod allocator_provider_diagnostic_inactive;"
require_text "$REGISTRY_SNAPSHOT_SOURCE" "REGISTRY_SNAPSHOT_INACTIVE_ACTIONS"
require_text "$REGISTRY_SNAPSHOT_SOURCE" "diagnostic_actions.would_select_provider"
require_text "$REGISTRY_SNAPSHOT_SOURCE" "inactive.active_registry_built"
require_text "$ACTIVATION_SAFETY_SOURCE" "SAFETY_GATE_INACTIVE_ACTIONS"
require_text "$ACTIVATION_SAFETY_SOURCE" "inactive.activation_gate_open"
require_text "$DECISION_SOURCE" "DIAGNOSTIC_INACTIVE_ACTIONS"
require_text "$DECISION_SOURCE" "inactive.would_select_provider"
require_text "$DECISION_SOURCE" "inactive.would_prepare_rollback"

if rg -n 'CURRENT_STATE|latest_card|latest_card_path' "$M93_GUARD" >/tmp/"$TAG".m93_pin 2>&1; then
  cat /tmp/"$TAG".m93_pin >&2
  rm -f /tmp/"$TAG".m93_pin
  fail "M93 guard must not pin current-state latest-card pointers after M93B"
fi
rm -f /tmp/"$TAG".m93_pin

require_text "$M83_GUARD" "$INACTIVE_SOURCE"
require_text "$M85_GUARD" "$INACTIVE_SOURCE"
require_text "$M89_GUARD" "$INACTIVE_SOURCE"
require_text "$M91_GUARD" "$INACTIVE_SOURCE"
require_text "$M93_GUARD" "$INACTIVE_SOURCE"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_diagnostic_inactive_actions_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_diagnostic_inactive_actions_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_diagnostic_inactive_actions_guard.sh"

cargo test -q allocator_provider_inactive -- --nocapture
cargo test -q activation_safety -- --nocapture
cargo test -q activation_decision -- --nocapture
cargo test -q registry_snapshot -- --nocapture

allocator_provider_forbid_activation_gate_open "$TAG"
allocator_provider_forbid_selection "$TAG"
allocator_provider_forbid_proof_consumption "$TAG"
allocator_provider_forbid_rollback_preparation "$TAG"
allocator_provider_forbid_hook_activation "$TAG"
allocator_provider_forbid_global_allocator "$TAG"
allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
