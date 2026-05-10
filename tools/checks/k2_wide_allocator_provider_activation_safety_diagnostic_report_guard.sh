#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-safety-diagnostic-report"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-report-ssot.md"
OWNER_SSOT="docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-owner-ssot.md"
GATE_SSOT="docs/development/current/main/design/allocator-provider-activation-safety-gate-ssot.md"
GATE_FIXTURE="docs/development/current/main/design/allocator-provider-activation-safety-gate-v0.toml"
SOURCE="src/runtime/allocator_provider_registry.rs"
RUNTIME_MOD="src/runtime/mod.rs"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-135-M83-ALLOCATOR-PROVIDER-ACTIVATION-SAFETY-DIAGNOSTIC-REPORT.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
M82_GUARD="tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_owner_guard.sh"

echo "[$TAG] checking M83 allocator provider activation safety diagnostic report"

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
require_file "$OWNER_SSOT"
require_file "$GATE_SSOT"
require_file "$GATE_FIXTURE"
require_file "$SOURCE"
require_file "$RUNTIME_MOD"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"
require_file "$M82_GUARD"

require_text "$SSOT" "Allocator Provider Activation Safety Diagnostic Report (SSOT)"
require_text "$SSOT" "validate_allocator_provider_activation_safety_gate_from_text"
require_text "$OWNER_SSOT" "src/runtime/allocator_provider_registry.rs"
require_text "$GATE_SSOT" "activation_safety_gate = \"inactive\""
require_text "$GATE_FIXTURE" 'activation_gate_open = false'
require_text "$SOURCE" "Diagnostic-only allocator provider registry and activation safety reports"
require_text "$SOURCE" "AllocatorProviderActivationSafetyFacts"
require_text "$SOURCE" "AllocatorProviderActivationSafetyReport"
require_text "$SOURCE" "AllocatorProviderActivationSafetyStatus"
require_text "$SOURCE" "validate_allocator_provider_activation_safety_gate("
require_text "$SOURCE" "validate_allocator_provider_activation_safety_gate_from_text"
require_text "$SOURCE" "DIAG_PROVIDER_ACTIVATION_SAFETY_BLOCKED"
require_text "$SOURCE" "activation_gate_open: false"
require_text "$SOURCE" "would_open_activation_gate: false"
require_text "$SOURCE" "would_activate_hook: false"
require_text "$SOURCE" "would_activate: false"
require_text "$SOURCE" "allocator-provider-activation-safety-gate-v0.toml"
require_text "$RUNTIME_MOD" "pub mod allocator_provider_registry;"
require_text "$TASK_BREAKDOWN" "M83 | activation safety diagnostic report"
require_text "$TASK_BREAKDOWN" "The next safe row is M84 activation safety diagnostic CLI surface."
require_text "$TASKBOARD" '| `M83 allocator provider activation safety diagnostic report` | `live-narrow` |'
require_text "$TASKBOARD" '106. `M83 allocator provider activation safety diagnostic report`'
require_text "$CARD" "293x-135 M83 Allocator Provider Activation Safety Diagnostic Report"
require_text "$PHASE_README" '`293x-135`'
require_text "$REAL_APP_TASKBOARD" '[x] `293x-135` M83 allocator provider activation safety diagnostic report'
require_text "$CURRENT_STATE" 'latest_card = "293x-135-M83-ALLOCATOR-PROVIDER-ACTIVATION-SAFETY-DIAGNOSTIC-REPORT"'
require_text "$CURRENT_STATE" 'latest_card_path = "docs/development/current/main/phases/phase-293x/293x-135-M83-ALLOCATOR-PROVIDER-ACTIVATION-SAFETY-DIAGNOSTIC-REPORT.md"'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh"

cargo test -q activation_safety -- --nocapture

if rg -n '(^|[^A-Za-z0-9_])open_activation_gate([^A-Za-z0-9_]|$)' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".gate_open 2>&1; then
  cat /tmp/"$TAG".gate_open >&2
  rm -f /tmp/"$TAG".gate_open
  fail "activation gate opening must stay absent in M83"
fi
rm -f /tmp/"$TAG".gate_open

if rg -n '(^|[^A-Za-z0-9_])select_allocator_provider([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])allocator_provider_select([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])allocator_provider_selection_env([^A-Za-z0-9_]|$)|NYASH_ALLOCATOR_PROVIDER' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".provider_selection 2>&1; then
  cat /tmp/"$TAG".provider_selection >&2
  rm -f /tmp/"$TAG".provider_selection
  fail "provider selection implementation/env toggle must stay absent in M83"
fi
rm -f /tmp/"$TAG".provider_selection

if rg -n 'consume_allocator_provider_proof|allocator_provider_proof_bundle_consume|consume_allocator_provider_proof_bundle|consume_provider_proof_bundle' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".proof_consumption 2>&1; then
  cat /tmp/"$TAG".proof_consumption >&2
  rm -f /tmp/"$TAG".proof_consumption
  fail "proof consumption implementation must stay absent in M83"
fi
rm -f /tmp/"$TAG".proof_consumption

if rg -n 'prepare_rollback' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".rollback 2>&1; then
  cat /tmp/"$TAG".rollback >&2
  rm -f /tmp/"$TAG".rollback
  fail "rollback preparation implementation must stay absent in M83"
fi
rm -f /tmp/"$TAG".rollback

if rg -n 'allocator_hook_activate|activate_allocator|install_allocator_hook|replace_allocator' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".hook_activation 2>&1; then
  cat /tmp/"$TAG".hook_activation >&2
  rm -f /tmp/"$TAG".hook_activation
  fail "hook activation/process allocator replacement must stay absent in M83"
fi
rm -f /tmp/"$TAG".hook_activation

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M83"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider activation safety diagnostics"
fi
rm -f /tmp/"$TAG".runner

if rg -n 'HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan|allocator_hook_activate|activate_allocator|debug_guarded_allocator|hako_model_allocator|native_mimalloc|native_system_malloc' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  fail "allocator provider/hook/facade/policy matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc

echo "[$TAG] ok"
