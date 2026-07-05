#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-next-capability-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-next-capability-selection-v0.json"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE"

python3 - "$FIXTURE" "$ROOT_DIR" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
root = Path(sys.argv[2])

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonNextCapabilitySelectionV1", "bad kind")
need(
    fixture.get("token") == "MIRBUILDER-PROGRAMJSON-NEXT-CAPABILITY-SELECTION-001",
    "bad token",
)

state = fixture.get("input_state") or {}
for key in ["previous_retire_candidate_guard", "policy", "dynamic_typing_inventory"]:
    path = root / (state.get(key) or "")
    need(path.exists(), f"missing input path: {key}")

selected = fixture.get("selected_capability") or {}
need(selected.get("name") == "ProgramJsonConditionShapeScanV1", "bad capability")
need(
    selected.get("next_card") == "MIRBUILDER-PROGRAMJSON-CONDITION-SHAPE-SCAN-CAPABILITY-001",
    "bad next card",
)
need(selected.get("input") == "ProgramJSON v0", "bad input contract")
need(selected.get("output") == "ConditionShapeSnapshotV1", "bad output contract")
need("condition" in (selected.get("target_projector_slice") or ""), "bad projector slice")
need(len(selected.get("why_this_next") or []) >= 4, "missing rationale")

rows = fixture.get("minimum_parity_rows") or []
need(len(rows) >= 8, "minimum parity rows must be at least 8")
for required in [
    "loop_cond_compare_var_lt_int",
    "loop_cond_compare_var_eq_int",
    "if_cond_compare_var_eq_int",
    "unsupported_call_condition",
]:
    need(required in rows, f"missing row: {required}")

acceptance = fixture.get("acceptance") or {}
need(acceptance.get("consumes_programjson_structure") == 1, "ProgramJSON consumption required")
need(acceptance.get("string_only_facade") == 0, "string-only facade forbidden")
need(acceptance.get("minimum_parity_row_count") >= 8, "row budget too small")
need(acceptance.get("retire_candidate_required") == 1, "retire candidate required")
need(acceptance.get("implementation_card_required") == 1, "implementation card required")
need(acceptance.get("parity_gate_required") == 1, "parity gate required")

stops = fixture.get("stop_conditions") or {}
for key in [
    "prebuilt_token_snapshot_input",
    "source_contains_or_regex_proof",
    "rust_astnode_projector_used_as_target_input",
    "mir_mutation_or_lowering_added",
    "recipe_matcher_execution_added",
    "unsupported_condition_silently_ignored",
]:
    need(stops.get(key) == 1, f"missing stop condition: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectProgramJsonTraversalCapability", "bad decision kind")
need(
    decision.get("selected_next_card")
    == "MIRBUILDER-PROGRAMJSON-CONDITION-SHAPE-SCAN-CAPABILITY-001",
    "bad selected next card",
)

claims = fixture.get("claims") or {}
for key in [
    "implementation_done",
    "parity_gate_green",
    "rust_astnode_projector_retire_candidate",
    "rust_astnode_projector_retired",
    "full_astnode_projector_retired",
    "programjson_full_parser_claim",
    "programjson_all_shapes_supported",
    "source_selfhost_claim",
    "hako_adopted_decision",
    "recipe_matching_migrated",
    "route_selection_migration",
    "backend_lowering_migration",
    "mir_mutation_migration",
    "id_allocation_migration",
    "new_backend_route",
    "new_abi",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")
PY

bash "$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_loop_body_control_flow_retire_rust_astnode_projector_candidate_guard.sh" >/dev/null

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-next-capability-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-NEXT-CAPABILITY-SELECTION-001
selected_capability=ProgramJsonConditionShapeScanV1
selected_next_card=MIRBUILDER-PROGRAMJSON-CONDITION-SHAPE-SCAN-CAPABILITY-001
output_contract_next=ConditionShapeSnapshotV1
minimum_parity_rows=8
consumes_programjson_structure=1
string_only_facade=0
implementation_done=0
parity_gate_green=0
rust_astnode_projector_retire_candidate=0
source_selfhost_claim=0
summary=ok
REPORT
