#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-003-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_crate_wide_native_owner_seed_capability_survey_rerun_003.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-003-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1962-MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-003.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-003-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1962-MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-003.md").read_text()

token = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-003"
if fixture.get("kind") != "MirBuilderCrateWideNativeOwnerSeedCapabilitySurveyRerunV3":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

queue = fixture["queue_exhaustion"]
if queue["global_projection_policy"]["selectable_cluster_count"] != 0:
    raise SystemExit("global projection queue must remain exhausted")
if queue["global_projection_policy"]["reason_token"] != "NoEligibleProjectionPolicyCluster":
    raise SystemExit("global projection reason drift")
if queue["other_shape_signature_queue"]["selection_eligible_shape_count"] != 0:
    raise SystemExit("Other shape queue must remain exhausted")
if queue["other_shape_signature_queue"]["reason_token"] != "NoUnclosedOtherShapeSignatureClusterEligible":
    raise SystemExit("Other queue reason drift")

strict = fixture["strict_emission_evidence"]
if strict["verified_hako_family_ir_count"] != 47:
    raise SystemExit("strict emission count drift")
if strict["hako_generation"] != 0:
    raise SystemExit("strict probe must not generate Hako")
if strict["strict_rules_changed"] != 0:
    raise SystemExit("strict probe must not weaken rules")

freshness = fixture["freshness"]
if freshness["needs_unconverted_surface_report_rerun"] is not False:
    raise SystemExit("source report should be fresh for rerun 003")
if freshness["unconverted_surface_report_covers_landed_descriptors"] is not True:
    raise SystemExit("report must cover projection descriptor ledger")
if freshness["projection_descriptor_ledger_hash"] != freshness["unconverted_surface_report_projection_descriptor_ledger_hash"]:
    raise SystemExit("projection descriptor ledger hash mismatch")

pool = fixture["candidate_pool"]
for key in [
    "native_seed_ready_count",
    "native_owner_seed_candidate_count",
    "generated_artifact_to_seed_candidate_count",
    "route_repairable_inconsistency_count",
    "other_blocker_axis_candidate_count",
    "ambiguous_candidate_count",
]:
    if pool.get(key) != 0:
        raise SystemExit(f"candidate pool must remain zero: {key}")

decision = fixture["decision"]
if decision["kind"] != "KeepStopped":
    raise SystemExit("decision kind drift")
if decision["reason_token"] != "NoMachineDerivedNativeOwnerSeedCandidateAfterStrictEmissionProbe":
    raise SystemExit("decision reason drift")
if decision["selected_next_card"] != "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001":
    raise SystemExit("decision next card drift")
if decision["selected_owner_edge_id"] is not None:
    raise SystemExit("selected owner must remain null")

claims = fixture["claims"]
for key in [
    "global_projection_policy_exhaustion_consumed",
    "other_shape_queue_exhaustion_consumed",
    "strict_emission_probe_consumed",
    "projection_descriptor_ledger_hash_recorded",
    "unconverted_surface_report_hash_recorded",
    "freshness_checked",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"positive claim must be 1: {key}")
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_native_edit_authority",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "family_name_based_policy",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim must be 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-003-v0
global_selectable_cluster_count=0
other_selection_eligible_shape_count=0
strict_verified_hako_family_ir_count=47
needs_unconverted_surface_report_rerun=0
native_owner_seed_candidate_count=0
selected_next_card=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
