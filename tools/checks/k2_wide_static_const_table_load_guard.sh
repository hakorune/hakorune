#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"
source tools/checks/lib/cargo_test_filter_group.sh

echo "[k2-wide-static-const-table-load] running M11b-load guard"

run_cargo_test_filter_group "k2-wide-static-const-table-load" "load acceptance" \
  static_const_table_load

rg -F -q 'StaticDataLoad' src/mir/instruction.rs
rg -F -q 'StaticDataLoad' src/mir/contracts/backend_core_ops.rs
rg -F -q 'static_data_load' src/runner/mir_json_emit/emitters/basic.rs
rg -F -q 'static_data_load' lang/src/shared/backend/ll_emit/ll_text_emit_box.hako
rg -F -q 'handle_static_data_load' src/backend/mir_interpreter/handlers/memory.rs

echo "[k2-wide-static-const-table-load] ok"
