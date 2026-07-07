#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-if-loop-compare-row-batch-closeout-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-if-loop-compare-row-batch-closeout-v0.json"
IF_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-if-cond-recipe-relational-row-batch-v0.json"
LOOP_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-nested-if-cond-recipe-relational-row-batch-v0.json"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
IF_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_if_cond_recipe_relational_row_batch_gate.sh"
LOOP_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_loop_nested_if_cond_recipe_relational_row_batch_gate.sh"
EMISSION_CONSULTATION_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_lowering_emission_consultation_guard.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$IF_FIXTURE" "$LOOP_FIXTURE" "$TASK_ORDER" "$IF_GATE" "$LOOP_GATE" "$EMISSION_CONSULTATION_GATE"

IF_OUT="$(guard_cached_run "$TAG" bash "$IF_GATE")"
if ! grep -q '^if_cond_recipe_relational_row_batch=1$' <<<"$IF_OUT"; then
  printf '%s\n' "$IF_OUT" >&2
  guard_fail "$TAG" "If relational row batch prerequisite is not green"
fi

LOOP_OUT="$(guard_cached_run "$TAG" bash "$LOOP_GATE")"
if ! grep -q '^loop_nested_if_cond_recipe_relational_row_batch=1$' <<<"$LOOP_OUT"; then
  printf '%s\n' "$LOOP_OUT" >&2
  guard_fail "$TAG" "Loop nested If relational row batch prerequisite is not green"
fi

EMISSION_OUT="$(guard_cached_run "$TAG" bash "$EMISSION_CONSULTATION_GATE")"
if ! grep -q '^bool_recipe_compare_emission_deferred=1$' <<<"$EMISSION_OUT"; then
  printf '%s\n' "$EMISSION_OUT" >&2
  guard_fail "$TAG" "BoolRecipe emission consultation prerequisite is not green"
fi

python3 - "$FIXTURE" "$IF_FIXTURE" "$LOOP_FIXTURE" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if_fixture = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
loop_fixture = json.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonIfLoopCompareRowBatchCloseoutV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-ROW-BATCH-CLOSEOUT-001", "bad token")
need(fixture.get("prerequisites") == [
    "MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-RELATIONAL-ROW-BATCH-001",
    "MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-RECIPE-RELATIONAL-ROW-BATCH-001",
    "MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-EMISSION-CONSULTATION-001",
], "bad prerequisites")

closed = fixture.get("closed_surface") or {}
need(closed.get("name") == "SharedIfLoopCompareRowBatchV1", "bad closed surface")
need(closed.get("top_level_if_rows") == 4, "bad top-level If row count")
need(closed.get("loop_nested_if_rows") == 4, "bad Loop nested If row count")
need(closed.get("operators") == ["<", "<=", ">", ">="], "bad operator set")
need(closed.get("legacy_cond_facts_preserved") is True, "legacy cond_facts must be preserved")
need(closed.get("closeout_is_docs_only") is False, "closeout must be guard-backed")

need(len(if_fixture.get("rows") or []) == 4, "If fixture row count drift")
need(len(loop_fixture.get("rows") or []) == 4, "Loop fixture row count drift")
need(if_fixture.get("claims", {}).get("if_cond_recipe_relational_row_batch") == 1, "If fixture claim missing")
need(loop_fixture.get("claims", {}).get("loop_nested_if_cond_recipe_relational_row_batch") == 1, "Loop fixture claim missing")

next_boundary = fixture.get("next_boundary") or {}
need(next_boundary.get("selected_next_card") == "MIRBUILDER-PROGRAMJSON-COMPARE-READER-FOLLOWON-SELECTION-001", "bad selected next")
need(next_boundary.get("mutation_bearing_lowering_owner_status") == "deferred", "mutation-bearing owner must remain deferred")

guard_contract = fixture.get("guard_contract") or {}
for key in [
    "must_run_if_relational_gate",
    "must_run_loop_nested_if_relational_gate",
    "must_run_lowering_emission_consultation_guard",
    "must_keep_mir_emission_unclaimed",
    "must_keep_runtime_authority_rust_astnode",
]:
    need(guard_contract.get(key) is True, f"guard contract missing {key}")

claims = fixture.get("claims") or {}
for key in [
    "if_loop_compare_row_batch_closeout",
    "top_level_if_relational_rows_closed",
    "loop_nested_if_relational_rows_closed",
    "shared_compare_reader_used",
    "legacy_cond_facts_preserved",
    "compare_reader_followon_selection_next",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "bool_recipe_lowering_executed",
    "mir_cmp_emission",
    "branch_emission",
    "basic_block_mutation",
    "value_id_allocation",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need("MIRBUILDER-COMPARE-LOWERING-MUTATION-OWNER-SELECTION-001" in task_order, "task-order missing future mutation owner selection")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-if-loop-compare-row-batch-closeout-guard-v0
token=MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-ROW-BATCH-CLOSEOUT-001
if_loop_compare_row_batch_closeout=1
top_level_if_relational_rows_closed=1
loop_nested_if_relational_rows_closed=1
shared_compare_reader_used=1
legacy_cond_facts_preserved=1
compare_reader_followon_selection_next=1
bool_recipe_lowering_executed=0
mir_cmp_emission=0
branch_emission=0
basic_block_mutation=0
value_id_allocation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-COMPARE-READER-FOLLOWON-SELECTION-001
summary=ok
REPORT
