#!/usr/bin/env python3
"""Guard the atomic SSA-I1-T trivial whole-owner production cutover."""

from __future__ import annotations

from pathlib import Path
import sys


EXPECTED_BOX = {
    "README.md",
    "callable_abi.rs",
    "identity.rs",
    "lowerer.rs",
    "mod.rs",
    "operation.rs",
    "parameter_entry.rs",
}


def fail(message: str) -> None:
    raise SystemExit(f"SSA-I1-T production cutover: {message}")


def require(text: str, anchor: str, owner: str) -> None:
    if anchor not in text:
        fail(f"{owner}: missing anchor {anchor!r}")


def source_files(root: Path):
    for path in (root / "src").rglob("*.rs"):
        if path.name == "tests.rs" or path.name.endswith("_tests.rs"):
            continue
        yield path


def callers(root: Path, token: str, excluded: set[Path]) -> list[str]:
    found = []
    for path in source_files(root):
        if path in excluded:
            continue
        if token in path.read_text(errors="ignore"):
            found.append(str(path.relative_to(root)))
    return sorted(found)


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: resolved_binding_ssa_i1_t.py ROOT")
    root = Path(sys.argv[1]).resolve()
    box = root / "src/mir/builder/resolved_lowering/trivial_ssa"
    actual = {path.name for path in box.iterdir() if path.is_file()}
    if actual != EXPECTED_BOX:
        fail(f"box manifest drifted: expected={sorted(EXPECTED_BOX)} actual={sorted(actual)}")

    paths = {
        "capability": root / "src/mir/compiler/capability.rs",
        "compiler": root / "src/mir/compiler/mod.rs",
        "builder": root / "src/mir/builder/resolved_lowering/mod.rs",
        "callable_abi": box / "callable_abi.rs",
        "identity": box / "identity.rs",
        "lowerer": box / "lowerer.rs",
        "operation": box / "operation.rs",
        "parameter": box / "parameter_entry.rs",
        "readme": box / "README.md",
    }
    text = {name: path.read_text() for name, path in paths.items()}

    for anchor in (
        "enum CanonicalFirstFamilyPlanV1",
        "TrivialBindingSsa(CanonicalTrivialBindingSsaPlanV1",
        "CurrentCanonicalAPlus(CanonicalCurrentAPlusPlanV1",
        "analyze_trivial_canonical_owner_v1",
    ):
        require(text["capability"], anchor, "pre-Builder route")

    for anchor in (
        "match plan {",
        "build_resolved_trivial_function_module(plan)",
        "CanonicalFinishScheduleV1::TrivialBindingSsa",
        "LegacyRcInsertionScheduleV1::Skip",
        "CanonicalFinishScheduleV1::CurrentCanonicalAPlus",
        "LegacyRcInsertionScheduleV1::Run",
        '"canonical_post_transform_verify"',
    ):
        require(text["compiler"], anchor, "compiler route/finalization")

    for anchor in (
        "fn build_resolved_trivial_function_module(",
        "CanonicalTrivialSsaLowererV1::new(",
        "finalize_preterminated_function_completion",
        "install_trivial_callable_abi_v1(builder, profile.parameter_entries())",
        "refresh_trivial_callable_boundary_contracts_v1(&mut draft)",
        ".verify_function(&draft)",
    ):
        require(text["builder"], anchor, "function draft publication")

    for anchor in (
        "row.abi().mir_param_decl(row.source_name())",
        "set_current_function_declared_signature(declared_parameters, None)",
        "refresh_function_parameter_entry_contracts",
        "refresh_function_return_exit_contract",
    ):
        require(text["callable_abi"], anchor, "trivial callable ABI facade")
    trivial_builder = text["builder"].split(
        "fn build_resolved_trivial_function_module(", 1
    )[1]
    if "return_type_name" in trivial_builder:
        fail("resolved trivial route still reads the raw return annotation")

    for anchor in (
        "BindingSsaBuilderV1<PhiToken>",
        "MirBindingSsaAdapterV1::new",
        ".define(binding, block, value)",
        ".read(&mut adapter, binding, block)",
        ".seal(&mut adapter, block, witness)",
        "self.ssa.finish()",
    ):
        require(text["identity"], anchor, "sole reaching-value authority")

    for anchor in (
        "struct CanonicalTrivialSsaLowererV1",
        "FunctionIfControlUseLedgerV1",
        "TrivialProfileConsumptionV1",
        "CanonicalCfgSessionV1::new()",
        "PhiTxn::begin(",
        "self.profile.finish()?",
        ".finish(function)",
        ".commit(self.builder)",
        "self.identity.finish()?",
    ):
        require(text["lowerer"], anchor, "trivial whole-owner lowerer")

    for anchor in (
        "profile.claim_parameter_entry(formal_index)",
        "ValueId::new(formal_index)",
        "row.abi().mir_type()",
        "identity.publish_declaration(",
        "MirValueKind::Parameter(formal_index)",
    ):
        require(text["parameter"], anchor, "exact parameter adoption")
    for forbidden in ("next_value_id", "variable_map", "binding_ctx"):
        if forbidden in text["parameter"]:
            fail(f"parameter adoption regained forbidden authority token: {forbidden}")
    if '"i64"' in text["capability"]:
        fail("pre-Builder route reclassified exact i64 outside the sealed profile")

    combined_box = "\n".join(path.read_text() for path in box.iterdir() if path.is_file())
    for forbidden in (
        "variable_map",
        "ResolvedIdentityStateV1",
        "ResolvedRegionFlow",
        "resolved_region_flow",
        "ReleaseStrong",
        "insert_rc_instructions",
        "may_rebind",
        "ResolvedActiveEffectStackV1",
        "IfCfgSessionV1",
        "build_binary_op_from_values",
        "materialize_all_phi_inputs",
        "CopyOwned",
        "DestroyOwned",
    ):
        if forbidden in combined_box:
            fail(f"trivial production box regained forbidden authority token: {forbidden}")

    binding_box = root / "src/mir/builder/ssa/binding"
    cfg_box = root / "src/mir/builder/resolved_lowering/canonical_cfg"
    binding_excluded = set(binding_box.rglob("*.rs"))
    cfg_excluded = set(cfg_box.rglob("*.rs"))
    expected_identity = ["src/mir/builder/resolved_lowering/trivial_ssa/identity.rs"]
    if callers(root, "BindingSsaBuilderV1", binding_excluded) != expected_identity:
        fail("BindingSsaBuilderV1 must have exactly one production caller")
    if callers(root, "MirBindingSsaAdapterV1", binding_excluded) != expected_identity:
        fail("MirBindingSsaAdapterV1 must have exactly one production caller")
    expected_lowerer = ["src/mir/builder/resolved_lowering/trivial_ssa/lowerer.rs"]
    if callers(root, "CanonicalCfgSessionV1", cfg_excluded) != expected_lowerer:
        fail("CanonicalCfgSessionV1 must have exactly one production caller")

    profile_box = root / "src/mir/resolved_value_profile"
    profile_callers = callers(
        root,
        "analyze_trivial_canonical_owner_v1",
        set(profile_box.rglob("*.rs")),
    )
    expected_profile = ["src/mir/compiler/capability.rs"]
    if profile_callers != expected_profile:
        fail(f"profile route callers drifted: {profile_callers}")

    for path in (*paths.values(), *box.iterdir(), Path(__file__)):
        if path.is_file() and len(path.read_text().splitlines()) >= 800:
            fail(f"source/check reached the 800-line stop boundary: {path}")

    print("canonical_ssa_i1_t_route_selection=pre-builder-once")
    print("canonical_ssa_i1_t_binding_ssa_callers=1")
    print("canonical_ssa_i1_t_cfg_callers=1")
    print("canonical_ssa_i1_t_legacy_rc_on_selected_route=0")
    print("canonical_ssa_i1_t_ownership_opcode_callers=0")
    print("canonical_ssa_i1_t_accepted_grammar_delta=exact-static-i64-parameters")


if __name__ == "__main__":
    main()
