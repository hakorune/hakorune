#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROFILE="${1:-}"

case "$PROFILE" in
  explicit_semantic_resource_domain_declaration_basis)
    FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-explicit-semantic-resource-domain-declaration-basis-v0.json"
    TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_explicit_semantic_resource_domain_declaration_basis.py"
    CARD="$ROOT/docs/development/current/main/phases/phase-296x/2076-MIRBUILDER-DOMAIN-OBJECT-ID-EXPLICIT-SEMANTIC-RESOURCE-DOMAIN-DECLARATION-BASIS-001.md"
    ;;
  semantic_resource_domain_declaration_inventory)
    FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-semantic-resource-domain-declaration-inventory-v0.json"
    TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_semantic_resource_domain_declaration_inventory.py"
    CARD="$ROOT/docs/development/current/main/phases/phase-296x/2077-MIRBUILDER-DOMAIN-OBJECT-ID-SEMANTIC-RESOURCE-DOMAIN-DECLARATION-INVENTORY-001.md"
    ;;
  post_domain_object_id_exhaustion_wider_selection)
    FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-basis-008-v0.json"
    TOOL="$ROOT/tools/rust_lifecycle/source_selfhost_wider_route_selection_basis_008.py"
    CARD="$ROOT/docs/development/current/main/phases/phase-296x/2078-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-008.md"
    ;;
  *)
    echo "usage: $0 {explicit_semantic_resource_domain_declaration_basis|semantic_resource_domain_declaration_inventory|post_domain_object_id_exhaustion_wider_selection}" >&2
    exit 2
    ;;
esac

STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 "$TOOL" --check

python3 - "$PROFILE" "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

