#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-task-breakdown"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-118-M66-ALLOCATOR-PROVIDER-TASK-BREAKDOWN.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
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
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Current Task Breakdown (SSOT)"
require_text "$SSOT" "Current Completed Checkpoint"
require_text "$SSOT" "Immediate Task Ladder"
require_text "$SSOT" "M67 | provider manifest diagnostic parser"
require_text "$SSOT" "M70 | combined hook/provider dry-run report"
require_text "$SSOT" "M75 | native mimalloc provider proof boundary"
require_text "$SSOT" "Provider proof boundary ladder is now closed"
require_text "$SSOT" 'Past card guards should not pin `CURRENT_STATE.latest_card`'
require_text "$TASKBOARD" '| `M66 allocator provider task breakdown` | `live-docs` |'
require_text "$TASKBOARD" '`M67 allocator provider manifest parser`'
require_text "$TASKBOARD" '`M75 native mimalloc provider proof boundary`'
require_text "$TASKBOARD" '89. `M66 allocator provider task breakdown`'
require_text "$TASKBOARD" '98. `M75 native mimalloc provider proof boundary`'
require_text "$PHASE_README" '`293x-118`'
require_text "$REAL_APP_TASKBOARD" '`293x-118` M66 allocator provider task breakdown'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_task_breakdown_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_task_breakdown_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_task_breakdown_guard.sh"

allocator_provider_forbid_global_allocator "$TAG"

allocator_provider_forbid_selection "$TAG"

echo "[$TAG] ok"
