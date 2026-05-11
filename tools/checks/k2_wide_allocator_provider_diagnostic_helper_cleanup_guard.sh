#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-diagnostic-helper-cleanup"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

HELPER="src/runtime/allocator_provider_toml_helpers.rs"
RUNTIME_MOD="src/runtime/mod.rs"
REGISTRY="src/runtime/allocator_provider_registry.rs"
REGISTRY_SNAPSHOT="src/runtime/allocator_provider_registry_snapshot.rs"
SELECTION_DECISION="src/runtime/allocator_provider_selection_decision.rs"
PROOF_BUNDLE_CONSUMPTION="src/runtime/allocator_provider_proof_bundle_consumption.rs"
ACTIVATION_SAFETY="src/runtime/allocator_provider_activation_safety.rs"
ACTIVATION_DECISION="src/runtime/allocator_provider_activation_decision.rs"
SSOT="docs/development/current/main/design/allocator-provider-diagnostic-helper-cleanup-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-152-M97B-ALLOCATOR-PROVIDER-DIAGNOSTIC-HELPER-CLEANUP.md"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M97B allocator provider diagnostic helper cleanup"

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

require_file "$HELPER"
require_file "$RUNTIME_MOD"
require_file "$REGISTRY"
require_file "$REGISTRY_SNAPSHOT"
require_file "$SELECTION_DECISION"
require_file "$PROOF_BUNDLE_CONSUMPTION"
require_file "$ACTIVATION_SAFETY"
require_file "$ACTIVATION_DECISION"
require_file "$SSOT"
require_file "$CARD"
require_file "$TASK_BREAKDOWN"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$HELPER" "DiagnosticFactCheck"
require_text "$HELPER" "text_field_matches"
require_text "$HELPER" "bool_field_false"
require_text "$HELPER" "nonempty_text_field"
require_text "$HELPER" "string_list_contains_all"
require_text "$HELPER" "allocator_provider_toml_helpers_match_text_and_false_bool_fields"
require_text "$RUNTIME_MOD" "pub(crate) mod allocator_provider_toml_helpers;"
require_text "$REGISTRY_SNAPSHOT" "allocator_provider_toml_helpers"
require_text "$REGISTRY_SNAPSHOT" "DiagnosticFactCheck"
require_text "$SELECTION_DECISION" "Vec<DiagnosticFactCheck>"
require_text "$PROOF_BUNDLE_CONSUMPTION" "Vec<DiagnosticFactCheck>"
require_text "$ACTIVATION_SAFETY" "[DiagnosticFactCheck; 24]"
require_text "$ACTIVATION_DECISION" "allocator_provider_toml_helpers"
require_text "$ACTIVATION_DECISION" "Vec<DiagnosticFactCheck>"
require_text "$SSOT" "Allocator Provider Diagnostic Helper Cleanup (SSOT)"
require_text "$SSOT" "src/runtime/allocator_provider_toml_helpers.rs"
require_text "$SSOT" "DiagnosticFactCheck"
require_text "$SSOT" "RegistrySnapshotFactCheck"
require_text "$SSOT" "ActivationDecisionFactCheck"
require_text "$CARD" "293x-152 M97B Allocator Provider Diagnostic Helper Cleanup"
require_text "$TASK_BREAKDOWN" "M97B | diagnostic helper cleanup"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_diagnostic_helper_cleanup_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_diagnostic_helper_cleanup_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_diagnostic_helper_cleanup_guard.sh"

if rg -n '^fn (text_field_matches|bool_field_false|nonempty_text_field|string_list_contains_all)\b' \
  "$REGISTRY" "$REGISTRY_SNAPSHOT" "$SELECTION_DECISION" "$PROOF_BUNDLE_CONSUMPTION" \
  "$ACTIVATION_SAFETY" "$ACTIVATION_DECISION" >/tmp/"$TAG".local_helpers 2>&1; then
  cat /tmp/"$TAG".local_helpers >&2
  rm -f /tmp/"$TAG".local_helpers
  fail "allocator provider diagnostic TOML helpers must stay in $HELPER"
fi
rm -f /tmp/"$TAG".local_helpers

if rg -n '^struct (RegistrySnapshotFactCheck|SelectionDecisionFactCheck|ActivationSafetyFactCheck|ActivationDecisionFactCheck)\b' \
  "$REGISTRY" "$REGISTRY_SNAPSHOT" "$SELECTION_DECISION" "$PROOF_BUNDLE_CONSUMPTION" \
  "$ACTIVATION_SAFETY" "$ACTIVATION_DECISION" >/tmp/"$TAG".fact_checks 2>&1; then
  cat /tmp/"$TAG".fact_checks >&2
  rm -f /tmp/"$TAG".fact_checks
  fail "allocator provider diagnostic fact checks must use DiagnosticFactCheck"
fi
rm -f /tmp/"$TAG".fact_checks

cargo test -q allocator_provider_toml_helpers -- --nocapture
cargo test -q activation_decision -- --nocapture
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
  fail "runner must not own allocator provider diagnostic helper cleanup"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
