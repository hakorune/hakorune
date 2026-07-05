#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-loop-cond-continue-with-return-plan-rule-hako-adoption-decision-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

DECISION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-continue-with-return-plan-rule-hako-adoption-decision-v0.json"
ORACLE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-continue-with-return-plan-rule-rust-oracle-v0.json"
HAKO_SOURCE="$ROOT_DIR/lang/src/compiler/lib/loop_cond_continue_with_return_plan_rule.hako"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_loop_cond_continue_with_return_plan_rule_parity_gate.sh"

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
need(decision.get("kind") == "MirBuilderLoopCondContinueWithReturnPlanRuleHakoAdoptedDecisionV1", "bad kind")
need(decision.get("token") == "MIRBUILDER-LOOP-COND-CONTINUE-WITH-RETURN-PLAN-RULE-HAKOADOPTED-DECISION-001", "bad token")

state = decision.get("input_state") or {}
hako = Path(state.get("hako_source") or "")
oracle_path = Path(state.get("rust_oracle_fixture") or "")
gate = Path(state.get("parity_gate") or "")
need(str(hako) == "lang/src/compiler/lib/loop_cond_continue_with_return_plan_rule.hako", "bad hako source")
need(str(oracle_path) == "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-continue-with-return-plan-rule-rust-oracle-v0.json", "bad oracle")
need(str(gate) == "tools/checks/rust_lifecycle_mirbuilder_loop_cond_continue_with_return_plan_rule_parity_gate.sh", "bad gate")

def sha256(path):
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()

need(sha256(hako) == state.get("hako_source_hash"), "hako hash drift")
need(sha256(oracle_path) == state.get("rust_oracle_fixture_hash"), "oracle hash drift")
need(sha256(gate) == state.get("parity_gate_hash"), "gate hash drift")

scope = decision.get("adoption_scope") or {}
need(scope.get("adopted_owner") == "loop_cond_continue_with_return_plan_rule.authority_facade", "bad owner")
need(scope.get("input_contract") == "BackendSafeLoopCondContinueWithReturnPlanRuleTokenSnapshotV1", "bad input contract")

owned = set(scope.get("owned_semantics") or [])
for field in [
    "plan_rule_acceptance",
    "active_rule_order_token",
    "planner_present_token",
    "recipe_only_token",
    "planner_rule_tag_label",
    "planner_rule_display_label",
    "planner_rule_route_label",
    "reject_reason_token",
]:
    need(field in owned, f"missing owned semantic: {field}")

excluded = set(scope.get("excluded_semantics") or [])
for field in [
    "build_plan_with_facts_ctx",
    "full_try_build_outcome",
    "recipe_matching",
    "router_execution",
    "route_execution",
    "backend_lowering",
    "MIR_mutation",
    "ID_allocation",
]:
    need(field in excluded, f"missing excluded semantic: {field}")

parity = decision.get("parity") or {}
need(parity.get("gate_status") == "Green", "parity must be Green")
need(parity.get("oracle_row_count") == 5, "row count must be 5")
rows = {row.get("case_id"): row for row in oracle.get("rows") or []}
need(rows["accept_active_rule"]["expected_summary"].startswith("accepted=1;rule=LoopCondContinueWithReturn"), "accept drift")
need(rows["reject_rule_order_not_active"]["expected_reason"] == "rule_order_not_active", "order reject drift")
need(rows["reject_candidate_rule_not_active"]["expected_reason"] == "candidate_rule_not_active", "candidate reject drift")
need(rows["reject_planner_absent"]["expected_reason"] == "planner_not_present", "planner reject drift")
need(rows["reject_not_recipe_only"]["expected_reason"] == "rule_not_recipe_only", "recipe reject drift")

decision_row = decision.get("decision") or {}
need(decision_row.get("kind") == "HakoAdoptedScoped", "bad decision kind")
need(decision_row.get("selected_next_card") == "MIRBUILDER-PLAN-TRACK-NEXT-PILOT-SELECTION-001", "bad next card")

claims = decision.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "build_plan_with_facts_ctx_migrated",
    "try_build_outcome_migrated",
    "recipe_matching_migrated",
    "router_execution_migrated",
    "route_execution_migrated",
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
output_contract=rust-lifecycle-mirbuilder-loop-cond-continue-with-return-plan-rule-hako-adoption-decision-guard-v0
token=MIRBUILDER-LOOP-COND-CONTINUE-WITH-RETURN-PLAN-RULE-HAKOADOPTED-DECISION-001
owner=loop_cond_continue_with_return_plan_rule.authority_facade
decision=HakoAdoptedScoped
parity_gate=green
oracle_rows=5
source_selfhost_claim=0
build_plan_with_facts_ctx_migrated=0
try_build_outcome_migrated=0
recipe_matching_migrated=0
route_execution_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
selected_next_card=MIRBUILDER-PLAN-TRACK-NEXT-PILOT-SELECTION-001
summary=ok
REPORT
