#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-borrow-surface-returned-mutable-borrow-policy-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_borrow_surface_returned_mutable_borrow_policy.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1], encoding="utf-8"))

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

need(data.get("kind") == "MirBuilderBorrowSurfaceReturnedMutableBorrowPolicyV1", "bad kind")
need(data.get("token") == "MIRBUILDER-BORROW-SURFACE-RETURNED-MUTABLE-BORROW-POLICY-001", "bad token")

selected = data.get("selected_cluster") or {}
need(selected.get("borrow_kind") == "ReturnedMutableBorrow", "selected borrow kind must be ReturnedMutableBorrow")
need(selected.get("return_shape") == "mutable_ref", "selected return shape must be mutable_ref")
need(selected.get("receiver_axis") == "mutable_receiver", "selected receiver axis must be mutable_receiver")
need(selected.get("source_module") == "loop_cond_return_in_body_phi_materializer", "bad source module")
need(selected.get("repaired_owner_edge_confidence") == "FileScoped", "owner edge confidence must be FileScoped")
need(selected.get("signature") == "&mut self -> &mut BTreeMap<String, ValueId>", "bad source signature")

evidence = data.get("source_evidence") or {}
for key in [
    "source_surface_verified",
    "source_signature_verified",
    "bounded_callsite_verified",
    "mutation_frame_consumers_verified",
    "field_return_verified",
]:
    need(evidence.get(key) is True, f"{key} must be true")
for key in [
    "alias_escape_allowed",
    "stored_borrow_allowed",
    "caller_owned_mutable_alias",
]:
    need(evidence.get(key) is False, f"{key} must be false")
for consumer in [
    "lower_stmt_ast",
    "lower_assignment_stmt",
    "lower_local_init_stmt",
    "lower_method_call_stmt",
    "lower_function_call_stmt",
]:
    need(consumer in evidence.get("mutation_frame_consumers", []), f"missing consumer {consumer}")

strict = data.get("strict_policy") or {}
need(strict.get("raw_returned_mutable_borrow") == "Deny", "raw returned mutable borrow must be denied")
need(strict.get("deny_reason") == "ReturnedMutableBorrow", "bad deny reason")
for key in [
    "raw_mutable_alias_transport",
    "rust_lifetime_syntax_transport",
    "converter_emitter_may_choose_representation",
]:
    need(strict.get(key) is False, f"{key} must be false")

replacement = data.get("replacement_policy") or {}
need(replacement.get("kind") == "BoundedWithMapOperation", "bad replacement policy")
need(replacement.get("alias") == "BoundedOwnerMutationFrame", "bad replacement alias")
need(
    replacement.get("replacement_id") == "LoopCondReturnInBodyPhiMaterializerCurrentBindingsMutationFrameV1",
    "bad replacement id",
)
need(replacement.get("owner") == "LoopCondReturnInBodyPhiMaterializer", "bad bounded frame owner")
need(replacement.get("owned_field") == "current_bindings", "bad bounded field")
need(replacement.get("entry_surface") == "current_bindings_mut", "bad entry surface")
need(replacement.get("bounded_callsite") == "lower_return_in_body_block", "bad bounded callsite")
need(replacement.get("requires_followup_descriptor") is True, "follow-up descriptor must be required")

decision = data.get("decision") or {}
need(decision.get("kind") == "SelectReplacementPolicy", "bad decision kind")
need(decision.get("selected_policy") == "BoundedWithMapOperation", "bad selected policy")
need(
    decision.get("selected_next_card")
    == "MIRBUILDER-LOOP-COND-RETURN-IN-BODY-PHI-MATERIALIZER-CURRENT-BINDINGS-MUTATION-FRAME-001",
    "bad next card",
)

claims = data.get("claims") or {}
need(claims.get("selected_cluster_consumed") == 1, "selected cluster must be consumed")
need(claims.get("bounded_mutation_frame_selected") == 1, "bounded mutation frame must be selected")
for key in [
    "raw_mutable_alias_selected",
    "returned_mutable_borrow_allowed",
    "explicit_mutation_api_selected",
    "mut_lease_selected",
    "replace_owned_transfer_selected",
    "hako_generation",
    "hako_adopted_decision",
    "native_seed_materialization",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "manual_borrow_policy_selection",
    "cluster_size_as_proof",
    "strict_rules_changed",
    "converter_tool_role_semantic_projector",
]:
    need(claims.get(key) == 0, f"{key} must be 0")

print("output_contract=rust-lifecycle-mirbuilder-borrow-surface-returned-mutable-borrow-policy")
print("raw_returned_mutable_borrow=Deny")
print("replacement_policy=BoundedWithMapOperation")
print("bounded_mutation_frame_selected=1")
print(f"selected_next_card={decision.get('selected_next_card')}")
print("hako_generation=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
