#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-join-i-r-route-verify-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_join_i_r_route_verify_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-join-i-r-route-verify-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1884-MIRBUILDER-JOIN-I-R-ROUTE-VERIFY-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-join-i-r-route-verify-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1884-MIRBUILDER-JOIN-I-R-ROUTE-VERIFY-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-JOIN-I-R-ROUTE-VERIFY-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderJoinIRRouteVerifyProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["source_count"] != 53:
    raise SystemExit("source count drift")

axes = fixture["selection_axes"]
expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.join_i_r_route_verify",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if axes != expected_axes:
    raise SystemExit(f"selection axes drift: {axes}")

buckets = fixture["helper_bucket_summary"]
for bucket in [
    "edgecfg_compose",
    "facts_or_recognizer",
    "merge_contract_or_logging",
    "merge_rewriter",
    "recipe_index",
    "verify_diagnostic",
    "verify_observability",
]:
    if bucket not in buckets:
        raise SystemExit(f"helper bucket missing: {bucket}")

symbols = {item["symbol"] for item in fixture["source_surfaces"]}
for symbol in [
    "is_supported_bool_expr_with_canon",
    "detect_break_in_body",
    "detect_continue_in_body",
    "emit_flowbox_adopt_tag",
    "emit_flowbox_freeze_contract",
    "clear_last_plan_reject_detail",
    "should_skip_boundary_input_const",
    "start_index",
    "end_index",
]:
    if symbol not in symbols:
        raise SystemExit(f"source symbol missing: {symbol}")

policy = fixture["selected_policy"]
if policy["policy"] != "KeepParentOwner":
    raise SystemExit("policy drift")
if policy["projection_surface_selected"] is not False:
    raise SystemExit("projection surface must not be selected")
if policy["owner_edge"] != "mirbuilder::join_i_r_route_verify":
    raise SystemExit("owner edge drift")

for marker in [
    "ControlFlowDetector",
    "is_supported_bool_expr_with_canon",
    "detect_break_in_body",
    "detect_continue_in_body",
    "FlowboxVia",
    "emit_flowbox_adopt_tag",
    "Freeze::",
    "planner_reject_detail",
    "is_effect_only_stmt",
    "should_skip_",
    "start_index",
    "end_index",
]:
    if marker not in fixture["route_verify_evidence"]:
        raise SystemExit(f"route verify marker missing: {marker}")

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
output_contract=rust-lifecycle-mirbuilder-join-i-r-route-verify-projection-policy-v0
policy=KeepParentOwner
projection_surface_selected=0
source_count=53
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
