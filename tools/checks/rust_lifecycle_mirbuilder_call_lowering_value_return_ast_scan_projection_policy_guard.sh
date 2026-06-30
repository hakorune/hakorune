#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-call-lowering-value-return-ast-scan-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_call_lowering_value_return_ast_scan_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-value-return-ast-scan-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1904-MIRBUILDER-CALL-LOWERING-VALUE-RETURN-AST-SCAN-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-value-return-ast-scan-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1904-MIRBUILDER-CALL-LOWERING-VALUE-RETURN-AST-SCAN-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-CALL-LOWERING-VALUE-RETURN-AST-SCAN-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderCallLoweringValueReturnAstScanProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["selected_feature_subcluster_id"] != "ValueReturnAstScan":
    raise SystemExit("selected feature subcluster drift")
if fixture["input_state"]["source_count"] != 1:
    raise SystemExit("source count drift")

surface = fixture["source_surface"]
if surface["symbol"] != "contains_value_return":
    raise SystemExit("source symbol drift")
expected_variants = [
    "FunctionDeclaration",
    "If",
    "Loop",
    "Program",
    "Return",
    "ScopeBox",
    "TryCatch",
]
if surface["ast_variants"] != expected_variants:
    raise SystemExit(f"AST variant drift: {surface['ast_variants']}")
for marker in [
    "contains_value_return(then_body)",
    "contains_value_return(body)",
    "contains_value_return(try_body)",
    "contains_value_return(&clause.body)",
    "contains_value_return(statements)",
    "nodes.iter().any(node_has_value_return)",
]:
    if marker not in surface["recursion_markers"]:
        raise SystemExit(f"recursion marker missing: {marker}")

axes = fixture["selection_axes"]
expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.ast_scan_predicate",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if axes != expected_axes:
    raise SystemExit(f"selection axes drift: {axes}")

policy = fixture["selected_policy"]
if policy["policy"] != "KeepParentAstScan":
    raise SystemExit("policy drift")
if policy["owner_edge"] != "mirbuilder::call_lowering_value_return_ast_scan":
    raise SystemExit("owner edge drift")
if policy["projection_surface_selected"] is not False:
    raise SystemExit("projection surface must not be selected")
if policy["ast_traversal_projection_selected"] is not False:
    raise SystemExit("AST traversal projection must not be selected")

decision = fixture["decision"]
if decision["kind"] != "KeepParentOwner":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-CALL-LOWERING-NAME-CANONICALIZATION-PROJECTION-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "manual_family_selection",
    "projection_surface_selected",
    "ast_traversal_projection_selected",
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
output_contract=rust-lifecycle-mirbuilder-call-lowering-value-return-ast-scan-projection-policy-v0
subcluster=ValueReturnAstScan
policy=KeepParentAstScan
ast_variant_count=7
projection_surface_selected=0
ast_traversal_projection_selected=0
selected_next_card=MIRBUILDER-CALL-LOWERING-NAME-CANONICALIZATION-PROJECTION-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
