#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-call-lowering-name-canonicalization-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_call_lowering_name_canonicalization_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-name-canonicalization-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1905-MIRBUILDER-CALL-LOWERING-NAME-CANONICALIZATION-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-name-canonicalization-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1905-MIRBUILDER-CALL-LOWERING-NAME-CANONICALIZATION-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-CALL-LOWERING-NAME-CANONICALIZATION-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderCallLoweringNameCanonicalizationProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["selected_subcluster_id"] != "CallNameCanonicalizationHelpers":
    raise SystemExit("selected subcluster drift")
if fixture["input_state"]["source_count"] != 1:
    raise SystemExit("source count drift")

surface = fixture["source_surface"]
if surface["symbol"] != "generate_method_function_name":
    raise SystemExit("source symbol drift")

axes = fixture["selection_axes"]
expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.name_canonicalization",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if axes != expected_axes:
    raise SystemExit(f"selection axes drift: {axes}")

descriptor = fixture["name_canonicalization_descriptor"]
if descriptor["descriptor_kind"] != "MethodFunctionNameCanonicalizationV1":
    raise SystemExit("descriptor kind drift")
if descriptor["source_extracted"] is not True:
    raise SystemExit("descriptor must be source-extracted")
if descriptor["format"] != "{}.{}/{}":
    raise SystemExit("format drift")
if descriptor["parts"] != ["box_name", ".", "method_name", "/", "arity"]:
    raise SystemExit("parts drift")
expected_callsites = [
    "src/mir/builder/builder_build.rs",
    "src/mir/builder/method_call_handlers.rs",
    "src/mir/builder/record_helper_args.rs",
    "src/mir/builder/rewrite/known.rs",
]
if descriptor["callsite_paths"] != expected_callsites:
    raise SystemExit(f"callsite path drift: {descriptor['callsite_paths']}")

policy = fixture["selected_policy"]
if policy["policy"] != "MaterializeNameCanonicalizationDescriptor":
    raise SystemExit("policy drift")
if policy["owner_edge"] != "mirbuilder::call_lowering_name_canonicalization":
    raise SystemExit("owner edge drift")
if policy["projection_surface_selected"] is not False:
    raise SystemExit("projection surface must not be selected")

decision = fixture["decision"]
if decision["kind"] != "ReturnToProjectionPolicyClusterResolution":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
if claims.get("source_extracted_descriptor") != 1:
    raise SystemExit("source_extracted_descriptor must be 1")
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
output_contract=rust-lifecycle-mirbuilder-call-lowering-name-canonicalization-projection-policy-v0
subcluster=CallNameCanonicalizationHelpers
policy=MaterializeNameCanonicalizationDescriptor
source_extracted_descriptor=1
format={}.{}/{}
projection_surface_selected=0
selected_next_card=MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
