#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-safety-cli-surface"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

CLI_FILE="src/cli/allocator_provider_activation_safety.rs"
CLI_ARGS="src/cli/args.rs"
CLI_MOD="src/cli/mod.rs"
CLI_DIAGNOSTIC_OUTPUT="src/cli/diagnostic_output.rs"
MAIN_FILE="src/main.rs"
RUNTIME_FILE="src/runtime/allocator_provider_registry.rs"
SSOT="docs/development/current/main/design/allocator-provider-activation-safety-cli-surface-ssot.md"
REPORT_SSOT="docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-report-ssot.md"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-136-M84-ALLOCATOR-PROVIDER-ACTIVATION-SAFETY-CLI-SURFACE.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M84 allocator provider activation safety CLI surface"

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

require_file "$CLI_FILE"
require_file "$CLI_ARGS"
require_file "$CLI_MOD"
require_file "$CLI_DIAGNOSTIC_OUTPUT"
require_file "$MAIN_FILE"
require_file "$RUNTIME_FILE"
require_file "$SSOT"
require_file "$REPORT_SSOT"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$CLI_ARGS" "allocator-provider-activation-safety-gate"
require_text "$CLI_MOD" "maybe_run_allocator_provider_activation_safety_diagnostic"
require_text "$MAIN_FILE" "maybe_run_allocator_provider_activation_safety_diagnostic"
require_text "$CLI_FILE" "maybe_run_allocator_provider_activation_safety_diagnostic"
require_text "$CLI_FILE" "build_allocator_provider_activation_safety_output"
require_text "$CLI_FILE" "validate_allocator_provider_activation_safety_gate_from_text"
require_text "$CLI_FILE" "read_labeled_file"
require_text "$CLI_FILE" "activation_safety_status"
require_text "$CLI_FILE" "parse_error"
require_text "$CLI_FILE" "missing_facts"
require_text "$CLI_FILE" "missing_diagnostics"
require_text "$CLI_FILE" "rollback_target_provider_id"
require_text "$CLI_FILE" "activation_target_provider_id"
require_text "$CLI_FILE" "activation_gate_open"
require_text "$CLI_FILE" "would_open_activation_gate"
require_text "$CLI_FILE" "would_activate_hook"
require_text "$CLI_FILE" "would_activate=false"
require_text "$CLI_FILE" "ready_gate_closed"
require_text "$CLI_FILE" "one_line_option_text"
require_text "$RUNTIME_FILE" "AllocatorProviderActivationSafetyReport"
require_text "$RUNTIME_FILE" "validate_allocator_provider_activation_safety_gate_from_text"
require_text "$CLI_MOD" "mod diagnostic_output"
require_text "$CLI_DIAGNOSTIC_OUTPUT" "std::fs::read_to_string"
require_text "$CLI_DIAGNOSTIC_OUTPUT" "finish_result"
require_text "$REPORT_SSOT" "M84 may expose this report through an explicit CLI diagnostic surface."
require_text "$SSOT" "Allocator Provider Activation Safety CLI Surface (SSOT)"
require_text "$SSOT" "hakorune --allocator-provider-activation-safety-gate <ACTIVATION_SAFETY_GATE_TOML>"
require_text "$SSOT" "activation_safety_status=ready_gate_closed"
require_text "$SSOT" "parse_error"
require_text "$SSOT" "activation_gate_open=false"
require_text "$SSOT" "would_open_activation_gate=false"
require_text "$SSOT" "would_activate_hook=false"
require_text "$SSOT" "would_activate=false"
require_text "$TASK_BREAKDOWN" "M84 | activation safety diagnostic CLI surface"
require_text "$TASK_BREAKDOWN" "explicit CLI over caller-provided safety TOML path"
require_text "$TASKBOARD" '| `M84 allocator provider activation safety CLI surface` | `live-narrow` |'
require_text "$TASKBOARD" '107. `M84 allocator provider activation safety CLI surface`'
require_text "$CARD" "293x-136 M84 Allocator Provider Activation Safety CLI Surface"
require_text "$PHASE_README" '`293x-136`'
require_text "$REAL_APP_TASKBOARD" '[x] `293x-136` M84 allocator provider activation safety CLI surface'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_activation_safety_cli_surface_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_activation_safety_cli_surface_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_activation_safety_cli_surface_guard.sh"

cargo test -q allocator_provider_activation_safety -- --nocapture

if rg -n 'std::env|set_var|var_os|env_bool|env_string|NYASH_ALLOCATOR_PROVIDER' "$CLI_FILE" >/tmp/"$TAG".env 2>&1; then
  cat /tmp/"$TAG".env >&2
  rm -f /tmp/"$TAG".env
  fail "activation safety CLI surface must not add hidden environment toggles"
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
  fail "runner must not own allocator provider activation safety CLI behavior"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
