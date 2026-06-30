#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-statement-value-construction-diagnostic-helpers-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_statement_value_construction_diagnostic_helpers_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-statement-value-construction-diagnostic-helpers-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1924-MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-statement-value-construction-diagnostic-helpers-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1924-MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderStatementValueConstructionDiagnosticHelpersProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["selected_subcluster_id"] != "DiagnosticStringHelpers":
    raise SystemExit("selected subcluster drift")
if fixture["input_state"]["source_count"] != 1:
    raise SystemExit("source count drift")

surfaces = fixture["source_surfaces"]
if len(surfaces) != 1:
    raise SystemExit("selected surface count drift")
surface = surfaces[0]
if surface["symbol"] != "undefined_variable_message":
    raise SystemExit(f"selected symbol drift: {surface['symbol']}")
if surface["return_type"] != "String":
    raise SystemExit("return type drift")
if surface["diagnostic_role"] != "undefined_variable_diagnostic_message":
    raise SystemExit("diagnostic role drift")

expected_roles = {"undefined_variable_diagnostic_message": 1}
if fixture["role_counts"] != expected_roles:
    raise SystemExit(f"role count drift: {fixture['role_counts']}")

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

for marker in [
    "Undefined variable:",
    "Stage-3 keyword",
    "parser_stage3_enabled()",
    "suggest_using_for_symbol(name)",
    "symbol appears in using module(s)",
    "Consider adding 'using <module> [as Alias]'",
]:
    if marker not in fixture["diagnostic_evidence"]:
        raise SystemExit(f"diagnostic evidence marker missing: {marker}")

policy = fixture["selected_policy"]
if policy["policy"] != "KeepParentOwner":
    raise SystemExit("policy drift")
if policy["owner_edge"] != "mirbuilder::statement_value_construction_diagnostic_helpers":
    raise SystemExit("owner edge drift")
if policy["projection_surface_selected"] is not False:
    raise SystemExit("projection surface must not be selected")
if policy["registry_descriptor_selected"] is not False:
    raise SystemExit("registry descriptor must not be selected")

decision = fixture["decision"]
if decision["kind"] != "KeepParentOwner":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BLOCK-TERMINATION-PREDICATE-PROJECTION-POLICY-001":
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
output_contract=rust-lifecycle-mirbuilder-statement-value-construction-diagnostic-helpers-projection-policy-v0
subcluster=DiagnosticStringHelpers
policy=KeepParentOwner
projection_surface_selected=0
registry_descriptor_selected=0
source_count=1
selected_next_card=MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BLOCK-TERMINATION-PREDICATE-PROJECTION-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
