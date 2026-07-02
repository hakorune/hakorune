#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-source-selfhost-wider-route-selection-basis-007"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/2056-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-007.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-basis-007-v0.json"
PARENT_BOUNDARY="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-parent-owned-subject-boundary-resolution-v0.json"
RERUN_010="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-010-v0.json"
REPORT="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
MANIFEST="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
ADOPTION="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-emission-ssa-phi-hako-adoption-decision-v0.json"
POLICY="docs/development/current/main/design/current-docs-update-policy-ssot.md"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TOOL="tools/rust_lifecycle/source_selfhost_wider_route_selection_basis_007.py"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$FIXTURE" \
  "$PARENT_BOUNDARY" \
  "$RERUN_010" \
  "$REPORT" \
  "$MANIFEST" \
  "$ADOPTION" \
  "$POLICY" \
  "$TASK_ORDER" \
  "$STATE" \
  "$TOOL"

# This selector intentionally consumed a stale report before
# MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-004 refreshed it. Do not rerun
# the generator against the mutable current report here; validate the frozen
# selector fixture and card contract below.

python3 - "$CARD" "$FIXTURE" "$PARENT_BOUNDARY" "$RERUN_010" "$REPORT" "$MANIFEST" "$ADOPTION" "$POLICY" "$TASK_ORDER" "$STATE" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
parent_path = Path(sys.argv[3])
rerun_path = Path(sys.argv[4])
report_path = Path(sys.argv[5])
manifest_path = Path(sys.argv[6])
adoption_path = Path(sys.argv[7])
policy_path = Path(sys.argv[8])
task_order_path = Path(sys.argv[9])
state_path = Path(sys.argv[10])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
parent = json.loads(parent_path.read_text(encoding="utf-8"))
rerun = json.loads(rerun_path.read_text(encoding="utf-8"))
report = json.loads(report_path.read_text(encoding="utf-8"))
manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
adoption = json.loads(adoption_path.read_text(encoding="utf-8"))
policy = policy_path.read_text(encoding="utf-8")
_task_order = task_order_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-007"
contract = "rust-lifecycle-source-selfhost-wider-route-selection-basis-007-v0"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
next_card = "MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-004"
reason = "SourceSurfaceReportStaleAfterEmissionSsaPhiAdoption"
parent_token = "MIRBUILDER-ID-SCALAR-PARENT-OWNED-SUBJECT-BOUNDARY-RESOLUTION-001"
adoption_token = "MIRBUILDER-EMISSION_SSA_PHI-HAKO-ADOPTION-DECISION-001"

for needle in [
    token,
    "LocalMechanicalSelectorAuthorityV1",
    "worker_inventory = consumed",
    "classification = RemainParentOwned",
    next_card,
    "manual_family_selection = 0",
    "source_selfhost_claim = 0",
]:
    require(needle in card, f"card missing {needle}")

require("LocalMechanicalSelectorAuthorityV1" in policy, "policy missing local selector authority")
require("read-only worker inventory" in policy, "policy missing worker inventory rule")

require(fixture.get("kind") == "SourceSelfhostWiderRouteSelectionBasisV7", "fixture kind drift")
require(fixture.get("token") == token, "fixture token drift")
require(fixture.get("output_contract") == contract, "fixture contract drift")
require((fixture.get("input_state") or {}).get("current_blocker") == design_stop, "input blocker drift")

local = fixture.get("local_authority") or {}
require(local.get("local_selection_authority") == "LocalMechanicalSelectorAuthorityV1", "local authority drift")
require(local.get("worker_inventory") == "consumed", "worker inventory drift")
require(local.get("worker_inventory_scope") == "read_only_current_fixtures_cards_ledgers", "worker scope drift")

parent_decision = parent.get("decision") or {}
parent_classification = parent.get("classification") or {}
closeout = fixture.get("parent_owned_closeout") or {}
require(parent.get("token") == parent_token, "parent token drift")
require(parent_decision.get("kind") == "SelectWiderRouteSelectionBasis", "parent decision drift")
require(parent_decision.get("selected_next_card") == token, "parent next drift")
require(parent_classification.get("kind") == "RemainParentOwned", "parent classification drift")
require(parent_classification.get("standalone_projection_subject_established") is False, "standalone subject drift")
require(parent_classification.get("source_plan_materialization_allowed") is False, "source plan allowed drift")
require(closeout.get("classification") == "RemainParentOwned", "fixture closeout classification drift")
require(closeout.get("standalone_projection_subject_established") is False, "fixture standalone drift")
require(closeout.get("source_plan_materialization_allowed") is False, "fixture source plan allowed drift")

