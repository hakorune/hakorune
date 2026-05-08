#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[k2-wide-static-const-table-eval] running M11b-eval guard"

cargo test -q static_const_table
cargo test -q source_to_program_json_v0_emits_static_data_plans_for_static_const_table

rg -F -q 'eval_static_const_u16_expr' src/parser/items/static_items.rs
rg -F -q 'static_const_eval_u16_expr' lang/src/compiler/parser/parser_box.hako
rg -F -q 'M11b-eval' docs/development/current/main/design/static-const-table-syntax-ssot.md
rg -F -q '293x-046-M11B-STATIC-CONST-TABLE-EVAL' docs/development/current/main/CURRENT_STATE.toml

echo "[k2-wide-static-const-table-eval] ok"
