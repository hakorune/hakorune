#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="k2-wide-box-new-field-initializer-support-provider-reports"

require_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "[$TAG][fail] missing file: $path" >&2
    exit 1
  fi
}

require_grep() {
  local pattern="$1"
  local path="$2"
  if ! rg -q "$pattern" "$path"; then
    echo "[$TAG][fail] missing pattern in $path: $pattern" >&2
    exit 1
  fi
}

require_no_grep() {
  local pattern="$1"
  local path="$2"
  if rg -q "$pattern" "$path"; then
    echo "[$TAG][fail] forbidden pattern in $path: $pattern" >&2
    exit 1
  fi
}

CARD="docs/development/current/main/phases/phase-293x/293x-995-BOX-INIT-004-SUPPORT-PROVIDER-REPORT-NEW-BOX-FIELD-INITIALIZER.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-996-BOX-INIT-005-POST-SUPPORT-PROVIDER-REPORT-INITIALIZER-ROW-SELECTION.md"
INDEX="docs/tools/check-scripts-index.md"

SUPPORT="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_box.hako"
SUPPORT_DIAG="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_diagnostic_box.hako"
PROVIDER_INACTIVE="lang/src/hako_alloc/memory/provider_inactive_boundary_inventory_box.hako"
OSVM="lang/src/hako_alloc/memory/osvm_page_source_pilot_box.hako"
ATOMIC="lang/src/hako_alloc/memory/atomic_bitmap_pilot_box.hako"

require_file "$CARD"
require_file "$NEXT_CARD"
require_file "$INDEX"
require_file "$SUPPORT"
require_file "$SUPPORT_DIAG"
require_file "$PROVIDER_INACTIVE"
require_file "$OSVM"
require_file "$ATOMIC"

require_grep "Status: landed" "$CARD"
require_grep "Status: current" "$NEXT_CARD"
require_grep "k2_wide_box_new_field_initializer_support_provider_reports_guard.sh" "$INDEX"

require_grep "new HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionSupportRequirementMatrixReport \\{" "$SUPPORT"
require_grep "new HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionSupportRequirementMatrixDiagnosticReport \\{" "$SUPPORT_DIAG"
require_grep "new HakoAllocProviderInactiveBoundaryInventoryReport \\{" "$PROVIDER_INACTIVE"
require_grep "new HakoAllocOSVMPageSourcePilotReport \\{" "$OSVM"
require_grep "new HakoAllocAtomicBitmapPilotReport \\{" "$ATOMIC"

require_no_grep "local result = new HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionSupportRequirementMatrixReport\\(\\)" "$SUPPORT"
require_no_grep "local result = new HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionSupportRequirementMatrixDiagnosticReport\\(\\)" "$SUPPORT_DIAG"
require_no_grep "local result = new HakoAllocProviderInactiveBoundaryInventoryReport\\(\\)" "$PROVIDER_INACTIVE"
require_no_grep "local result = new HakoAllocOSVMPageSourcePilotReport\\(\\)" "$OSVM"
require_no_grep "local result = new HakoAllocAtomicBitmapPilotReport\\(\\)" "$ATOMIC"

for owner in "$SUPPORT" "$SUPPORT_DIAG" "$PROVIDER_INACTIVE" "$OSVM" "$ATOMIC"; do
  require_no_grep "fields\\.\\*" "$owner"
  require_no_grep "\\.\\.\\.fields" "$owner"
done

bash tools/checks/k2_wide_box_new_field_initializer_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_diagnostics_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_provider_inactive_boundary_inventory_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_osvm_page_source_pilot_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_atomic_bitmap_pilot_guard.sh --level L2

echo "[$TAG][ok] support/provider report helpers use explicit new-box field initializers"
