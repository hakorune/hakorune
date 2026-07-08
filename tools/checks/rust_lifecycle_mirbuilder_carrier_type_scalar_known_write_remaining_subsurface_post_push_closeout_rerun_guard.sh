#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-push-closeout-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_remaining_subsurface_post_push_closeout_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2122-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-POST-PUSH-CLOSEOUT-RERUN-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-push-closeout-rerun"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-POST-PUSH-CLOSEOUT-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteRemainingSubsurfacePostPushCloseoutRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("accepted_scoped_closeout_count") == 4, "accepted closeout count drift")
need(inputs.get("remaining_write_subsurface_count") == 2, "remaining count drift")

rows = {row.get("subsurface_id"): row for row in fixture.get("remaining_subsurfaces") or []}
need(set(rows) == {"DeleteSurfacePolicy", "SetSurfacePolicy"}, "remaining subsurface drift")
for subsurface_id in ["DeleteSurfacePolicy", "SetSurfacePolicy"]:
    row = rows[subsurface_id]
    need(row.get("hako_adopted") is False, f"{subsurface_id} adoption drift")
    need(row.get("basis_selection_eligible") is False, f"{subsurface_id} eligibility drift")
    blockers = set(row.get("blocked_by") or [])
    need("NoHakoAdoptedWriteSubsurfacePilot" in blockers, f"{subsurface_id} missing adoption blocker")
    need("NoConsultationApprovedNextWritePilotProofAxis" in blockers, f"{subsurface_id} missing consultation blocker")

rule = fixture.get("selector_rule") or {}
need(rule.get("if_zero_hako_adopted_remaining_subsurfaces_keep_stopped") is True, "zero adopted rule drift")
need(rule.get("next_pilot_requires_design_consultation") is True, "consult rule drift")
for key in [
    "manual_subsurface_selection",
    "route_count_as_proof",
    "apparent_simplicity_as_proof",
    "accepted_read_contract_similarity_as_proof",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

summary = fixture.get("summary") or {}
need(summary.get("remaining_write_subsurface_count") == 2, "summary remaining count drift")
need(summary.get("hako_adopted_remaining_write_subsurface_count") == 0, "summary adopted remaining drift")
need(summary.get("basis_selection_eligible_subsurface_count") == 0, "summary eligible drift")
need(summary.get("selected_write_subsurface_count") == 0, "summary selected drift")
for key in [
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "NoHakoAdoptedRemainingWriteSubsurfacePilot", "reason drift")
need(decision.get("recommended_consultation_topic") == "WriteRemainingSubsurfaceHakoPilotSelection", "consult topic drift")
need(decision.get("selected_next_card") == design_stop, "next drift")
need(decision.get("selected_subsurface") is None, "selected subsurface drift")

claims = fixture.get("claims") or {}
need(claims.get("write_remaining_subsurface_post_push_closeout_rerun") == 1, "missing rerun claim")
need(claims.get("remaining_write_subsurface_count") == 2, "claim remaining count drift")
need(claims.get("hako_adopted_remaining_write_subsurface_count") == 0, "claim adopted remaining drift")
need(claims.get("basis_selection_eligible_subsurface_count") == 0, "claim eligible count drift")
for key in [
    "write_subsurface_selected",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
    "hako_generation",
    "new_route_authority",
    "behavior_change",
    "runtime_mutation_authority",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "new_python_semantic_projector",
    "manual_axis_selection",
    "manual_carrier_selection",
    "manual_subsurface_selection",
    "row_count_as_proof",
    "route_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
    "apparent_simplicity_as_proof",
    "accepted_read_contract_similarity_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2122-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-POST-PUSH-CLOSEOUT-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-push-closeout-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_remaining_subsurface_post_push_closeout_rerun_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={design_stop}" in task_order, "task order design stop drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-push-closeout-rerun")
print("remaining_write_subsurface_count=2")
print("hako_adopted_remaining_write_subsurface_count=0")
print("basis_selection_eligible_subsurface_count=0")
print("decision=KeepStopped")
print("recommended_consultation_topic=WriteRemainingSubsurfaceHakoPilotSelection")
print("selected_next_card=" + design_stop)
print("source_selfhost_claim=0")
print("summary=ok")
PY
