#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-loop-condition-numeric-compare-canon-selection"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-condition-numeric-compare-canon-selection-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3257-MIRBUILDER-LOOP-CONDITION-NUMERIC-COMPARE-CANON-SELECTION-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
RUST_SHAPE="$ROOT_DIR/src/mir/builder/control_flow/plan/facts/loop_condition_shape.rs"
COND_CANON="$ROOT_DIR/src/mir/builder/control_flow/generic_loop_canon/condition/candidates.rs"
COND_BOUND="$ROOT_DIR/src/mir/builder/control_flow/generic_loop_canon/condition/bound.rs"
PROGRAMJSON_SNAPSHOT="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$RUST_SHAPE" "$COND_CANON" "$COND_BOUND" "$PROGRAMJSON_SNAPSHOT"

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$RUST_SHAPE" "$COND_CANON" "$COND_BOUND" "$PROGRAMJSON_SNAPSHOT" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, rust_path, canon_path, bound_path, snapshot_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")
rust_shape = Path(rust_path).read_text(encoding="utf-8")
cond_canon = Path(canon_path).read_text(encoding="utf-8")
cond_bound = Path(bound_path).read_text(encoding="utf-8")
snapshot = Path(snapshot_path).read_text(encoding="utf-8")

token = "MIRBUILDER-LOOP-CONDITION-NUMERIC-COMPARE-CANON-SELECTION-001"
next_card = "MIRBUILDER-RUST-CONDITION-NUMERIC-COMPARE-CANON-AUTHORITY-001"
if fixture.get("kind") != "MirBuilderLoopConditionNumericCompareCanonSelectionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
decision = fixture.get("decision") or {}
if decision.get("selected_next_card") != next_card:
    raise SystemExit("wrong selected next card")
if decision.get("supersedes_next_card") != "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-UNSUPPORTED-UPDATE-OPERATOR-REJECT-ROW-001":
    raise SystemExit("wrong superseded card")
contract = fixture.get("authority_contract") or {}
if contract.get("rust_first") is not True:
    raise SystemExit("rust_first must be true")
if contract.get("raw_rewrite_allowed") is not False:
    raise SystemExit("raw rewrite must stay forbidden")
if contract.get("lowering_change_allowed") is not False:
    raise SystemExit("lowering change must stay forbidden")
if contract.get("target_pipeline") != [
    "ProgramJSON Compare",
    "NumericCompareCanonSnapshot",
    "CanonicalLoopFacts",
    "RecipeMatcher",
]:
    raise SystemExit("target pipeline drift")
if contract.get("canonical_loop_facts_must_not_reparse_raw_compare") is not True:
    raise SystemExit("CanonicalLoopFacts boundary drift")
if contract.get("recipe_matcher_must_not_read_raw_compare_shape") is not True:
    raise SystemExit("RecipeMatcher boundary drift")
stops = fixture.get("stop_conditions") or {}
for key in [
    "per_spelling_readers_forbidden",
    "raw_ast_or_programjson_rewrite_forbidden",
    "constant_compare_loop_authority_forbidden",
    "programjson_before_rust_authority_forbidden",
]:
    if stops.get(key) is not True:
        raise SystemExit(f"missing stop condition: {key}")

rows = {row.get("row_id"): row for row in fixture.get("expected_future_rows") or []}
for row_id in [
    "accept_var_less_equal_literal",
    "accept_literal_greater_equal_var",
    "constant_numeric_compare_diagnostic",
]:
    if row_id not in rows:
        raise SystemExit(f"missing row: {row_id}")

claims = fixture.get("claims") or {}
positive = {
    "numeric_compare_canon_selected",
    "rust_condition_authority_change_required",
    "programjson_numeric_compare_parity_required",
    "programjson_recipe_matcher_consume_required",
    "unsupported_update_operator_next_superseded",
}
for key in positive:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in positive:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    token,
    next_card,
    "i <= 3",
    "3 >= i",
    "1 <= 3",
    "NumericCompareCanonSnapshot",
    "per-spelling readers",
    "raw ProgramJSON rewrite = 0",
    "Source Selfhost remains unclaimed",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [
    token,
    next_card,
    "numeric_compare_canon",
    "unsupported_update_operator_next_superseded",
]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
if f'latest_card = "{token}"' not in current_state:
    raise SystemExit("CURRENT_STATE latest card drift")

for needle in [
    "BinaryOperator::LessEqual",
    "ConditionShape::VarLessEqualLengthMinusNeedle",
    "ConditionShape::VarLessLiteral",
]:
    if needle not in rust_shape:
        raise SystemExit(f"rust shape baseline missing: {needle}")
for needle in [
    "collect_candidates_from_top_level_comparison",
    "is_supported_comparison_operator",
    "BinaryOperator::GreaterEqual",
]:
    if needle not in cond_canon:
        raise SystemExit(f"CondCanon candidate baseline missing: {needle}")
for needle in [
    "extract_bound_from_condition",
    "bound_from_expr(left)",
    "bound_from_expr(right)",
    "BoundExpr::LiteralI64",
]:
    if needle not in cond_bound:
        raise SystemExit(f"CondCanon bound baseline missing: {needle}")
for needle in [
    "_read_var_lt_int",
    "unsupported_loop_cond",
]:
    if needle not in snapshot:
        raise SystemExit(f"ProgramJSON snapshot baseline missing: {needle}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-loop-condition-numeric-compare-canon-selection-v0
token=MIRBUILDER-LOOP-CONDITION-NUMERIC-COMPARE-CANON-SELECTION-001
numeric_compare_canon_selected=1
selected_next_card=MIRBUILDER-RUST-CONDITION-NUMERIC-COMPARE-CANON-AUTHORITY-001
rust_condition_authority_change_required=1
programjson_numeric_compare_parity_required=1
programjson_recipe_matcher_consume_required=1
unsupported_update_operator_next_superseded=1
initial_row_var_le_literal=1
initial_row_literal_ge_var=1
initial_row_constant_compare_diagnostic=1
numeric_compare_canon_supported_now=0
constant_compare_loop_authority=0
programjson_recipematcher_accepted_floor_green=0
programjson_runtime_route_authority=0
runtime_route_switch=0
recipe_matcher_input_authority=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
