#!/usr/bin/env bash
set -euo pipefail

ssot='docs/development/current/main/design/language-minimal-surface-ssot.md'
taskboard='docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md'
reference_index='docs/reference/language/README.md'

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[language-surface-admission] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text "$ssot" "Small surface, strong semantics"
require_text "$ssot" "Surface admission checklist"
require_text "$ssot" "Fold-first rule"
require_text "$ssot" 'while` / `for` / `repeat` / `until` / `do'
require_text "$ssot" 'try` / `throw` / `?'
require_text "$ssot" 'Vec<T>` / `List<T>` / canonical `T[]'
require_text "$ssot" "Reserved protocol surface"
require_text "$taskboard" "Feature admission policy"
require_text "$reference_index" "Minimal surface policy"
require_text docs/tools/check-scripts-index.md "k2_wide_language_surface_admission_guard.sh"

echo "[language-surface-admission] OK"
