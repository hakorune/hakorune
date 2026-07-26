#!/usr/bin/env python3
"""Validate the behavior-neutral SSA-RC-P0 ownership production profile."""

from __future__ import annotations

import json
from pathlib import Path
import sys


EXPECTED_IDS = set(
    """
origin.receiver
origin.parameter
origin.local
origin.outbox
origin.literal.inline_i64
origin.literal.inline_bool
origin.literal.inline_f64
origin.literal.borrowed_text
origin.literal.void
origin.literal.null
origin.phi
origin.blockexpr_tail
origin.call_argument
origin.call_result
origin.binary_result
origin.binding_read
origin.assignment_value
origin.function_return
""".split()
)

EXPECTED_PROFILES = {
    "trivial_exact",
    "derived_trivial_only",
    "typed_preflight_reject",
    "not_in_first_family",
}

ROW_FIELDS = {
    "id",
    "origin",
    "current_path",
    "anchors",
    "current_status",
    "profile",
    "storage_authority",
    "reason",
}


def fail(message: str) -> None:
    raise SystemExit(f"SSA-RC-P0 ownership profile: {message}")


def require_count(text: str, literal: str, expected: int, label: str) -> None:
    actual = text.count(literal)
    if actual != expected:
        fail(f"{label} drifted: expected={expected} actual={actual}")


