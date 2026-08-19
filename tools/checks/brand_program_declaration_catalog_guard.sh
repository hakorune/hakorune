#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[brand-program-catalog] missing '$text' in $file" >&2
    exit 1
  fi
}

reject_text() {
  local file="$1"
  local text="$2"
  if grep -Fq "$text" "$file"; then
    echo "[brand-program-catalog] forbidden '$text' in $file" >&2
    exit 1
  fi
}

OWNER=src/analysis/brand_program_declaration_catalog.rs
FACTS=src/mir/builder/program_declaration_facts.rs
STAGE1=src/stage1/program_json_v0/authority.rs

require_text "$OWNER" "VerifiedBrandProgramDeclarationCatalogV1"
require_text "$OWNER" "[freeze:contract][brand/duplicate-declaration]"
require_text "$FACTS" "brand_catalog: VerifiedBrandProgramDeclarationCatalogV1"
require_text "$FACTS" "with_brand_catalog"
require_text src/mir/builder/compilation_context.rs "compatibility cache projected only"
require_text "$STAGE1" "issue_brand_program_declaration_catalog_v1"
reject_text "$STAGE1" "collect_brand_decl_index"
reject_text src/stage1/program_json_v0/lowering.rs "known_brands: BTreeMap<String, String>"

CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  brand_program_declaration_catalog --lib
CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust duplicate_brand --lib

echo "[brand-program-catalog] OK"
