#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-exit-allowed-block-recipe-token-snapshot-hako-adoption-decision-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

DECISION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-exit-allowed-block-recipe-token-snapshot-hako-adoption-decision-v0.json"
ORACLE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-exit-allowed-block-recipe-rust-oracle-v0.json"
HAKO_SOURCE="$ROOT_DIR/lang/src/compiler/lib/exit_allowed_block_recipe.hako"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_exit_allowed_block_recipe_parity_gate.sh"

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
need(decision.get("kind") == "MirBuilderExitAllowedBlockRecipeTokenSnapshotHakoAdoptedDecisionV1", "bad kind")
need(decision.get("token") == "MIRBUILDER-EXIT-ALLOWED-BLOCK-RECIPE-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001", "bad token")

state = decision.get("input_state") or {}
hako = Path(state.get("hako_source") or "")
oracle_path = Path(state.get("rust_oracle_fixture") or "")
gate = Path(state.get("parity_gate") or "")
need(str(hako) == "lang/src/compiler/lib/exit_allowed_block_recipe.hako", "bad hako source")
need(str(oracle_path) == "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-exit-allowed-block-recipe-rust-oracle-v0.json", "bad oracle")
need(str(gate) == "tools/checks/rust_lifecycle_mirbuilder_exit_allowed_block_recipe_parity_gate.sh", "bad gate")

def sha256(path):
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()

need(sha256(hako) == state.get("hako_source_hash"), "hako hash drift")
need(sha256(oracle_path) == state.get("rust_oracle_fixture_hash"), "oracle hash drift")
need(sha256(gate) == state.get("parity_gate_hash"), "gate hash drift")

scope = decision.get("adoption_scope") or {}
need(scope.get("adopted_owner") == "exit_allowed_block_recipe.backend_safe_token_snapshot_reducer", "bad owner")
need(scope.get("rust_oracle_symbol") == "try_build_exit_allowed_block_recipe", "bad symbol")
need(scope.get("input_contract") == "BackendSafeExitAllowedBlockTokenSnapshotV1", "bad input contract")

owned = set(scope.get("owned_semantics") or [])
for field in ["exit_allowed_acceptance", "block_contract_token", "stmt_count", "item_kind_sequence", "if_mode_sequence", "loop_body_contract_token_snapshot", "reject_reason_token"]:
    need(field in owned, f"missing owned semantic: {field}")

excluded = set(scope.get("excluded_semantics") or [])
for field in ["full_AST_traversal", "try_build_exit_allowed_block_in_arena", "recursive_RecipeBodies_materialization", "RecipeTree_materialization", "CondBlockView_materialization", "NoExitBlockRecipe", "join_if_semantics", "LoopV0_lowering", "MIR_mutation", "route_selection", "ID_allocation"]:
    need(field in excluded, f"missing excluded semantic: {field}")

parity = decision.get("parity") or {}
need(parity.get("gate_status") == "Green", "parity must be Green")
need(parity.get("oracle_row_count") == 10, "row count must be 10")
rows = {row.get("case_id"): row for row in oracle.get("rows") or []}
need(rows["accept_then_only_exit"]["expected_if_modes"] == ["ThenOnlyExit"], "then-only drift")
need(rows["accept_else_only_exit"]["expected_if_modes"] == ["ElseOnlyExit"], "else-only drift")
need(rows["accept_loop_exit_allowed_body_token"]["expected_loop_body_contract"] == "ExitAllowed", "loop contract drift")
need(rows["reject_unsupported_if_condition"]["expected_reason"] == "unsupported_bool_expr", "condition reject drift")

decision_row = decision.get("decision") or {}
need(decision_row.get("kind") == "HakoAdoptedScoped", "bad decision kind")
need(decision_row.get("selected_next_card") == "MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-006", "bad next card")

claims = decision.get("claims") or {}
for key in ["source_selfhost_claim", "full_try_build_exit_allowed_block_recipe_ast_owner_adopted", "try_build_exit_allowed_block_in_arena_adopted", "recipe_bodies_materialization", "recipe_tree_materialization", "cond_block_view_adopted", "no_exit_block_recipe_adopted", "join_if_semantics_adopted", "loop_v0_lowering_adopted", "backend_capability_expansion", "mir_mutation_migrated", "route_selection_migrated", "id_allocation_migrated", "hako_generation", "runtime_fallback", "new_backend_route", "new_abi"]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")
PY

bash "$PARITY_GATE" >/dev/null

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-exit-allowed-block-recipe-token-snapshot-hako-adoption-decision-guard-v0
token=MIRBUILDER-EXIT-ALLOWED-BLOCK-RECIPE-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001
owner=exit_allowed_block_recipe.backend_safe_token_snapshot_reducer
decision=HakoAdoptedScoped
parity_gate=green
oracle_rows=10
source_selfhost_claim=0
full_ast_traversal_adopted=0
recipe_bodies_materialization=0
no_exit_block_recipe_adopted=0
join_if_semantics_adopted=0
loop_v0_lowering_adopted=0
route_selection_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
selected_next_card=MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-006
summary=ok
REPORT
