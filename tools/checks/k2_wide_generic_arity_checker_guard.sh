#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[generic-arity-checker] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text docs/development/current/main/design/generic-arity-checker-ssot.md "GEN-002 Generic Arity Checker SSOT"
require_text docs/development/current/main/phases/phase-293x/293x-287-GEN-002-GENERIC-ARITY-CHECKER.md "Status: complete"
require_text docs/reference/language/EBNF.md "GEN-002 Generic Arity Checker"
require_text src/stage1/program_json_v0/generic_arity_checker.rs "check_generic_arities"
require_text src/stage1/program_json_v0/generic_arity_checker.rs "[generic/arity]"
require_text src/stage1/program_json_v0/tests/basics_and_enums.rs "source_to_program_json_v0_rejects_builtin_generic_arity_mismatch"
require_text src/stage1/program_json_v0/tests/basics_and_enums.rs "source_to_program_json_v0_rejects_declared_generic_arity_mismatch"
require_text docs/tools/check-scripts-index.md "k2_wide_generic_arity_checker_guard.sh"

cargo test -q generic_arity
cargo test -q source_to_program_json_v0_transports_generic_type_metadata

echo "[generic-arity-checker] OK"
