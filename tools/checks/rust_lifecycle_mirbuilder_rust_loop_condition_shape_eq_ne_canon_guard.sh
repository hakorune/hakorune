#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-rust-loop-condition-shape-eq-ne-canon-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-rust-loop-condition-shape-eq-ne-canon-v0.json"
LOOP_CONDITION="$ROOT_DIR/src/mir/builder/control_flow/plan/facts/loop_condition_shape.rs"
GENERIC_BOUND="$ROOT_DIR/src/mir/builder/control_flow/generic_loop_canon/condition/bound.rs"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$LOOP_CONDITION" "$GENERIC_BOUND" "$TASK_ORDER"

python3 - "$FIXTURE" "$LOOP_CONDITION" "$GENERIC_BOUND" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
loop_condition = Path(sys.argv[2]).read_text(encoding="utf-8")
generic_bound = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderRustLoopConditionShapeEqNeCanonV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-RUST-LOOP-CONDITION-SHAPE-EQ-NE-CANON-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-LOOP-COND-RECIPE-CONSTRUCTOR-CLEANUP-001", "bad prerequisite")

contract = fixture.get("contract") or {}
need(contract.get("analysis_only") is True, "analysis_only must be true")
need(contract.get("raw_ast_rewrite") is False, "raw_ast_rewrite must be false")
need(contract.get("hako_consumer_change") is False, "hako consumer must not change")
need(contract.get("programjson_consumer_change") is False, "ProgramJSON consumer must not change")
need(contract.get("new_lowering_behavior") is False, "lowering must not change")

rows = {row.get("row_id"): row for row in fixture.get("rows") or []}
for row_id in ["var_eq_bound_var", "var_ne_literal", "literal_eq_var", "literal_ne_var", "constant_eq_compare_no_loop_var"]:
    need(row_id in rows, f"missing row: {row_id}")
need(rows["var_eq_bound_var"].get("expected_cmp") == "Eq", "Eq var row drift")
need(rows["var_ne_literal"].get("expected_cmp") == "Ne", "Ne literal row drift")
need(rows["literal_eq_var"].get("expected_idx_var") == "i", "literal Eq inversion drift")
need(rows["literal_ne_var"].get("expected_idx_var") == "i", "literal Ne inversion drift")
need(rows["constant_eq_compare_no_loop_var"].get("expected_accept") is False, "constant Eq must reject")

claims = fixture.get("claims") or {}
for key in ["rust_loop_condition_shape_eq_ne", "analysis_only_numeric_compare_canon"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "hako_consumer_change",
    "programjson_consumer_change",
    "condskeleton_ifcond",
    "recipe_matcher_input_authority",
    "bool_recipe_lowering",
    "mir_cmp_emission",
    "branch_emission",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need("BinaryOperator::Equal | BinaryOperator::NotEqual => Ok(numeric_compare_shape(" in loop_condition, "Eq/Ne not routed to numeric compare shape")
need("BinaryOperator::Equal => Some(CmpOp::Eq)" in loop_condition, "Eq cmp mapping missing")
need("BinaryOperator::NotEqual => Some(CmpOp::Ne)" in loop_condition, "Ne cmp mapping missing")
need("CmpOp::Eq => Some(CmpOp::Eq)" in loop_condition, "Eq inversion missing")
need("CmpOp::Ne => Some(CmpOp::Ne)" in loop_condition, "Ne inversion missing")
for test_name in [
    "condition_shape_accepts_var_eq_bound_var",
    "condition_shape_accepts_var_ne_literal",
    "condition_shape_inverts_literal_eq_and_ne_var",
    "condition_shape_rejects_constant_eq_compare",
]:
    need(test_name in loop_condition, f"unit test missing: {test_name}")

need("BinaryOperator::Equal => Some(CmpOp::Eq)" in generic_bound, "generic Eq mapping missing")
need("BinaryOperator::NotEqual => Some(CmpOp::Ne)" in generic_bound, "generic Ne mapping missing")
for needle in [
    "MIRBUILDER-RUST-LOOP-CONDITION-SHAPE-EQ-NE-CANON-001",
    "MIRBUILDER-CONDSKELETON-IFCOND-CONSULTATION-001",
]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

cargo test condition_shape_ --lib >/tmp/hakorune-loop-condition-shape-eq-ne-canon-cargo.log

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-rust-loop-condition-shape-eq-ne-canon-guard-v0
token=MIRBUILDER-RUST-LOOP-CONDITION-SHAPE-EQ-NE-CANON-001
owner=src/mir/builder/control_flow/plan/facts/loop_condition_shape.rs
rust_loop_condition_shape_eq_ne=1
analysis_only_numeric_compare_canon=1
var_eq_bound_var=1
var_ne_literal=1
literal_eq_var=1
literal_ne_var=1
constant_eq_compare_no_loop_var=1
hako_consumer_change=0
programjson_consumer_change=0
condskeleton_ifcond=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-CONDSKELETON-IFCOND-CONSULTATION-001
summary=ok
REPORT