profile, fixture_path, card_path, state_path, task_order_path, manifest_path = sys.argv[1:]
fixture = json.load(open(fixture_path, encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
state = tomllib.loads(Path(state_path).read_text(encoding="utf-8"))
task_order = Path(task_order_path).read_text(encoding="utf-8")
manifest = json.load(open(manifest_path, encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


def check_forbidden_claims(claims):
    for key in [
        "source_selfhost_claim",
        "native_seed_materialization",
        "hako_generation",
        "hako_adopted_decision",
        "accepted_typed_dependency_edge_materialized",
        "manual_subaxis_selection",
        "manual_type_to_subaxis_assignment",
        "return_type_string_to_subaxis_mapping",
        "source_path_as_policy_authority",
        "observed_subaxis_set_as_policy_authority",
        "row_count_as_proof",
        "owner_name_as_proof",
        "shape_signature_as_proof",
        "route_membership_alone_as_proof",
        "self_signed_taxonomy",
    ]:
        need(claims.get(key) == 0, f"forbidden claim drift: {key}")


if profile == "explicit_semantic_resource_domain_declaration_basis":
    token = "MIRBUILDER-DOMAIN-OBJECT-ID-EXPLICIT-SEMANTIC-RESOURCE-DOMAIN-DECLARATION-BASIS-001"
    next_card = "MIRBUILDER-DOMAIN-OBJECT-ID-SEMANTIC-RESOURCE-DOMAIN-DECLARATION-INVENTORY-001"
    design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

    need(fixture.get("kind") == "MirBuilderDomainObjectIdExplicitSemanticResourceDomainDeclarationBasisV1", "bad kind")
    need(fixture.get("token") == token, "bad token")
    need(token in card, "card missing token")
    need(token in task_order, "task-order missing token")
    need(next_card in task_order, "task-order missing next card")

    inputs = fixture.get("input_state") or {}
    need(inputs.get("current_blocker") == design_stop, "blocker drift")
    need(inputs.get("stable_type_resource_registry_authority_rerun", "").endswith("mirbuilder-domain-object-id-stable-type-resource-registry-authority-rerun-v0.json"), "rerun input drift")

    rule = fixture.get("authority_rule") or {}
    need(rule.get("name") == "ExplicitSemanticResourceDomainDeclarationBasisV1", "rule drift")
    for key in [
        "type_identity_only_is_not_resource_domain_authority",
        "declared_resource_domain_subaxis_requires_explicit_semantic_declaration",
        "semantic_resource_id_required",
        "type_decl_ref_required",
        "proof_source_hash_required",
        "self_signed_declaration_forbidden",
    ]:
        need(rule.get(key) is True, f"rule drift: {key}")
    for key in [
        "manual_type_to_subaxis_assignment",
        "return_type_string_to_subaxis_mapping",
        "source_path_as_policy_authority",
        "observed_subaxis_set_as_policy_authority",
        "owner_name_as_proof",
        "shape_signature_as_proof",
        "row_count_as_proof",
        "route_membership_alone_as_proof",
    ]:
        need(rule.get(key) is False, f"forbidden rule drift: {key}")

    sources = fixture.get("allowed_authority_sources") or []
    need({row.get("source_kind") for row in sources} == {
        "ExistingSemanticResourceDeclarationFixture",
        "ProjectionDescriptorLedgerExplicitResourceDeclaration",
        "NewReadOnlySemanticResourceDeclarationInventory",
    }, "allowed source set drift")

    requirements = fixture.get("resource_domain_declaration_requirements") or {}
    for key in [
        "semantic_resource_id_must_join_registry_row",
        "type_decl_ref_must_join_registry_row",
        "declared_resource_domain_subaxis_must_be_present",
        "declared_resource_domain_subaxis_must_be_from_allowed_set",
        "dependency_role_must_be_declared",
        "proof_source_must_be_stable",
        "proof_source_hash_must_be_recorded",
    ]:
        need(requirements.get(key) is True, f"requirement drift: {key}")

    summary = fixture.get("summary") or {}
    need(summary.get("candidate_registry_row_count") == 42, "candidate row count drift")
    need(summary.get("type_identity_only_row_count") == 42, "type identity count drift")
    need(summary.get("resource_domain_declaration_ready_count") == 0, "resource declaration ready drift")
    need(summary.get("registry_ready_row_count") == 0, "registry ready drift")
    need(summary.get("accepted_typed_dependency_edge_count") == 0, "accepted edge drift")

    rows = fixture.get("candidate_registry_rows") or []
    need(len(rows) == 42, "candidate registry row count drift")
    for row in rows:
        need(row.get("current_registry_row_state") == "TypeIdentityOnly", "row state drift")
        need(row.get("resource_domain_declaration_state") == "Missing", "declaration state drift")
        need(row.get("declared_resource_domain_subaxis") is None, "subaxis must not be assigned")
        need(row.get("declared_resource_domain_subaxis_authority") is None, "subaxis authority must be absent")
        blocked = row.get("blocked_by") or []
        need("ExplicitResourceDomainDeclarationMissing" in blocked, "missing explicit declaration blocker")
        need("TypeIdentityOnlyIsNotResourceDomainAuthority" in blocked, "missing identity-only blocker")

    decision = fixture.get("decision") or {}
    need(decision.get("kind") == "SelectSemanticResourceDomainDeclarationInventory", "decision kind drift")
    need(decision.get("reason_token") == "ExplicitSemanticResourceDomainDeclarationBasisDefined", "reason drift")
    need(decision.get("selected_domain_subaxis") is None, "subaxis must not be selected")
    need(decision.get("selected_next_card") == next_card, "next card drift")

    guard = fixture.get("guard") or {}
    need(guard.get("profile") == profile, "guard profile drift")
    need(guard.get("row_specific_guard_added") is False, "row-specific guard drift")

    check_forbidden_claims(fixture.get("claims") or {})

    rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
    manifest_row = rows_by_token.get(token) or {}
    need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_domain_object_id_lane_guard.sh"), "manifest guard drift")
    need(manifest_row.get("guard_profile") == profile, "manifest guard profile drift")

    need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

    print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-lane-guard")
    print("profile=" + profile)
    print("candidate_registry_row_count=42")
    print("decision=SelectSemanticResourceDomainDeclarationInventory")
    print("selected_next_card=" + next_card)
    print("source_selfhost_claim=0")
    print("summary=ok")
elif profile == "semantic_resource_domain_declaration_inventory":
    token = "MIRBUILDER-DOMAIN-OBJECT-ID-SEMANTIC-RESOURCE-DOMAIN-DECLARATION-INVENTORY-001"
    next_card = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-008"
    design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

    need(fixture.get("kind") == "MirBuilderDomainObjectIdSemanticResourceDomainDeclarationInventoryV1", "bad kind")
    need(fixture.get("token") == token, "bad token")
    need(token in card, "card missing token")
    need(token in task_order, "task-order missing token")
    need(next_card in task_order, "task-order missing next card")

    inputs = fixture.get("input_state") or {}
    need(inputs.get("current_blocker") == design_stop, "blocker drift")
    need(inputs.get("explicit_semantic_resource_domain_declaration_basis", "").endswith("mirbuilder-domain-object-id-explicit-semantic-resource-domain-declaration-basis-v0.json"), "basis input drift")

    rule = fixture.get("inventory_rule") or {}
    need(rule.get("name") == "SemanticResourceDomainDeclarationInventoryV1", "rule drift")
    for key in [
        "reads_existing_explicit_semantic_declarations_only",
        "self_signed_declaration_forbidden",
    ]:
        need(rule.get(key) is True, f"rule drift: {key}")
    for key in [
        "manual_type_to_subaxis_assignment",
        "return_type_string_to_subaxis_mapping",
        "type_name_or_source_path_inference",
        "observed_subaxis_set_inference",
        "source_path_as_policy_authority",
    ]:
        need(rule.get(key) is False, f"forbidden rule drift: {key}")

    summary = fixture.get("summary") or {}
    need(summary.get("candidate_registry_row_count") == 42, "candidate row count drift")
    need(summary.get("explicit_semantic_resource_domain_declaration_source_count") == 0, "declaration source count drift")
    need(summary.get("resource_domain_declaration_ready_count") == 0, "declaration ready drift")
    need(summary.get("stable_closed_resource_manifest_count") == 0, "closed manifest count drift")
    need(summary.get("registry_ready_row_count") == 0, "registry ready drift")
    need(summary.get("accepted_typed_dependency_edge_count") == 0, "accepted edge drift")

    rows = fixture.get("resource_domain_declaration_inventory_rows") or []
    need(len(rows) == 42, "inventory row count drift")
    for row in rows:
        need(row.get("resource_domain_declaration_state") == "Missing", "declaration state drift")
        need(row.get("declared_resource_domain_subaxis") is None, "subaxis must not be assigned")
        need(row.get("declared_resource_domain_subaxis_authority") is None, "subaxis authority must be absent")
        blocked = row.get("blocked_by") or []
        need("ExplicitSemanticResourceDomainDeclarationSourceMissing" in blocked, "missing declaration source blocker")
        need("TypeIdentityOnlyIsNotResourceDomainAuthority" in blocked, "missing identity-only blocker")

    decision = fixture.get("decision") or {}
    need(decision.get("kind") == "SelectWiderRouteSelectionBasis", "decision kind drift")
    need(decision.get("reason_token") == "ExplicitSemanticResourceDomainDeclarationSourceMissing", "reason drift")
    need(decision.get("selected_domain_subaxis") is None, "subaxis must not be selected")
    need(decision.get("selected_next_card") == next_card, "next card drift")

    check_forbidden_claims(fixture.get("claims") or {})

    rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
    manifest_row = rows_by_token.get(token) or {}
    need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_domain_object_id_lane_guard.sh"), "manifest guard drift")
    need(manifest_row.get("guard_profile") == profile, "manifest guard profile drift")

    need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

    print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-lane-guard")
    print("profile=" + profile)
    print("explicit_semantic_resource_domain_declaration_source_count=0")
    print("stable_closed_resource_manifest_count=0")
    print("decision=SelectWiderRouteSelectionBasis")
    print("selected_next_card=" + next_card)
    print("source_selfhost_claim=0")
    print("summary=ok")
elif profile == "post_domain_object_id_exhaustion_wider_selection":
    token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-008"
    next_card = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-BASIS-001"
    design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

    need(fixture.get("kind") == "SourceSelfhostWiderRouteSelectionBasis008V1", "bad kind")
    need(fixture.get("token") == token, "bad token")
    need(token in card, "card missing token")
    need(token in task_order, "task-order missing token")
    need(next_card in task_order, "task-order missing next card")

    inputs = fixture.get("input_state") or {}
    need(inputs.get("current_blocker") == design_stop, "blocker drift")
    need(inputs.get("domain_object_id_semantic_resource_domain_declaration_inventory", "").endswith("mirbuilder-domain-object-id-semantic-resource-domain-declaration-inventory-v0.json"), "declaration inventory input drift")
    need(inputs.get("carrier_type_transport_unclassified_evidence_resolution", "").endswith("mirbuilder-carrier-type-transport-unclassified-evidence-resolution-002-v0.json"), "carrier parent input drift")

    rule = fixture.get("selector_rule") or {}
    need(rule.get("name") == "PostDomainObjectIdAuthorityExhaustionWiderLaneSelectorV1", "selector rule drift")
    for key in [
        "domain_object_id_lane_must_be_parked_before_wider_selection",
        "subaxis_selection_forbidden",
        "semantic_owner_selection_forbidden",
        "freshness_repair_precedes_semantic_lane_selection",
        "native_checkpoint_requires_adoption_delta_or_stale_checkpoint",
        "nearest_unexhausted_parent_lane_allowed",
        "remaining_axis_priority_must_open_basis_not_axis",
    ]:
        need(rule.get(key) is True, f"rule drift: {key}")
    for key in [
        "row_count_as_proof",
        "owner_name_as_proof",
        "source_path_as_authority",
        "shape_signature_as_proof",
        "route_membership_alone_as_proof",
        "observed_subaxis_set_as_proof",
        "manual_family_shape_axis_selection",
    ]:
        need(rule.get(key) is False, f"forbidden rule drift: {key}")

    parking = fixture.get("domain_object_id_lane_parking") or {}
    need(parking.get("parked") is True, "DomainObject/Id lane must be parked")
    need(parking.get("park_reason_token") == "ExplicitSemanticResourceDomainDeclarationSourceMissing", "park reason drift")
    exhaustion = parking.get("authority_exhaustion") or {}
    need(exhaustion.get("candidate_registry_row_count") == 42, "candidate registry count drift")
    need(exhaustion.get("explicit_semantic_resource_domain_declaration_source_count") == 0, "declaration source count drift")
    need(exhaustion.get("stable_closed_resource_manifest_count") == 0, "closed manifest count drift")
    need(exhaustion.get("resource_domain_declaration_ready_count") == 0, "declaration ready drift")
    need(exhaustion.get("registry_ready_row_count") == 0, "registry ready drift")
    need(exhaustion.get("accepted_typed_dependency_edge_count") == 0, "accepted edge drift")

    parent = fixture.get("parent_lane_evidence") or {}
    need(parent.get("carrier_type_parent_ledger_fresh") is True, "carrier parent freshness drift")
    need(parent.get("remaining_non_domain_object_carrier_axes_present") is True, "remaining carrier axes drift")
    need(parent.get("carrier_type_previous_selected_axis") == "DomainObjectOrIdTransportAxis", "previous selected axis drift")

    lanes = fixture.get("candidate_lanes") or []
    need(len(lanes) == 6, "candidate lane count drift")
    eligible = [row for row in lanes if row.get("selection_eligible") is True]
    need(len(eligible) == 1, "eligible lane count drift")
    need(eligible[0].get("lane_id") == "CarrierTypeTransportRemainingLanePriority", "eligible lane drift")
    need(eligible[0].get("selected_next_card_if_eligible") == next_card, "eligible next card drift")

    summary = fixture.get("summary") or {}
    need(summary.get("domain_object_id_lane_parked") == 1, "parked summary drift")
    need(summary.get("domain_object_id_subaxis_selection_eligible") == 0, "subaxis selection drift")
    need(summary.get("candidate_lane_count") == 6, "candidate lane summary drift")
    need(summary.get("selection_eligible_lane_count") == 1, "eligible lane summary drift")

    decision = fixture.get("decision") or {}
    need(decision.get("kind") == "SelectCarrierTypeTransportRemainingAxisPriorityBasis", "decision kind drift")
    need(decision.get("reason_token") == "DomainObjectIdAuthorityExhaustedReturnToNearestUnexhaustedParentLane", "reason drift")
    need(decision.get("selected_domain_subaxis") is None, "subaxis must not be selected")
    need(decision.get("selected_next_card") == next_card, "next card drift")

    check_forbidden_claims(fixture.get("claims") or {})

    rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
    manifest_row = rows_by_token.get(token) or {}
    need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_domain_object_id_lane_guard.sh"), "manifest guard drift")
    need(manifest_row.get("guard_profile") == profile, "manifest guard profile drift")

    need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

    print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-lane-guard")
    print("profile=" + profile)
    print("domain_object_id_lane_parked=1")
    print("selection_eligible_lane_count=1")
    print("decision=SelectCarrierTypeTransportRemainingAxisPriorityBasis")
    print("selected_next_card=" + next_card)
    print("source_selfhost_claim=0")
    print("summary=ok")
else:
    raise SystemExit(f"unhandled profile: {profile}")
PY
