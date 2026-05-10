#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-readiness-preflight"
cd "$ROOT_DIR"

RUNTIME_FILE="src/runtime/allocator_provider_manifest.rs"
SSOT="docs/development/current/main/design/allocator-provider-readiness-preflight-ssot.md"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-121-M69-ALLOCATOR-PROVIDER-READINESS-PREFLIGHT-SHAPE.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M69 allocator provider readiness preflight"

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
require_file "$SSOT"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$RUNTIME_FILE" "AllocatorProviderReadinessPreflightFacts"
require_text "$RUNTIME_FILE" "AllocatorProviderReadinessPreflightReport"
require_text "$RUNTIME_FILE" "AllocatorProviderReadinessPreflightStatus"
require_text "$RUNTIME_FILE" "validate_allocator_provider_readiness_preflight"
require_text "$RUNTIME_FILE" "validate_allocator_provider_readiness_preflight_from_manifest_texts"
require_text "$RUNTIME_FILE" "DIAG_PROVIDER_READINESS_PREFLIGHT_READY"
require_text "$RUNTIME_FILE" "DIAG_PROVIDER_READINESS_PREFLIGHT_MISSING"
require_text "$RUNTIME_FILE" "provider_manifest_ready"
require_text "$RUNTIME_FILE" "activation_preflight_ready"
require_text "$RUNTIME_FILE" "provider_ids_reserved_set"
require_text "$RUNTIME_FILE" "would_select_provider: false"
require_text "$RUNTIME_FILE" "would_activate: false"
require_text "$RUNTIME_FILE" "provider_readiness_preflight_fixtures_report_ready_without_activation"
require_text "$RUNTIME_FILE" "provider_readiness_preflight_missing_provider_manifest_reports_missing"
require_text "$RUNTIME_FILE" "provider_readiness_preflight_missing_activation_preflight_reports_missing"
require_text "$SSOT" "Allocator Provider Readiness Preflight (SSOT)"
require_text "$SSOT" "would_select_provider = false"
require_text "$SSOT" "would_activate = false"
require_text "$SSOT" "CLI composition is deferred to M70"
require_text "$TASK_BREAKDOWN" "M69 | provider readiness preflight shape"
require_text "$TASK_BREAKDOWN" "M70 | combined hook/provider dry-run report"
require_text "$TASKBOARD" '| `M69 allocator provider readiness preflight shape` | `live-narrow` |'
require_text "$TASKBOARD" '92. `M69 allocator provider readiness preflight shape`'
require_text "$PHASE_README" '`293x-121`'
require_text "$REAL_APP_TASKBOARD" '[x] `293x-121` M69 allocator provider readiness preflight shape'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_readiness_preflight_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_readiness_preflight_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_readiness_preflight_guard.sh"

cargo test -q allocator_provider

if rg -n 'std::env|std::fs|read_to_string|set_var|var_os|std::alloc|GlobalAlloc|#\[global_allocator\]' \
  "$RUNTIME_FILE" >/tmp/"$TAG".forbidden_runtime 2>&1; then
  cat /tmp/"$TAG".forbidden_runtime >&2
  rm -f /tmp/"$TAG".forbidden_runtime
  fail "provider readiness preflight must not add env/fs/allocator replacement behavior"
fi
rm -f /tmp/"$TAG".forbidden_runtime

if rg -n '(^|[^A-Za-z0-9_])select_allocator_provider([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])allocator_provider_select([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])allocator_provider_selection_env([^A-Za-z0-9_]|$)|NYASH_ALLOCATOR_PROVIDER' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".provider_selection 2>&1; then
  cat /tmp/"$TAG".provider_selection >&2
  rm -f /tmp/"$TAG".provider_selection
  fail "provider selection implementation/env toggle must stay absent in M69"
fi
rm -f /tmp/"$TAG".provider_selection

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M69"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider readiness behavior"
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