def main() -> None:
    if len(sys.argv) != 3:
        fail("usage: resolved_ownership_production_profile.py ROOT PROFILE")

    root = Path(sys.argv[1]).resolve()
    profile_path = Path(sys.argv[2]).resolve()
    data = json.loads(profile_path.read_text())

    if set(data) != {"schema", "decision", "claims", "profiles", "rows"}:
        fail("top-level schema drifted")
    if data["schema"] != "CanonicalOwnershipProductionProfileV1":
        fail("schema name drifted")
    if data["decision"] != "exact_boxref_or_trivial_only":
        fail("decision drifted")
    if set(data["profiles"]) != EXPECTED_PROFILES:
        fail("profile vocabulary drifted")

    expected_claims = {
        "exact_boxref_source_producers": 0,
        "production_ownership_opcode_callers": 0,
        "production_activation": 0,
        "first_atomic_cutover": "trivial_only_until_exact_boxref_witness",
    }
    if data["claims"] != expected_claims:
        fail("claims drifted")

    rows = data["rows"]
    ids = [row.get("id") for row in rows]
    if len(ids) != len(set(ids)):
        fail("duplicate row id")
    if set(ids) != EXPECTED_IDS:
        fail(
            f"required row set drifted: missing={sorted(EXPECTED_IDS - set(ids))} "
            f"extra={sorted(set(ids) - EXPECTED_IDS)}"
        )

    for row in rows:
        row_id = row["id"]
        if set(row) != ROW_FIELDS:
            fail(f"{row_id}: row schema drifted")
        if row["profile"] not in EXPECTED_PROFILES:
            fail(f"{row_id}: unknown profile {row['profile']}")
        path = root / row["current_path"]
        if not path.is_file():
            fail(f"{row_id}: missing current_path {path}")
        anchors = row["anchors"]
        if not isinstance(anchors, list) or not anchors:
            fail(f"{row_id}: anchors must be a non-empty list")
        text = path.read_text(errors="ignore")
        for anchor in anchors:
            if not isinstance(anchor, str) or not anchor or anchor not in text:
                fail(f"{row_id}: stale anchor {anchor!r}")
        for field in ("origin", "current_status", "storage_authority", "reason"):
            if not isinstance(row[field], str) or not row[field]:
                fail(f"{row_id}: {field} must be non-empty")

    capability = (root / "src/mir/compiler/capability.rs").read_text()
    function_role_policy = (
        root / "src/mir/compiler/capability/function_role_policy.rs"
    ).read_text()
    storage = (root / "src/mir/storage_class.rs").read_text()
    representation = (root / "src/mir/value_representation_fact.rs").read_text()
    instruction = (root / "src/mir/instruction.rs").read_text()
    lowerer = (root / "src/mir/builder/resolved_lowering/lowerer.rs").read_text()
    json_emit = (root / "src/runner/mir_json_emit/metadata.rs").read_text()
    ownership_json_emit = (
        root / "src/runner/mir_json_emit/ownership_ssa.rs"
    ).read_text()
    ownership_backend_capability = (
        root / "src/mir/ownership_backend_capability.rs"
    ).read_text()
    ownership_json = (
        root / "src/runner/mir_json/ownership_witness.rs"
    ).read_text()
    rust_lifecycle = (
        root / "src/backend/mir_interpreter/handlers/lifecycle.rs"
    ).read_text()
    vm_dispatch = (
        root / "src/backend/mir_interpreter/handlers/mod.rs"
    ).read_text()
    rust_interpreter = (
        root / "src/backend/mir_interpreter/mod.rs"
    ).read_text()
    rust_frame = (
        root / "src/backend/mir_interpreter/exec/frame_transaction.rs"
    ).read_text()
    rust_phi = (
        root / "src/backend/mir_interpreter/exec/phi.rs"
    ).read_text()
    rust_block = (
        root / "src/backend/mir_interpreter/exec/block.rs"
    ).read_text()
    backend_allowlists = (
        root / "src/mir/contracts/backend_core_ops/allowlists.rs"
    ).read_text()
    llvm_ownership_transport = (
        root / "src/llvm_py/ownership_lowering.py"
    ).read_text()
    llvm_ownership_handler = (
        root / "src/llvm_py/instructions/ownership.py"
    ).read_text()
    llvm_dispatch = (
        root / "src/llvm_py/builders/instruction_lower.py"
    ).read_text()
    llvm_function_lower = (
        root / "src/llvm_py/builders/function_lower.py"
    ).read_text()
    pyvm = (root / "src/llvm_py/pyvm/vm.py").read_text()
    ownership_dir = root / "src/mir/ownership_ssa"
    ownership_sources = "\n".join(
        path.read_text()
        for path in sorted(ownership_dir.glob("*.rs"))
        if path.name != "tests.rs"
    )
    json_v0_files = [
        root / "src/runner/mir_json_v0.rs",
        *(root / "src/runner/mir_json_v0").glob("*.rs"),
    ]
    json_v0_parse = "\n".join(path.read_text() for path in json_v0_files)

    required_capability_anchors = (
        "typed_signature_not_activated",
        "expression_not_in_first_family",
    )
    for anchor in required_capability_anchors:
        if anchor not in capability:
            fail(f"canonical capability boundary lost {anchor!r}")
    if "SourceBindingSiteV1::Receiver" not in capability:
        fail("canonical capability product lost its receiver-defense boundary")
    if "Self::OrdinaryFirstFamily => \"owner_kind_not_first_family\"" not in function_role_policy:
        fail("canonical function-role policy lost its first-family receiver rejection")

    if "no-behavior-change inventory" not in storage:
        fail("StorageClass stopped declaring itself non-authoritative inventory")
    require_count(storage, "MirType::Box(_) => StorageClass::BoxRef", 1, "BoxRef inventory map")
    if "BoxRef" in representation:
        fail("generic BoxRef representation fact appeared without P0/A0 schema decision")
    require_count(representation, "BoxedSumHandle", 3, "existing representation vocabulary")

    require_count(instruction, "CopyOwned {", 1, "passive CopyOwned variant")
    require_count(instruction, "DestroyOwned {", 1, "passive DestroyOwned variant")
    if "CopyOwned" in lowerer or "DestroyOwned" in lowerer:
        fail("canonical production ownership caller appeared during passive A0")
    require_count(lowerer, "MirInstruction::ReleaseStrong", 1, "legacy canonical release caller")

    require_count(json_emit, '"storage_classes"', 1, "JSON storage inventory emitter")
    for anchor in (
        'required_map(metadata, "value_types")',
        'required_map(metadata, "storage_classes")',
        "StorageClass::BoxRef",
        "copy_owned type mismatch",
    ):
        if anchor not in ownership_json:
            fail(f"direct JSON ownership witness lost {anchor!r}")
    for anchor in ('"copy_owned"', '"destroy_owned"'):
        if anchor not in json_v0_parse:
            fail(f"JSON v0 passive transport lost {anchor}")
    for anchor in (
        "pub(super) fn copy_owned",
        "VMValue::BoxRef(value.clone())",
        "vm/ownership:dst_already_defined",
        "pub(super) fn destroy_owned",
        "self.take_reg(value)",
    ):
        if anchor not in rust_lifecycle:
            fail(f"Rust ownership handler lost {anchor!r}")
    for anchor in (
        "MirInstruction::CopyOwned",
        "MirInstruction::DestroyOwned",
    ):
        if anchor not in vm_dispatch:
            fail(f"Rust ownership dispatch lost {anchor!r}")
    for anchor in (
        "enum MirOwnershipKindV1",
        "struct VerifiedOwnershipSsaV1",
        "abi: OwnershipFunctionAbiV1",
        "collect_ownership_operations",
        "matches_function",
        "OwnershipDispositionV1::PhiEdge",
        "transfer_phi_edge",
        "DuplicateConsumeOnEdge",
        "MissingDispositionAtExit",
        "BorrowedPhiForbidden",
        "ManagedCallOwnershipUnsupported",
        "EdgeArgumentsForbidden",
        "UnreachableBlock",
    ):
        if anchor not in ownership_sources:
            fail(f"Ownership SSA verifier lost {anchor!r}")
    for text, anchors, label in (
        (
            rust_interpreter,
            ("active_ownership_ssa",),
            "Rust ownership session",
        ),
        (
            rust_frame,
            (
                "with_verified_ownership_function_frame",
                "seed_parameters_owned",
                "active_ownership_ssa = Some(witness)",
            ),
            "Rust ownership ABI forwarding",
        ),
        (
            rust_phi,
            (
                "apply_owned_phi_nodes",
                "self.take_reg(*source)",
                "for (destination, value) in values",
            ),
            "Rust Owned Phi forwarding",
        ),
        (
            rust_block,
            ("Owned Return value is undefined", "self.take_reg(*v)"),
            "Rust Owned Return forwarding",
        ),
        (
            backend_allowlists,
            (
                'MirInstruction::CopyOwned { .. } => &["copy_owned"]',
                'MirInstruction::DestroyOwned { .. } => &["destroy_owned"]',
            ),
            "llvm_py ownership allowlist",
        ),
        (
            llvm_ownership_transport,
            (
                '"VerifiedOwnershipSsaV1"',
                '"rust_ownership_ssa_verifier_v1"',
                '"nyash_kernel"',
                "VerifiedOwnershipLoweringSessionV1",
                "legacy_mix",
                "missing_boxref",
                "incomplete_coverage",
            ),
            "llvm_py ownership transport",
        ),
        (
            llvm_ownership_handler,
            (
                '"nyrt_handle_retain_h"',
                '"nyrt_handle_release_h"',
                "resolve_i64_strict",
                "session.claim",
            ),
            "llvm_py ownership materializer",
        ),
        (
            llvm_dispatch,
            ('"copy_owned"', '"destroy_owned"', "lower_copy_owned", "lower_destroy_owned"),
            "llvm_py ownership dispatch",
        ),
        (
            llvm_function_lower,
            ("verify_ownership_lowering_v1", "context.ownership_ssa_v1.finish()"),
            "llvm_py ownership session boundary",
        ),
        (
            pyvm,
            ("pyvm/ownership:missing_capability", 'op in ("copy_owned", "destroy_owned")'),
            "PyVM ownership fail-fast",
        ),
        (
            ownership_json_emit,
            (
                "build_ownership_ssa_json",
                '"VerifiedOwnershipSsaV1"',
                '"rust_ownership_ssa_verifier_v1"',
                '"llvm_py"',
                '"nyash_kernel"',
                ".operations()",
            ),
            "Rust ownership witness transport",
        ),
        (
            ownership_backend_capability,
            (
                "backend-missing-capability:owned-value-lifecycle-v1",
                'backend != "llvmlite-obj"',
                "ownership-backend:missing-witness",
                "ownership-backend:stale-witness",
                "witness.matches_function(function)",
            ),
            "ownership backend preflight",
        ),
    ):
        for anchor in anchors:
            if anchor not in text:
                fail(f"{label} lost {anchor!r}")
    def without_cfg_tests(text: str) -> str:
        return text.split("#[cfg(test)]", 1)[0]

    production_verifier_callers = 0
    for path in (root / "src").rglob("*.rs"):
        if "tests" in path.parts:
            continue
        if ownership_dir in path.parents:
            continue
        if path.name == "ownership_forwarding_tests.rs":
            continue
        production_verifier_callers += without_cfg_tests(
            path.read_text(errors="ignore")
        ).count(
            "verify_ownership_ssa_v1("
        )
    if production_verifier_callers != 0:
        fail(
            "Ownership SSA production callers appeared before A1c/SSA-I1: "
            f"{production_verifier_callers}"
        )

    rust_witness_emitters = 0
    production_witness_installers = 0
    for path in (root / "src").rglob("*.rs"):
        if "tests" in path.parts:
            continue
        production_text = without_cfg_tests(path.read_text(errors="ignore"))
        rust_witness_emitters += production_text.count('metadata_json["ownership_ssa_v1"]')
        production_witness_installers += production_text.count(".ownership_ssa_v1 = Some(")
    if rust_witness_emitters != 1:
        fail(f"Rust ownership witness transport emitter drifted: {rust_witness_emitters}")
    if production_witness_installers != 0:
        fail(
            "canonical Rust ownership witness installer appeared before SSA-I1: "
            f"{production_witness_installers}"
        )

    counts = {profile: 0 for profile in EXPECTED_PROFILES}
    for row in rows:
        counts[row["profile"]] += 1
    expected_counts = {
        "trivial_exact": 5,
        "derived_trivial_only": 7,
        "typed_preflight_reject": 3,
        "not_in_first_family": 3,
    }
    if counts != expected_counts:
        fail(f"profile counts drifted: expected={expected_counts} actual={counts}")

    print(
        "SSA-RC-A1c ownership profile: 18/18 rows, exact trivial=5, Rust handlers=2, llvm_py handlers=2, "
        "path-sensitive verifier=1, Rust witness consumer=1, Rust witness transport=1, "
        "production witness installers=0, "
        "production callers=0, BoxRef producers=0, "
        "production activation=0, trivial-only first cutover"
    )


if __name__ == "__main__":
    main()
