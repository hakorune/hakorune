#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-loop-cond-plan-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_loop_cond_plan_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-plan-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1887-MIRBUILDER-LOOP-COND-PLAN-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-plan-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1887-MIRBUILDER-LOOP-COND-PLAN-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-LOOP-COND-PLAN-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderLoopCondPlanProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["source_count"] != 24:
    raise SystemExit("source count drift")

required_symbols = {
    "is_general_if_stmt",
    "body_has_any_exit",
    "branch_has_exit_or_loop",
    "is_nested_loop_allowed",
    "matches_parse_string2_shape",
    "is_conditional_update_if",
    "is_else_guard_break_if_shape",
    "is_else_only_break_if_shape",
    "is_else_only_return_if_shape",
    "is_then_only_break_if_shape",
    "is_then_only_return_if_shape",
    "is_exit_if_stmt",
    "is_exit_if_with_nested_exit",
    "is_exit_if_with_prelude",
    "is_exit_if_with_return_before_continue",
    "is_return_if_stmt",
    "returns_only_in_exit_if",
    "branch_effects_only",
    "branch_effects_only_for_break",
    "exit_prelude_is_allowed",
    "exit_prelude_is_allowed_for_break",
    "return_prelude_is_allowed",
    "then_only_return_prelude_is_allowed_local_then_return_value",
    "is_supported_nested_loop_condition",
}
symbols = {surface["symbol"] for surface in fixture["source_surfaces"]}
if symbols != required_symbols:
    raise SystemExit(f"selected surface set drift: {sorted(symbols ^ required_symbols)}")
if any(surface["return_type"] != "bool" for surface in fixture["source_surfaces"]):
    raise SystemExit("all selected surfaces must be bool predicates")

axes = fixture["selection_axes"]
expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.loop_cond_plan",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if axes != expected_axes:
    raise SystemExit(f"selection axes drift: {axes}")

expected_roles = {
    "branch_exit_helper": 5,
    "conditional_update_validator": 1,
    "else_shape_validator": 5,
    "exit_shape_validator": 6,
    "general_if_classifier": 1,
    "prelude_validator": 6,
}
if fixture["role_counts"] != expected_roles:
    raise SystemExit(f"role count drift: {fixture['role_counts']}")

policy = fixture["selected_policy"]
if policy["policy"] != "KeepParentOwner":
    raise SystemExit("policy drift")
if policy["projection_surface_selected"] is not False:
    raise SystemExit("projection surface must not be selected")
if policy["owner_edge"] != "mirbuilder::loop_cond_plan":
    raise SystemExit("owner edge drift")

for marker in [
    "loop_cond_break_continue facts extraction",
    "pattern validators",
    "is_supported_bool_expr_with_canon",
    "branch_has_exit_or_loop",
    "exit_prelude_is_allowed",
    "return_prelude_is_allowed",
    "is_exit_if_stmt",
    "allow_extended",
    "allow_return",
    "ASTNode::If",
    "ASTNode::Break",
    "ASTNode::Return",
]:
    if marker not in fixture["plan_evidence"]:
        raise SystemExit(f"plan marker missing: {marker}")

decision = fixture["decision"]
if decision["kind"] != "KeepParentOwner":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "manual_family_selection",
    "projection_surface_selected",
    "hako_generation",
    "hako_adopted_decision",
    "native_seed_materialization",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim must be 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-loop-cond-plan-projection-policy-v0
policy=KeepParentOwner
projection_surface_selected=0
source_count=24
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
