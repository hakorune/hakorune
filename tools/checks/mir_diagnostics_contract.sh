#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

FILE_DIAG="src/mir/diagnostics.rs"
FILE_MIR_MOD="src/mir/mod.rs"
FILE_BUILDER_EMIT="src/mir/builder/builder_emit.rs"
FILE_CF_UTIL="src/mir/utils/control_flow.rs"
FILE_PHI_UTIL="src/mir/utils/phi_helpers.rs"

require_pattern() {
  local file="$1"
  local needle="$2"
  if ! rg -n --fixed-strings "$needle" "$file" >/dev/null; then
    echo "[mir-diagnostics-contract] ERROR: missing pattern in $file"
    echo "  expected: $needle"
    exit 1
  fi
}

forbid_pattern() {
  local file="$1"
  local needle="$2"
  if rg -n --fixed-strings "$needle" "$file" >/dev/null; then
    echo "[mir-diagnostics-contract] ERROR: forbidden pattern in $file"
    echo "  found: $needle"
    exit 1
  fi
}

require_pattern "$FILE_DIAG" "pub(crate) struct FreezeContract"
require_pattern "$FILE_DIAG" "pub(crate) fn caller_string("
require_pattern "$FILE_DIAG" "pub(crate) fn mir_dump_value("
require_pattern "$FILE_MIR_MOD" "pub mod diagnostics; // freeze diagnostics helpers (SSOT)"

require_pattern "$FILE_BUILDER_EMIT" "use crate::mir::diagnostics::{caller_string, mir_dump_value, FreezeContract};"
require_pattern "$FILE_BUILDER_EMIT" "FreezeContract::new(\"builder/emit_missing_block\")"
require_pattern "$FILE_BUILDER_EMIT" "FreezeContract::new(\"builder/non_dominating_copy\")"
require_pattern "$FILE_BUILDER_EMIT" "FreezeContract::new(\"builder/binop_operand_out_of_function_scope\")"
require_pattern "$FILE_BUILDER_EMIT" ".field(\"caller\", caller_string(caller))"
require_pattern "$FILE_BUILDER_EMIT" ".field(\"mir_dump\", mir_dump_value(mir_dump_path))"

forbid_pattern "$FILE_BUILDER_EMIT" "[freeze:contract][builder/emit_missing_block]"
forbid_pattern "$FILE_BUILDER_EMIT" "[freeze:contract][builder/non_dominating_copy]"
forbid_pattern "$FILE_BUILDER_EMIT" "[freeze:contract][builder/binop_operand_out_of_function_scope]"
forbid_pattern "$FILE_BUILDER_EMIT" "caller.file(), caller.line(), caller.column()"
forbid_pattern "$FILE_BUILDER_EMIT" "mir_dump_path.unwrap_or_else(|| \"disabled\".to_string())"

require_pattern "$FILE_CF_UTIL" "use crate::mir::diagnostics::FreezeContract;"
require_pattern "$FILE_CF_UTIL" "FreezeContract::new(\"builder/capture_jump_without_function\")"
forbid_pattern "$FILE_CF_UTIL" "[freeze:contract][builder/capture_jump_without_function]"

require_pattern "$FILE_PHI_UTIL" "use crate::mir::diagnostics::FreezeContract;"
require_pattern "$FILE_PHI_UTIL" "FreezeContract::new(\"builder/phi_insert_without_function_context\")"
forbid_pattern "$FILE_PHI_UTIL" "[freeze:contract][builder/phi_insert_without_function_context]"

echo "[mir-diagnostics-contract] OK"
