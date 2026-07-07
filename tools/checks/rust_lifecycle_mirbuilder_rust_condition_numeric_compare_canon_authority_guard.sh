#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-rust-condition-numeric-compare-canon-authority-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-rust-condition-numeric-compare-canon-authority-v0.json"
SCAN_SHAPES="$ROOT_DIR/src/mir/builder/control_flow/plan/facts/scan_shapes.rs"
LOOP_CONDITION="$ROOT_DIR/src/mir/builder/control_flow/plan/facts/loop_condition_shape.rs"
COND_BOUND="$ROOT_DIR/src/mir/builder/control_flow/generic_loop_canon/condition/bound.rs"
COND_MOD="$ROOT_DIR/src/mir/builder/control_flow/generic_loop_canon/condition/mod.rs"
ORACLE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-condition-shape-rust-oracle-v0.json"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$SCAN_SHAPES" "$LOOP_CONDITION" "$COND_BOUND" "$COND_MOD" "$ORACLE"

python3 - "$FIXTURE" "$SCAN_SHAPES" "$LOOP_CONDITION" "$COND_BOUND" "$COND_MOD" "$ORACLE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
scan_shapes = Path(sys.argv[2]).read_text(encoding="utf-8")
loop_condition = Path(sys.argv[3]).read_text(encoding="utf-8")
cond_bound = Path(sys.argv[4]).read_text(encoding="utf-8")
cond_mod = Path(sys.argv[5]).read_text(encoding="utf-8")
oracle = json.loads(Path(sys.argv[6]).read_text(encoding="utf-8"))

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderRustConditionNumericCompareCanonAuthorityV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-RUST-CONDITION-NUMERIC-COMPARE-CANON-AUTHORITY-001", "bad token")

authority = fixture.get("authority") or {}
need(authority.get("rust_condition_shape") == "ConditionShape::VarCompareBound", "bad shape authority")
need(authority.get("analysis_only") is True, "analysis_only must be true")
need(authority.get("raw_ast_rewrite") is False, "raw_ast_rewrite must be false")
need(authority.get("raw_programjson_rewrite") is False, "raw_programjson_rewrite must be false")

rows = {row.get("row_id"): row for row in fixture.get("rows") or []}
for row_id in ["var_le_bound_var", "var_le_literal", "literal_ge_var", "constant_compare_no_loop_var"]:
    need(row_id in rows, f"missing row: {row_id}")
need(rows["var_le_bound_var"].get("expected_bound_kind") == "Var", "var bound row drift")
need(rows["var_le_literal"].get("expected_bound_kind") == "LiteralI64", "literal row drift")
need(rows["literal_ge_var"].get("expected_cmp") == "Le", "inverted cmp row drift")
need(rows["constant_compare_no_loop_var"].get("expected_reason") == "no_loop_var", "constant compare reason drift")

for key in ["programjson_consume", "route_selection", "mir_lowering", "mir_mutation", "id_allocation", "runtime_route_switch", "programjson_runtime_route_authority", "runtime_fallback", "source_selfhost_claim"]:
    need((fixture.get("non_claims") or {}).get(key) == 0, f"forbidden claim drift: {key}")

need("VarCompareBound" in scan_shapes, "ConditionShape::VarCompareBound missing")
need("bound: BoundExpr" in scan_shapes, "VarCompareBound bound type drift")
need("CondParam::Cmp" in scan_shapes, "scan shape cmp param missing")
need("numeric_compare_shape" in loop_condition, "numeric compare helper missing")
need("bound_from_numeric_expr" in loop_condition, "numeric bound helper missing")
need("ConditionShape::VarCompareBound" in loop_condition, "loop condition shape emission missing")
need("condition_shape_accepts_var_le_bound_var" in loop_condition, "var bound unit test missing")
need("condition_shape_inverts_literal_ge_var" in loop_condition, "literal inversion unit test missing")
need("extract_cmp_from_condition" in cond_bound, "generic loop cmp extractor missing")
need("invert_cmp" in cond_bound, "generic loop cmp inversion missing")
need("CondParam::Cmp" in cond_mod, "generic loop cond profile cmp param missing")

oracle_rows = {row.get("case_id"): row for row in oracle.get("rows") or []}
need(oracle_rows["accept_var_less_equal_bound_var"]["expected_shape"] == "VarCompareBound", "oracle var bound row drift")
need(oracle_rows["accept_var_less_equal_literal"]["expected_shape"] == "VarCompareBound", "oracle literal row drift")
need(oracle_rows["accept_literal_greater_equal_var"]["expected_shape"] == "VarCompareBound", "oracle inverted literal row drift")
need(oracle_rows["reject_constant_numeric_compare_no_loop_var"]["expected_reason"] == "no_loop_var", "oracle constant reject drift")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-rust-condition-numeric-compare-canon-authority-guard-v0
token=MIRBUILDER-RUST-CONDITION-NUMERIC-COMPARE-CANON-AUTHORITY-001
condition_shape=VarCompareBound
analysis_only_numeric_compare_canon=1
var_bound_row=1
literal_bound_row=1
literal_reversed_row=1
constant_compare_loop_authority=0
programjson_consume=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-LOOP-CONDITION-NUMERIC-COMPARE-CANON-PARITY-001
summary=ok
REPORT
