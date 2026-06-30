#!/usr/bin/env python3
"""Select the returned mutable borrow replacement policy for the current cluster."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
RERUN = FIXTURES / "mirbuilder-borrow-surface-policy-cluster-rerun-v0.json"
OUTPUT = FIXTURES / "mirbuilder-borrow-surface-returned-mutable-borrow-policy-v0.json"
CURRENT_STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
STRICT_REFERENCE = ROOT / "docs/reference/architecture/rust-to-hako-lifecycle-projection.md"
VARIABLE_CONTEXT_PRECEDENT = FIXTURES / "variable-context-reference-projection-contract-v0.json"

PHI_MATERIALIZER = ROOT / "src/mir/builder/control_flow/plan/features/loop_cond_return_in_body_phi_materializer.rs"
PIPELINE = ROOT / "src/mir/builder/control_flow/plan/features/loop_cond_return_in_body_pipeline.rs"

TOKEN = "MIRBUILDER-BORROW-SURFACE-RETURNED-MUTABLE-BORROW-POLICY-001"
FOLLOW_UP = (
    "MIRBUILDER-LOOP-COND-RETURN-IN-BODY-PHI-MATERIALIZER-"
    "CURRENT-BINDINGS-MUTATION-FRAME-001"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def find_line(path: Path, pattern: str) -> int | None:
    for index, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        if pattern in line:
            return index
    return None


def source_evidence() -> dict[str, Any]:
    phi_text = PHI_MATERIALIZER.read_text(encoding="utf-8")
    pipeline_text = PIPELINE.read_text(encoding="utf-8")

    current_bindings_sig = re.search(
        r"fn\s+current_bindings_mut\s*\(\s*&mut\s+self,\s*\)\s*->\s*&mut\s+BTreeMap<String,\s*ValueId>",
        phi_text,
        re.S,
    )
    lower_block_sig = re.search(
        r"fn\s+lower_return_in_body_block\s*\([^)]*current_bindings:\s*&mut\s+BTreeMap<String,\s*crate::mir::ValueId>",
        pipeline_text,
        re.S,
    )

    consumers = [
        "lower_stmt_ast",
        "lower_assignment_stmt",
        "lower_local_init_stmt",
        "lower_method_call_stmt",
        "lower_function_call_stmt",
    ]

    return {
        "source_surface_verified": current_bindings_sig is not None,
        "source_signature_verified": current_bindings_sig is not None,
        "source_signature": "&mut self -> &mut BTreeMap<String, ValueId>",
        "source_surface": f"{rel(PHI_MATERIALIZER)}::current_bindings_mut:L{find_line(PHI_MATERIALIZER, 'fn current_bindings_mut')}",
        "bounded_callsite_verified": "phi_materializer.current_bindings_mut()" in pipeline_text
        and "lower_return_in_body_block(" in pipeline_text
        and lower_block_sig is not None,
        "bounded_callsite": f"{rel(PIPELINE)}::lower_return_in_body_block:L{find_line(PIPELINE, 'fn lower_return_in_body_block')}",
        "mutation_frame_consumers_verified": all(consumer in pipeline_text for consumer in consumers),
        "mutation_frame_consumers": consumers,
        "field_return_verified": "&mut self.current_bindings" in phi_text,
        "alias_escape_allowed": False,
        "stored_borrow_allowed": False,
        "caller_owned_mutable_alias": False,
    }


def build_policy() -> dict[str, Any]:
    rerun = read_json(RERUN)
    selected = rerun.get("selected_cluster") or {}
    evidence = source_evidence()

    return {
        "schema_version": 0,
        "kind": "MirBuilderBorrowSurfaceReturnedMutableBorrowPolicyV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "cluster_rerun": rel(RERUN),
            "selected_cluster_id": selected.get("cluster_id"),
            "strict_reference": rel(STRICT_REFERENCE),
            "variable_context_precedent": rel(VARIABLE_CONTEXT_PRECEDENT),
        },
        "provenance": {
            "cluster_rerun_hash": sha256_file(RERUN),
            "phi_materializer_source_hash": sha256_file(PHI_MATERIALIZER),
            "pipeline_source_hash": sha256_file(PIPELINE),
        },
        "selected_cluster": {
            "borrow_kind": selected.get("borrow_kind"),
            "return_shape": selected.get("return_shape"),
            "receiver_axis": selected.get("receiver_axis"),
            "source_module": selected.get("source_module"),
            "source_surface": evidence["source_surface"],
            "signature": evidence["source_signature"],
            "repaired_owner_edge_id": selected.get("repaired_owner_edge_id"),
            "repaired_owner_edge_confidence": selected.get("repaired_owner_edge_confidence"),
        },
        "source_evidence": evidence,
        "strict_policy": {
            "raw_returned_mutable_borrow": "Deny",
            "deny_reason": "ReturnedMutableBorrow",
            "raw_mutable_alias_transport": False,
            "rust_lifetime_syntax_transport": False,
            "converter_emitter_may_choose_representation": False,
        },
        "replacement_policy": {
            "kind": "BoundedWithMapOperation",
            "alias": "BoundedOwnerMutationFrame",
            "replacement_id": "LoopCondReturnInBodyPhiMaterializerCurrentBindingsMutationFrameV1",
            "owner": "LoopCondReturnInBodyPhiMaterializer",
            "owned_field": "current_bindings",
            "entry_surface": "current_bindings_mut",
            "bounded_callsite": "lower_return_in_body_block",
            "mutation_frame": [
                "current_bindings map is mutable only inside body-lowering frame",
                "assignment/local-init/method/function lowering may update current_bindings",
                "no returned alias escapes the replacement operation",
                "owner remains LoopCondReturnInBodyPhiMaterializer after frame",
            ],
            "allowed_operations": [
                "MapGetCopied",
                "MapSet",
                "MapRemoveIfExisting",
                "MapClearOnlyIfSourceEvidenceExists",
            ],
            "requires_followup_descriptor": True,
        },
        "decision": {
            "kind": "SelectReplacementPolicy",
            "reason_token": "ReturnedMutableBorrowReplacedByBoundedWithMapOperation",
            "selected_policy": "BoundedWithMapOperation",
            "selected_next_card": FOLLOW_UP,
        },
        "claims": {
            "selected_cluster_consumed": 1,
            "raw_mutable_alias_selected": 0,
            "returned_mutable_borrow_allowed": 0,
            "bounded_mutation_frame_selected": 1,
            "explicit_mutation_api_selected": 0,
            "mut_lease_selected": 0,
            "replace_owned_transfer_selected": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "native_seed_materialization": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
            "manual_borrow_policy_selection": 0,
            "cluster_size_as_proof": 0,
            "strict_rules_changed": 0,
            "converter_tool_role_semantic_projector": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify the checked-in policy fixture.")
    args = parser.parse_args()

    output = stable_json(build_policy())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-borrow-surface-returned-mutable-borrow-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
