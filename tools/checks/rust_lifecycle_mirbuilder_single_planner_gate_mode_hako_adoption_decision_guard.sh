#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-single-planner-gate-mode-hako-adoption-decision-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

DECISION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-single-planner-gate-mode-hako-adoption-decision-v0.json"
ORACLE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-single-planner-gate-mode-rust-oracle-v0.json"
HAKO_SOURCE="$ROOT_DIR/lang/src/compiler/lib/single_planner_gate_mode.hako"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_single_planner_gate_mode_parity_gate.sh"

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
need(decision.get("kind") == "MirBuilderSinglePlannerGateModeHakoAdoptedDecisionV1", "bad kind")
need(decision.get("token") == "MIRBUILDER-SINGLE-PLANNER-GATE-MODE-HAKOADOPTED-DECISION-001", "bad token")

state = decision.get("input_state") or {}
hako = Path(state.get("hako_source") or "")
oracle_path = Path(state.get("rust_oracle_fixture") or "")
gate = Path(state.get("parity_gate") or "")
need(str(hako) == "lang/src/compiler/lib/single_planner_gate_mode.hako", "bad hako source")
need(str(oracle_path) == "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-single-planner-gate-mode-rust-oracle-v0.json", "bad oracle")
need(str(gate) == "tools/checks/rust_lifecycle_mirbuilder_single_planner_gate_mode_parity_gate.sh", "bad gate")

def sha256(path):
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()

need(sha256(hako) == state.get("hako_source_hash"), "hako hash drift")
need(sha256(oracle_path) == state.get("rust_oracle_fixture_hash"), "oracle hash drift")
need(sha256(gate) == state.get("parity_gate_hash"), "gate hash drift")

scope = decision.get("adoption_scope") or {}
need(scope.get("adopted_owner") == "single_planner_gate_mode.authority_facade", "bad owner")
need(scope.get("input_contract") == "BackendSafeSinglePlannerGateModeTokenSnapshotV1", "bad input contract")

owned = set(scope.get("owned_semantics") or [])
for field in [
    "strict_token",
    "dev_token",
    "planner_required_env_token",
    "strict_or_dev_dto",
    "planner_required_dto",
    "unsupported_token_reject_reason",
]:
    need(field in owned, f"missing owned semantic: {field}")

excluded = set(scope.get("excluded_semantics") or [])
for field in [
    "environment_access",
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
need(parity.get("oracle_row_count") == 7, "row count must be 7")
rows = {row.get("case_id"): row for row in oracle.get("rows") or []}
need(rows["strict_required_enabled"]["expected_summary"].endswith("planner_required=1"), "strict row drift")
need(rows["optional_when_no_strict_or_dev"]["expected_summary"].endswith("planner_required=0"), "optional row drift")
need(rows["reject_bad_strict_token"]["expected_summary"] == "accepted=0;reason=unsupported_strict_token", "strict reject drift")
need(rows["reject_bad_dev_token"]["expected_summary"] == "accepted=0;reason=unsupported_dev_token", "dev reject drift")
need(rows["reject_bad_planner_required_env_token"]["expected_summary"] == "accepted=0;reason=unsupported_planner_required_env_token", "required-env reject drift")

decision_row = decision.get("decision") or {}
need(decision_row.get("kind") == "HakoAdoptedScoped", "bad decision kind")
need(decision_row.get("selected_next_card") == "MIRBUILDER-PLAN-TRACK-NEXT-PILOT-SELECTION-004", "bad next card")

claims = decision.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "environment_access_migrated",
    "build_plan_with_facts_ctx_migrated",
    "full_try_build_outcome_migrated",
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
output_contract=rust-lifecycle-mirbuilder-single-planner-gate-mode-hako-adoption-decision-guard-v0
token=MIRBUILDER-SINGLE-PLANNER-GATE-MODE-HAKOADOPTED-DECISION-001
owner=single_planner_gate_mode.authority_facade
decision=HakoAdoptedScoped
parity_gate=green
oracle_rows=7
source_selfhost_claim=0
environment_access_migrated=0
build_plan_with_facts_ctx_migrated=0
full_try_build_outcome_migrated=0
recipe_matching_migrated=0
route_execution_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
selected_next_card=MIRBUILDER-PLAN-TRACK-NEXT-PILOT-SELECTION-004
summary=ok
REPORT
