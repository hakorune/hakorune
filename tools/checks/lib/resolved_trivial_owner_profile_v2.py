#!/usr/bin/env python3
"""Validate the current V2 receipt for the split trivial-owner profile."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys


EXPECTED_MANIFEST = {
    "README.md", "analyzer.rs", "analyzer_policy.rs", "consumption.rs",
    "coverage.rs", "direct_call.rs", "direct_call_tests.rs", "error.rs",
    "function_return.rs", "mod.rs", "operator.rs", "parameter_entry.rs",
    "parameter_tests.rs", "product.rs", "return_tests.rs", "tests.rs",
}
EXPECTED_TESTS = {
    "direct_call_tests.rs", "parameter_tests.rs", "return_tests.rs", "tests.rs",
}
EXPECTED_ENTRIES = {
    "analyze_trivial_canonical_owner_v1",
    "analyze_trivial_canonical_main_owner_v1",
    "analyze_trivial_canonical_main_owner_with_finite_direct_calls_v1",
    "analyze_trivial_canonical_owner_with_finite_direct_calls_v1",
}
FORBIDDEN = {
    "BasicBlockId", "BindingSsaBuilderV1", "CopyOwned", "DestroyOwned", "IfJoin",
    "MirBindingSsaAdapterV1", "MirBuilder", "MirInstruction", "MirType", "Span",
    "StorageClass", "VMValue", "ValueId", "may_rebind", "name_lookup",
    "pointer_identity", "variable_map",
}


def fail(message: str) -> None:
    raise SystemExit(f"SSA-I0-PROFILE V2 guard: {message}")


def production_text(path: Path) -> str:
    return path.read_text(errors="ignore").split("#[cfg(test)]", 1)[0]


def production_rust(root: Path):
    for path in (root / "src").rglob("*.rs"):
        if "tests" in path.parts or path.name == "tests.rs" or path.name.endswith("_tests.rs"):
            continue
        yield path


def main() -> None:
    if len(sys.argv) != 3:
        fail("usage: resolved_trivial_owner_profile_v2.py ROOT RECEIPT")
    root = Path(sys.argv[1]).resolve()
    receipt_path = Path(sys.argv[2]).resolve()
    receipt = json.loads(receipt_path.read_text())
    if set(receipt) != {"schema", "decision", "historical_v1", "current_ssa_seam", "ownership_profile", "claims", "entries"}:
        fail("receipt schema drifted")
    if receipt["schema"] != "CurrentTrivialOwnerProfileReceiptV2":
        fail("V2 schema drifted")
    if receipt["decision"] != "current_split_profile_receipt_without_semantic_widening":
        fail("V2 decision drifted")
    claims = receipt["claims"]
    expected_claims = {
        "profile_manifest_entries": 16,
        "profile_production_rust_files": 11,
        "profile_test_rust_files": 4,
        "profile_entry_definitions": 4,
        "profile_entry_consumer": "src/mir/compiler/capability.rs",
        "binding_ssa_production_callers": 1,
        "ownership_ssa_production_callers": 0,
        "ownership_ssa_witness_installers": 0,
        "production_ownership_opcode_callers": 0,
        "exact_boxref_source_producers": 0,
        "semantic_widening": 0,
    }
    if claims != expected_claims or set(receipt["entries"]) != EXPECTED_ENTRIES:
        fail("V2 current claims or entry vocabulary drifted")

    historical = receipt["historical_v1"]
    historical_data = json.loads((root / historical["path"]).read_text())
    if historical != {"path": "tools/checks/fixtures/canonical_trivial_owner_profile_v1.json", "schema": "VerifiedTrivialCanonicalOwnerProfileContractV1"} or historical_data.get("schema") != historical["schema"]:
        fail("V1 historical receipt is not retained")
    for field, schema, rows in (
        ("current_ssa_seam", "CanonicalSsaSeamInventoryV1", 92),
        ("ownership_profile", "CanonicalOwnershipProductionProfileV1", 18),
    ):
        entry = receipt[field]
        data = json.loads((root / entry["path"]).read_text())
        if entry["schema"] != schema or entry["rows"] != rows or data.get("schema") != schema or len(data.get("rows", [])) != rows:
            fail(f"{field} receipt drifted")

    owner = root / "src/mir/resolved_value_profile"
    manifest = {path.name for path in owner.iterdir() if path.is_file()}
    if manifest != EXPECTED_MANIFEST:
        fail(f"profile manifest drifted: expected={sorted(EXPECTED_MANIFEST)} actual={sorted(manifest)}")
    production_files = [path for path in owner.glob("*.rs") if path.name not in EXPECTED_TESTS]
    if len(production_files) != claims["profile_production_rust_files"]:
        fail("current production profile file count drifted")
    if len(EXPECTED_TESTS) != claims["profile_test_rust_files"]:
        fail("current test profile file count drifted")
    owner_text = "\n".join(path.read_text() for path in production_files)
    for token in FORBIDDEN:
        if token in owner_text:
            fail(f"profile regained forbidden authority token: {token}")
    for entry in EXPECTED_ENTRIES:
        if len(re.findall(rf"pub\(crate\) fn {entry}\s*\(", owner_text)) != 1:
            fail(f"profile entry definition drifted: {entry}")
    if "mod analyzer_policy;" not in (owner / "mod.rs").read_text():
        fail("current V2 receipt lost the split analyzer-policy module")

    callers = []
    references = []
    for path in production_rust(root):
        if owner in path.parents:
            continue
        text = production_text(path)
        if any(f"{entry}(" in text for entry in EXPECTED_ENTRIES):
            callers.append(str(path.relative_to(root)))
        if "resolved_value_profile" in text:
            references.append(str(path.relative_to(root)))
    expected_consumer = claims["profile_entry_consumer"]
    if callers != [expected_consumer]:
        fail(f"profile entry caller set drifted: {callers}")
    if expected_consumer not in references:
        fail("current profile entry consumer lost its module boundary")

    binding_callers = [
        str(path.relative_to(root))
        for path in production_rust(root)
        if "src/mir/builder/ssa/binding" not in str(path)
        and ("BindingSsaBuilderV1" in production_text(path) or "MirBindingSsaAdapterV1" in production_text(path))
    ]
    if binding_callers != ["src/mir/builder/resolved_lowering/trivial_ssa/identity.rs"]:
        fail(f"Binding SSA caller set drifted: {binding_callers}")
    ownership_callers = sum(
        production_text(path).count("verify_ownership_ssa_v1(")
        for path in production_rust(root)
        if "src/mir/ownership_ssa" not in str(path)
    )
    installers = sum(production_text(path).count(".ownership_ssa_v1 = Some(") for path in production_rust(root))
    if ownership_callers or installers:
        fail(f"ownership receipt drifted: callers={ownership_callers} installers={installers}")
    print("canonical_ssa_i0_profile_receipt=v2-current-split")
    print("canonical_ssa_i0_profile_v1=historical-retained")
    print("canonical_ssa_i0_profile_semantic_widening=0")


if __name__ == "__main__":
    main()
