#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-call-lowering-extern-interface-registry-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_call_lowering_extern_interface_registry_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-extern-interface-registry-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1899-MIRBUILDER-CALL-LOWERING-EXTERN-INTERFACE-REGISTRY-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-extern-interface-registry-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1899-MIRBUILDER-CALL-LOWERING-EXTERN-INTERFACE-REGISTRY-POLICY-001.md").read_text()

token = "MIRBUILDER-CALL-LOWERING-EXTERN-INTERFACE-REGISTRY-POLICY-001"
if fixture.get("kind") != "MirBuilderCallLoweringExternInterfaceRegistryPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["selected_subcluster_id"] != "ExternInterfaceRegistry":
    raise SystemExit("selected subcluster drift")
if fixture["input_state"]["source_count"] != 2:
    raise SystemExit("source count drift")

symbols = [surface["symbol"] for surface in fixture["source_surfaces"]]
if symbols != ["is_env_interface", "is_extern_function"]:
    raise SystemExit(f"selected surface drift: {symbols}")
roles = [surface["registry_role"] for surface in fixture["source_surfaces"]]
if roles != ["env_interface_membership_predicate", "extern_prefix_membership_predicate"]:
    raise SystemExit(f"registry role drift: {roles}")

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

descriptor = fixture["registry_descriptor"]
if descriptor["descriptor_id"] != "call_lowering_extern_interface_registry_v1":
    raise SystemExit("descriptor id drift")
if descriptor["source_extraction"] != "rust_starts_with_and_matches_literals":
    raise SystemExit("descriptor must be source-extracted")
if descriptor["extern_prefixes"] != ["nyash.", "env.", "system."]:
    raise SystemExit(f"extern prefix drift: {descriptor['extern_prefixes']}")
if descriptor["extern_prefix_count"] != 3:
    raise SystemExit("extern prefix count drift")
if descriptor["env_interface_count"] != 9:
    raise SystemExit("env interface count drift")
if descriptor["method_spec_surface_selected"] is not False:
    raise SystemExit("method spec surface must not be selected")

expected_interfaces = [
    "env",
    "env.canvas",
    "env.codegen",
    "env.console",
    "env.fs",
    "env.future",
    "env.net",
    "env.process",
    "env.task",
]
interfaces = [entry["name"] for entry in descriptor["env_interfaces"]]
if interfaces != expected_interfaces:
    raise SystemExit(f"env interface drift: {interfaces}")
if any(entry["interface_root"] != "env" for entry in descriptor["env_interfaces"]):
    raise SystemExit("env interface root drift")
if any(entry["is_env_interface"] is not True for entry in descriptor["env_interfaces"]):
    raise SystemExit("env interface membership drift")

policy = fixture["selected_policy"]
if policy["policy"] != "RegistryDescriptorFixture":
    raise SystemExit("policy drift")
if policy["registry_descriptor_selected"] is not True:
    raise SystemExit("registry descriptor must be selected")
if policy["projection_surface_selected"] is not False:
    raise SystemExit("projection surface must not be selected")

decision = fixture["decision"]
if decision["kind"] != "SelectRegistryDescriptorPolicy":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-CALL-LOWERING-STATIC-RECEIVER-METHOD-CATALOG-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
if claims.get("registry_descriptor_selected") != 1:
    raise SystemExit("registry descriptor selected claim must be 1")
for key in [
    "manual_family_selection",
    "projection_surface_selected",
    "method_spec_surface_selected",
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
output_contract=rust-lifecycle-mirbuilder-call-lowering-extern-interface-registry-policy-v0
subcluster=ExternInterfaceRegistry
policy=RegistryDescriptorFixture
registry_descriptor_selected=1
projection_surface_selected=0
source_count=2
extern_prefix_count=3
env_interface_count=9
method_spec_surface_selected=0
selected_next_card=MIRBUILDER-CALL-LOWERING-STATIC-RECEIVER-METHOD-CATALOG-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
