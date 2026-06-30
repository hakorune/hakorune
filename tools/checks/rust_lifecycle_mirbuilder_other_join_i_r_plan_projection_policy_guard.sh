#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-other-join-i-r-plan-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_other_join_i_r_plan_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-other-join-i-r-plan-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1882-MIRBUILDER-OTHER-JOIN-I-R-PLAN-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-other-join-i-r-plan-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1882-MIRBUILDER-OTHER-JOIN-I-R-PLAN-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-OTHER-JOIN-I-R-PLAN-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderOtherJoinIRPlanProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["source_count"] != 19:
    raise SystemExit("source count drift")
symbols = {item["symbol"] for item in fixture["source_surfaces"]}
for symbol in [
    "branchn_to_if_chain",
    "entry_gate_ok",
    "validate_loop_condition",
    "count_control_flow_with_returns",
    "emit_and_seal",
    "try_build_outcome",
    "build_join_payload",
    "trace_outcome_path",
]:
    if symbol not in symbols:
        raise SystemExit(f"source symbol missing: {symbol}")

buckets = fixture["helper_bucket_summary"]
for bucket in [
    "join_payload",
    "planner_gate_or_fact_count",
    "planner_session_or_rule_dispatch",
    "route_rewrite_helper",
    "skeleton_or_wiring",
    "trace_or_debug",
]:
    if bucket not in buckets:
        raise SystemExit(f"helper bucket missing: {bucket}")

policy = fixture["selected_policy"]
if policy["policy"] != "KeepParentOwner":
    raise SystemExit("policy drift")
if policy["projection_surface_selected"] is not False:
    raise SystemExit("projection surface must not be selected")
if policy["owner_edge"] != "mirbuilder::join_i_r_plan":
    raise SystemExit("owner edge drift")

for marker in [
    "CorePlan::If",
    "PlanBuildSession",
    "PLAN_RULE_ORDER",
    "GenericLoopSkeleton",
    "LoopTrueSkeleton",
    "LoopStepMode",
    "CoreIfJoin",
    "FragEmitSession",
    "[plan/trace]",
]:
    if marker not in fixture["joinir_plan_evidence"]:
        raise SystemExit(f"JoinIR plan marker missing: {marker}")

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
output_contract=rust-lifecycle-mirbuilder-other-join-i-r-plan-projection-policy-v0
policy=KeepParentOwner
projection_surface_selected=0
source_count=19
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
