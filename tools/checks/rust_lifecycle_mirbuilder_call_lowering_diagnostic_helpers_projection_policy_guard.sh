#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-call-lowering-diagnostic-helpers-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_call_lowering_diagnostic_helpers_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-diagnostic-helpers-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1897-MIRBUILDER-CALL-LOWERING-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-diagnostic-helpers-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1897-MIRBUILDER-CALL-LOWERING-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-CALL-LOWERING-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderCallLoweringDiagnosticHelpersProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["selected_subcluster_id"] != "DiagnosticStringHelpers":
    raise SystemExit("selected subcluster drift")
if fixture["input_state"]["source_count"] != 3:
    raise SystemExit("source count drift")

symbols = [surface["symbol"] for surface in fixture["source_surfaces"]]
expected_symbols = [
    "generate_self_recursion_warning",
    "is_commonly_shadowed_method",
    "suggest_resolution",
]
if symbols != expected_symbols:
    raise SystemExit(f"selected surface drift: {symbols}")

expected_roles = {
    "diagnostic_shadow_warning_predicate": 1,
    "self_recursion_warning_message": 1,
    "unresolved_function_hint_message": 1,
}
if fixture["role_counts"] != expected_roles:
    raise SystemExit(f"role count drift: {fixture['role_counts']}")

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

policy = fixture["selected_policy"]
if policy["policy"] != "KeepParentOwner":
    raise SystemExit("policy drift")
if policy["owner_edge"] != "mirbuilder::call_lowering_diagnostic_helpers":
    raise SystemExit("owner edge drift")
if policy["projection_surface_selected"] is not False:
    raise SystemExit("projection surface must not be selected")
if policy["registry_descriptor_selected"] is not False:
    raise SystemExit("registry descriptor must not be selected")

for marker in [
    "Check if method is commonly shadowed (for warning generation)",
    "Generate warning about potential self-recursion",
    "Suggest resolution for unresolved function",
    "Did you mean 'env.console.log' or 'print'?",
    "Check function name or ensure it's in scope.",
]:
    if marker not in fixture["diagnostic_evidence"]:
        raise SystemExit(f"diagnostic evidence marker missing: {marker}")

decision = fixture["decision"]
if decision["kind"] != "KeepParentOwner":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-CALL-LOWERING-BUILTIN-GLOBAL-FUNCTION-REGISTRY-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "manual_family_selection",
    "projection_surface_selected",
    "registry_descriptor_selected",
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
output_contract=rust-lifecycle-mirbuilder-call-lowering-diagnostic-helpers-projection-policy-v0
subcluster=DiagnosticStringHelpers
policy=KeepParentOwner
projection_surface_selected=0
registry_descriptor_selected=0
source_count=3
selected_next_card=MIRBUILDER-CALL-LOWERING-BUILTIN-GLOBAL-FUNCTION-REGISTRY-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
