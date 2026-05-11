#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-proof-bundle-consumption-cli-surface"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

CLI_FILE="src/cli/allocator_provider_proof_bundle_consumption.rs"
CLI_ARGS="src/cli/args.rs"
CLI_MOD="src/cli/mod.rs"
CLI_DIAGNOSTIC_OUTPUT="src/cli/diagnostic_output.rs"
MAIN_FILE="src/main.rs"
RUNTIME_FILE="src/runtime/allocator_provider_registry.rs"
SSOT="docs/development/current/main/design/allocator-provider-proof-bundle-consumption-cli-surface-ssot.md"
REPORT_SSOT="docs/development/current/main/design/allocator-provider-proof-bundle-consumption-diagnostic-report-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-proof-bundle-consumption-v0.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-155-M99-ALLOCATOR-PROVIDER-PROOF-BUNDLE-CONSUMPTION-CLI-SURFACE.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
M98_GUARD="tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_diagnostic_report_guard.sh"

echo "[$TAG] checking M99 allocator provider proof bundle consumption CLI surface"

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
require_file "$M98_GUARD"

require_text "$CLI_ARGS" "allocator-provider-proof-bundle-consumption"
require_text "$CLI_MOD" "maybe_run_allocator_provider_proof_bundle_consumption_diagnostic"
require_text "$CLI_MOD" "CLI_ALLOCATOR_DIAGNOSTIC_CONFLICT"
require_text "$CLI_MOD" "maybe_reject_allocator_diagnostic_conflicts"
require_text "$CLI_MOD" "allocator_provider_proof_bundle_consumption"
require_text "$MAIN_FILE" "maybe_run_allocator_provider_proof_bundle_consumption_diagnostic"
require_text "$MAIN_FILE" "maybe_reject_allocator_diagnostic_conflicts"
require_text "$CLI_FILE" "maybe_run_allocator_provider_proof_bundle_consumption_diagnostic"
require_text "$CLI_FILE" "build_allocator_provider_proof_bundle_consumption_output"
require_text "$CLI_FILE" "validate_allocator_provider_proof_bundle_consumption_from_text"
require_text "$CLI_FILE" "read_labeled_file"
require_text "$CLI_FILE" "[allocator-provider/proof-bundle-consumption-cli-read-error]"
require_text "$CLI_FILE" "proof_bundle_consumption_status"
require_text "$CLI_FILE" "ready_inactive"
require_text "$CLI_FILE" "parse_error"
require_text "$CLI_FILE" "missing_facts"
require_text "$CLI_FILE" "missing_diagnostics"
require_text "$CLI_FILE" "requested_provider_id"
require_text "$CLI_FILE" "selected_provider_id"
require_text "$CLI_FILE" "selected_provider_id_absent"
require_text "$CLI_FILE" "requested_operations"
require_text "$CLI_FILE" "candidate_provider_ids"
require_text "$CLI_FILE" "provider_proof_ids"
require_text "$CLI_FILE" "provider_proof_count"
require_text "$CLI_FILE" "proof_bundle_consumed"
require_text "$CLI_FILE" "active_registry_built"
require_text "$CLI_FILE" "would_build_registry"
require_text "$CLI_FILE" "would_select_provider"
require_text "$CLI_FILE" "would_consume_proof_bundle"
require_text "$CLI_FILE" "would_prepare_rollback"
require_text "$CLI_FILE" "would_open_activation_gate"
require_text "$CLI_FILE" "would_install_hook"
require_text "$CLI_FILE" "would_replace_process_allocator"
require_text "$CLI_FILE" "would_activate"
require_text "$CLI_FILE" "one_line_option_text"
require_text "$RUNTIME_FILE" "AllocatorProviderProofBundleConsumptionReport"
require_text "$RUNTIME_FILE" "validate_allocator_provider_proof_bundle_consumption_from_text"
require_text "$CLI_MOD" "mod diagnostic_output"
require_text "$CLI_DIAGNOSTIC_OUTPUT" "std::fs::read_to_string"
require_text "$CLI_DIAGNOSTIC_OUTPUT" "finish_result"
require_text "$REPORT_SSOT" "A later row may expose this report through an"
require_text "$SSOT" "Allocator Provider Proof Bundle Consumption CLI Surface (SSOT)"
require_text "$SSOT" "hakorune --allocator-provider-proof-bundle-consumption <PROOF_BUNDLE_CONSUMPTION_TOML>"
require_text "$SSOT" "[allocator-provider/proof-bundle-consumption-cli-read-error]"
require_text "$SSOT" "[allocator-diagnostic/cli-conflicting-modes]"
require_text "$SSOT" "proof_bundle_consumption_status=ready_inactive"
require_text "$SSOT" "requested_provider_id=native_mimalloc"
require_text "$SSOT" "selected_provider_id=none_reserved"
require_text "$SSOT" "selected_provider_id_absent=true"
require_text "$SSOT" "requested_operations=alloc,realloc,free"
require_text "$SSOT" "candidate_provider_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator"
require_text "$SSOT" "provider_proof_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator"
require_text "$SSOT" "provider_proof_count=4"
require_text "$SSOT" "proof_bundle_consumed=false"
require_text "$SSOT" "active_registry_built=false"
require_text "$SSOT" "would_build_registry=false"
require_text "$SSOT" "would_select_provider=false"
require_text "$SSOT" "would_consume_proof_bundle=false"
require_text "$SSOT" "would_prepare_rollback=false"
require_text "$SSOT" "would_open_activation_gate=false"
require_text "$SSOT" "would_install_hook=false"
require_text "$SSOT" "would_replace_process_allocator=false"
require_text "$SSOT" "would_activate=false"
require_text "$CARD" "293x-155 M99 Allocator Provider Proof Bundle Consumption CLI Surface"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_cli_surface_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_cli_surface_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_cli_surface_guard.sh"

