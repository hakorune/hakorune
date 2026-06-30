#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-generic-loop-body-check-control-return-expr-matchers-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_generic_loop_body_check_control_return_expr_matchers_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-control-return-expr-matchers-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1911-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CONTROL-RETURN-EXPR-MATCHERS-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-control-return-expr-matchers-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1911-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CONTROL-RETURN-EXPR-MATCHERS-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CONTROL-RETURN-EXPR-MATCHERS-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderGenericLoopBodyCheckControlReturnExprMatchersProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

state = fixture["input_state"]
if state["selected_subcluster_id"] != "ControlReturnExprMatchers":
    raise SystemExit("selected subcluster drift")
if state["source_count"] != 6:
    raise SystemExit("source count drift")
if not state["source_module"].endswith("expr_matchers/control.rs"):
    raise SystemExit("source module drift")

expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.generic_loop_body_check_control_return_expr_matchers",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if fixture["selection_axes"] != expected_axes:
    raise SystemExit(f"selection axes drift: {fixture['selection_axes']}")

surfaces = fixture["source_surfaces"]
expected_symbols = [
    "matches_if_return_literal",
    "matches_if_return_var",
    "matches_if_return_local",
    "matches_if_else_return_literal",
    "matches_if_else_return_literal_var",
    "matches_if_else_return_literal_local",
]
if [surface["symbol"] for surface in surfaces] != expected_symbols:
    raise SystemExit(f"source surface drift: {surfaces}")
for surface in surfaces:
    if surface["matcher_role"] != "if_return_shape_predicate":
        raise SystemExit("matcher role drift")

descriptor = fixture["matcher_descriptor"]
if descriptor["descriptor_id"] != "generic_loop_body_check_control_return_expr_matchers_v1":
    raise SystemExit("descriptor id drift")
if descriptor["source_extraction"] != "rust_if_return_patterns":
    raise SystemExit("descriptor extraction drift")
if descriptor["entry_count"] != 6:
    raise SystemExit("descriptor entry count drift")
if descriptor["ast_root"] != "ASTNode::If":
    raise SystemExit("descriptor ast root drift")

entries = {entry["symbol"]: entry for entry in descriptor["entries"]}
if entries["matches_if_return_literal"]["else_body_policy"] != "Absent":
    raise SystemExit("if-return-literal else policy drift")
if entries["matches_if_return_var"]["condition_policy"] != "LoopVarEqualZero":
    raise SystemExit("if-return-var condition policy drift")
if entries["matches_if_return_local"]["then_body_policy"] != "LocalInitLiteralThenReturnLocal":
    raise SystemExit("if-return-local then policy drift")
if entries["matches_if_else_return_literal"]["else_body_policy"] != "SingleReturnIntegerLiteral":
    raise SystemExit("if-else-return-literal else policy drift")
if entries["matches_if_else_return_literal_var"]["return_value_kind"] != "OptionString":
    raise SystemExit("literal-var return kind drift")
if entries["matches_if_else_return_literal_local"]["depends_on"] != [
    "matches_loop_var_equal_literal",
    "matches_local_init_literal",
]:
    raise SystemExit("literal-local dependency drift")

policy = fixture["selected_policy"]
if policy["policy"] != "SourceExtractedControlReturnMatcherDescriptor":
    raise SystemExit("selected policy drift")
if policy["matcher_descriptor_selected"] is not True:
    raise SystemExit("matcher descriptor must be selected")
if policy["hako_projection_selected"] is not False:
    raise SystemExit("Hako projection must not be selected")

decision = fixture["decision"]
if decision["kind"] != "SelectMatcherDescriptorPolicy":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TRIM-CONDITION-MATCHER-DECOMPOSITION-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
if claims.get("matcher_descriptor_selected") != 1:
    raise SystemExit("matcher descriptor selected claim must be 1")
for key in [
    "manual_family_selection",
    "hako_projection_selected",
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

provenance = fixture["provenance"]
if provenance["tool_role"] != "FactsAdapterGuardOrchestrator":
    raise SystemExit("tool role drift")
if provenance["semantic_projection_inference"] != 0:
    raise SystemExit("tool must not infer semantic projection")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-generic-loop-body-check-control-return-expr-matchers-projection-policy-v0
subcluster=ControlReturnExprMatchers
source_count=6
policy=SourceExtractedControlReturnMatcherDescriptor
matcher_descriptor_selected=1
hako_projection_selected=0
selected_next_card=MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TRIM-CONDITION-MATCHER-DECOMPOSITION-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
