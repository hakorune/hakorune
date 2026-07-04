#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-loop-break-body-local-facts-hako-adoption-decision-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

DECISION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-break-body-local-facts-hako-adoption-decision-v0.json"
ORACLE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-break-body-local-facts-rust-oracle-v0.json"
HAKO_SOURCE="$ROOT_DIR/lang/src/compiler/lib/loop_break_body_local_facts.hako"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_loop_break_body_local_facts_parity_gate.sh"

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
need(decision.get("kind") == "MirBuilderLoopBreakBodyLocalFactsHakoAdoptedDecisionV1", "bad kind")
need(decision.get("token") == "MIRBUILDER-LOOP-BREAK-BODY-LOCAL-FACTS-HAKOADOPTED-DECISION-001", "bad token")

state = decision.get("input_state") or {}
hako = Path(state.get("hako_source") or "")
oracle_path = Path(state.get("rust_oracle_fixture") or "")
gate = Path(state.get("parity_gate") or "")
need(str(hako) == "lang/src/compiler/lib/loop_break_body_local_facts.hako", "bad hako source")
need(str(oracle_path) == "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-break-body-local-facts-rust-oracle-v0.json", "bad oracle")
need(str(gate) == "tools/checks/rust_lifecycle_mirbuilder_loop_break_body_local_facts_parity_gate.sh", "bad gate")

def sha256(path):
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()

need(sha256(hako) == state.get("hako_source_hash"), "hako hash drift")
need(sha256(oracle_path) == state.get("rust_oracle_fixture_hash"), "oracle hash drift")
need(sha256(gate) == state.get("parity_gate_hash"), "gate hash drift")

scope = decision.get("adoption_scope") or {}
need(scope.get("adopted_owner") == "loop_break_body_local_facts.authority_facade", "bad owner")
need(scope.get("rust_oracle_symbol") == "try_extract_loop_break_body_local_facts", "bad symbol")
need(scope.get("input_contract") == "BackendSafeLoopBreakBodyLocalFactsTokenSnapshotV1", "bad input contract")

owned = set(scope.get("owned_semantics") or [])
for field in [
    "loop_break_body_local_acceptance",
    "condition_len_loop_token",
    "body_local_shape_token",
    "loop_var_token",
    "body_local_var_token",
    "reject_reason_token",
]:
    need(field in owned, f"missing owned semantic: {field}")

excluded = set(scope.get("excluded_semantics") or [])
for field in [
    "full_AST_traversal",
    "loop_break_subset_dispatch",
    "break_if_analysis",
    "loop_increment_extraction",
    "synthetic_break_condition_construction",
    "route_selection",
    "backend_lowering",
    "MIR_mutation",
    "ID_allocation",
]:
    need(field in excluded, f"missing excluded semantic: {field}")

parity = decision.get("parity") or {}
need(parity.get("gate_status") == "Green", "parity must be Green")
need(parity.get("oracle_row_count") == 6, "row count must be 6")
rows = {row.get("case_id"): row for row in oracle.get("rows") or []}
need(rows["accept_trim_seg"]["expected_shape"] == "trim_seg", "trim accept drift")
need(rows["accept_digit_pos"]["expected_shape"] == "digit_pos", "digit accept drift")
need(rows["accept_digit_pos_alt_names"]["expected_body_local_var"] == "c", "digit alt accept drift")
need(rows["reject_condition_not_len_loop"]["expected_reason"] == "condition_not_len_loop", "condition reject drift")
need(rows["reject_shape_not_supported"]["expected_reason"] == "body_local_shape_not_supported", "shape reject drift")
need(rows["reject_trim_loop_var_mismatch"]["expected_reason"] == "loop_var_mismatch", "mismatch reject drift")

decision_row = decision.get("decision") or {}
need(decision_row.get("kind") == "HakoAdoptedScoped", "bad decision kind")
need(decision_row.get("selected_next_card") == "MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-022", "bad next card")

claims = decision.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "full_ast_traversal_adopted",
    "loop_break_subset_dispatch_migrated",
    "break_if_analysis_migrated",
    "loop_increment_extraction_migrated",
    "synthetic_break_condition_construction_migrated",
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
output_contract=rust-lifecycle-mirbuilder-loop-break-body-local-facts-hako-adoption-decision-guard-v0
token=MIRBUILDER-LOOP-BREAK-BODY-LOCAL-FACTS-HAKOADOPTED-DECISION-001
owner=loop_break_body_local_facts.authority_facade
decision=HakoAdoptedScoped
parity_gate=green
oracle_rows=6
source_selfhost_claim=0
full_ast_traversal_adopted=0
loop_break_subset_dispatch_migrated=0
break_if_analysis_migrated=0
loop_increment_extraction_migrated=0
synthetic_break_condition_construction_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
selected_next_card=MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-022
summary=ok
REPORT
