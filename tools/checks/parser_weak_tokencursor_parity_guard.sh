#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$root"

rg -q 'row_id = "weak_unary_expr"' grammar/language-v1-registry.toml
rg -q 'row_id = "weak_paren_expr"' grammar/language-v1-registry.toml
rg -q 'TokenType::WEAK => \{' crates/hakorune_frontend_parser/src/parser/expr_cursor/precedence.rs
rg -q 'parser/weak_paren_call_rejected' crates/hakorune_frontend_parser/src/parser/expr_cursor/precedence.rs
if rg -q 'WeakReject|Rejecting weak|Invalid syntax: weak' src/mir/builder/calls/function_call_preflight_route.rs; then
  echo '[parser-weak-tokencursor-parity-guard] duplicate raw weak rejection remains' >&2
  exit 1
fi

strict_cohort="$(sed -n '/12.7 Strict mode/,/if is_extended/p' crates/hakorune_frontend_parser/src/tokenizer/lex_ident.rs)"
if grep -q 'TokenType::WEAK' <<<"$strict_cohort"; then
  echo '[parser-weak-tokencursor-parity-guard] strict-12.7 still downgrades WEAK' >&2
  exit 1
fi
for sibling in INTERFACE USING OUTBOX NOWAIT OVERRIDE DELEGATE PACK; do
  grep -q "TokenType::$sibling" <<<"$strict_cohort"
done
rg -q "Some\('>'\).*!Self::strict_12_7" crates/hakorune_frontend_parser/src/tokenizer/engine.rs
rg -q "Some\('<'\).*!Self::strict_12_7" crates/hakorune_frontend_parser/src/tokenizer/engine.rs

cargo test --profile quick -p hakorune-frontend-parser weak_grammar_parity_tests --quiet
NYASH_STRICT_12_7=1 cargo test --profile quick -p nyash-rust --test parser_grammar_profile weak_expression_surface_follows_the_v1_profile_contract -- --exact --quiet
NYASH_STRICT_12_7=1 NYASH_PARSER_TOKEN_CURSOR=1 cargo test --profile quick -p nyash-rust --test parser_grammar_profile weak_expression_surface_follows_the_v1_profile_contract -- --exact --quiet
echo '[parser-weak-tokencursor-parity-guard] ok'
