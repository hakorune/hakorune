#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-call-lowering-builtin-global-function-registry-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_call_lowering_builtin_global_function_registry_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-builtin-global-function-registry-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1898-MIRBUILDER-CALL-LOWERING-BUILTIN-GLOBAL-FUNCTION-REGISTRY-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-builtin-global-function-registry-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1898-MIRBUILDER-CALL-LOWERING-BUILTIN-GLOBAL-FUNCTION-REGISTRY-POLICY-001.md").read_text()

token = "MIRBUILDER-CALL-LOWERING-BUILTIN-GLOBAL-FUNCTION-REGISTRY-POLICY-001"
if fixture.get("kind") != "MirBuilderCallLoweringBuiltinGlobalFunctionRegistryPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["selected_subcluster_id"] != "BuiltinGlobalFunctionRegistry":
    raise SystemExit("selected subcluster drift")
if fixture["input_state"]["source_count"] != 2:
    raise SystemExit("source count drift")

symbols = [surface["symbol"] for surface in fixture["source_surfaces"]]
if symbols != ["is_builtin_function", "is_math_function"]:
    raise SystemExit(f"selected surface drift: {symbols}")

roles = [surface["registry_role"] for surface in fixture["source_surfaces"]]
if roles != ["builtin_global_membership_predicate", "math_special_membership_predicate"]:
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
if descriptor["descriptor_id"] != "call_lowering_builtin_global_function_registry_v1":
    raise SystemExit("descriptor id drift")
if descriptor["source_extraction"] != "rust_matches_string_literals":
    raise SystemExit("descriptor must be source-extracted")
if descriptor["builtin_function_count"] != 12:
    raise SystemExit("builtin function count drift")
if descriptor["math_function_count"] != 9:
    raise SystemExit("math function count drift")
if descriptor["shared_builtin_math_count"] != 5:
    raise SystemExit("shared builtin/math count drift")

entries = {entry["name"]: entry for entry in descriptor["entries"]}
expected_names = [
    "abs",
    "ceil",
    "cos",
    "error",
    "exit",
    "floor",
    "gc_collect",
    "gc_stats",
    "max",
    "min",
    "now",
    "panic",
    "pow",
    "print",
    "sin",
    "sqrt",
]
if list(entries) != expected_names:
    raise SystemExit(f"registry entry drift: {list(entries)}")
for name in ["sin", "cos", "abs", "min", "max"]:
    if entries[name]["is_builtin_function"] is not True or entries[name]["is_math_function"] is not True:
        raise SystemExit(f"shared builtin/math entry drift: {name}")
for name in ["sqrt", "pow", "floor", "ceil"]:
    if entries[name]["is_builtin_function"] is not False or entries[name]["is_math_function"] is not True:
        raise SystemExit(f"math-only entry drift: {name}")
for name in ["print", "error", "panic", "exit", "now", "gc_collect", "gc_stats"]:
    if entries[name]["is_builtin_function"] is not True or entries[name]["is_math_function"] is not False:
        raise SystemExit(f"builtin-only entry drift: {name}")

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
if decision["selected_next_card"] != "MIRBUILDER-CALL-LOWERING-EXTERN-INTERFACE-REGISTRY-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
if claims.get("registry_descriptor_selected") != 1:
    raise SystemExit("registry descriptor selected claim must be 1")
for key in [
    "manual_family_selection",
    "projection_surface_selected",
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
output_contract=rust-lifecycle-mirbuilder-call-lowering-builtin-global-function-registry-policy-v0
subcluster=BuiltinGlobalFunctionRegistry
policy=RegistryDescriptorFixture
registry_descriptor_selected=1
projection_surface_selected=0
source_count=2
builtin_function_count=12
math_function_count=9
shared_builtin_math_count=5
selected_next_card=MIRBUILDER-CALL-LOWERING-EXTERN-INTERFACE-REGISTRY-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
