#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-emission-ssa-phi-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_emission_ssa_phi_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-emission-ssa-phi-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1934-MIRBUILDER-EMISSION-SSA-PHI-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-emission-ssa-phi-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1934-MIRBUILDER-EMISSION-SSA-PHI-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-EMISSION-SSA-PHI-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderEmissionSsaPhiProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

state = fixture["input_state"]
if state["source_count"] != 13:
    raise SystemExit("source count drift")
if set(state["selected_cluster_ids"]) != {
    "projection_policy::UnsupportedDirectShape::shape.emission_ssa_phi::FixtureMapped::EmissionSsaPhiCluster::borrow=NoBorrow::control=PhiRequired::type=Known::call=AllKnown::verifier=Present",
    "projection_policy::UnsupportedDirectShape::shape.emission_ssa_phi::FixtureMapped::EmissionSsaPhiCluster::borrow=NoReturnedBorrow::control=PhiRequired::type=Known::call=AllKnown::verifier=Present",
}:
    raise SystemExit("selected cluster ids drift")

symbols = {surface["symbol"] for surface in fixture["source_surfaces"]}
for symbol in [
    "strict_planner_required",
    "value_defined_in_current_function",
    "format_value_ids",
    "has_dominated_same_field_set_after_root",
    "check_phi_input_contract",
    "patch_phi_inputs",
    "define_phi_final",
    "define_phi_final_with_type_hint",
    "define_phi_final_fn",
    "define_phi_final_fn_with_type_hint_and_tag",
    "define_current_block_phi_final",
    "define_current_block_phi_final_with_type_hint",
]:
    if symbol not in symbols:
        raise SystemExit(f"missing source symbol: {symbol}")
if sum(1 for surface in fixture["source_surfaces"] if surface["symbol"] == "patch_phi_inputs") != 2:
    raise SystemExit("expected two patch_phi_inputs surfaces")

descriptor = fixture["emission_ssa_phi_descriptor"]
if descriptor["descriptor_id"] != "emission_ssa_phi_contract_lifecycle_v1":
    raise SystemExit("descriptor id drift")
if descriptor["returned_borrow"] != 0:
    raise SystemExit("EmissionSsaPhi descriptor must not return borrow")
if descriptor["mutation_frame"] != [
    "PHI inputs are sorted/materialized before insertion or patch",
    "builder/function PHI instruction state may be inserted or updated",
    "debug metadata may record value origin callers when debug is enabled",
]:
    raise SystemExit("mutation frame drift")
if "check_phi_input_contract" not in descriptor["contract_validators"]:
    raise SystemExit("contract validator missing")
if "define_phi_final_with_type_hint" not in descriptor["mutation_entrypoints"]:
    raise SystemExit("mutation entrypoint missing")

policy = fixture["selected_policy"]
if policy["policy"] != "EmissionSsaPhiContractLifecycleDescriptor":
    raise SystemExit("selected policy drift")
if policy["descriptor_selected"] is not True:
    raise SystemExit("descriptor must be selected")
if policy["hako_projection_selected"] is not False:
    raise SystemExit("Hako projection must not be selected")

decision = fixture["decision"]
if decision["kind"] != "SelectProjectionPolicyDescriptor":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001":
    raise SystemExit("decision next card drift")

claims = fixture["claims"]
if claims.get("descriptor_selected") != 1:
    raise SystemExit("descriptor selected claim must be 1")
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
output_contract=rust-lifecycle-mirbuilder-emission-ssa-phi-projection-policy-v0
source_count=13
policy=EmissionSsaPhiContractLifecycleDescriptor
descriptor_selected=1
hako_projection_selected=0
selected_next_card=MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
