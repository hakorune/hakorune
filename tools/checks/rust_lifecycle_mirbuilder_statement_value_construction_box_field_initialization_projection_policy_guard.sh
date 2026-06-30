#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-statement-value-construction-box-field-initialization-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_statement_value_construction_box_field_initialization_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-statement-value-construction-box-field-initialization-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1926-MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-statement-value-construction-box-field-initialization-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1926-MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderStatementValueConstructionBoxFieldInitializationProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["selected_subcluster_id"] != "BoxFieldInitialization":
    raise SystemExit("selected subcluster drift")
if fixture["input_state"]["source_count"] != 2:
    raise SystemExit("source count drift")

surfaces = fixture["source_surfaces"]
symbols = [surface["symbol"] for surface in surfaces]
expected_symbols = [
    "build_new_expression_with_field_initializers",
    "build_box_field_initializers",
]
if symbols != expected_symbols:
    raise SystemExit(f"selected symbol drift: {symbols}")

markers_by_symbol = {
    "build_new_expression_with_field_initializers": [
        "field_initializers.is_empty()",
        "self.is_record_constructor_class(&class)",
        "[box-init/record-reject]",
        "let dst = self.build_new_expression(class.clone(), arguments)?;",
        "self.build_box_field_initializers(dst, &class, field_initializers)?;",
        "Ok(dst)",
    ],
    "build_box_field_initializers": [
        "let mut seen = std::collections::BTreeSet::new();",
        "for (field, value) in field_initializers",
        "[box-init/duplicate-field]",
        "self.comp_ctx.user_defined_boxes.contains_key(class)",
        "[box-init/unknown-field]",
        "self.build_field_assignment_from_value(object_value, field, value)?;",
        "Ok(())",
    ],
}
for surface in surfaces:
    expected = markers_by_symbol[surface["symbol"]]
    if surface["source_markers"] != expected:
        raise SystemExit(f"source marker drift for {surface['symbol']}: {surface['source_markers']}")

expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.statement_value_construction",
    "borrow_axis": "NoReturnedBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if fixture["selection_axes"] != expected_axes:
    raise SystemExit(f"selection axes drift: {fixture['selection_axes']}")

policy = fixture["selected_policy"]
if policy["policy"] != "MutationFrameContractRequired":
    raise SystemExit("policy drift")
if policy["owner_edge"] != "mirbuilder::statement_value_construction_box_field_initialization":
    raise SystemExit("owner edge drift")
if policy["projection_surface_selected"] is not False:
    raise SystemExit("projection surface must not be selected before mutation-frame contract")

for key, value in fixture["mutation_frame_evidence"].items():
    if value is not True:
        raise SystemExit(f"mutation-frame evidence must be true: {key}")

decision = fixture["decision"]
if decision["kind"] != "SelectMutationFrameContract":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-MUTATION-FRAME-CONTRACT-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "manual_family_selection",
    "projection_surface_selected",
    "hako_generation",
    "hako_shadow_projector_selected",
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
output_contract=rust-lifecycle-mirbuilder-statement-value-construction-box-field-initialization-projection-policy-v0
subcluster=BoxFieldInitialization
policy=MutationFrameContractRequired
decision=SelectMutationFrameContract
projection_surface_selected=0
source_count=2
selected_next_card=MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-MUTATION-FRAME-CONTRACT-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
