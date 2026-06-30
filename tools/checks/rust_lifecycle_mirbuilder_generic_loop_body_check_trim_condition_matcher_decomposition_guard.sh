#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-generic-loop-body-check-trim-condition-matcher-decomposition-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_generic_loop_body_check_trim_condition_matcher_decomposition.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-trim-condition-matcher-decomposition-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1912-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TRIM-CONDITION-MATCHER-DECOMPOSITION-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-trim-condition-matcher-decomposition-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1912-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TRIM-CONDITION-MATCHER-DECOMPOSITION-001.md").read_text()

token = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TRIM-CONDITION-MATCHER-DECOMPOSITION-001"
if fixture.get("kind") != "MirBuilderGenericLoopBodyCheckTrimConditionMatcherDecompositionV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

state = fixture["input_state"]
if state["selected_subcluster_id"] != "CompositeTrimConditionMatcher":
    raise SystemExit("selected subcluster drift")
if state["source_count"] != 1:
    raise SystemExit("source count drift")
if not state["source_module"].endswith("expr_matchers/mod.rs"):
    raise SystemExit("source module drift")

expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.generic_loop_body_check_trim_condition_matcher",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if fixture["selection_axes"] != expected_axes:
    raise SystemExit(f"selection axes drift: {fixture['selection_axes']}")

surface = fixture["source_surfaces"][0]
if surface["symbol"] != "matches_trim_cond_with_methodcall":
    raise SystemExit("source surface drift")
if surface["matcher_role"] != "composite_binary_and_predicate":
    raise SystemExit("matcher role drift")
if surface["return_type"] != "bool":
    raise SystemExit("return type drift")

descriptor = fixture["composition_descriptor"]
if descriptor["descriptor_id"] != "generic_loop_body_check_trim_condition_matcher_v1":
    raise SystemExit("descriptor id drift")
if descriptor["source_extraction"] != "rust_binary_and_composition":
    raise SystemExit("source extraction drift")
if descriptor["operator"] != "BinaryOperator::And":
    raise SystemExit("operator drift")
if descriptor["commutative_conjuncts"] is not True:
    raise SystemExit("commutative conjunct claim missing")
roles = descriptor["left_or_right_roles"]
if [role["required_matcher"] for role in roles] != ["matches_loop_var_compare", "matches_is_space_call"]:
    raise SystemExit(f"required matcher drift: {roles}")
if roles[0]["source_descriptor"] != "generic_loop_body_check_compare_expr_matchers_v1":
    raise SystemExit("compare descriptor dependency drift")
if roles[1]["source_descriptor"] != "generic_loop_body_check_call_expr_matchers_v1":
    raise SystemExit("call descriptor dependency drift")

policy = fixture["decomposition_policy"]
if policy["composite_descriptor_selected"] is not True:
    raise SystemExit("composite descriptor must be selected")
if policy["standalone_projection_selected"] is not False:
    raise SystemExit("standalone projection must not be selected")
if policy["new_matcher_semantics_invented"] is not False:
    raise SystemExit("new matcher semantics must not be invented")
if policy["uses_existing_compare_descriptor"] is not True:
    raise SystemExit("compare descriptor dependency missing")
if policy["uses_existing_call_descriptor"] is not True:
    raise SystemExit("call descriptor dependency missing")

decision = fixture["decision"]
if decision["kind"] != "SelectNextGenericLoopPlanSubcluster":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-PROJECTION-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
if claims.get("composite_descriptor_selected") != 1:
    raise SystemExit("composite descriptor selected claim must be 1")
for key in [
    "manual_family_selection",
    "standalone_projection_selected",
    "new_matcher_semantics_invented",
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
output_contract=rust-lifecycle-mirbuilder-generic-loop-body-check-trim-condition-matcher-decomposition-v0
subcluster=CompositeTrimConditionMatcher
source_count=1
policy=CompositeDescriptorFromExistingMatchers
composite_descriptor_selected=1
standalone_projection_selected=0
selected_next_card=MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-PROJECTION-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
