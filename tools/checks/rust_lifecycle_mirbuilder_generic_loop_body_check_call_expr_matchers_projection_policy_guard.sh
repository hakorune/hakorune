#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-generic-loop-body-check-call-expr-matchers-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_generic_loop_body_check_call_expr_matchers_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-call-expr-matchers-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1909-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CALL-EXPR-MATCHERS-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-call-expr-matchers-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1909-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CALL-EXPR-MATCHERS-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CALL-EXPR-MATCHERS-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderGenericLoopBodyCheckCallExprMatchersProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

state = fixture["input_state"]
if state["selected_subcluster_id"] != "CallExprMatchers":
    raise SystemExit("selected subcluster drift")
if state["source_count"] != 2:
    raise SystemExit("source count drift")
if not state["source_module"].endswith("expr_matchers/call.rs"):
    raise SystemExit("source module drift")

expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.generic_loop_body_check_call_expr_matchers",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if fixture["selection_axes"] != expected_axes:
    raise SystemExit(f"selection axes drift: {fixture['selection_axes']}")

surfaces = fixture["source_surfaces"]
if [surface["symbol"] for surface in surfaces] != [
    "matches_is_space_call",
    "matches_substring_call_with_loop_var",
]:
    raise SystemExit(f"source surface drift: {surfaces}")
for surface in surfaces:
    if surface["matcher_role"] != "method_call_predicate":
        raise SystemExit("matcher role drift")
    if surface["return_type"] != "bool":
        raise SystemExit("return type drift")

descriptor = fixture["matcher_descriptor"]
if descriptor["descriptor_id"] != "generic_loop_body_check_call_expr_matchers_v1":
    raise SystemExit("descriptor id drift")
if descriptor["source_extraction"] != "rust_method_call_patterns":
    raise SystemExit("descriptor extraction drift")
if descriptor["entry_count"] != 2:
    raise SystemExit("descriptor entry count drift")
if descriptor["ast_root"] != "ASTNode::MethodCall":
    raise SystemExit("descriptor ast root drift")

entries = {entry["symbol"]: entry for entry in descriptor["entries"]}
space = entries["matches_is_space_call"]
if space["method_name"] != "_is_space":
    raise SystemExit("is-space method drift")
if space["argument_policy"] != "AnyArgumentMatchesSubstringLoopVar":
    raise SystemExit("is-space argument policy drift")
if space["depends_on"] != ["matches_substring_call_with_loop_var"]:
    raise SystemExit("is-space dependency drift")

substring = entries["matches_substring_call_with_loop_var"]
if substring["method_name"] != "substring":
    raise SystemExit("substring method drift")
if substring["argument_policy"] != "TwoArgumentLoopVarPlusMinusOneWindow":
    raise SystemExit("substring argument policy drift")
if substring["accepted_argument_shapes"] != [["LoopVar", "LoopVarPlusOne"], ["LoopVarMinusOne", "LoopVar"]]:
    raise SystemExit("substring accepted argument shape drift")
if substring["depends_on"] != ["is_loop_var_plus_one", "is_loop_var_minus_one"]:
    raise SystemExit("substring dependency drift")

policy = fixture["selected_policy"]
if policy["policy"] != "SourceExtractedCallMatcherDescriptor":
    raise SystemExit("selected policy drift")
if policy["matcher_descriptor_selected"] is not True:
    raise SystemExit("matcher descriptor must be selected")
if policy["hako_projection_selected"] is not False:
    raise SystemExit("Hako projection must not be selected")

decision = fixture["decision"]
if decision["kind"] != "SelectMatcherDescriptorPolicy":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-COMPARE-EXPR-MATCHERS-PROJECTION-POLICY-001":
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
output_contract=rust-lifecycle-mirbuilder-generic-loop-body-check-call-expr-matchers-projection-policy-v0
subcluster=CallExprMatchers
source_count=2
policy=SourceExtractedCallMatcherDescriptor
matcher_descriptor_selected=1
hako_projection_selected=0
selected_next_card=MIRBUILDER-GENERIC-LOOP-BODY-CHECK-COMPARE-EXPR-MATCHERS-PROJECTION-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
