#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-loop-feature-facts-token-snapshot-hako-adoption-decision-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

DECISION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-feature-facts-token-snapshot-hako-adoption-decision-v0.json"
ORACLE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-feature-facts-rust-oracle-v0.json"
HAKO_SOURCE="$ROOT_DIR/lang/src/compiler/lib/loop_feature_facts.hako"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_loop_feature_facts_parity_gate.sh"

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
    decision.get("kind") == "MirBuilderLoopFeatureFactsTokenSnapshotHakoAdoptedDecisionV1",
    "bad kind",
)
need(
    decision.get("token")
    == "MIRBUILDER-LOOP-FEATURE-FACTS-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001",
    "bad token",
)

input_state = decision.get("input_state") or {}
hako_source_path = Path(input_state.get("hako_source") or "")
oracle_path = Path(input_state.get("rust_oracle_fixture") or "")
parity_gate_path = Path(input_state.get("parity_gate") or "")
need(
    str(hako_source_path) == "lang/src/compiler/lib/loop_feature_facts.hako",
    "bad hako source",
)
need(
    str(oracle_path)
    == "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-feature-facts-rust-oracle-v0.json",
    "bad rust oracle fixture",
)
need(
    str(parity_gate_path)
    == "tools/checks/rust_lifecycle_mirbuilder_loop_feature_facts_parity_gate.sh",
    "bad parity gate",
)


def sha256(path):
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


need(sha256(hako_source_path) == input_state.get("hako_source_hash"), "hako source hash drift")
need(sha256(oracle_path) == input_state.get("rust_oracle_fixture_hash"), "oracle fixture hash drift")
need(sha256(parity_gate_path) == input_state.get("parity_gate_hash"), "parity gate hash drift")

scope = decision.get("adoption_scope") or {}
need(
    scope.get("adopted_owner")
    == "loop_feature_facts.backend_safe_token_snapshot_reducer",
    "bad adopted owner scope",
)
need(
    scope.get("rust_oracle_symbol") == "try_extract_loop_feature_facts",
    "bad rust oracle symbol",
)
need(
    scope.get("input_contract") == "BackendSafeLoopBodySnapshotTokenV1",
    "bad input contract",
)
need(
    scope.get("native_edit_authority") == "lang/src/compiler/lib/loop_feature_facts.hako",
    "bad native edit authority",
)

owned = set(scope.get("owned_semantics") or [])
for field in [
    "exit_usage",
    "nested_loop",
    "derived_exit_map",
    "value_join_none",
    "cleanup_none",
]:
    need(field in owned, f"missing owned semantic: {field}")

excluded = set(scope.get("excluded_semantics") or [])
for field in [
    "ASTNode_MapBox_ArrayBox_body_traversal",
    "Rust_body_snapshot_token_materialization",
    "MIR_mutation",
    "backend_lowering_capability_expansion",
    "route_selection",
    "ID_allocation",
]:
    need(field in excluded, f"missing excluded semantic: {field}")

parity = decision.get("parity") or {}
need(parity.get("gate_status") == "Green", "parity status must be Green")
need(parity.get("oracle_row_count") == 3, "oracle row count must be 3")
required_rows = set(parity.get("required_rows") or [])
for row in [
    "if_break_continue_return",
    "if_hidden_nested_loop",
    "if_hidden_nested_loop_break_ignored",
]:
    need(row in required_rows, f"missing required row: {row}")

oracle_rows = {row.get("case_id"): row for row in oracle.get("rows") or []}
ignored = oracle_rows.get("if_hidden_nested_loop_break_ignored")
need(ignored is not None, "missing ignored nested-loop row in oracle")
need(ignored["expected_nested_loop"] is True, "ignored row must detect nested loop")
usage = ignored["expected_exit_usage"]
need(usage == {"break": False, "continue": False, "return": False, "unwind": False}, "nested-loop exits must not count as outer exit_usage")
need(ignored["expected_exit_map_kinds"] == [], "ignored row exit_map must be empty")
need(ignored["expected_value_join"] is None, "value_join must stay null")
need(ignored["expected_cleanup"] is None, "cleanup must stay null")

decision_row = decision.get("decision") or {}
need(decision_row.get("kind") == "HakoAdoptedScoped", "bad decision kind")
need(
    decision_row.get("selected_next_card")
    == "MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-002",
    "bad next card",
)
need(
    decision_row.get("backlog_card")
    == "MIRBUILDER-AST-BODY-SNAPSHOT-TRAVERSAL-BACKEND-CAPABILITY-CONSULTATION-001",
    "bad backlog card",
)

claims = decision.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "full_try_extract_loop_feature_facts_ast_owner_adopted",
    "ast_body_snapshot_traversal_adopted",
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
output_contract=rust-lifecycle-mirbuilder-loop-feature-facts-token-snapshot-hako-adoption-decision-guard-v0
token=MIRBUILDER-LOOP-FEATURE-FACTS-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001
owner=loop_feature_facts.backend_safe_token_snapshot_reducer
decision=HakoAdoptedScoped
parity_gate=green
oracle_rows=3
source_selfhost_claim=0
full_ast_traversal_adopted=0
backend_capability_expansion=0
route_selection_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
selected_next_card=MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-002
summary=ok
REPORT
