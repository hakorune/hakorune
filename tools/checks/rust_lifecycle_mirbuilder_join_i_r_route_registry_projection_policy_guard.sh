#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-join-i-r-route-registry-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_join_i_r_route_registry_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-join-i-r-route-registry-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1883-MIRBUILDER-JOIN-I-R-ROUTE-REGISTRY-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-join-i-r-route-registry-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1883-MIRBUILDER-JOIN-I-R-ROUTE-REGISTRY-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-JOIN-I-R-ROUTE-REGISTRY-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderJoinIRRouteRegistryProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["source_count"] != 9:
    raise SystemExit("source count drift")

axes = fixture["selection_axes"]
expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.join_i_r_route_registry",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if axes != expected_axes:
    raise SystemExit(f"selection axes drift: {axes}")

symbols = {item["symbol"] for item in fixture["source_surfaces"]}
for symbol in [
    "summary",
    "pred_loop_simple_while",
    "pred_loop_cond_break_continue",
    "pred_loop_cond_continue_only",
    "pred_loop_cond_return_in_body",
    "pred_loop_true_break_continue",
    "pred_generic_loop_v1",
    "emit_planner_first",
    "loop_break_recipe_needs_flowbox_adopt_tag_in_strict",
]:
    if symbol not in symbols:
        raise SystemExit(f"source symbol missing: {symbol}")

buckets = fixture["helper_bucket_summary"]
if buckets != {"observer_summary": 1, "predicate": 6, "route_utility": 2}:
    raise SystemExit(f"helper bucket summary drift: {buckets}")

policy = fixture["selected_policy"]
if policy["policy"] != "KeepParentOwner":
    raise SystemExit("policy drift")
if policy["projection_surface_selected"] is not False:
    raise SystemExit("projection surface must not be selected")
if policy["owner_edge"] != "mirbuilder::join_i_r_route_registry":
    raise SystemExit("owner edge drift")

for marker in [
    "pred_accessor!",
    "ScanFamilyPresence",
    "pred_loop_cond_break_continue",
    "pred_generic_loop_v1",
    "planner_first_tag_with_label",
    "loop_break_recipe_needs_flowbox_adopt_tag_in_strict",
    "LoopRouteDecision",
    "summary(self)",
]:
    if marker not in fixture["route_registry_evidence"]:
        raise SystemExit(f"route registry marker missing: {marker}")

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
output_contract=rust-lifecycle-mirbuilder-join-i-r-route-registry-projection-policy-v0
policy=KeepParentOwner
projection_surface_selected=0
source_count=9
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
