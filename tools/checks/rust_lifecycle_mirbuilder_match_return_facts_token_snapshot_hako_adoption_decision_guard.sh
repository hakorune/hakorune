#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-match-return-facts-token-snapshot-hako-adoption-decision-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

DECISION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-match-return-facts-token-snapshot-hako-adoption-decision-v0.json"
ORACLE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-match-return-facts-rust-oracle-v0.json"
HAKO_SOURCE="$ROOT_DIR/lang/src/compiler/lib/match_return_facts.hako"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_match_return_facts_parity_gate.sh"

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
need(
    decision.get("kind") == "MirBuilderMatchReturnFactsTokenSnapshotHakoAdoptedDecisionV1",
    "bad kind",
)
need(
    decision.get("token") == "MIRBUILDER-MATCH-RETURN-FACTS-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001",
    "bad token",
)

input_state = decision.get("input_state") or {}
hako_source_path = Path(input_state.get("hako_source") or "")
oracle_path = Path(input_state.get("rust_oracle_fixture") or "")
parity_gate_path = Path(input_state.get("parity_gate") or "")
need(str(hako_source_path) == "lang/src/compiler/lib/match_return_facts.hako", "bad hako source")
need(
    str(oracle_path)
    == "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-match-return-facts-rust-oracle-v0.json",
    "bad rust oracle fixture",
)
need(
    str(parity_gate_path)
    == "tools/checks/rust_lifecycle_mirbuilder_match_return_facts_parity_gate.sh",
    "bad parity gate",
)


def sha256(path):
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


need(sha256(hako_source_path) == input_state.get("hako_source_hash"), "hako source hash drift")
need(sha256(oracle_path) == input_state.get("rust_oracle_fixture_hash"), "oracle fixture hash drift")
need(sha256(parity_gate_path) == input_state.get("parity_gate_hash"), "parity gate hash drift")

scope = decision.get("adoption_scope") or {}
need(
    scope.get("adopted_owner") == "match_return_facts.backend_safe_token_snapshot_reducer",
    "bad adopted owner scope",
)
need(scope.get("rust_oracle_symbol") == "try_extract_match_return_facts", "bad rust oracle symbol")
need(scope.get("input_contract") == "BackendSafeMatchReturnTokenSnapshotV1", "bad input contract")
need(
    scope.get("native_edit_authority") == "lang/src/compiler/lib/match_return_facts.hako",
    "bad native edit authority",
)

owned = set(scope.get("owned_semantics") or [])
for field in [
    "match_expr_detection",
    "scrutinee_token_support",
    "arm_count_minimum",
    "arm_label_literal_support",
    "arm_return_literal_support",
    "else_return_literal_support",
    "reject_reason_token",
]:
    need(field in owned, f"missing owned semantic: {field}")

excluded = set(scope.get("excluded_semantics") or [])
for field in [
    "strict_release_policy",
    "Freeze_construction",
    "reject_logging_handoff_tables",
    "full_AST_traversal",
    "BranchN_composition",
    "return_lowering",
    "MIR_mutation",
    "backend_lowering_capability_expansion",
    "route_selection",
    "ID_allocation",
]:
    need(field in excluded, f"missing excluded semantic: {field}")

parity = decision.get("parity") or {}
need(parity.get("gate_status") == "Green", "parity status must be Green")
need(parity.get("oracle_row_count") == 7, "oracle row count must be 7")
required_rows = set(parity.get("required_rows") or [])
for row in [
    "accept_var_int_returns",
    "accept_int_bool_returns",
    "skip_not_match_expr",
    "reject_scrutinee_unsupported",
    "reject_too_few_arms",
    "reject_non_literal_arm",
    "reject_nonliteral_else",
]:
    need(row in required_rows, f"missing required row: {row}")

oracle_rows = {row.get("case_id"): row for row in oracle.get("rows") or []}
need(oracle_rows["accept_var_int_returns"]["expected_accept"] is True, "var accept drift")
need(oracle_rows["accept_int_bool_returns"]["expected_accept"] is True, "int/bool accept drift")
need(oracle_rows["reject_scrutinee_unsupported"]["expected_reason"] == "scrutinee_not_supported", "scrutinee reason drift")
need(oracle_rows["reject_too_few_arms"]["expected_reason"] == "too_few_arms", "too few reason drift")
need(oracle_rows["reject_non_literal_arm"]["expected_reason"] == "arm_not_literal", "arm reason drift")
need(oracle_rows["reject_nonliteral_else"]["expected_reason"] == "else_not_literal", "else reason drift")

decision_row = decision.get("decision") or {}
need(decision_row.get("kind") == "HakoAdoptedScoped", "bad decision kind")
need(
    decision_row.get("selected_next_card") == "MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-004",
    "bad next card",
)
need(
    decision_row.get("backlog_card")
    == "MIRBUILDER-MATCH-RETURN-FACTS-FULL-AST-AND-FREEZE-CONSULTATION-001",
    "bad backlog card",
)

claims = decision.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "full_try_extract_match_return_facts_ast_owner_adopted",
    "strict_release_policy_adopted",
    "freeze_construction_adopted",
    "reject_logging_handoff_tables_adopted",
    "branchn_composition_adopted",
    "return_lowering_migrated",
    "backend_capability_expansion",
    "mir_mutation_migrated",
    "route_selection_migrated",
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
output_contract=rust-lifecycle-mirbuilder-match-return-facts-token-snapshot-hako-adoption-decision-guard-v0
token=MIRBUILDER-MATCH-RETURN-FACTS-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001
owner=match_return_facts.backend_safe_token_snapshot_reducer
decision=HakoAdoptedScoped
parity_gate=green
oracle_rows=7
source_selfhost_claim=0
full_ast_traversal_adopted=0
strict_release_policy_adopted=0
freeze_construction_adopted=0
branchn_composition_adopted=0
return_lowering_migrated=0
route_selection_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
selected_next_card=MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-004
summary=ok
REPORT