rerun_pool = rerun.get("candidate_pool") or {}
rerun_decision = rerun.get("decision") or {}
seed = fixture.get("seed_capability_after_adoption") or {}
require(rerun_pool.get("remaining_owner_count") == 1, "rerun remaining owner drift")
require(rerun_pool.get("selection_eligible_count") == 0, "rerun eligible drift")
require(rerun_pool.get("native_seed_candidate_count") == 0, "rerun seed candidate drift")
require(rerun_decision.get("kind") == "KeepStopped", "rerun decision drift")
require(seed.get("remaining_owner_count") == 1, "fixture remaining owner drift")
require(seed.get("selection_eligible_count") == 0, "fixture eligible drift")
require(seed.get("native_seed_candidate_count") == 0, "fixture native seed drift")

adoption_claims = adoption.get("claims") or {}
freshness = fixture.get("freshness") or {}
require(adoption.get("token") == adoption_token, "adoption token drift")
require(adoption_claims.get("hako_adopted") == 1, "emission_ssa_phi must be adopted")
require(adoption_claims.get("source_selfhost_claim") == 0, "adoption must not claim Source Selfhost")
require(freshness.get("emission_ssa_phi_hako_adopted") == 1, "fixture adoption drift")
require(freshness.get("emission_ssa_phi_source_selfhost_claim") == 0, "fixture source selfhost drift")
require(freshness.get("unconverted_surface_report_fresh_after_emission_ssa_phi_adoption") is False, "report must be stale")
require(freshness.get("native_owner_adoption_delta_count") == 1, "adoption delta count drift")
require(freshness.get("latest_native_owner_delta_tokens") == [adoption_token], "adoption delta token drift")
require(freshness.get("freshness_reason_token") == reason, "freshness reason drift")

require(freshness.get("report_native_owner_adoption_ledger_hash"), "report ledger hash missing")
require(freshness.get("current_native_owner_manifest_hash"), "current manifest hash missing")
require(manifest.get("kind") == "SourceSelfhostFamilyGuardManifestV1", "manifest kind drift")

lanes = fixture.get("candidate_lanes") or []
eligible = [lane for lane in lanes if lane.get("selection_eligible") is True]
require(len(eligible) == 1, "exactly one lane must be eligible")
require(eligible[0].get("lane") == "UnconvertedSurfaceReportRerun004", "selected lane drift")
require(eligible[0].get("next_card") == next_card, "selected lane next drift")

selection_rule = fixture.get("selection_rule") or {}
for key in [
    "consume_parent_owned_boundary",
    "context_registry_remain_parent_owned_required",
    "report_freshness_precedes_checkpoint",
    "native_owner_checkpoint_precedes_blocker_class_selection",
    "exactly_one_lane_or_keep_stopped",
    "local_mechanical_selector_authority",
    "worker_inventory_required_or_waived",
]:
    require(selection_rule.get(key) is True, f"selection rule drift: {key}")
for key in [
    "manual_lane_selection",
    "remaining_owner_count_as_proof",
    "owner_name_as_proof",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
]:
    require(selection_rule.get(key) is False, f"forbidden selection rule drift: {key}")

decision = fixture.get("decision") or {}
require(decision.get("kind") == "SelectUnconvertedSurfaceReportRerun", "decision kind drift")
require(decision.get("reason_token") == reason, "decision reason drift")
require(decision.get("selected_next_card") == next_card, "decision next drift")
require(decision.get("selected_owner_edge_id") is None, "selector must not choose owner")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "manual_lane_selection",
    "remaining_owner_count_as_proof",
    "owner_name_as_proof",
    "source_symbol_as_proof",
    "source_path_as_authority",
    "keep_parent_owner_as_standalone_proof",
    "projection_descriptor_coverage_as_standalone_proof",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_native_edit_authority",
    "source_plan_materialization",
    "behavior_recipe_materialization",
    "verifier_result_materialization",
    "derived_artifact_seed_draft_materialization",
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
    require(claims.get(key) == 0, f"forbidden claim drift: {key}")

require(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print(f"output_contract={contract}")
print(f"decision=SelectUnconvertedSurfaceReportRerun")
print(f"reason_token={reason}")
print(f"selected_next_card={next_card}")
print("worker_inventory=consumed")
print("source_selfhost_claim=0")
print("summary=ok")
PY
