#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-stmt-only-block-recipe-token-snapshot-hako-adoption-decision-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

DECISION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-stmt-only-block-recipe-token-snapshot-hako-adoption-decision-v0.json"
ORACLE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-stmt-only-block-recipe-rust-oracle-v0.json"
HAKO_SOURCE="$ROOT_DIR/lang/src/compiler/lib/stmt_only_block_recipe.hako"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_stmt_only_block_recipe_parity_gate.sh"

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
    decision.get("kind")
    == "MirBuilderStmtOnlyBlockRecipeTokenSnapshotHakoAdoptedDecisionV1",
    "bad kind",
)
need(
    decision.get("token")
    == "MIRBUILDER-STMT-ONLY-BLOCK-RECIPE-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001",
    "bad token",
)

input_state = decision.get("input_state") or {}
hako_source_path = Path(input_state.get("hako_source") or "")
oracle_path = Path(input_state.get("rust_oracle_fixture") or "")
parity_gate_path = Path(input_state.get("parity_gate") or "")
need(
    str(hako_source_path) == "lang/src/compiler/lib/stmt_only_block_recipe.hako",
    "bad hako source",
)
need(
    str(oracle_path)
    == "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-stmt-only-block-recipe-rust-oracle-v0.json",
    "bad rust oracle fixture",
)
need(
    str(parity_gate_path)
    == "tools/checks/rust_lifecycle_mirbuilder_stmt_only_block_recipe_parity_gate.sh",
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
    == "stmt_only_block_recipe.backend_safe_token_snapshot_reducer",
    "bad adopted owner scope",
)
need(
    scope.get("rust_oracle_symbol") == "try_build_stmt_only_block_recipe",
    "bad rust oracle symbol",
)
need(
    scope.get("input_contract") == "BackendSafeStmtOnlyBlockTokenSnapshotV1",
    "bad input contract",
)
need(
    scope.get("native_edit_authority") == "lang/src/compiler/lib/stmt_only_block_recipe.hako",
    "bad native edit authority",
)

owned = set(scope.get("owned_semantics") or [])
for field in [
    "stmt_only_acceptance",
    "stmt_count",
    "stmt_kind_sequence",
    "empty_block_rejection",
    "non_local_exit_rejection",
    "unsupported_vocab_rejection",
]:
    need(field in owned, f"missing owned semantic: {field}")

excluded = set(scope.get("excluded_semantics") or [])
for field in [
    "ScopeBox_flattening",
    "full_AST_traversal",
    "RecipeBodies_materialization",
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
    "accept_local_print",
    "accept_if_no_exit",
    "accept_loop_no_exit",
    "reject_empty",
    "reject_break",
    "reject_if_break",
    "reject_scopebox_not_flattened",
]:
    need(row in required_rows, f"missing required row: {row}")

oracle_rows = {row.get("case_id"): row for row in oracle.get("rows") or []}
need(oracle_rows["accept_local_print"]["expected_stmt_count"] == 2, "local_print stmt_count drift")
need(oracle_rows["accept_if_no_exit"]["expected_block_contract"] == "StmtOnly", "if_no_exit contract drift")
need(oracle_rows["accept_loop_no_exit"]["expected_block_contract"] == "StmtOnly", "loop_no_exit contract drift")
need(oracle_rows["reject_break"]["expected_reason"] == "contains_non_local_exit", "break reason drift")
need(oracle_rows["reject_if_break"]["expected_reason"] == "contains_non_local_exit", "if_break reason drift")
need(oracle_rows["reject_scopebox_not_flattened"]["expected_reason"] == "unsupported_stmt_vocab", "scopebox reason drift")

decision_row = decision.get("decision") or {}
need(decision_row.get("kind") == "HakoAdoptedScoped", "bad decision kind")
need(
    decision_row.get("selected_next_card")
    == "MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-003",
    "bad next card",
)
need(
    decision_row.get("backlog_card")
    == "MIRBUILDER-STMT-ONLY-BLOCK-RECIPE-FULL-AST-TRAVERSAL-CONSULTATION-001",
    "bad backlog card",
)

claims = decision.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "full_try_build_stmt_only_block_recipe_ast_owner_adopted",
    "scopebox_flattening_adopted",
    "recipe_bodies_materialization",
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
output_contract=rust-lifecycle-mirbuilder-stmt-only-block-recipe-token-snapshot-hako-adoption-decision-guard-v0
token=MIRBUILDER-STMT-ONLY-BLOCK-RECIPE-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001
owner=stmt_only_block_recipe.backend_safe_token_snapshot_reducer
decision=HakoAdoptedScoped
parity_gate=green
oracle_rows=7
source_selfhost_claim=0
full_ast_traversal_adopted=0
scopebox_flattening_adopted=0
recipe_bodies_materialization=0
backend_capability_expansion=0
route_selection_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
selected_next_card=MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-003
summary=ok
REPORT
