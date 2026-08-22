#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$root"

rg -q 'row_id = "brand_declaration"' grammar/language-v1-registry.toml
rg -q 'fixture_id = "brand_declaration_canonical"' \
  grammar/language-v1-grammar-contract-corpus/declarations.toml
rg -q 'fixture_id = "brand_declaration_compat"' \
  grammar/language-v1-grammar-contract-corpus/declarations.toml
rg -q 'parser/brand_declaration_invalid' src/parser/declarations/brand_def.rs
rg -q 'ParserBrandDeclarationBox.parse' \
  lang/src/compiler/parser/decl/parser_declaration_box.hako
rg -Fq '\"semantic_publication_allowed\":false' \
  lang/src/compiler/parser/decl/parser_declaration_box.hako
if rg -q 'row_id = "brand_(constructor|unwrap)' grammar/language-v1-registry.toml; then
  echo '[parser-brand-declaration-capsule-guard] contextual Brand calls became grammar rows' >&2
  exit 1
fi

python3 tools/language_v1/generate_hako_grammar_contract.py --check
python3 -m unittest tools.language_v1.test_witness_projection
CARGO_BUILD_JOBS=4 cargo test --profile quick -p nyash-rust \
  --test parser_grammar_profile brand_declaration_follows_the_v1_profile_contract \
  -- --exact --quiet

echo '[parser-brand-declaration-capsule-guard] ok'
