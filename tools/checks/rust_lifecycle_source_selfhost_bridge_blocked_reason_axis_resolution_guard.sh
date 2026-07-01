#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-bridge-blocked-reason-axis-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/source_selfhost_bridge_blocked_reason_axis_resolution.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/1978-SOURCE-SELFHOST-BRIDGE-BLOCKED-REASON-AXIS-RESOLUTION-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "SOURCE-SELFHOST-BRIDGE-BLOCKED-REASON-AXIS-RESOLUTION-001"
next_card = "MIRBUILDER-BRIDGE-BLOCKED-GAP-CLUSTER-RESOLUTION-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "SourceSelfhostBridgeBlockedReasonAxisResolutionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

pool = fixture.get("input_candidate_pool") or {}
need(pool.get("verified_hako_family_ir_count") == 47, "verified count drift")
need(pool.get("bridge_eligible_count") == 0, "bridge eligible count drift")
need(pool.get("bridge_blocked_count") == 47, "bridge blocked count drift")
need(pool.get("gap_blocked_count") == 36, "gap blocked count drift")

axes = fixture.get("reason_axis_resolution") or {}
need(axes.get("bridge_blocked_count") == 47, "axis blocked count drift")
axis_by_name = {axis.get("axis"): axis for axis in axes.get("axes", [])}
need(axis_by_name["PolicyGapInDeniedBoundaries"]["candidate_count"] == 24, "pure policy gap count drift")
need(axis_by_name["PolicyGapInDeniedBoundaries"]["selection_eligible"] is True, "policy gap must be eligible")
need(axis_by_name["CompositeOrIntegrationOwner"]["selection_eligible"] is False, "composite must not be eligible")
need(axis_by_name["AlreadyCoveredByUnscopedAdoptionDecision"]["selection_eligible"] is False, "unscoped adoption must not be eligible")
need(axis_by_name["AlreadyHakoAdopted"]["selection_eligible"] is False, "adopted must not be eligible")

rule = fixture.get("selection_rule") or {}
need(rule.get("manual_axis_selection") is False, "manual axis selection forbidden")
need(rule.get("cluster_size_as_proof") is False, "cluster size proof forbidden")
need(rule.get("select_pure_policy_gap_axis_if_unique") is True, "policy gap selection rule drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectBridgeBlockedGapClusterResolution", "bad decision kind")
need(decision.get("reason_token") == "ExactlyOneBridgeBlockedReasonAxisEligible", "bad reason")
need(decision.get("selected_axis") == "PolicyGapInDeniedBoundaries", "bad selected axis")
need(decision.get("selected_next_card") == next_card, "bad next card")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_native_edit_authority",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"{key} must be 0")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")
need(next_card in task_order, "task-order missing next card")
need("BridgeBlockedReasonAxisResolution" in task_order, "task-order missing owner kind")

print("output_contract=rust-lifecycle-source-selfhost-bridge-blocked-reason-axis-resolution")
print("bridge_eligible_count=0")
print("bridge_blocked_count=47")
print("selected_axis=PolicyGapInDeniedBoundaries")
print(f"selected_next_card={next_card}")
print("manual_axis_selection=0")
print("cluster_size_as_proof=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
