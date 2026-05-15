#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="concurrency-context-surface-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TASKBOARD="$ROOT_DIR/docs/development/current/main/design/concurrency-boundary-migration-taskboard-ssot.md"
SEMANTICS="$ROOT_DIR/docs/reference/concurrency/semantics.md"
AST="$ROOT_DIR/src/ast/mod.rs"
AST_UTILS="$ROOT_DIR/src/ast/utils/node_type.rs"
PARSER="$ROOT_DIR/src/parser/statements/context_scope.rs"
AST_JSON="$ROOT_DIR/src/macro/ast_json/joinir_compat.rs"
AST_JSON_ROUNDTRIP="$ROOT_DIR/src/macro/ast_json/roundtrip.rs"
PROGRAM_JSON="$ROOT_DIR/src/stage1/program_json_v0/lowering.rs"
MIR_BUILDER="$ROOT_DIR/src/mir/builder/exprs.rs"
PARSER_TEST="$ROOT_DIR/src/tests/parser_context_surface.rs"

guard_require_command "$TAG" rg
guard_require_files "$TAG" \
  "$TASKBOARD" \
  "$SEMANTICS" \
  "$AST" \
  "$AST_UTILS" \
  "$PARSER" \
  "$AST_JSON" \
  "$AST_JSON_ROUNDTRIP" \
  "$PROGRAM_JSON" \
  "$MIR_BUILDER" \
  "$PARSER_TEST"

guard_expect_in_file "$TAG" "CONC-CONTEXT-001" "$TASKBOARD" "taskboard must keep context parser row"
guard_expect_in_file "$TAG" "landed-parser-json" "$TASKBOARD" "taskboard must record context parser/json landing"
guard_expect_in_file "$TAG" "Parser/AST JSON capsule is active" "$SEMANTICS" "semantics quick status must record capsule-only state"
guard_expect_in_file "$TAG" "ContextScope" "$AST" "AST must carry context scope capsule"
guard_expect_in_file "$TAG" "ContextScope" "$AST_UTILS" "AST node type helpers must distinguish context scope"
guard_expect_in_file "$TAG" "is_context_scope_statement_start" "$PARSER" "parser must keep context contextual"
guard_expect_in_file "$TAG" '"ContextScope"' "$AST_JSON" "AST JSON must emit ContextScope"
guard_expect_in_file "$TAG" '"ContextScope"' "$AST_JSON_ROUNDTRIP" "AST JSON roundtrip must decode ContextScope"
guard_expect_in_file "$TAG" "program_json_v0/context_scope_not_supported" "$PROGRAM_JSON" "Program JSON must fail-fast until propagation lands"
guard_expect_in_file "$TAG" "context_scope_lowering_missing" "$MIR_BUILDER" "MIR lowering must fail-fast until propagation lands"
guard_expect_in_file "$TAG" "parser_accepts_canonical_context_scope_surface" "$PARSER_TEST" "tests must cover canonical context spelling"
guard_expect_in_file "$TAG" "parser_accepts_scoped_compat_spelling" "$PARSER_TEST" "tests must cover scoped compat spelling"
guard_expect_in_file "$TAG" "parser_keeps_context_contextual_for_calls_and_bindings" "$PARSER_TEST" "tests must cover contextual identifiers"

legacy_hits="$(
  rg -n --glob '*.hako' --glob '!**/archive/**' --glob '!tools/archive/**' \
    '\bscoped[[:space:]]+[A-Za-z_][A-Za-z0-9_]*[[:space:]]*(?::|=)' "$ROOT_DIR" || true
)"
if [[ -n "$legacy_hits" ]]; then
  printf '%s\n' "$legacy_hits" >&2
  guard_fail "$TAG" "active .hako source still uses scoped context compatibility spelling"
fi

echo "[$TAG] ok"
