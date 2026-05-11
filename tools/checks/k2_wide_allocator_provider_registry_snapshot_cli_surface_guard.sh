#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-registry-snapshot-cli-surface"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

CLI_FILE="src/cli/allocator_provider_registry_snapshot.rs"
CLI_ARGS="src/cli/args.rs"
CLI_MOD="src/cli/mod.rs"
CLI_DIAGNOSTIC_OUTPUT="src/cli/diagnostic_output.rs"
MAIN_FILE="src/main.rs"
RUNTIME_FILE="src/runtime/allocator_provider_registry.rs"
SSOT="docs/development/current/main/design/allocator-provider-registry-snapshot-cli-surface-ssot.md"
REPORT_SSOT="docs/development/current/main/design/allocator-provider-registry-snapshot-diagnostic-report-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-148-M94-ALLOCATOR-PROVIDER-REGISTRY-SNAPSHOT-CLI-SURFACE.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M94 allocator provider registry snapshot CLI surface"

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

require_output_text() {
  local output="$1"
  local needle="$2"
  [[ "$output" == *"$needle"* ]] || fail "missing CLI output text: $needle"
}

require_file "$CLI_FILE"
require_file "$CLI_ARGS"
require_file "$CLI_MOD"
require_file "$CLI_DIAGNOSTIC_OUTPUT"
require_file "$MAIN_FILE"
require_file "$RUNTIME_FILE"
require_file "$SSOT"
require_file "$REPORT_SSOT"
require_file "$FIXTURE"
require_file "$CARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$CLI_ARGS" "allocator-provider-registry-snapshot"
require_text "$CLI_MOD" "maybe_run_allocator_provider_registry_snapshot_diagnostic"
require_text "$CLI_MOD" "CLI_ALLOCATOR_DIAGNOSTIC_CONFLICT"
require_text "$MAIN_FILE" "maybe_run_allocator_provider_registry_snapshot_diagnostic"
require_text "$MAIN_FILE" "maybe_reject_allocator_diagnostic_conflicts"
require_text "$CLI_FILE" "maybe_run_allocator_provider_registry_snapshot_diagnostic"
require_text "$CLI_FILE" "build_allocator_provider_registry_snapshot_output"
require_text "$CLI_FILE" "validate_allocator_provider_registry_snapshot_from_text"
require_text "$CLI_FILE" "read_labeled_file"
require_text "$CLI_FILE" "registry_snapshot_status"
require_text "$CLI_FILE" "ready_inactive"
require_text "$CLI_FILE" "parse_error"
require_text "$CLI_FILE" "missing_facts"
require_text "$CLI_FILE" "missing_diagnostics"
require_text "$CLI_FILE" "provider_ids"
require_text "$CLI_FILE" "provider_count"
require_text "$CLI_FILE" "active_registry_built"
require_text "$CLI_FILE" "would_build_registry"
require_text "$CLI_FILE" "would_select_provider"
require_text "$CLI_FILE" "would_consume_proof"
require_text "$CLI_FILE" "would_prepare_rollback"
require_text "$CLI_FILE" "would_open_activation_gate"
require_text "$CLI_FILE" "would_install_hook"
require_text "$CLI_FILE" "would_replace_process_allocator"
require_text "$CLI_FILE" "would_activate"
require_text "$CLI_FILE" "one_line_option_text"
require_text "$RUNTIME_FILE" "AllocatorProviderRegistrySnapshotReport"
require_text "$RUNTIME_FILE" "validate_allocator_provider_registry_snapshot_from_text"
require_text "$CLI_MOD" "mod diagnostic_output"
require_text "$CLI_DIAGNOSTIC_OUTPUT" "std::fs::read_to_string"
require_text "$CLI_DIAGNOSTIC_OUTPUT" "finish_result"
require_text "$REPORT_SSOT" "M94 may expose this report through an explicit CLI diagnostic surface"
require_text "$SSOT" "Allocator Provider Registry Snapshot CLI Surface (SSOT)"
require_text "$SSOT" "hakorune --allocator-provider-registry-snapshot <REGISTRY_SNAPSHOT_TOML>"
require_text "$SSOT" "[allocator-diagnostic/cli-conflicting-modes]"
require_text "$SSOT" "registry_snapshot_status=ready_inactive"
require_text "$SSOT" "provider_count=4"
require_text "$SSOT" "active_registry_built=false"
require_text "$SSOT" "would_build_registry=false"
require_text "$SSOT" "would_select_provider=false"
require_text "$SSOT" "would_consume_proof=false"
require_text "$SSOT" "would_prepare_rollback=false"
require_text "$SSOT" "would_open_activation_gate=false"
require_text "$SSOT" "would_install_hook=false"
require_text "$SSOT" "would_replace_process_allocator=false"
require_text "$SSOT" "would_activate=false"
require_text "$CARD" "293x-148 M94 Allocator Provider Registry Snapshot CLI Surface"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_registry_snapshot_cli_surface_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_registry_snapshot_cli_surface_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_registry_snapshot_cli_surface_guard.sh"

cargo test -q allocator_provider_registry_snapshot -- --nocapture

cli_output="$(cargo run -q --bin hakorune -- --allocator-provider-registry-snapshot "$FIXTURE")"
require_output_text "$cli_output" "diagnostic=[allocator-provider/registry-snapshot-inactive]"
require_output_text "$cli_output" "registry_snapshot_status=ready_inactive"
require_output_text "$cli_output" "parse_error="
require_output_text "$cli_output" "missing_facts="
require_output_text "$cli_output" "missing_diagnostics="
require_output_text "$cli_output" "provider_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator"
require_output_text "$cli_output" "provider_count=4"
require_output_text "$cli_output" "active_registry_built=false"
require_output_text "$cli_output" "would_build_registry=false"
require_output_text "$cli_output" "would_select_provider=false"
require_output_text "$cli_output" "would_consume_proof=false"
require_output_text "$cli_output" "would_prepare_rollback=false"
require_output_text "$cli_output" "would_open_activation_gate=false"
require_output_text "$cli_output" "would_install_hook=false"
require_output_text "$cli_output" "would_replace_process_allocator=false"
require_output_text "$cli_output" "would_activate=false"

set +e
conflict_output="$(cargo run -q --bin hakorune -- --allocator-provider-registry-snapshot /tmp/registry.toml --allocator-provider-activation-decision /tmp/decision.toml 2>&1)"
conflict_status=$?
set -e
[[ "$conflict_status" -eq 2 ]] || fail "conflicting allocator diagnostic CLI modes must exit 2"
require_output_text "$conflict_output" "[allocator-diagnostic/cli-conflicting-modes]"
require_output_text "$conflict_output" "allocator_provider_registry_snapshot"
require_output_text "$conflict_output" "allocator_provider_activation_decision"

if rg -n 'std::env|set_var|var_os|env_bool|env_string|NYASH_ALLOCATOR_PROVIDER|HAKO_ALLOCATOR_PROVIDER|ALLOCATOR_PROVIDER_' "$CLI_FILE" >/tmp/"$TAG".env 2>&1; then
  cat /tmp/"$TAG".env >&2
  rm -f /tmp/"$TAG".env
  fail "registry snapshot CLI surface must not add hidden environment toggles"
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
  fail "runner must not own allocator provider registry snapshot CLI behavior"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
