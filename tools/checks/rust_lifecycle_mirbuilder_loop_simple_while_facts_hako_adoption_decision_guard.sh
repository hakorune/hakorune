#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-loop-simple-while-facts-hako-adoption-decision-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

DECISION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-simple-while-facts-hako-adoption-decision-v0.json"
ORACLE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-simple-while-facts-rust-oracle-v0.json"
HAKO_SOURCE="$ROOT_DIR/lang/src/compiler/lib/loop_simple_while_facts.hako"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_loop_simple_while_facts_parity_gate.sh"

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
need(decision.get("kind") == "MirBuilderLoopSimpleWhileFactsHakoAdoptedDecisionV1", "bad kind")
need(decision.get("token") == "MIRBUILDER-LOOP-SIMPLE-WHILE-FACTS-HAKOADOPTED-DECISION-001", "bad token")

state = decision.get("input_state") or {}
hako = Path(state.get("hako_source") or "")
oracle_path = Path(state.get("rust_oracle_fixture") or "")
gate = Path(state.get("parity_gate") or "")
need(str(hako) == "lang/src/compiler/lib/loop_simple_while_facts.hako", "bad hako source")
need(str(oracle_path) == "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-simple-while-facts-rust-oracle-v0.json", "bad oracle")
need(str(gate) == "tools/checks/rust_lifecycle_mirbuilder_loop_simple_while_facts_parity_gate.sh", "bad gate")

def sha256(path):
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()

need(sha256(hako) == state.get("hako_source_hash"), "hako hash drift")
need(sha256(oracle_path) == state.get("rust_oracle_fixture_hash"), "oracle hash drift")
need(sha256(gate) == state.get("parity_gate_hash"), "gate hash drift")

scope = decision.get("adoption_scope") or {}
need(scope.get("adopted_owner") == "loop_simple_while_facts.backend_safe_token_snapshot_reducer", "bad owner")
need(scope.get("rust_oracle_symbol") == "try_extract_loop_simple_while_facts", "bad symbol")
need(scope.get("input_contract") == "BackendSafeLoopSimpleWhileFactsTokenSnapshotV1", "bad input contract")

owned = set(scope.get("owned_semantics") or [])
for field in ["simple_while_acceptance", "loop_var_token", "increment_token", "reject_reason_token"]:
    need(field in owned, f"missing owned semantic: {field}")

excluded = set(scope.get("excluded_semantics") or [])
for field in ["full_AST_traversal", "loop_builder_composition", "loop_simple_while_recipe_construction", "step_only_body_policy", "loop_increment_plan_extraction", "route_selection", "backend_lowering", "MIR_mutation", "ID_allocation"]:
    need(field in excluded, f"missing excluded semantic: {field}")

parity = decision.get("parity") or {}
need(parity.get("gate_status") == "Green", "parity must be Green")
need(parity.get("oracle_row_count") == 13, "row count must be 13")
rows = {row.get("case_id"): row for row in oracle.get("rows") or []}
need(rows["accept_i_less_3_step_one"]["expected_loop_var"] == "i", "i loop var drift")
need(rows["accept_j_less_5_step_one"]["expected_loop_var"] == "j", "j loop var drift")
need(rows["reject_nested_loop"]["expected_reason"] == "has_nested_loop", "nested reject drift")
need(rows["reject_increment_not_step_one"]["expected_reason"] == "increment_not_step_one", "increment reject drift")

decision_row = decision.get("decision") or {}
need(decision_row.get("kind") == "HakoAdoptedScoped", "bad decision kind")
need(decision_row.get("selected_next_card") == "MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-010", "bad next card")

claims = decision.get("claims") or {}
for key in ["source_selfhost_claim", "full_try_extract_loop_simple_while_facts_ast_owner_adopted", "loop_builder_adopted", "loop_simple_while_recipe_adopted", "step_only_body_policy_migrated", "loop_increment_plan_migrated", "route_selection_migrated", "backend_lowering_migrated", "mir_mutation_migrated", "id_allocation_migrated", "hako_generation", "runtime_fallback", "new_backend_route", "new_abi"]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")
PY

bash "$PARITY_GATE" >/dev/null

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-loop-simple-while-facts-hako-adoption-decision-guard-v0
token=MIRBUILDER-LOOP-SIMPLE-WHILE-FACTS-HAKOADOPTED-DECISION-001
owner=loop_simple_while_facts.backend_safe_token_snapshot_reducer
decision=HakoAdoptedScoped
parity_gate=green
oracle_rows=13
source_selfhost_claim=0
full_ast_traversal_adopted=0
loop_builder_adopted=0
loop_simple_while_recipe_adopted=0
step_only_body_policy_migrated=0
loop_increment_plan_migrated=0
route_selection_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
selected_next_card=MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-010
summary=ok
REPORT
