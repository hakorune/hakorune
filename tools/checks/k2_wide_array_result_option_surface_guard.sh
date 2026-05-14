#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[array-result-option-surface] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text docs/development/current/main/design/array-result-option-canonical-surface-ssot.md "Array / Result / Option Canonical Surface SSOT"
require_text docs/development/current/main/design/array-result-option-canonical-surface-ssot.md "Type::Variant"
require_text docs/development/current/main/design/array-result-option-canonical-surface-ssot.md "LOCALTYPE-001"
require_text docs/development/current/main/design/array-result-option-canonical-surface-ssot.md "PACKED-001"
require_text docs/development/current/main/phases/phase-293x/293x-288-ARRAY-RESULT-OPTION-CANONICAL-SURFACE-SSOT.md "Status: complete"
require_text docs/reference/language/EBNF.md "Array / PackedArray / Result / Option Canonical Surface"
require_text docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md "ARRAY-RESULT-SSOT"
require_text docs/tools/check-scripts-index.md "k2_wide_array_result_option_surface_guard.sh"

echo "[array-result-option-surface] OK"