if rg -n 'latest_card[[:space:]]*=|latest_card_path[[:space:]]*=' "$M98_GUARD" >/tmp/"$TAG".m98_guard_pins 2>&1; then
  cat /tmp/"$TAG".m98_guard_pins >&2
  rm -f /tmp/"$TAG".m98_guard_pins
  fail "M98 guard must not pin CURRENT_STATE latest-card pointers after M99"
fi
rm -f /tmp/"$TAG".m98_guard_pins

cargo test -q allocator_provider_proof_bundle_consumption -- --nocapture

cli_output="$(cargo run -q --bin hakorune -- --allocator-provider-proof-bundle-consumption "$FIXTURE")"
require_output_text "$cli_output" "diagnostic=[allocator-provider/proof-bundle-consumption-inactive]"
require_output_text "$cli_output" "proof_bundle_consumption_status=ready_inactive"
require_output_text "$cli_output" "parse_error="
require_output_text "$cli_output" "missing_facts="
require_output_text "$cli_output" "missing_diagnostics="
require_output_text "$cli_output" "requested_provider_id=native_mimalloc"
require_output_text "$cli_output" "selected_provider_id=none_reserved"
require_output_text "$cli_output" "selected_provider_id_absent=true"
require_output_text "$cli_output" "requested_operations=alloc,realloc,free"
require_output_text "$cli_output" "candidate_provider_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator"
require_output_text "$cli_output" "provider_proof_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator"
require_output_text "$cli_output" "provider_proof_count=4"
require_output_text "$cli_output" "proof_bundle_consumed=false"
require_output_text "$cli_output" "active_registry_built=false"
require_output_text "$cli_output" "would_build_registry=false"
require_output_text "$cli_output" "would_select_provider=false"
require_output_text "$cli_output" "would_consume_proof_bundle=false"
require_output_text "$cli_output" "would_prepare_rollback=false"
require_output_text "$cli_output" "would_open_activation_gate=false"
require_output_text "$cli_output" "would_install_hook=false"
require_output_text "$cli_output" "would_replace_process_allocator=false"
require_output_text "$cli_output" "would_activate=false"

set +e
conflict_output="$(cargo run -q --bin hakorune -- --allocator-provider-proof-bundle-consumption /tmp/proof-bundle.toml --allocator-provider-selection-decision /tmp/selection.toml 2>&1)"
conflict_status=$?
set -e
[[ "$conflict_status" -eq 2 ]] || fail "conflicting allocator diagnostic CLI modes must exit 2"
require_output_text "$conflict_output" "[allocator-diagnostic/cli-conflicting-modes]"
require_output_text "$conflict_output" "allocator_provider_proof_bundle_consumption"
require_output_text "$conflict_output" "allocator_provider_selection_decision"

if rg -n 'std::env|set_var|var_os|env_bool|env_string|NYASH_ALLOCATOR_PROVIDER|HAKO_ALLOCATOR_PROVIDER|ALLOCATOR_PROVIDER_' "$CLI_FILE" >/tmp/"$TAG".env 2>&1; then
  cat /tmp/"$TAG".env >&2
  rm -f /tmp/"$TAG".env
  fail "proof bundle consumption CLI surface must not add hidden environment toggles"
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
  fail "runner must not own allocator provider proof bundle diagnostics"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
