#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="k2-wide-box-new-field-initializer-provider-reports"

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

CARD="docs/development/current/main/phases/phase-293x/293x-993-BOX-INIT-002-PROVIDER-REPORT-NEW-BOX-FIELD-INITIALIZER.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-994-BOX-INIT-003-SEGMENT-WORKER-REPORT-NEW-BOX-FIELD-INITIALIZER.md"
INDEX="docs/tools/check-scripts-index.md"

BOUNDARY="lang/src/hako_alloc/memory/provider_boundary_diagnostic_vocabulary_box.hako"
READINESS="lang/src/hako_alloc/memory/provider_readiness_preflight_box.hako"
SELECTION="lang/src/hako_alloc/memory/provider_selection_inventory_box.hako"
UNSUPPORTED="lang/src/hako_alloc/memory/provider_activation_unsupported_outcome_ledger_box.hako"

require_file "$CARD"
require_file "$NEXT_CARD"
require_file "$INDEX"
require_file "$BOUNDARY"
require_file "$READINESS"
require_file "$SELECTION"
require_file "$UNSUPPORTED"

require_grep "Status: landed" "$CARD"
require_grep "Status: (current|landed)" "$NEXT_CARD"
require_grep "k2_wide_box_new_field_initializer_provider_reports_guard.sh" "$INDEX"

require_grep "new HakoAllocProviderBoundaryDiagnosticVocabularyReport \\{" "$BOUNDARY"
require_grep "new HakoAllocProviderReadinessPreflightReport \\{" "$READINESS"
require_grep "new HakoAllocProviderSelectionInventoryReport \\{" "$SELECTION"
require_grep "new HakoAllocProviderActivationUnsupportedOutcomeLedgerReport \\{" "$UNSUPPORTED"

require_no_grep "local result = new HakoAllocProviderBoundaryDiagnosticVocabularyReport\\(\\)" "$BOUNDARY"
require_no_grep "local result = new HakoAllocProviderReadinessPreflightReport\\(\\)" "$READINESS"
require_no_grep "local result = new HakoAllocProviderSelectionInventoryReport\\(\\)" "$SELECTION"
require_no_grep "local result = new HakoAllocProviderActivationUnsupportedOutcomeLedgerReport\\(\\)" "$UNSUPPORTED"

require_no_grep "fields\\.\\*" "$BOUNDARY"
require_no_grep "fields\\.\\*" "$READINESS"
require_no_grep "fields\\.\\*" "$SELECTION"
require_no_grep "fields\\.\\*" "$UNSUPPORTED"

bash tools/checks/k2_wide_box_new_field_initializer_guard.sh
bash tools/checks/k2_wide_hako_alloc_provider_boundary_diagnostic_vocabulary_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_provider_readiness_preflight_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_provider_selection_inventory_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_provider_activation_unsupported_outcome_ledger_guard.sh --level L2

echo "[$TAG][ok] provider report helpers use explicit new-box field initializers"
