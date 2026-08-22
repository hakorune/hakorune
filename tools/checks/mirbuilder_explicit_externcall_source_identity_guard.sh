#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$root"

rg -q 'ExplicitExternCall \{' crates/hakorune_frontend_ast/src/ast_node.rs
rg -q 'row_id = "explicit_externcall"' grammar/language-v1-registry.toml
rg -q 'record_explicit_extern_call' src/mir/resolved_semantics/shadow/expr.rs
rg -q 'missing-resolved-relation' src/mir/builder/raw_expression_dispatch/mod.rs
if rg -q 'name == "externcall"' src/mir/builder/calls/function_call_preflight_route.rs; then
  echo '[explicit-externcall-guard] raw FunctionCall name authority remains' >&2
  exit 1
fi

cargo test --profile quick -p nyash-rust explicit_extern --quiet
echo '[explicit-externcall-guard] ok'
