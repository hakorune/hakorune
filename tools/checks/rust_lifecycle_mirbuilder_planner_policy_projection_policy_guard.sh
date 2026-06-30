#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-planner-policy-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_planner_policy_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-planner-policy-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1892-MIRBUILDER-PLANNER-POLICY-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-planner-policy-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1892-MIRBUILDER-PLANNER-POLICY-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-PLANNER-POLICY-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderPlannerPolicyProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["source_count"] != 2:
    raise SystemExit("source count drift")
symbols = [surface["symbol"] for surface in fixture["source_surfaces"]]
if symbols != ["planner_first_tag", "planner_first_tag_with_label"]:
    raise SystemExit(f"selected surface drift: {symbols}")
if {surface["return_type"] for surface in fixture["source_surfaces"]} != {"String"}:
    raise SystemExit("return type drift")

expected_roles = {
    "planner_first_stable_tag_builder": 1,
    "planner_first_stable_tag_with_label_builder": 1,
}
if fixture["role_counts"] != expected_roles:
    raise SystemExit(f"role count drift: {fixture['role_counts']}")

axes = fixture["selection_axes"]
expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.planner_policy",
    "borrow_axis": "NoBorrow",
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
if policy["owner_edge"] != "mirbuilder::planner_policy":
    raise SystemExit("owner edge drift")

for marker in [
    "Planner tag SSOT",
    "Keep `[joinir/planner_first rule=...]` tags stable across refactors",
    "Prevent TSV expectations from drifting due to incidental formatting changes",
    "planner_rule_tag_name",
    "planner_rule_semantic_label",
    "planner_first_tag",
    "planner_first_tag_with_label",
]:
    if marker not in fixture["planner_policy_evidence"]:
        raise SystemExit(f"planner policy marker missing: {marker}")

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
output_contract=rust-lifecycle-mirbuilder-planner-policy-projection-policy-v0
policy=KeepParentOwner
projection_surface_selected=0
source_count=2
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
