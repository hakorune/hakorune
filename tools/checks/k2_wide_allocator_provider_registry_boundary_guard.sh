#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-registry-boundary"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-registry-boundary-ssot.md"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-123-M71-ALLOCATOR-PROVIDER-REGISTRY-BOUNDARY.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

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
require_text "$TASK_BREAKDOWN" "M71 | provider registry boundary docs"
require_text "$TASK_BREAKDOWN" "M72 | hako model provider proof fixture"
require_text "$TASKBOARD" '| `M71 allocator provider registry boundary` | `live-docs` |'
require_text "$TASKBOARD" '94. `M71 allocator provider registry boundary`'
require_text "$PHASE_README" '`293x-123`'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_registry_boundary_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_registry_boundary_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_registry_boundary_guard.sh"


allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider registry behavior"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
