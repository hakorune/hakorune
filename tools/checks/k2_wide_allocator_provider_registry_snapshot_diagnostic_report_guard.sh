#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-registry-snapshot-diagnostic-report"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-registry-snapshot-diagnostic-report-ssot.md"
SHAPE_SSOT="docs/development/current/main/design/allocator-provider-registry-snapshot-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml"
SOURCE="src/runtime/allocator_provider_registry.rs"
INACTIVE_SOURCE="src/runtime/allocator_provider_diagnostic_inactive.rs"
RUNTIME_MOD="src/runtime/mod.rs"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-146-M93-ALLOCATOR-PROVIDER-REGISTRY-SNAPSHOT-DIAGNOSTIC-REPORT.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M93 allocator provider registry snapshot diagnostic report"

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

require_text "$SSOT" "Allocator Provider Registry Snapshot Diagnostic Report (SSOT)"
require_text "$SSOT" "validate_allocator_provider_registry_snapshot_from_text"
require_text "$SSOT" "ReadyInactive"
require_text "$SSOT" "active_registry_built = false"
require_text "$SSOT" "would_build_registry = false"
require_text "$SSOT" "would_select_provider = false"
require_text "$SSOT" "would_consume_proof = false"
require_text "$SSOT" "would_prepare_rollback = false"
require_text "$SSOT" "would_open_activation_gate = false"
require_text "$SSOT" "would_install_hook = false"
require_text "$SSOT" "would_replace_process_allocator = false"
require_text "$SSOT" "would_activate = false"
require_text "$SSOT" "M94 may expose this report through an explicit CLI diagnostic surface"
require_text "$SHAPE_SSOT" "Allocator Provider Registry Snapshot (SSOT)"
require_text "$FIXTURE" 'schema_version = "allocator_provider_registry_snapshot_v0"'
require_text "$FIXTURE" 'would_build_registry = false'
require_text "$SOURCE" "Diagnostic-only allocator provider registry and activation safety reports"
require_text "$SOURCE" "AllocatorProviderRegistrySnapshotFacts"
require_text "$SOURCE" "AllocatorProviderRegistrySnapshotReport"
require_text "$SOURCE" "AllocatorProviderRegistrySnapshotStatus"
require_text "$SOURCE" "validate_allocator_provider_registry_snapshot("
require_text "$SOURCE" "validate_allocator_provider_registry_snapshot_from_text"
require_text "$SOURCE" "parse_error: Option<String>"
require_text "$SOURCE" "registry_snapshot_fact_checks"
require_text "$SOURCE" "registry_snapshot_malformed_text_reports_parse_error_without_building_registry"
require_text "$SOURCE" "DIAG_PROVIDER_REGISTRY_SNAPSHOT_INACTIVE"
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
require_text "$INACTIVE_SOURCE" "active_registry_built: false"
require_text "$INACTIVE_SOURCE" "would_build_registry: false"
require_text "$INACTIVE_SOURCE" "would_select_provider: false"
require_text "$INACTIVE_SOURCE" "would_consume_proof: false"
require_text "$INACTIVE_SOURCE" "would_prepare_rollback: false"
require_text "$INACTIVE_SOURCE" "would_open_activation_gate: false"
require_text "$INACTIVE_SOURCE" "would_install_hook: false"
require_text "$INACTIVE_SOURCE" "would_replace_process_allocator: false"
require_text "$INACTIVE_SOURCE" "would_activate: false"
require_text "$SOURCE" "allocator-provider-registry-snapshot-v0.toml"
require_text "$RUNTIME_MOD" "pub mod allocator_provider_registry;"
require_text "$TASK_BREAKDOWN" "M93 | registry snapshot diagnostic report"
require_text "$TASK_BREAKDOWN" "M94 registry snapshot CLI surface"
require_text "$CARD" "293x-146 M93 Allocator Provider Registry Snapshot Diagnostic Report"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_registry_snapshot_diagnostic_report_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_registry_snapshot_diagnostic_report_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_registry_snapshot_diagnostic_report_guard.sh"

cargo test -q registry_snapshot -- --nocapture

allocator_provider_forbid_activation_gate_open "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_proof_consumption "$TAG"

allocator_provider_forbid_rollback_preparation "$TAG"

allocator_provider_forbid_hook_activation "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider registry snapshot diagnostics"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
