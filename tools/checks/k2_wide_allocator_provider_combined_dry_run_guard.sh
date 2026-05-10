#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-combined-dry-run"
cd "$ROOT_DIR"

RUNTIME_FILE="src/runtime/allocator_provider_manifest.rs"
CLI_FILE="src/cli/allocator_provider_manifest.rs"
CLI_ARGS="src/cli/args.rs"
CLI_MOD="src/cli/mod.rs"
MAIN_FILE="src/main.rs"
SSOT="docs/development/current/main/design/allocator-provider-combined-dry-run-ssot.md"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-122-M70-COMBINED-HOOK-PROVIDER-DRY-RUN-REPORT.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M70 combined hook/provider dry-run report"

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

require_file "$RUNTIME_FILE"
require_file "$CLI_FILE"
require_file "$CLI_ARGS"
require_file "$CLI_MOD"
require_file "$MAIN_FILE"
require_file "$SSOT"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$RUNTIME_FILE" "AllocatorProviderCombinedDryRunReport"
require_text "$RUNTIME_FILE" "AllocatorProviderCombinedDryRunStatus"
require_text "$RUNTIME_FILE" "validate_allocator_provider_combined_dry_run_from_manifest_texts"
require_text "$RUNTIME_FILE" "DIAG_PROVIDER_COMBINED_DRY_RUN_READY"
require_text "$RUNTIME_FILE" "DIAG_PROVIDER_COMBINED_DRY_RUN_MISSING"
require_text "$RUNTIME_FILE" "hook_dry_run_ready"
require_text "$RUNTIME_FILE" "activation_proof_ready"
require_text "$RUNTIME_FILE" "provider_readiness_preflight_ready"
require_text "$RUNTIME_FILE" "would_install: false"
require_text "$RUNTIME_FILE" "would_select_provider: false"
require_text "$RUNTIME_FILE" "would_activate: false"
require_text "$RUNTIME_FILE" "provider_combined_dry_run_fixtures_report_ready_without_replacement"
require_text "$CLI_FILE" "maybe_run_allocator_provider_combined_dry_run"
require_text "$CLI_FILE" "build_allocator_provider_combined_dry_run_output"
require_text "$CLI_FILE" "combined_status"
require_text "$CLI_FILE" "would_install={}"
require_text "$CLI_FILE" "would_select_provider={}"
require_text "$CLI_FILE" "would_activate={}"
require_text "$CLI_ARGS" "allocator_provider_manifest_combines_with_hook_dry_run"
require_text "$CLI_MOD" "maybe_run_allocator_provider_combined_dry_run"
require_text "$MAIN_FILE" "maybe_run_allocator_provider_combined_dry_run"
require_text "$SSOT" "Allocator Provider Combined Dry-Run (SSOT)"
require_text "$SSOT" "would_install=false"
require_text "$SSOT" "would_select_provider=false"
require_text "$SSOT" "would_activate=false"
require_text "$SSOT" 'Standalone `--allocator-hook-dry-run` and standalone'
require_text "$TASK_BREAKDOWN" "M70 | combined hook/provider dry-run report"
require_text "$TASK_BREAKDOWN" "M71 | provider registry boundary docs"
require_text "$TASKBOARD" '| `M70 combined hook/provider dry-run report` | `live-narrow` |'
require_text "$TASKBOARD" '93. `M70 combined hook/provider dry-run report`'
require_text "$PHASE_README" '`293x-122`'
require_text "$REAL_APP_TASKBOARD" '[x] `293x-122` M70 combined hook/provider dry-run report'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_combined_dry_run_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_combined_dry_run_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_combined_dry_run_guard.sh"

cargo test -q allocator_provider
cargo test -q allocator_provider_manifest_combines_with_hook_dry_run

if rg -n 'std::env|set_var|var_os|env_bool|env_string' "$CLI_FILE" >/tmp/"$TAG".env 2>&1; then
  cat /tmp/"$TAG".env >&2
  rm -f /tmp/"$TAG".env
  fail "combined dry-run CLI must not add hidden environment toggles"
fi
rm -f /tmp/"$TAG".env

if rg -n 'std::env|std::fs|read_to_string|set_var|var_os|std::alloc|GlobalAlloc|#\[global_allocator\]' \
  "$RUNTIME_FILE" >/tmp/"$TAG".forbidden_runtime 2>&1; then
  cat /tmp/"$TAG".forbidden_runtime >&2
  rm -f /tmp/"$TAG".forbidden_runtime
  fail "combined dry-run runtime must not add env/fs/allocator replacement behavior"
fi
rm -f /tmp/"$TAG".forbidden_runtime

if rg -n '(^|[^A-Za-z0-9_])select_allocator_provider([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])allocator_provider_select([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])allocator_provider_selection_env([^A-Za-z0-9_]|$)|NYASH_ALLOCATOR_PROVIDER' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".provider_selection 2>&1; then
  cat /tmp/"$TAG".provider_selection >&2
  rm -f /tmp/"$TAG".provider_selection
  fail "provider selection implementation/env toggle must stay absent in M70"
fi
rm -f /tmp/"$TAG".provider_selection

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M70"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own combined allocator provider dry-run behavior"
fi
rm -f /tmp/"$TAG".runner

if rg -n 'HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan|allocator_hook_activate|activate_allocator|native_mimalloc|native_system_malloc' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  fail "allocator provider/hook/facade/policy matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc

echo "[$TAG] ok"
