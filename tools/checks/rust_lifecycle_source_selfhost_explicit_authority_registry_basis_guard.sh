#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-source-selfhost-explicit-authority-registry-basis"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/3324-SOURCE-SELFHOST-EXPLICIT-AUTHORITY-REGISTRY-BASIS-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-explicit-authority-registry-basis-v0.json"
DESIGN_STOP_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-design-stop-v0.json"
REPAIR_AUDIT="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-machine-derived-route-repair-audit-refresh-v0.json"
LOCAL_POLICY="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-local-candidate-selection-policy-v0.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

guard_require_files "$TAG" \
  "$CARD" \
  "$FIXTURE" \
  "$DESIGN_STOP_FIXTURE" \
  "$REPAIR_AUDIT" \
  "$LOCAL_POLICY" \
  "$STATE" \
  "$TASK_ORDER" \
  "$INDEX"

python3 - "$CARD" "$FIXTURE" "$DESIGN_STOP_FIXTURE" "$REPAIR_AUDIT" "$LOCAL_POLICY" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
design_stop_path = Path(sys.argv[3])
repair_audit_path = Path(sys.argv[4])
local_policy_path = Path(sys.argv[5])
state_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
index_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
design_stop = json.loads(design_stop_path.read_text(encoding="utf-8"))
repair_audit = json.loads(repair_audit_path.read_text(encoding="utf-8"))
local_policy = json.loads(local_policy_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")


def need(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


token = "SOURCE-SELFHOST-EXPLICIT-AUTHORITY-REGISTRY-BASIS-001"
design_stop_token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
next_card = "MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001"
output_contract = "rust-lifecycle-source-selfhost-explicit-authority-registry-basis-v0"

need(f"# 3324 - {token}" in card, "card token drift")
need(output_contract in card, "card output contract drift")
need("HardAuthoritySeamProofAxis" in card, "card missing registered axis")
need(next_card in card, "card selected next drift")

need(fixture.get("kind") == "SourceSelfhostExplicitAuthorityRegistryBasisV1", "fixture kind drift")
need(fixture.get("token") == token, "fixture token drift")
need(fixture.get("output_contract") == output_contract, "fixture output contract drift")
need((fixture.get("current_state") or {}).get("latest_card") == token, "fixture latest card drift")
need((fixture.get("current_state") or {}).get("current_blocker_token") == design_stop_token, "fixture blocker drift")

entries = fixture.get("registry_entries") or []
need(len(entries) == 1, "registry entry count drift")
entry = entries[0]
need(entry.get("authority_source_kind") == "HardAuthoritySeamProofAxis", "authority source drift")
need(entry.get("proof_type") == "RustOracleParityWithAotExeGuard", "proof type drift")
need(entry.get("allowed_selection_use") == "HardAuthorityCandidateSelectorInputOnly", "selection use drift")
need(entry.get("claim_ceiling") == "candidate_basis_only", "claim ceiling drift")
need(entry.get("selection_mode") == "exactly_one_or_none", "selection mode drift")
need(entry.get("evidence_freshness") == "current", "freshness drift")
need(entry.get("reentry_condition") == "ConsultationGatedWiderRouteSelection", "reentry drift")
need(entry.get("downstream_consumer_required") is True, "downstream consumer drift")
need(entry.get("negative_guard_required") is True, "negative guard drift")
need(entry.get("mutation_allowed") is False, "mutation allowance drift")

for field in [
    "owner_id",
    "seam_kind",
    "input_surface",
    "output_surface",
    "rust_oracle_available",
    "hako_impl_available",
    "aot_guard_available",
    "downstream_consumer",
]:
    need(field in (entry.get("required_fields") or []), f"required field missing: {field}")

for source in [
    "ManualFamilySelection",
    "RouteMembershipAlone",
    "CoveragePercentage",
    "BundleSize",
    "SupportLaneProjectorAsAdoptionCandidate",
    "StringOnlyFacade",
]:
    need(source in (entry.get("forbidden_selection_use") or []), f"forbidden selection missing: {source}")
    need(source in (fixture.get("explicitly_rejected_authority_sources") or []), f"rejected authority source missing: {source}")

forbidden_claims = entry.get("forbidden_claims") or {}
for key in [
    "source_selfhost_claim",
    "hako_adopted_decision",
    "native_seed_materialization",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    need(forbidden_claims.get(key) == 0, f"entry forbidden claim drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "RegisterExplicitAuthorityProofAxis", "decision kind drift")
need(decision.get("reason_token") == "ConsultationApprovedHardAuthoritySeamProofAxis", "reason token drift")
need(decision.get("selected_axis") == "HardAuthoritySeamProofAxis", "selected axis drift")
need(decision.get("selected_next_card") == next_card, "selected next drift")

claims = fixture.get("claims") or {}
for key in [
    "consultation_gated_wider_route_selection",
    "new_proof_axis_registered",
    "hard_authority_seam_proof_axis_registered",
    "candidate_selector_input_only",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
    "source_selfhost_claim",
    "hako_adopted_decision",
    "native_seed_materialization",
    "manual_family_selection",
    "route_membership_alone_as_proof",
    "coverage_percentage_as_proof",
    "bundle_size_as_proof",
    "support_lane_projector_as_adoption_candidate",
    "string_only_facade_as_authority",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need((design_stop.get("recovery") or {}).get("resume_condition") == "ConsultationGatedWiderRouteSelectionOrMachineDerivedRouteRepair", "design-stop resume drift")
need((repair_audit.get("audit_summary") or {}).get("current_unblock_repair_count") == 0, "repair audit drift")
need((repair_audit.get("audit_summary") or {}).get("route_matrix_concrete_inconsistency_count") == 0, "repair inconsistency drift")
need((local_policy.get("policy_rule") or {}).get("external_consultation_only_for_new_authority") is True, "local policy external gate drift")

need(state.get("current_blocker_token") == design_stop_token, "CURRENT_STATE blocker drift")
latest_card = state.get("latest_card")
latest_path = state.get("latest_card_path")
need(isinstance(latest_card, str) and latest_card, "CURRENT_STATE latest card missing")
need(isinstance(latest_path, str) and Path(latest_path).exists(), "CURRENT_STATE latest path missing")

for needle in [
    token,
    output_contract,
    "HardAuthoritySeamProofAxis",
    "consultation_gated_wider_route_selection = 1",
    "new_proof_axis_registered = 1",
    "source_selfhost_claim = 0",
    next_card,
]:
    need(needle in task_order, f"task-order missing {needle}")

need("tools/checks/rust_lifecycle_source_selfhost_explicit_authority_registry_basis_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("decision=RegisterExplicitAuthorityProofAxis")
print("reason_token=ConsultationApprovedHardAuthoritySeamProofAxis")
print("registered_axis=HardAuthoritySeamProofAxis")
print("consultation_gated_wider_route_selection=1")
print("new_proof_axis_registered=1")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("runtime_route_switch=0")
print("summary=ok")
PY
