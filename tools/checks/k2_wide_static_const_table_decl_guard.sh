#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"
TAG="k2-wide-static-const-table-decl"
source tools/checks/lib/cargo_test_filter_group.sh
source tools/checks/lib/guard_common.sh

echo "[$TAG] running M11b-decl guard"

run_cargo_test_filter_group "$TAG" "parser/MIR acceptance" \
  static_const_table \
  static_data_plan

guard_require_files "$TAG" \
  crates/hakorune_frontend_ast/src/ast_node.rs \
  src/parser/items/static_items.rs \
  src/stage1/program_json_v0/authority.rs \
  src/runner/json_v0_bridge/ast.rs \
  src/runner/mir_json_emit/root.rs \
  lang/src/compiler/parser/parser_box.hako \
  lang/src/compiler/stage1/json_program_box.hako \
  lang/src/shared/backend/ll_emit/ll_text_emit_box.hako

guard_expect_fixed_in_file "$TAG" \
  "StaticConstTable" \
  "crates/hakorune_frontend_ast/src/ast_node.rs" \
  "frontend AST must define the static const table node"
guard_expect_fixed_in_file "$TAG" \
  "parse_static_const_table" \
  "src/parser/items/static_items.rs" \
  "parser must own static const table parsing"
guard_expect_fixed_in_file "$TAG" \
  "static_data_plans" \
  "src/stage1/program_json_v0/authority.rs" \
  "Program(JSON v0) authority must publish static data plans"
guard_expect_fixed_in_file "$TAG" \
  "static_data_plans" \
  "src/runner/json_v0_bridge/ast.rs" \
  "JSON v0 bridge AST must preserve static data plans"
guard_expect_fixed_in_file "$TAG" \
  "static_data_plans" \
  "src/runner/mir_json_emit/root.rs" \
  "MIR JSON root must export static data plans"
guard_expect_fixed_in_file "$TAG" \
  "parse_static_const_table_decl(src, i)" \
  "lang/src/compiler/parser/parser_box.hako" \
  "Stage1 parser must route static const table declarations"
guard_expect_fixed_in_file "$TAG" \
  "static_data_raw" \
  "lang/src/compiler/stage1/json_program_box.hako" \
  "Stage1 Program(JSON) emitter must expose raw static data"
guard_expect_fixed_in_file "$TAG" \
  "StaticDataRegistryBox.emit_globals_for_root(root)" \
  "lang/src/shared/backend/ll_emit/ll_text_emit_box.hako" \
  "Stage1 LLVM text emitter must route static data globals through registry"

echo "[$TAG] ok"
