#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-plan-facts-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_plan_facts_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-plan-facts-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1894-MIRBUILDER-PLAN-FACTS-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-plan-facts-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1894-MIRBUILDER-PLAN-FACTS-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-PLAN-FACTS-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderPlanFactsProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["source_count"] != 8:
    raise SystemExit("source count drift")
symbols = [surface["symbol"] for surface in fixture["source_surfaces"]]
expected_symbols = [
    "exit_only_block_ends_with_exit_on_all_paths",
    "is_pure_value_expr_for_generic_loop",
    "is_supported_bool_expr_for_generic_loop",
    "is_supported_value_expr_for_generic_loop",
    "detect_nested_loop",
    "match_index_of_bound",
    "scan_nested_loop_body",
    "log_accept",
]
if symbols != expected_symbols:
    raise SystemExit(f"selected surface drift: {symbols}")

expected_roles = {
    "debug_accept_log_helper": 1,
    "exit_only_terminality_fact": 1,
    "generic_loop_expr_fact_predicate": 3,
    "nested_loop_body_profile_fact": 1,
    "nested_loop_presence_fact": 1,
    "scan_bound_matcher_fact": 1,
}
if fixture["role_counts"] != expected_roles:
    raise SystemExit(f"role count drift: {fixture['role_counts']}")

axes = fixture["selection_axes"]
expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.plan_facts",
    "borrow_axis": "NoReturnedOrMutableBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if axes != expected_axes:
    raise SystemExit(f"selection axes drift: {axes}")

policy = fixture["selected_policy"]
if policy["policy"] != "KeepParentOwner":
    raise SystemExit("policy drift")
if policy["projection_surface_selected"] is not False:
    raise SystemExit("projection surface must not be selected")
if policy["owner_edge"] != "mirbuilder::plan_facts":
    raise SystemExit("owner edge drift")

for marker in [
    "generic_loop 専用 expr 判定 helpers (SSOT)",
    "is_supported_value_expr_for_generic_loop",
    "is_pure_value_expr_for_generic_loop",
    "is_supported_bool_expr_for_generic_loop",
    "exit_only_block_ends_with_exit_on_all_paths",
    "detect_nested_loop",
    "match_index_of_bound",
    "Nested-loop body profile (analysis-only, no AST rewrite)",
    "Emit structured accept log",
]:
    if marker not in fixture["plan_facts_evidence"]:
        raise SystemExit(f"plan facts marker missing: {marker}")

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
output_contract=rust-lifecycle-mirbuilder-plan-facts-projection-policy-v0
policy=KeepParentOwner
projection_surface_selected=0
source_count=8
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
