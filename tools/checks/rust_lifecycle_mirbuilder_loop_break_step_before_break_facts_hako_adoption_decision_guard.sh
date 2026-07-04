#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-loop-break-step-before-break-facts-hako-adoption-decision-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

DECISION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-break-step-before-break-facts-hako-adoption-decision-v0.json"
ORACLE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-break-step-before-break-facts-rust-oracle-v0.json"
HAKO_SOURCE="$ROOT_DIR/lang/src/compiler/lib/loop_break_step_before_break_facts.hako"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_loop_break_step_before_break_facts_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$DECISION" "$ORACLE" "$HAKO_SOURCE" "$PARITY_GATE"

python3 - "$DECISION" "$ORACLE" <<'PY'
import hashlib
import json
import sys
from pathlib import Path

decision = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
oracle = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(decision.get("schema_version") == 0, "bad schema_version")
need(decision.get("kind") == "MirBuilderLoopBreakStepBeforeBreakFactsHakoAdoptedDecisionV1", "bad kind")
need(decision.get("token") == "MIRBUILDER-LOOP-BREAK-STEP-BEFORE-BREAK-FACTS-HAKOADOPTED-DECISION-001", "bad token")

state = decision.get("input_state") or {}
hako = Path(state.get("hako_source") or "")
oracle_path = Path(state.get("rust_oracle_fixture") or "")
gate = Path(state.get("parity_gate") or "")
need(str(hako) == "lang/src/compiler/lib/loop_break_step_before_break_facts.hako", "bad hako source")
need(str(oracle_path) == "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-break-step-before-break-facts-rust-oracle-v0.json", "bad oracle")
need(str(gate) == "tools/checks/rust_lifecycle_mirbuilder_loop_break_step_before_break_facts_parity_gate.sh", "bad gate")

def sha256(path):
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()

need(sha256(hako) == state.get("hako_source_hash"), "hako hash drift")
need(sha256(oracle_path) == state.get("rust_oracle_fixture_hash"), "oracle hash drift")
need(sha256(gate) == state.get("parity_gate_hash"), "gate hash drift")

scope = decision.get("adoption_scope") or {}
need(scope.get("adopted_owner") == "loop_break_step_before_break_facts.authority_facade", "bad owner")
need(scope.get("rust_oracle_symbol") == "try_extract_loop_break_step_before_break_subset", "bad symbol")
need(scope.get("input_contract") == "BackendSafeLoopBreakStepBeforeBreakFactsTokenSnapshotV1", "bad input contract")

owned = set(scope.get("owned_semantics") or [])
for field in [
    "loop_break_step_before_break_acceptance",
    "planner_gate_token",
    "condition_plan_subset_token",
    "control_flow_count_token",
    "body_shape_token",
    "loop_var_token",
    "carrier_var_token",
    "step_placement_token",
    "reject_reason_token",
]:
    need(field in owned, f"missing owned semantic: {field}")

excluded = set(scope.get("excluded_semantics") or [])
for field in [
    "full_AST_traversal",
    "dev_planner_environment_read",
    "break_if_AST_payload_extraction",
    "loop_increment_AST_payload_extraction",
    "carrier_update_AST_payload_extraction",
    "loop_break_subset_dispatch",
    "route_selection",
    "backend_lowering",
    "MIR_mutation",
    "ID_allocation",
]:
    need(field in excluded, f"missing excluded semantic: {field}")

parity = decision.get("parity") or {}
need(parity.get("gate_status") == "Green", "parity must be Green")
need(parity.get("oracle_row_count") == 9, "row count must be 9")
rows = {row.get("case_id"): row for row in oracle.get("rows") or []}
need(rows["accept_i_acc"]["expected_step_placement"] == "BeforeBreak", "accept i drift")
need(rows["accept_j_acc"]["expected_loop_var"] == "j", "accept j drift")
need(rows["reject_planner_required_disabled"]["expected_reason"] == "planner_required_disabled", "planner reject drift")
need(rows["reject_condition_not_plan_subset"]["expected_reason"] == "condition_not_plan_subset", "condition reject drift")
need(rows["reject_control_flow_count"]["expected_reason"] == "control_flow_count_not_supported", "control-flow reject drift")
need(rows["reject_body_len"]["expected_reason"] == "body_len_not_3", "body-len reject drift")
need(rows["reject_loop_increment_not_first"]["expected_reason"] == "loop_increment_not_first", "step reject drift")
need(rows["reject_break_if_missing"]["expected_reason"] == "break_if_missing", "break-if reject drift")
need(rows["reject_carrier_same_as_loop_var"]["expected_reason"] == "carrier_same_as_loop_var", "carrier reject drift")

decision_row = decision.get("decision") or {}
need(decision_row.get("kind") == "HakoAdoptedScoped", "bad decision kind")
need(decision_row.get("selected_next_card") == "MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-023", "bad next card")

claims = decision.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "full_ast_traversal_adopted",
    "dev_planner_gate_migrated",
    "break_if_ast_payload_migrated",
    "loop_increment_ast_payload_migrated",
    "carrier_update_ast_payload_migrated",
    "loop_break_subset_dispatch_migrated",
    "route_selection_migrated",
    "backend_lowering_migrated",
    "mir_mutation_migrated",
    "id_allocation_migrated",
    "hako_generation",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")
PY

bash "$PARITY_GATE" >/dev/null

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-loop-break-step-before-break-facts-hako-adoption-decision-guard-v0
token=MIRBUILDER-LOOP-BREAK-STEP-BEFORE-BREAK-FACTS-HAKOADOPTED-DECISION-001
owner=loop_break_step_before_break_facts.authority_facade
decision=HakoAdoptedScoped
parity_gate=green
oracle_rows=9
source_selfhost_claim=0
full_ast_traversal_adopted=0
dev_planner_gate_migrated=0
break_if_ast_payload_migrated=0
loop_increment_ast_payload_migrated=0
carrier_update_ast_payload_migrated=0
loop_break_subset_dispatch_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
selected_next_card=MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-023
summary=ok
REPORT
