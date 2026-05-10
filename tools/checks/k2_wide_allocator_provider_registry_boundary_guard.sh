#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-registry-boundary"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/allocator-provider-registry-boundary-ssot.md"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-123-M71-ALLOCATOR-PROVIDER-REGISTRY-BOUNDARY.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
FUTURE_REGISTRY_FILE="src/runtime/allocator_provider_registry.rs"

echo "[$TAG] checking M71 allocator provider registry boundary"

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
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Registry Boundary (SSOT)"
require_text "$SSOT" "src/runtime/allocator_provider_registry.rs"
require_text "$SSOT" "ProviderRegistryEntry"
require_text "$SSOT" "ProviderRegistrySnapshot"
require_text "$SSOT" "ProviderRegistryBuildInput"
require_text "$SSOT" "ProviderSelectionRequest"
require_text "$SSOT" "ProviderSelectionDecision"
require_text "$SSOT" "would_select_provider = false"
require_text "$SSOT" "would_activate = false"
require_text "$SSOT" "runtime provider registry implementation"
require_text "$TASK_BREAKDOWN" "M64-M71"
require_text "$TASK_BREAKDOWN" "M72 may add a hako model provider proof fixture"
require_text "$TASKBOARD" '| `M71 allocator provider registry boundary` | `live-docs` |'
require_text "$TASKBOARD" '94. `M71 allocator provider registry boundary`'
require_text "$PHASE_README" '`293x-123`'
require_text "$REAL_APP_TASKBOARD" '[x] `293x-123` M71 allocator provider registry boundary'
require_text "$CURRENT_STATE" 'latest_card = "293x-123-M71-ALLOCATOR-PROVIDER-REGISTRY-BOUNDARY"'
require_text "$CURRENT_STATE" 'latest_card_path = "docs/development/current/main/phases/phase-293x/293x-123-M71-ALLOCATOR-PROVIDER-REGISTRY-BOUNDARY.md"'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_registry_boundary_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_registry_boundary_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_registry_boundary_guard.sh"

if [[ -e "$FUTURE_REGISTRY_FILE" ]]; then
  fail "future registry owner file must remain absent in M71: $FUTURE_REGISTRY_FILE"
fi

if rg -n 'AllocatorProviderRegistry|allocator_provider_registry|select_allocator_provider|allocator_provider_select|allocator_provider_selection_env|NYASH_ALLOCATOR_PROVIDER|ProviderRegistryEntry|ProviderRegistrySnapshot|ProviderRegistryBuildInput|ProviderSelectionRequest|ProviderSelectionDecision' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".provider_registry 2>&1; then
  cat /tmp/"$TAG".provider_registry >&2
  rm -f /tmp/"$TAG".provider_registry
  fail "provider registry/selection implementation must stay absent in M71"
fi
rm -f /tmp/"$TAG".provider_registry

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M71"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider registry behavior"
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
