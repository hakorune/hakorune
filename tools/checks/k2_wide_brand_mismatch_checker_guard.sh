#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[brand-mismatch-checker] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text docs/development/current/main/design/brand-mismatch-checker-ssot.md "BRAND-003 Stage1 Brand Mismatch Checker SSOT"
require_text docs/development/current/main/phases/phase-293x/293x-277-BRAND-003-STAGE1-BRAND-MISMATCH-CHECKER.md "Status: complete"
require_text src/stage1/program_json_v0/brand_checker.rs "check_brand_mismatches"
require_text src/stage1/program_json_v0/brand_checker.rs "[brand/mismatch]"
require_text src/stage1/program_json_v0/tests/basics_and_enums.rs "source_to_program_json_v0_rejects_mismatched_brand_method_arg"
require_text src/stage1/program_json_v0/tests/basics_and_enums.rs "source_to_program_json_v0_rejects_unbranded_value_for_brand_arg"
require_text docs/tools/check-scripts-index.md "k2_wide_brand_mismatch_checker_guard.sh"

cargo test -q brand_

echo "[brand-mismatch-checker] OK"
