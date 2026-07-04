#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-exit-only-block-recipe-token-snapshot-hako-adoption-decision-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

DECISION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-exit-only-block-recipe-token-snapshot-hako-adoption-decision-v0.json"
ORACLE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-exit-only-block-recipe-rust-oracle-v0.json"
HAKO_SOURCE="$ROOT_DIR/lang/src/compiler/lib/exit_only_block_recipe.hako"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_exit_only_block_recipe_parity_gate.sh"

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
    decision.get("kind") == "MirBuilderExitOnlyBlockRecipeTokenSnapshotHakoAdoptedDecisionV1",
    "bad kind",
)
need(
    decision.get("token")
    == "MIRBUILDER-EXIT-ONLY-BLOCK-RECIPE-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001",
    "bad token",
)

input_state = decision.get("input_state") or {}
hako_source_path = Path(input_state.get("hako_source") or "")
oracle_path = Path(input_state.get("rust_oracle_fixture") or "")
parity_gate_path = Path(input_state.get("parity_gate") or "")
need(str(hako_source_path) == "lang/src/compiler/lib/exit_only_block_recipe.hako", "bad hako source")
need(
    str(oracle_path)
    == "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-exit-only-block-recipe-rust-oracle-v0.json",
    "bad rust oracle fixture",
)
need(
    str(parity_gate_path)
    == "tools/checks/rust_lifecycle_mirbuilder_exit_only_block_recipe_parity_gate.sh",
    "bad parity gate",
)


def sha256(path):
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


need(sha256(hako_source_path) == input_state.get("hako_source_hash"), "hako source hash drift")
need(sha256(oracle_path) == input_state.get("rust_oracle_fixture_hash"), "oracle fixture hash drift")
need(sha256(parity_gate_path) == input_state.get("parity_gate_hash"), "parity gate hash drift")

scope = decision.get("adoption_scope") or {}
need(
    scope.get("adopted_owner") == "exit_only_block_recipe.backend_safe_token_snapshot_reducer",
    "bad adopted owner scope",
)
need(scope.get("rust_oracle_symbol") == "try_build_exit_only_block_recipe", "bad rust oracle symbol")
need(scope.get("input_contract") == "BackendSafeExitOnlyBlockTokenSnapshotV1", "bad input contract")
need(
    scope.get("native_edit_authority") == "lang/src/compiler/lib/exit_only_block_recipe.hako",
    "bad native edit authority",
)

owned = set(scope.get("owned_semantics") or [])
for field in [
    "exit_only_acceptance",
    "block_contract_token",
    "stmt_count",
    "item_kind_sequence",
    "if_mode_sequence",
    "ends_with_exit_on_all_paths",
    "reject_reason_token",
]:
    need(field in owned, f"missing owned semantic: {field}")

excluded = set(scope.get("excluded_semantics") or [])
for field in [
    "full_AST_traversal",
    "recursive_RecipeBodies_materialization",
    "RecipeTree_materialization",
    "CondBlockView_materialization",
    "ExitAllowedBlockRecipe",
    "NoExitBlockRecipe",
    "join_if_semantics",
    "loop_body_recipe",
    "backend_lowering_capability_expansion",
    "MIR_mutation",
    "route_selection",
    "ID_allocation",
]:
    need(field in excluded, f"missing excluded semantic: {field}")

parity = decision.get("parity") or {}
need(parity.get("gate_status") == "Green", "parity status must be Green")
need(parity.get("oracle_row_count") == 8, "oracle row count must be 8")
required_rows = set(parity.get("required_rows") or [])
for row in [
    "accept_break",
    "accept_stmt_then_return",
    "accept_if_exit_all",
    "accept_if_exit_if_not_all_paths",
    "accept_effect_only_not_all_paths",
    "reject_empty",
    "reject_then_fallthrough_else_exit",
    "reject_unsupported_if_condition",
]:
    need(row in required_rows, f"missing required row: {row}")

oracle_rows = {row.get("case_id"): row for row in oracle.get("rows") or []}
need(oracle_rows["accept_break"]["expected_ends_all"] is True, "break ends_all drift")
need(oracle_rows["accept_if_exit_all"]["expected_if_modes"] == ["ExitAll"], "exit_all mode drift")
need(oracle_rows["accept_if_exit_if_not_all_paths"]["expected_ends_all"] is False, "exit_if ends_all drift")
need(oracle_rows["accept_effect_only_not_all_paths"]["expected_ends_all"] is False, "effect ends_all drift")
need(oracle_rows["reject_then_fallthrough_else_exit"]["expected_reason"] == "branch_not_exit_only", "branch reject drift")
need(oracle_rows["reject_unsupported_if_condition"]["expected_reason"] == "unsupported_bool_expr", "cond reject drift")

decision_row = decision.get("decision") or {}
need(decision_row.get("kind") == "HakoAdoptedScoped", "bad decision kind")
need(
    decision_row.get("selected_next_card") == "MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-005",
    "bad next card",
)
need(
    decision_row.get("backlog_card")
    == "MIRBUILDER-EXIT-ONLY-BLOCK-RECIPE-FULL-AST-AND-RECIPEBODIES-CONSULTATION-001",
    "bad backlog card",
)

claims = decision.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "full_try_build_exit_only_block_recipe_ast_owner_adopted",
    "recipe_bodies_materialization",
    "recipe_tree_materialization",
    "cond_block_view_adopted",
    "exit_allowed_block_recipe_adopted",
    "no_exit_block_recipe_adopted",
    "join_if_semantics_adopted",
    "loop_body_recipe_adopted",
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
output_contract=rust-lifecycle-mirbuilder-exit-only-block-recipe-token-snapshot-hako-adoption-decision-guard-v0
token=MIRBUILDER-EXIT-ONLY-BLOCK-RECIPE-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001
owner=exit_only_block_recipe.backend_safe_token_snapshot_reducer
decision=HakoAdoptedScoped
parity_gate=green
oracle_rows=8
source_selfhost_claim=0
full_ast_traversal_adopted=0
recipe_bodies_materialization=0
exit_allowed_block_recipe_adopted=0
no_exit_block_recipe_adopted=0
route_selection_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
selected_next_card=MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-005
summary=ok
REPORT
