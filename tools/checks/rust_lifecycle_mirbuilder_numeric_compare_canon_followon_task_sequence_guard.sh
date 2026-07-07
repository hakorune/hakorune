#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-numeric-compare-canon-followon-task-sequence-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-numeric-compare-canon-followon-task-sequence-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3259-MIRBUILDER-NUMERIC-COMPARE-CANON-FOLLOWON-TASK-SEQUENCE-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER"

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-NUMERIC-COMPARE-CANON-FOLLOWON-TASK-SEQUENCE-001"
next_card = "MIRBUILDER-PROGRAMJSON-LOOP-CONDITION-NUMERIC-COMPARE-CANON-PARITY-001"
bool_design = "MIRBUILDER-BOOL-RECIPE-COMPARE-BOUNDARY-DESIGN-001"
consume = "MIRBUILDER-CANONICAL-LOOP-FACTS-NUMERIC-COMPARE-CANON-CONSUME-001"
publication = "MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-PARITY-001"

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderNumericCompareCanonFollowonTaskSequenceV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(fixture.get("selected_sequence") == [next_card, bool_design, consume, publication], "bad selected sequence")

next_state = fixture.get("next_card") or {}
need(next_state.get("token") == next_card, "bad next card")
need(next_state.get("input") == "ProgramJSON Compare", "bad input")
need(next_state.get("output") == "NumericCompareCanonSnapshotV1", "bad output")
need(next_state.get("rust_oracle") == "ConditionShape::VarCompareBound", "bad rust oracle")
need(next_state.get("analysis_only") is True, "analysis_only must be true")

allowed = fixture.get("next_card_allowed_claims") or {}
for key in [
    "numeric_compare_canon_snapshot_v1",
    "programjson_compare_to_numeric_compare_canon",
    "rust_oracle_parity_for_numeric_compare_canon",
    "bound_expr_shared",
    "analysis_only",
]:
    need(allowed.get(key) == 1, f"missing allowed claim: {key}")

forbidden = fixture.get("next_card_forbidden_claims") or {}
for key in [
    "canonical_loop_facts_consume",
    "recipe_matcher_input_authority",
    "bool_recipe_lowering",
    "mir_cmp_emission",
    "branch_emission",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "source_selfhost_claim",
]:
    need(forbidden.get(key) == 0, f"forbidden claim drift: {key}")

guardrails = fixture.get("guardrails") or {}
for key in [
    "raw_ast_rewrite",
    "raw_programjson_rewrite",
    "i_le_3_to_i_lt_4_rewrite",
    "canonical_loop_facts_reads_raw_compare",
    "bool_recipe_carries_parser_offsets",
    "variable_variable_reversed_without_context_claim",
    "literal_only_bound_kind_design",
]:
    need(guardrails.get(key) == 0, f"guardrail drift: {key}")

for needle in [token, next_card, bool_design, consume, publication]:
    need(needle in card, f"card missing {needle}")
need("next active:\n  " + next_card in task_order, "task-order next active drift")
need("next_after_active_3 =\n  " + bool_design in task_order, "task-order next_after drift")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-numeric-compare-canon-followon-task-sequence-guard-v0
token=MIRBUILDER-NUMERIC-COMPARE-CANON-FOLLOWON-TASK-SEQUENCE-001
selected_next_card=MIRBUILDER-PROGRAMJSON-LOOP-CONDITION-NUMERIC-COMPARE-CANON-PARITY-001
next_after_active_3=MIRBUILDER-BOOL-RECIPE-COMPARE-BOUNDARY-DESIGN-001
numeric_compare_canon_snapshot_v1=1
canonical_loop_facts_consume=0
bool_recipe_lowering=0
mir_cmp_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
source_selfhost_claim=0
summary=ok
REPORT
