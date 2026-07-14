#!/usr/bin/env python3
"""Validate the executable but production-disconnected SSA-I0-PROFILE."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import re
import sys


SCHEMA = "VerifiedTrivialCanonicalOwnerProfileContractV1"
HISTORICAL_SHA256 = (
    "0c8395ce8f893ee0e7faf427490f77f8da6a5f3f7a729aae06d2a2cd382927b4"
)
EXPECTED_CLAIMS = {
    "profile_manifest_entries": 11,
    "profile_production_rust_files": 8,
    "profile_test_rust_files": 2,
    "profile_entry_definitions": 1,
    "sealed_product_definitions": 1,
    "focused_profile_fixtures": 18,
    "profile_production_callers": 1,
    "production_route_delta": 1,
    "accepted_grammar_delta": 0,
    "binding_ssa_production_callers": 1,
    "ownership_ssa_production_callers": 0,
    "ownership_ssa_witness_installers": 0,
    "production_ownership_opcode_callers": 0,
    "exact_boxref_source_producers": 0,
}
EXPECTED_SYMBOLS = {
    "VerifiedTrivialCanonicalOwnerV1",
    "TrivialCanonicalOwnerAnalysisV1",
    "analyze_trivial_canonical_owner_v1",
}
EXPECTED_MANIFEST = {
    "README.md",
    "analyzer.rs",
    "consumption.rs",
    "coverage.rs",
    "error.rs",
    "mod.rs",
    "operator.rs",
    "parameter_entry.rs",
    "parameter_tests.rs",
    "product.rs",
    "tests.rs",
}
EXPECTED_PRODUCTION_RUST = {
    "analyzer.rs",
    "consumption.rs",
    "coverage.rs",
    "error.rs",
    "mod.rs",
    "operator.rs",
    "parameter_entry.rs",
    "product.rs",
}
EXPECTED_TEST_RUST = {"parameter_tests.rs", "tests.rs"}
EXPECTED_FORBIDDEN = {
    "BasicBlockId",
    "BindingSsaBuilderV1",
    "CopyOwned",
    "DestroyOwned",
    "IfJoin",
    "MirBindingSsaAdapterV1",
    "MirBuilder",
    "MirInstruction",
    "MirType",
    "Span",
    "StorageClass",
    "VMValue",
    "ValueId",
    "may_rebind",
    "name_lookup",
    "pointer_identity",
    "variable_map",
}
EXPECTED_VALUES = {
    "literal.inline_i64": ("origin.literal.inline_i64", "exact_trivial"),
    "literal.inline_bool": ("origin.literal.inline_bool", "exact_trivial"),
    "literal.inline_f64": ("origin.literal.inline_f64", "exact_trivial"),
    "literal.explicit_void_value": ("origin.literal.void", "exact_trivial"),
    "literal.null_sentinel": ("origin.literal.null", "exact_trivial"),
    "local.initializer": ("origin.local", "forward_exact_trivial"),
    "binding.read": ("origin.binding_read", "forward_exact_trivial"),
    "assignment.value": ("origin.assignment_value", "forward_exact_trivial"),
    "binary.result": ("origin.binary_result", "derive_homogeneous_trivial"),
    "blockexpr.tail": ("origin.blockexpr_tail", "forward_exact_trivial"),
    "if.merge.homogeneous": ("origin.phi", "derive_homogeneous_trivial"),
    "return.value": ("origin.function_return", "forward_exact_trivial"),
}
EXPECTED_TERMINALS = {
    "explicit_trivial_value_return": ("value", "exact_trivial"),
    "explicit_empty_return": ("no_value", "none"),
    "implicit_fallthrough": ("no_value", "none"),
    "nested_or_nonterminal_return": ("typed_reject", "none"),
}
EXPECTED_REJECTIONS = {
    "receiver": ("origin.receiver", "owner_family_not_admitted"),
    "parameter.unsupported": (
        "origin.parameter",
        "unsupported_parameter_abi_not_sealed",
    ),
    "outbox": ("origin.outbox", "outbox_void_seed_is_not_a_value"),
    "string_literal": (
        "origin.literal.borrowed_text",
        "borrowed_text_not_admitted",
    ),
    "local_without_initializer": ("origin.local", "definition_profile_missing"),
    "mixed_if_merge": ("origin.phi", "incoming_profiles_not_homogeneous"),
}
EXPECTED_PARAMETER_ROWS = {
    "parameter.inline_i64": (
        "i64",
        "InlineI64",
        "definition",
        "disconnected",
    )
}


def fail(message: str) -> None:
    raise SystemExit(f"SSA-I0-PROFILE guard: {message}")


def checked_rows(rows: object, fields: set[str], label: str) -> list[dict]:
    if not isinstance(rows, list):
        fail(f"{label} must be a list")
    result: list[dict] = []
    ids: list[str] = []
    for row in rows:
        if not isinstance(row, dict) or set(row) != fields:
            fail(f"{label} row schema drifted: {row!r}")
        row_id = row.get("id")
        if not isinstance(row_id, str) or not row_id:
            fail(f"{label} row id must be non-empty")
        ids.append(row_id)
        result.append(row)
    if len(ids) != len(set(ids)):
        fail(f"{label} contains duplicate ids")
    return result


def row_pairs(rows: list[dict], second: str, third: str) -> dict[str, tuple[str, str]]:
    return {row["id"]: (row[second], row[third]) for row in rows}


def production_text(path: Path) -> str:
    return path.read_text(errors="ignore").split("#[cfg(test)]", 1)[0]


def production_rust(root: Path):
    for path in (root / "src").rglob("*.rs"):
        if (
            "tests" in path.parts
            or path.name == "tests.rs"
            or path.name.endswith("_tests.rs")
        ):
            continue
        yield path


def main() -> None:
    if len(sys.argv) != 3:
        fail("usage: resolved_trivial_owner_profile.py ROOT PROFILE")
    root = Path(sys.argv[1]).resolve()
    profile_path = Path(sys.argv[2]).resolve()
    data = json.loads(profile_path.read_text())

    expected_top = {
        "schema",
        "decision",
        "physical_owner",
        "implementation",
        "historical_ssa_seam",
        "ownership_inventory",
        "claims",
        "profile_symbols",
        "parameter_rows",
        "value_rows",
        "terminal_rows",
        "rejection_rows",
        "forbidden_authorities",
    }
    if set(data) != expected_top:
        fail("top-level schema drifted")
    if data["schema"] != SCHEMA:
        fail("schema name drifted")
    if data["decision"] != "a_prime_executable_sealed_whole_owner_profile":
        fail("A-prime decision drifted")
    if data["physical_owner"] != "src/mir/resolved_value_profile":
        fail("future Rust physical owner drifted")
    if data["claims"] != EXPECTED_CLAIMS:
        fail("executable-profile claims drifted")
    if set(data["profile_symbols"]) != EXPECTED_SYMBOLS:
        fail("reserved profile symbol vocabulary drifted")
    parameter_rows = checked_rows(
        data["parameter_rows"],
        {
            "id",
            "source_spelling",
            "representation",
            "coverage_authority",
            "production_route",
        },
        "parameter_rows",
    )
    actual_parameter_rows = {
        row["id"]: (
            row["source_spelling"],
            row["representation"],
            row["coverage_authority"],
            row["production_route"],
        )
        for row in parameter_rows
    }
    if actual_parameter_rows != EXPECTED_PARAMETER_ROWS:
        fail("exact parameter profile matrix drifted")
    if set(data["forbidden_authorities"]) != EXPECTED_FORBIDDEN:
        fail("forbidden authority vocabulary drifted")

    implementation = data["implementation"]
    if set(implementation) != {
        "manifest",
        "production_rust",
        "test_rust",
        "entry_symbol",
        "entry_inputs",
        "sealed_product",
        "analysis_result",
    }:
        fail("implementation schema drifted")
    if set(implementation["manifest"]) != EXPECTED_MANIFEST:
        fail("exact implementation manifest declaration drifted")
    if set(implementation["production_rust"]) != EXPECTED_PRODUCTION_RUST:
        fail("production Rust manifest declaration drifted")
    if set(implementation["test_rust"]) != EXPECTED_TEST_RUST:
        fail("test Rust manifest declaration drifted")
    if implementation["entry_symbol"] != "analyze_trivial_canonical_owner_v1":
        fail("disconnected analyzer entry symbol drifted")
    if set(implementation["entry_inputs"]) != {
        "ResolvedFunctionLoweringInputV1",
        "VerifiedFunctionCompletionV1",
        "VerifiedResolvedFunctionIfControlV1",
    }:
        fail("co-sealed analyzer input vocabulary drifted")
    if implementation["sealed_product"] != "VerifiedTrivialCanonicalOwnerV1":
        fail("sealed product symbol drifted")
    if implementation["analysis_result"] != "TrivialCanonicalOwnerAnalysisV1":
        fail("analysis result symbol drifted")

    historical = data["historical_ssa_seam"]
    if historical != {
        "path": "tools/checks/fixtures/canonical_ssa_seam_inventory_v1.json",
        "schema": "CanonicalSsaSeamInventoryV1",
        "rows": 92,
        "sha256": HISTORICAL_SHA256,
    }:
        fail("historical seam declaration drifted")
    historical_path = root / historical["path"]
    historical_bytes = historical_path.read_bytes()
    actual_sha = hashlib.sha256(historical_bytes).hexdigest()
    if actual_sha != HISTORICAL_SHA256:
        fail(f"historical 92-row seam hash drifted: {actual_sha}")
    historical_data = json.loads(historical_bytes)
    if historical_data.get("schema") != historical["schema"]:
        fail("historical seam schema drifted")
    historical_rows = historical_data.get("rows")
    if not isinstance(historical_rows, list) or len(historical_rows) != 92:
        fail("historical seam must remain exactly 92 rows")
    historical_ids = [row.get("id") for row in historical_rows]
    if len(historical_ids) != len(set(historical_ids)):
        fail("historical seam ids are no longer unique")

    inventory = data["ownership_inventory"]
    if inventory != {
        "path": "tools/checks/fixtures/canonical_ownership_production_profile_v1.json",
        "schema": "CanonicalOwnershipProductionProfileV1",
        "rows": 18,
    }:
        fail("ownership inventory declaration drifted")
    inventory_data = json.loads((root / inventory["path"]).read_text())
    if inventory_data.get("schema") != inventory["schema"]:
        fail("ownership inventory schema drifted")
    inventory_rows = inventory_data.get("rows")
    if not isinstance(inventory_rows, list) or len(inventory_rows) != 18:
        fail("ownership inventory must remain exactly 18 rows")
    inventory_by_id = {row.get("id"): row for row in inventory_rows}

    values = checked_rows(
        data["value_rows"], {"id", "source_row", "disposition"}, "value_rows"
    )
    if row_pairs(values, "source_row", "disposition") != EXPECTED_VALUES:
        fail("exact/derived trivial value matrix drifted")
    terminals = checked_rows(
        data["terminal_rows"], {"id", "disposition", "profile"}, "terminal_rows"
    )
    if row_pairs(terminals, "disposition", "profile") != EXPECTED_TERMINALS:
        fail("terminal disposition matrix drifted")
    rejections = checked_rows(
        data["rejection_rows"], {"id", "source_row", "reason"}, "rejection_rows"
    )
    if row_pairs(rejections, "source_row", "reason") != EXPECTED_REJECTIONS:
        fail("typed rejection matrix drifted")

    referenced_rows = {
        row["source_row"] for row in values
    } | {row["source_row"] for row in rejections}
    missing_inventory = sorted(referenced_rows - set(inventory_by_id))
    if missing_inventory:
        fail(f"profile references missing P0 rows: {missing_inventory}")
    for row in values:
        source_profile = inventory_by_id[row["source_row"]].get("profile")
        expected = (
            "trivial_exact"
            if row["disposition"] == "exact_trivial"
            else "derived_trivial_only"
        )
        if source_profile != expected:
            fail(
                f"{row['id']}: P0 source profile mismatch: "
                f"expected={expected} actual={source_profile}"
            )
    for row in rejections:
        source_profile = inventory_by_id[row["source_row"]].get("profile")
        if source_profile not in {
            "typed_preflight_reject",
            "not_in_first_family",
            "derived_trivial_only",
        }:
            fail(f"{row['id']}: rejection is not grounded in a closed P0 row")

    contract_payload = json.dumps(
        {
            "claims": data["claims"],
            "parameter_rows": parameter_rows,
            "value_rows": values,
            "terminal_rows": terminals,
            "rejection_rows": rejections,
        },
        sort_keys=True,
    )
    for token in EXPECTED_FORBIDDEN:
        if token in contract_payload:
            fail(f"guard contract imported forbidden authority token: {token}")

    physical_owner = root / data["physical_owner"]
    if not physical_owner.is_dir():
        fail("resolved_value_profile physical owner is missing")
    actual_manifest = {path.name for path in physical_owner.iterdir()}
    if actual_manifest != EXPECTED_MANIFEST:
        fail(
            "physical owner manifest drifted: "
            f"expected={sorted(EXPECTED_MANIFEST)} actual={sorted(actual_manifest)}"
        )
    production_files = [physical_owner / name for name in sorted(EXPECTED_PRODUCTION_RUST)]
    test_files = [physical_owner / name for name in sorted(EXPECTED_TEST_RUST)]
    for path in production_files:
        text = path.read_text(errors="ignore")
        for token in EXPECTED_FORBIDDEN:
            if token in text:
                fail(f"{path}: forbidden pre-Builder authority token: {token}")

    owner_production_text = "\n".join(path.read_text() for path in production_files)
    entry_definition = "pub(crate) fn analyze_trivial_canonical_owner_v1"
    if owner_production_text.count(entry_definition) != 1:
        fail("profile analyzer entry definition count must remain exactly one")
    entry_match = re.search(
        r"pub\(crate\) fn analyze_trivial_canonical_owner_v1\s*\((?P<inputs>.*?)\)\s*->",
        owner_production_text,
        re.S,
    )
    if entry_match is None:
        fail("profile analyzer entry signature is missing")
    entry_inputs = entry_match.group("inputs")
    for input_type in implementation["entry_inputs"]:
        if entry_inputs.count(input_type) != 1:
            fail(f"profile analyzer must co-seal exactly one {input_type}")
    for borrowed_type in (
        "VerifiedFunctionCompletionV1",
        "VerifiedResolvedFunctionIfControlV1",
    ):
        qualified = rf"(?:[A-Za-z_][A-Za-z0-9_]*::)*{borrowed_type}\b"
        if re.search(rf":\s*&\s*{qualified}", entry_inputs) is None:
            fail(f"profile analyzer must borrow co-sealed {borrowed_type}")
    sealed_definition = "pub(crate) struct VerifiedTrivialCanonicalOwnerV1"
    if owner_production_text.count(sealed_definition) != 1:
        fail("sealed product definition count must remain exactly one")
    result_definition = "pub(crate) enum TrivialCanonicalOwnerAnalysisV1"
    if owner_production_text.count(result_definition) != 1:
        fail("analysis result definition count must remain exactly one")
    test_text = "\n".join(path.read_text() for path in test_files)
    if test_text.count("#[test]") != 18:
        fail("focused profile fixture count must remain exactly 18")
    for fixture in (
        "exact_literals_binary_and_value_return_seal",
        "local_assignment_and_blockexpr_tail_preserve_exact_profile",
        "homogeneous_if_merge_seals_and_mixed_merge_rejects",
        "null_sentinel_flows_locally_and_compares_to_bool",
        "explicit_void_value_flows_and_terminal_stays_distinct",
        "if_condition_must_be_exact_bool",
        "explicit_empty_return_and_implicit_fallthrough_are_distinct",
        "parameter_outbox_and_missing_initializer_are_typed_stops",
        "string_value_remains_a_typed_stop",
        "null_terminal_and_mixed_merge_remain_typed_stops",
        "mixed_binary_and_short_circuit_are_typed_stops",
        "duplicate_coverage_and_foreign_if_control_cannot_seal",
        "current_a_plus_acceptance_is_not_narrowed_by_disconnected_profile",
        "exact_i64_parameters_are_sealed_before_body_subjects",
        "parameter_profile_consumption_uses_one_global_ordered_ledger",
        "parameter_rebind_and_if_merge_reuse_the_existing_profile_environment",
        "unsupported_parameter_types_and_untyped_parameters_do_not_admit",
        "exact_parameter_profile_remains_disconnected_from_production_preflight",
    ):
        if f"fn {fixture}(" not in test_text:
            fail(f"focused profile fixture missing: {fixture}")
    if "pub(crate) mod resolved_value_profile;" not in (root / "src/mir/mod.rs").read_text():
        fail("crate-private resolved_value_profile module declaration is missing")

    production = list(production_rust(root))
    external_profile_callers = []
    external_profile_references = []
    external_module_references = []
    call_token = "analyze_trivial_canonical_owner_v1("
    for path in production:
        if physical_owner in path.parents:
            continue
        text = production_text(path)
        if call_token in text:
            external_profile_callers.append(str(path.relative_to(root)))
        if any(symbol in text for symbol in EXPECTED_SYMBOLS):
            external_profile_references.append(str(path.relative_to(root)))
        if path != root / "src/mir/mod.rs" and "resolved_value_profile" in text:
            external_module_references.append(str(path.relative_to(root)))
    if external_profile_callers != ["src/mir/compiler/capability.rs"]:
        fail(f"profile analyzer caller set drifted: {external_profile_callers}")
    if set(external_profile_references) != {
        "src/mir/builder/resolved_lowering/trivial_ssa/lowerer.rs",
        "src/mir/compiler/capability.rs",
    }:
        fail(
            "sealed profile production consumer set drifted: "
            f"{external_profile_references}"
        )
    if set(external_module_references) != {
        "src/mir/builder/resolved_lowering/trivial_ssa/operation.rs",
        "src/mir/builder/resolved_lowering/trivial_ssa/lowerer.rs",
        "src/mir/compiler/capability.rs",
    }:
        fail(
            "resolved_value_profile production reference set drifted: "
            f"{external_module_references}"
        )

    binding_box = root / "src/mir/builder/ssa/binding"
    binding_callers = []
    for path in production:
        if binding_box in path.parents:
            continue
        text = production_text(path)
        if "BindingSsaBuilderV1" in text or "MirBindingSsaAdapterV1" in text:
            binding_callers.append(str(path.relative_to(root)))
    if binding_callers != [
        "src/mir/builder/resolved_lowering/trivial_ssa/identity.rs"
    ]:
        fail(f"Binding SSA production caller set drifted: {binding_callers}")

    ownership_dir = root / "src/mir/ownership_ssa"
    ownership_callers = 0
    ownership_installers = 0
    for path in production:
        text = production_text(path)
        if ownership_dir not in path.parents:
            ownership_callers += text.count("verify_ownership_ssa_v1(")
        ownership_installers += text.count(".ownership_ssa_v1 = Some(")
    if ownership_callers != 0:
        fail(f"Ownership SSA gained production callers: {ownership_callers}")
    if ownership_installers != 0:
        fail(f"Ownership SSA gained production witness installers: {ownership_installers}")

    resolved_lowering = root / "src/mir/builder/resolved_lowering"
    ownership_opcode_callers = 0
    for path in resolved_lowering.rglob("*.rs"):
        if path.name == "tests.rs" or path.name.endswith("_tests.rs"):
            continue
        text = production_text(path)
        ownership_opcode_callers += text.count("MirInstruction::CopyOwned")
        ownership_opcode_callers += text.count("MirInstruction::DestroyOwned")
    if ownership_opcode_callers != 0:
        fail(f"canonical ownership opcode callers appeared: {ownership_opcode_callers}")

    resolved_callers = 0
    for path in production:
        resolved_callers += production_text(path).count(".compile_resolved(")
    if resolved_callers != 0:
        fail(f"default/non-test resolved caller activated: {resolved_callers}")

    print("canonical_ssa_i0_profile_owner=src/mir/resolved_value_profile")
    print("canonical_ssa_i0_profile_manifest_entries=11")
    print("canonical_ssa_i0_profile_production_rust_files=8")
    print("canonical_ssa_i0_profile_test_rust_files=2")
    print("canonical_ssa_i0_profile_entry_definitions=1")
    print("canonical_ssa_i0_profile_sealed_product_definitions=1")
    print("canonical_ssa_i0_profile_focused_fixtures=18")
    print("canonical_ssa_i0_profile_parameter_rows=1")
    print("canonical_ssa_i0_profile_production_callers=1")
    print("canonical_ssa_i0_profile_binding_ssa_callers=1")
    print("canonical_ssa_i0_profile_ownership_ssa_callers=0")
    print("canonical_ssa_i0_profile_ownership_witness_installers=0")
    print("canonical_ssa_i0_profile_ownership_opcode_callers=0")
    print("canonical_ssa_i0_profile_accepted_grammar_delta=0")
    print("canonical_ssa_i0_profile_historical_seam_rows=92")
    print(f"canonical_ssa_i0_profile_historical_seam_sha256={actual_sha}")


if __name__ == "__main__":
    main()
