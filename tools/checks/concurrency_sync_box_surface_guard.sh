#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="concurrency-sync-box-surface-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TASKBOARD="$ROOT_DIR/docs/development/current/main/design/concurrency-boundary-migration-taskboard-ssot.md"
SEMANTICS="$ROOT_DIR/docs/reference/concurrency/semantics.md"
AST="$ROOT_DIR/src/ast/mod.rs"
AST_UTILS="$ROOT_DIR/src/ast/utils/node_type.rs"
PARSER_DECLS="$ROOT_DIR/src/parser/declarations/box_def/mod.rs"
PARSER_STMTS="$ROOT_DIR/src/parser/statements/declarations.rs"
AST_JSON="$ROOT_DIR/src/macro/ast_json/joinir_compat.rs"
AST_JSON_ROUNDTRIP="$ROOT_DIR/src/macro/ast_json/roundtrip.rs"
MIR_BUILDER="$ROOT_DIR/src/mir/builder/exprs.rs"
PROGRAM_JSON="$ROOT_DIR/src/stage1/program_json_v0/authority.rs"
PARSER_TEST="$ROOT_DIR/src/tests/parser_sync_box_surface.rs"

guard_require_command "$TAG" rg
guard_require_files "$TAG" \
  "$TASKBOARD" \
  "$SEMANTICS" \
  "$AST" \
  "$AST_UTILS" \
  "$PARSER_DECLS" \
  "$PARSER_STMTS" \
  "$AST_JSON" \
  "$AST_JSON_ROUNDTRIP" \
  "$MIR_BUILDER" \
  "$PROGRAM_JSON" \
  "$PARSER_TEST"

guard_expect_in_file "$TAG" "CONC-SYNCBOX-001" "$TASKBOARD" "taskboard must keep sync box parser row"
guard_expect_in_file "$TAG" "landed-parser-json" "$TASKBOARD" "taskboard must record sync box parser/json landing"
guard_expect_in_file "$TAG" "Parser/AST JSON capsule is active" "$SEMANTICS" "semantics quick status must record capsule-only state"
guard_expect_in_file "$TAG" "is_sync: bool" "$AST" "AST must carry sync box capsule"
guard_expect_in_file "$TAG" "SyncBoxDeclaration" "$AST_UTILS" "AST node type helpers must distinguish sync box"
guard_expect_in_file "$TAG" "parse_sync_box_declaration" "$PARSER_DECLS" "parser must own sync box declaration entry"
guard_expect_in_file "$TAG" "is_sync_box_declaration_start" "$PARSER_STMTS" "statement parser must keep sync contextual"
guard_expect_in_file "$TAG" '"is_sync": is_sync' "$AST_JSON" "AST JSON must emit is_sync"
guard_expect_in_file "$TAG" 'get\("is_sync"\)' "$AST_JSON_ROUNDTRIP" "AST JSON roundtrip must decode is_sync"
guard_expect_in_file "$TAG" "sync_box_lowering_missing" "$MIR_BUILDER" "MIR lowering must fail-fast until serialized behavior lands"
guard_expect_in_file "$TAG" "program_json_v0/sync_box_not_supported" "$PROGRAM_JSON" "Program JSON must not silently lower sync box"
guard_expect_in_file "$TAG" "ast_json_roundtrip_preserves_sync_box_capsule" "$PARSER_TEST" "parser tests must pin AST JSON roundtrip"

echo "[$TAG] ok"
