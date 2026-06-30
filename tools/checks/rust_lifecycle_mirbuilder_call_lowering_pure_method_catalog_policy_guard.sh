#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-call-lowering-pure-method-catalog-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_call_lowering_pure_method_catalog_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-pure-method-catalog-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1903-MIRBUILDER-CALL-LOWERING-PURE-METHOD-CATALOG-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-pure-method-catalog-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1903-MIRBUILDER-CALL-LOWERING-PURE-METHOD-CATALOG-POLICY-001.md").read_text()

token = "MIRBUILDER-CALL-LOWERING-PURE-METHOD-CATALOG-POLICY-001"
if fixture.get("kind") != "MirBuilderCallLoweringPureMethodCatalogPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["selected_feature_subcluster_id"] != "PureMethodCatalog":
    raise SystemExit("selected feature subcluster drift")
if fixture["input_state"]["source_count"] != 1:
    raise SystemExit("source count drift")

surface = fixture["source_surface"]
if surface["symbol"] != "is_pure_method":
    raise SystemExit("source symbol drift")
if surface["catalog_source"] != "match (box_name, method)":
    raise SystemExit("catalog source drift")

axes = fixture["selection_axes"]
expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.catalog_predicate",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if axes != expected_axes:
    raise SystemExit(f"selection axes drift: {axes}")

descriptor = fixture["catalog_descriptor"]
if descriptor["descriptor_kind"] != "PureMethodCatalogDescriptorV1":
    raise SystemExit("descriptor kind drift")
if descriptor["source_extracted"] is not True:
    raise SystemExit("catalog must be source-extracted")
expected_entries = [
    {"box_name": "BoolBox", "methods": ["not"], "method_count": 1},
    {"box_name": "FloatBox", "methods": ["ceil", "floor", "round"], "method_count": 3},
    {"box_name": "IntegerBox", "methods": ["abs", "toString"], "method_count": 2},
    {"box_name": "StringBox", "methods": ["length", "lower", "trim", "upper"], "method_count": 4},
]
if descriptor["entries"] != expected_entries:
    raise SystemExit(f"catalog descriptor drift: {descriptor['entries']}")
if descriptor["box_count"] != 4 or descriptor["entry_count"] != 10:
    raise SystemExit("catalog counts drift")

policy = fixture["selected_policy"]
if policy["policy"] != "MaterializeCatalogDescriptor":
    raise SystemExit("policy drift")
if policy["owner_edge"] != "mirbuilder::call_lowering_pure_method_catalog":
    raise SystemExit("owner edge drift")
if policy["registry_descriptor_selected"] is not True:
    raise SystemExit("registry descriptor must be selected")
if policy["projection_surface_selected"] is not False:
    raise SystemExit("projection surface must not be selected")

decision = fixture["decision"]
if decision["kind"] != "SelectCatalogDescriptor":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-CALL-LOWERING-VALUE-RETURN-AST-SCAN-PROJECTION-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
if claims.get("source_extracted_catalog") != 1:
    raise SystemExit("source_extracted_catalog must be 1")
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
output_contract=rust-lifecycle-mirbuilder-call-lowering-pure-method-catalog-policy-v0
subcluster=PureMethodCatalog
policy=MaterializeCatalogDescriptor
source_extracted_catalog=1
box_count=4
entry_count=10
projection_surface_selected=0
selected_next_card=MIRBUILDER-CALL-LOWERING-VALUE-RETURN-AST-SCAN-PROJECTION-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
