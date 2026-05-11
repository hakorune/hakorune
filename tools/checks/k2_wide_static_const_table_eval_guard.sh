#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"
source tools/checks/lib/cargo_test_filter_group.sh

echo "[k2-wide-static-const-table-eval] running M11b-eval guard"

run_cargo_test_filter_group "k2-wide-static-const-table-eval" "eval acceptance" \
  static_const_table \
  source_to_program_json_v0_emits_static_data_plans_for_static_const_table

rg -F -q 'eval_static_const_u16_expr' src/parser/items/static_items.rs
rg -F -q 'static_const_eval_u16_expr' lang/src/compiler/parser/parser_box.hako
rg -F -q 'M11b-eval' docs/development/current/main/design/static-const-table-syntax-ssot.md
rg -F -q '293x-046 M11b static const table eval landed' docs/development/current/main/CURRENT_STATE.toml

echo "[k2-wide-static-const-table-eval] ok"
