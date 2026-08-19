#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$root"

rg -q 'row_id = "weak_unary_expr"' grammar/language-v1-registry.toml
rg -q 'row_id = "weak_paren_expr"' grammar/language-v1-registry.toml
rg -q 'TokenType::WEAK => \{' crates/hakorune_frontend_parser/src/parser/expr_cursor/precedence.rs
rg -q 'parser/weak_paren_call_rejected' crates/hakorune_frontend_parser/src/parser/expr_cursor/precedence.rs
rg -q 'PreparedRawFunctionPreflightRouteV1::WeakReject' src/mir/builder/calls/function_call_preflight_route.rs

cargo test --profile quick -p hakorune-frontend-parser weak_grammar_parity_tests --quiet
echo '[parser-weak-tokencursor-parity-guard] ok'
