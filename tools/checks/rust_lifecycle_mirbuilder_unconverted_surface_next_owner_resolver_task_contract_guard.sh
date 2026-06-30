#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-unconverted-surface-next-owner-resolver-task-contract-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-unconverted-surface-next-owner-resolver-task-contract-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1873-MIRBUILDER-UNCONVERTED-SURFACE-NEXT-OWNER-RESOLVER-TASK-CONTRACT-001.md"
REPORT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$REPORT"

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-unconverted-surface-next-owner-resolver-task-contract-v0.json").read_text())
report = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1873-MIRBUILDER-UNCONVERTED-SURFACE-NEXT-OWNER-RESOLVER-TASK-CONTRACT-001.md").read_text()

token = "MIRBUILDER-UNCONVERTED-SURFACE-NEXT-OWNER-RESOLVER-TASK-CONTRACT-001"
if fixture.get("kind") != "MirBuilderUnconvertedSurfaceNextOwnerResolverTaskContractV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if f"# 1873 - {token}" not in card:
    raise SystemExit("card token mismatch")

inventory = fixture["current_inventory"]
summary = report["summary"]
if report["decision"]["kind"] != "KeepStopped":
    raise SystemExit("report decision drift")
if inventory["unconverted_report_decision"] != report["decision"]["kind"]:
    raise SystemExit("inventory report decision drift")
if inventory["missing_projection_policy_count"] != summary["missing_projection_policy_count"]:
    raise SystemExit("missing projection count drift")
if inventory["borrow_policy_needed_count"] != summary["borrow_policy_needed_count"]:
    raise SystemExit("borrow policy count drift")
if inventory["composite_suspected_count"] != summary["composite_suspected_count"]:
    raise SystemExit("composite suspected count drift")
if inventory["mapped_to_known_owner_count"] != summary["mapped_to_known_owner_count"]:
    raise SystemExit("mapped owner count drift")

next_task = fixture["next_task"]
if next_task["token"] != "MIRBUILDER-UNCONVERTED-SURFACE-NEXT-OWNER-RESOLVER-001":
    raise SystemExit("next task token drift")
if next_task["kind"] != "DeterministicResolver":
    raise SystemExit("next task kind drift")

required_decisions = {
    "SelectOwnerEdgeClassification",
    "SelectProjectionPolicy",
    "SelectBorrowPolicy",
    "SelectCompositeDecomposition",
    "SelectCompositeEvidenceInventory",
    "SelectVerifierOrOracleRepair",
    "SelectNativeSourceSeed",
    "SelectHakoAdoptionDecision",
    "KeepStopped",
}
if set(fixture["decision_enum"]) != required_decisions:
    raise SystemExit("decision enum drift")

if fixture["tie_breaker"]["manual_selection_allowed"] != 0:
    raise SystemExit("manual selection must be forbidden")
if fixture["tie_breaker"]["same_priority_reason_token"] != "AmbiguousNextOwnerCandidates":
    raise SystemExit("ambiguous reason drift")
if fixture["tie_breaker"]["zero_candidates_reason_token"] != "NoMachineDerivedNextOwner":
    raise SystemExit("zero candidate reason drift")

seedability = fixture["generated_artifact_seedability_requirements"]
for key in [
    "shadow_parity_green",
    "hako_mainline_or_promotion_green",
    "bounded_surface",
    "borrow_policy_resolved",
    "deterministic_regeneration_evidence",
    "verifier_or_oracle_present",
]:
    if seedability.get(key) != 1:
        raise SystemExit(f"seedability positive requirement drift: {key}")
for key in ["composite_owner", "generated_artifact_as_edit_authority"]:
    if seedability.get(key) != 0:
        raise SystemExit(f"seedability non-claim drift: {key}")

acceptance = fixture["acceptance"]
for key in [
    "report_consumed",
    "resolver_selects_exactly_one_next_owner_when_unambiguous",
    "multiple_candidates_keep_stopped",
    "zero_candidates_keep_stopped",
]:
    if acceptance.get(key) != 1:
        raise SystemExit(f"acceptance positive drift: {key}")
for key in [
    "support_lane_projector_as_hako_adoption_candidate",
    "generated_artifact_as_edit_authority",
    "manual_family_selection",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    if acceptance.get(key) != 0:
        raise SystemExit(f"acceptance non-claim drift: {key}")

claims = fixture["claims"]
if claims["task_contract_only"] != 1:
    raise SystemExit("task contract claim missing")
for key in [
    "resolver_implemented",
    "hako_generation",
    "hako_adopted_decision",
    "native_seed_materialization",
    "source_selfhost_claim",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"claim must be 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-unconverted-surface-next-owner-resolver-task-contract-v0
next_task=MIRBUILDER-UNCONVERTED-SURFACE-NEXT-OWNER-RESOLVER-001
resolver_implemented=0
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
