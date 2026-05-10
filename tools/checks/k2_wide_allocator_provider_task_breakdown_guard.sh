#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-task-breakdown"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-118-M66-ALLOCATOR-PROVIDER-TASK-BREAKDOWN.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
CURRENT_TASK="CURRENT_TASK.md"
NOW_DOC="docs/development/current/main/10-Now.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M66 allocator provider task breakdown"

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
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$CURRENT_TASK"
require_file "$NOW_DOC"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Current Task Breakdown (SSOT)"
require_text "$SSOT" "Current Completed Checkpoint"
require_text "$SSOT" "Immediate Task Ladder"
require_text "$SSOT" "M67 | provider manifest diagnostic parser"
require_text "$SSOT" "M70 | combined hook/provider dry-run report"
require_text "$SSOT" "M75 | native mimalloc provider proof boundary"
require_text "$SSOT" 'Past card guards should not pin `CURRENT_STATE.latest_card`'
require_text "$TASKBOARD" '| `M66 allocator provider task breakdown` | `live-docs` |'
require_text "$TASKBOARD" '| `M67 allocator provider manifest parser` | `planned` |'
require_text "$TASKBOARD" '| `M75 native mimalloc provider proof boundary` | `planned` |'
require_text "$TASKBOARD" '89. `M66 allocator provider task breakdown`'
require_text "$TASKBOARD" '98. `M75 native mimalloc provider proof boundary`'
require_text "$PHASE_README" '`293x-118`'
require_text "$REAL_APP_TASKBOARD" '`293x-118` M66 allocator provider task breakdown'
require_text "$CURRENT_STATE" 'latest_card = "293x-118-M66-ALLOCATOR-PROVIDER-TASK-BREAKDOWN"'
require_text "$CURRENT_TASK" "allocator-provider-current-task-breakdown-ssot.md"
require_text "$CURRENT_TASK" 'next: continue the allocator provider/replacement ladder from `M67`'
require_text "$NOW_DOC" "allocator-provider-current-task-breakdown-ssot.md"
require_text "$NOW_DOC" 'next implementation row is `M67` provider manifest diagnostic parser'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_task_breakdown_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_task_breakdown_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_task_breakdown_guard.sh"

if rg -n 'native_system_malloc|native_mimalloc|hako_model_allocator|debug_guarded_allocator|AllocatorProvider|ProviderManifest|ProviderBoundary' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".provider_symbols 2>&1; then
  cat /tmp/"$TAG".provider_symbols >&2
  rm -f /tmp/"$TAG".provider_symbols
  fail "M66 task breakdown must not become runtime/backend symbols"
fi
rm -f /tmp/"$TAG".provider_symbols

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M66"
fi
rm -f /tmp/"$TAG".global_allocator

echo "[$TAG] ok"
