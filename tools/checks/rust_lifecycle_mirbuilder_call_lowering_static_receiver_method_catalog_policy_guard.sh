#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-call-lowering-static-receiver-method-catalog-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_call_lowering_static_receiver_method_catalog_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-static-receiver-method-catalog-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1900-MIRBUILDER-CALL-LOWERING-STATIC-RECEIVER-METHOD-CATALOG-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-static-receiver-method-catalog-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1900-MIRBUILDER-CALL-LOWERING-STATIC-RECEIVER-METHOD-CATALOG-POLICY-001.md").read_text()

token = "MIRBUILDER-CALL-LOWERING-STATIC-RECEIVER-METHOD-CATALOG-POLICY-001"
if fixture.get("kind") != "MirBuilderCallLoweringStaticReceiverMethodCatalogPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["selected_subcluster_id"] != "StaticReceiverMethodCatalog":
    raise SystemExit("selected subcluster drift")
if fixture["input_state"]["source_count"] != 1:
    raise SystemExit("source count drift")

surface = fixture["source_surfaces"][0]
if surface["symbol"] != "has_method":
    raise SystemExit("selected surface drift")
if surface["registry_role"] != "static_receiver_method_catalog_predicate":
    raise SystemExit("registry role drift")
if surface["return_type"] != "bool":
    raise SystemExit("return type drift")

axes = fixture["selection_axes"]
expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.call_lowering",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if axes != expected_axes:
    raise SystemExit(f"selection axes drift: {axes}")

descriptor = fixture["catalog_descriptor"]
if descriptor["descriptor_id"] != "call_lowering_static_receiver_method_catalog_v1":
    raise SystemExit("descriptor id drift")
if descriptor["source_extraction"] != "rust_match_arms":
    raise SystemExit("descriptor must be source-extracted")
if descriptor["entry_count"] != 6:
    raise SystemExit("entry count drift")
if descriptor["explicit_entry_count"] != 3:
    raise SystemExit("explicit entry count drift")
if descriptor["delegated_catalog_entry_count"] != 3:
    raise SystemExit("delegated entry count drift")
if descriptor["conservative_unknown_box_policy"] != "RejectUnknownBoxes":
    raise SystemExit("unknown box policy drift")

entries = {entry["box_name"]: entry for entry in descriptor["entries"]}
expected_boxes = ["ArrayBox", "ConsoleStd", "IntegerBox", "MapBox", "MathBox", "StringBox"]
if list(entries) != expected_boxes:
    raise SystemExit(f"catalog box drift: {list(entries)}")

expected_explicit = {
    "ConsoleStd": ["print", "println", "log"],
    "IntegerBox": ["add", "sub", "mul", "div"],
    "MathBox": ["sin", "cos", "abs", "min", "max"],
}
for box, names in expected_explicit.items():
    entry = entries[box]
    if entry["catalog_kind"] != "explicit_method_names":
        raise SystemExit(f"explicit catalog kind drift: {box}")
    if entry["method_names"] != names:
        raise SystemExit(f"method names drift for {box}: {entry['method_names']}")

expected_delegated = {
    "ArrayBox": "crate::boxes::array::ArrayMethodId::from_name",
    "MapBox": "crate::boxes::MapMethodId::from_name",
    "StringBox": "crate::boxes::basic::StringMethodId::from_name",
}
for box, resolver in expected_delegated.items():
    entry = entries[box]
    if entry["catalog_kind"] != "delegated_catalog_resolver":
        raise SystemExit(f"delegated catalog kind drift: {box}")
    if entry["resolver"] != resolver:
        raise SystemExit(f"resolver drift for {box}: {entry['resolver']}")

policy = fixture["selected_policy"]
if policy["policy"] != "RegistryDescriptorFixture":
    raise SystemExit("policy drift")
if policy["registry_descriptor_selected"] is not True:
    raise SystemExit("registry descriptor must be selected")
if policy["projection_surface_selected"] is not False:
    raise SystemExit("projection surface must not be selected")
if policy["delegated_catalogs_expanded"] is not False:
    raise SystemExit("delegated catalogs must not be expanded")

decision = fixture["decision"]
if decision["kind"] != "SelectRegistryDescriptorPolicy":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-CALL-LOWERING-FEATURE-PREDICATES-PROJECTION-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
if claims.get("registry_descriptor_selected") != 1:
    raise SystemExit("registry descriptor selected claim must be 1")
for key in [
    "manual_family_selection",
    "projection_surface_selected",
    "delegated_catalogs_expanded",
    "ad_hoc_by_name_policy",
    "runtime_or_projection_policy_by_name",
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
output_contract=rust-lifecycle-mirbuilder-call-lowering-static-receiver-method-catalog-policy-v0
subcluster=StaticReceiverMethodCatalog
policy=RegistryDescriptorFixture
registry_descriptor_selected=1
projection_surface_selected=0
source_count=1
entry_count=6
explicit_entry_count=3
delegated_catalog_entry_count=3
delegated_catalogs_expanded=0
selected_next_card=MIRBUILDER-CALL-LOWERING-FEATURE-PREDICATES-PROJECTION-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
