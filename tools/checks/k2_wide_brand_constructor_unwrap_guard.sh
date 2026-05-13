#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[brand-constructor-unwrap] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text docs/development/current/main/design/brand-constructor-unwrap-policy-ssot.md "BRAND-002 Stage1 Brand Constructor / Unwrap Policy SSOT"
require_text docs/development/current/main/phases/phase-293x/293x-276-BRAND-002-STAGE1-BRAND-CONSTRUCTOR-UNWRAP-POLICY.md "Status: complete"
require_text src/stage1/program_json_v0/lowering.rs "BrandConstruct"
require_text src/stage1/program_json_v0/lowering.rs "BrandUnwrap"
require_text src/stage1/program_json_v0/lowering.rs "[brand/constructor-arity]"
require_text src/stage1/program_json_v0/lowering.rs "[brand/unsupported-static-method]"
require_text src/stage1/program_json_v0/tests/basics_and_enums.rs "source_to_program_json_v0_emits_brand_inventory_constructor_and_unwrap"
require_text docs/tools/check-scripts-index.md "k2_wide_brand_constructor_unwrap_guard.sh"

cargo test -q brand_

echo "[brand-constructor-unwrap] OK"
