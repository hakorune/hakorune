#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="k2-wide-box-new-field-initializer-segment-worker-reports"

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

CARD="docs/development/current/main/phases/phase-293x/293x-994-BOX-INIT-003-SEGMENT-WORKER-REPORT-NEW-BOX-FIELD-INITIALIZER.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-995-BOX-INIT-004-SUPPORT-PROVIDER-REPORT-NEW-BOX-FIELD-INITIALIZER.md"
INDEX="docs/tools/check-scripts-index.md"

REMAINING="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_remaining_execution_prerequisite_ledger_box.hako"
WORKER_TLS="lang/src/hako_alloc/memory/worker_tls_pilot_box.hako"
READINESS_DIAG="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_diagnostic_box.hako"

require_file "$CARD"
require_file "$NEXT_CARD"
require_file "$INDEX"
require_file "$REMAINING"
require_file "$WORKER_TLS"
require_file "$READINESS_DIAG"

require_grep "Status: landed" "$CARD"
require_grep "Status: (current|landed)" "$NEXT_CARD"
require_grep "k2_wide_box_new_field_initializer_segment_worker_reports_guard.sh" "$INDEX"

require_grep "new HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleRemainingExecutionPrerequisiteLedgerReport \\{" "$REMAINING"
require_grep "new HakoAllocWorkerTlsPilotReport \\{" "$WORKER_TLS"
require_grep "new HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionReadinessMatrixDiagnosticReport \\{" "$READINESS_DIAG"

require_no_grep "local result = new HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleRemainingExecutionPrerequisiteLedgerReport\\(\\)" "$REMAINING"
require_no_grep "local result = new HakoAllocWorkerTlsPilotReport\\(\\)" "$WORKER_TLS"
require_no_grep "local result = new HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionReadinessMatrixDiagnosticReport\\(\\)" "$READINESS_DIAG"

require_no_grep "fields\\.\\*" "$REMAINING"
require_no_grep "fields\\.\\*" "$WORKER_TLS"
require_no_grep "fields\\.\\*" "$READINESS_DIAG"
require_no_grep "\\.\\.\\.fields" "$REMAINING"
require_no_grep "\\.\\.\\.fields" "$WORKER_TLS"
require_no_grep "\\.\\.\\.fields" "$READINESS_DIAG"

bash tools/checks/k2_wide_box_new_field_initializer_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_remaining_execution_prerequisite_ledger_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_worker_tls_pilot_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_diagnostics_guard.sh --level L2

echo "[$TAG][ok] segment/worker report helpers use explicit new-box field initializers"
