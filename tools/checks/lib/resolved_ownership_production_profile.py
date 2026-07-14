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
origin.literal.void_null
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
    storage = (root / "src/mir/storage_class.rs").read_text()
    representation = (root / "src/mir/value_representation_fact.rs").read_text()
    instruction = (root / "src/mir/instruction.rs").read_text()
    lowerer = (root / "src/mir/builder/resolved_lowering/lowerer.rs").read_text()
    json_emit = (root / "src/runner/mir_json_emit/metadata.rs").read_text()
    ownership_json = (
        root / "src/runner/mir_json/ownership_witness.rs"
    ).read_text()
    rust_lifecycle = (
        root / "src/backend/mir_interpreter/handlers/lifecycle.rs"
    ).read_text()
    vm_dispatch = (
        root / "src/backend/mir_interpreter/handlers/mod.rs"
    ).read_text()
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
        "owner_kind_not_first_family",
        "expression_not_in_first_family",
    )
    for anchor in required_capability_anchors:
        if anchor not in capability:
            fail(f"canonical capability boundary lost {anchor!r}")

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
    production_verifier_callers = 0
    for path in (root / "src").rglob("*.rs"):
        if ownership_dir in path.parents:
            continue
        production_verifier_callers += path.read_text(errors="ignore").count(
            "verify_ownership_ssa_v1("
        )
    if production_verifier_callers != 0:
        fail(
            "Ownership SSA production callers appeared before A1b/SSA-I1: "
            f"{production_verifier_callers}"
        )

    counts = {profile: 0 for profile in EXPECTED_PROFILES}
    for row in rows:
        counts[row["profile"]] += 1
    expected_counts = {
        "trivial_exact": 3,
        "derived_trivial_only": 7,
        "typed_preflight_reject": 4,
        "not_in_first_family": 3,
    }
    if counts != expected_counts:
        fail(f"profile counts drifted: expected={expected_counts} actual={counts}")

    print(
        "SSA-RC-V0 ownership profile: 17/17 rows, Rust handlers=2, "
        "path-sensitive verifier=1, production callers=0, BoxRef producers=0, "
        "production activation=0, trivial-only first cutover"
    )


if __name__ == "__main__":
    main()
