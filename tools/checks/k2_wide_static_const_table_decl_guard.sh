#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"
source tools/checks/lib/cargo_test_filter_group.sh

echo "[k2-wide-static-const-table-decl] running M11b-decl guard"

run_cargo_test_filter_group "k2-wide-static-const-table-decl" "parser/MIR acceptance" \
  static_const_table \
  static_data_plan

rg -F -q 'StaticConstTable' src/ast/mod.rs
rg -F -q 'parse_static_const_table' src/parser/items/static_items.rs
rg -F -q 'static_data_plans' src/stage1/program_json_v0/authority.rs
rg -F -q 'static_data_plans' src/runner/json_v0_bridge/ast.rs
rg -F -q 'static_data_plans' src/runner/mir_json_emit/root.rs
rg -F -q 'parse_static_const_table_decl(src, i)' lang/src/compiler/parser/parser_box.hako
rg -F -q 'static_data_raw' lang/src/compiler/stage1/json_program_box.hako
rg -F -q 'StaticDataRegistryBox.emit_globals_for_root(root)' lang/src/shared/backend/ll_emit/ll_text_emit_box.hako

echo "[k2-wide-static-const-table-decl] ok"
