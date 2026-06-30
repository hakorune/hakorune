#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-loop-cond-utility-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_loop_cond_utility_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-utility-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1886-MIRBUILDER-LOOP-COND-UTILITY-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-utility-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1886-MIRBUILDER-LOOP-COND-UTILITY-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-LOOP-COND-UTILITY-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderLoopCondUtilityProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["source_count"] != 2:
    raise SystemExit("source count drift")
symbols = [surface["symbol"] for surface in fixture["source_surfaces"]]
if symbols != ["direct_exit_reject", "is_direct_exit_reject"]:
    raise SystemExit(f"selected surface drift: {symbols}")
returns = {surface["symbol"]: surface["return_type"] for surface in fixture["source_surfaces"]}
if returns["direct_exit_reject"] != "String":
    raise SystemExit("direct_exit_reject return type drift")
if returns["is_direct_exit_reject"] != "bool":
    raise SystemExit("is_direct_exit_reject return type drift")

axes = fixture["selection_axes"]
expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.loop_cond_utility",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if axes != expected_axes:
    raise SystemExit(f"selection axes drift: {axes}")

policy = fixture["selected_policy"]
if policy["policy"] != "KeepParentOwner":
    raise SystemExit("policy drift")
if policy["projection_surface_selected"] is not False:
    raise SystemExit("projection surface must not be selected")
if policy["owner_edge"] != "mirbuilder::loop_cond_utility":
    raise SystemExit("owner edge drift")

for marker in [
    "DirectExitRejectReason",
    "DIRECT_EXIT_REJECT_TAG",
    "direct_exit_reason_text",
    "Backward-compatible fallback for legacy message shapes.",
    "BlockContainsDirectExit",
    "ExitMustBeInsideIf",
    "BreakMustBeLast",
    "ReturnMustBeLast",
]:
    if marker not in fixture["utility_evidence"]:
        raise SystemExit(f"utility marker missing: {marker}")

decision = fixture["decision"]
if decision["kind"] != "KeepParentOwner":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "manual_family_selection",
    "projection_surface_selected",
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
output_contract=rust-lifecycle-mirbuilder-loop-cond-utility-projection-policy-v0
policy=KeepParentOwner
projection_surface_selected=0
source_count=2
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
