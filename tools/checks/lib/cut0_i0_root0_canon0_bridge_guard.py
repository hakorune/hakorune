#!/usr/bin/env python3
"""Reusable census guard for CUT0-I0 ROOT0 CANON-BRIDGE0."""

from __future__ import annotations

import pathlib
import re


ROOT = pathlib.Path(__file__).resolve().parents[3]
SHARED = ROOT / "src/mir/module_invocation_identity.rs"
BUILDER_ID = ROOT / "src/mir/builder/module_invocation_identity.rs"
ROUTE = ROOT / "src/mir/builder/module_invocation_route_matrix.rs"
SOURCE = ROOT / "src/mir/compiler/source_bound_package.rs"
SOURCE_P0 = ROOT / "src/mir/compiler/source_bound_package_p0.rs"
COMPLETION = ROOT / "src/mir/compiler/canonical_physical_completion.rs"
COMPLETION_P0 = ROOT / "src/mir/compiler/canonical_physical_completion_p0.rs"
COMPILER_MOD = ROOT / "src/mir/compiler/mod.rs"
MIR_MOD = ROOT / "src/mir/mod.rs"
BUILDER = ROOT / "src/mir/builder.rs"
BRAND0 = ROOT / "src/mir/builder/module_invocation_brand0.rs"
OWNER_CHAIN = ROOT / "src/mir/builder/module_invocation_owner_chain.rs"
COLLECTOR = ROOT / "src/mir/builder/module_draft_collector.rs"
COLLECTED_PRODUCT = ROOT / "src/mir/builder/module_draft_collector/collected_product.rs"
CALLABLE_BATCH = ROOT / "src/mir/builder/module_draft_collector/callable_batch.rs"
CALLABLE_TX = ROOT / "src/mir/builder/resolved_lowering/callable_module_transaction.rs"
SESSION = ROOT / "src/mir/builder/module_invocation_session.rs"
SHELL = ROOT / "src/mir/builder/module_lowering_shell.rs"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-root0-canon0-bridge-execution-task-2026-07-23.md"
)
OWNER_TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-root0-canon0-bridge-owner0-execution-task-2026-07-23.md"
)
MANIFEST = (
    SHARED,
    BUILDER_ID,
    ROUTE,
    SOURCE,
    SOURCE_P0,
    COMPLETION,
    COMPLETION_P0,
    COMPILER_MOD,
    MIR_MOD,
    BUILDER,
    BRAND0,
    OWNER_CHAIN,
    COLLECTOR,
    COLLECTED_PRODUCT,
    CALLABLE_BATCH,
    CALLABLE_TX,
    SESSION,
    SHELL,
    TASK,
    OWNER_TASK,
    pathlib.Path(__file__),
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def production_rust_files() -> list[pathlib.Path]:
    return [
        path
        for path in ROOT.glob("src/**/*.rs")
        if not path.name.endswith("_p0.rs")
        and not path.name.endswith("_tests.rs")
        and "tests" not in path.parts
    ]


def main() -> int:
    shared = SHARED.read_text()
    builder_id = BUILDER_ID.read_text()
    route = ROUTE.read_text()
    source = SOURCE.read_text()
    mir_mod = MIR_MOD.read_text()
    builder = BUILDER.read_text()
    task = TASK.read_text()
    completion = COMPLETION.read_text()
    completion_p0 = COMPLETION_P0.read_text()
    compiler_mod = COMPILER_MOD.read_text()

    for path in MANIFEST:
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"CANON-BRIDGE0 file must remain below 800 lines: {path}")

    require(task, "CANON-BRIDGE0", "CANON-BRIDGE0 lane card")
    require(task, "Shared guard policy", "shared bridge guard policy")
    owner_task = OWNER_TASK.read_text()
    require(owner_task, "OWNER0", "OWNER0 card")
    require(owner_task, "collector admission count = 0", "OWNER0 non-claim")
    require(mir_mod, "pub(crate) mod module_invocation_identity;", "shared identity module")
    for fragment, label in (
        ("enum ModuleInvocationFamilyV1", "one shared family definition"),
        ("struct ModuleInvocationBrandV1", "one shared brand definition"),
        ("struct ModuleInvocationIdV1", "one shared id definition"),
        ("struct ModuleInvocationTokenV1", "one shared token definition"),
        ("compiler_domain: NonZeroU64", "compiler domain field"),
        ("invocation_ordinal: NonZeroU64", "local ordinal field"),
        ("pub(crate) fn from_issued", "issuer terminal"),
    ):
        require(shared, fragment, label)

    if re.search(r"\b(struct|enum)\s+(CanonicalInvocationBrandV1|CanonicalInvocationTokenV1)", source):
        raise AssertionError("compiler-local identity structs remain")
    if re.search(r"\benum\s+InvocationRootFamilyV1", route):
        raise AssertionError("Builder-local family enum remains")
    require(builder_id, "pub(in crate::mir::builder) use crate::mir::module_invocation_identity", "Builder shared re-export")

    issued_calls = []
    for path in production_rust_files():
        if "ModuleInvocationTokenV1::from_issued(" in path.read_text():
            issued_calls.append(path.relative_to(ROOT))
    if issued_calls != [SOURCE.relative_to(ROOT)]:
        raise AssertionError(f"expected MirCompiler issuer caller only, got {issued_calls}")

    for fragment, label in (
        ("InvocationIdentityIssuerV1", "compiler-owned issuer"),
        ("ModuleInvocationBrandV1", "shared source brand usage"),
        ("ModuleInvocationTokenV1", "shared source token usage"),
    ):
        require(source, fragment, label)

    legacy_completion = (ROOT / "src/mir/builder/canonical_root_completion.rs").read_text()
    if "TestInvocationPreflightFactoryV1" in legacy_completion or "ModuleInvocationTokenV1::from_test" in legacy_completion:
        raise AssertionError("canonical completion still mints test identity")
    if "ModuleInvocationBrandV1 {" in builder_id or "ModuleInvocationTokenV1 {" in builder_id:
        raise AssertionError("Builder shim reconstructs shared identity")
    if "ordinal copy" in source.lower() or "from_source(brand" in source:
        raise AssertionError("post-hoc identity conversion/rebrand remains in compiler source")
    require(builder, "mod module_invocation_identity;", "Builder identity module registration")

    require(source, "pub(super) fn open_physical(", "package physical-open terminal")
    require(source, "pub(super) fn lower(", "same-owner lowering terminal")
    require(source, "CanonicalPhysicalInvocationV1", "physical invocation owner")
    require(source, "ModuleBuilderInvocationSessionV1::open_for_token", "shared Builder session open")
    require(source, "InvocationPhysicalStateV1::from_token", "shared shell/collector open")
    require(SOURCE_P0.read_text(), "canonical_source_binding_owner0_uses_one_physical_owner", "OWNER0 fixture")
    require(COMPILER_MOD.read_text(), "begin_canonical_invocation", "MirCompiler bridge terminal")
    if "CanonicalModuleLoweringSessionV1" in source:
        raise AssertionError("new package owner still depends on legacy canonical session")
    if "let package = SourceBoundCanonicalPackageV1 {" in source:
        raise AssertionError("physical lowering reconstructs a consumed package")

    require(BRAND0.read_text(), "pub(in crate::mir) fn collect_single(", "typed single collector terminal")
    require(BRAND0.read_text(), "pub(in crate::mir) fn collect_callable_batch(", "typed batch collector terminal")
    require(source, "CollectedCanonicalPhysicalInvocationV1", "COLLECT0 collected owner")
    require(source, "pub(in crate::mir) fn collect(", "COLLECT0 source terminal")
    source_p0 = SOURCE_P0.read_text()
    require(source_p0, "canonical_source_binding_collect0_retains_same_brand_and_receipt", "single COLLECT0 fixture")
    require(source_p0, "canonical_source_binding_collect0_projects_callable_catalog_atomically", "batch COLLECT0 fixture")
    require(CALLABLE_TX.read_text(), "into_canonical_entries", "source-driven callable projection")
    require(COLLECTED_PRODUCT.read_text(), "pub(in crate::mir) fn receipt_brand", "single receipt provenance")
    require(CALLABLE_BATCH.read_text(), "pub(in crate::mir) fn receipt_brand", "batch receipt provenance")
    if "collect_canonical_single(key" not in BRAND0.read_text():
        raise AssertionError("single physical terminal no longer derives canonical admission")
    if "FunctionDraftKeyV1::Main" in BRAND0.read_text() or "FunctionDraftKeyV1::SyntheticConditionFn" in BRAND0.read_text():
        raise AssertionError("canonical physical collector references synthetic root keys")

    require(compiler_mod, "mod canonical_physical_completion_p0;", "completion fixture registration")
    require(completion, "CollectedCanonicalPhysicalInvocationV1", "completion source owner")
    require(completion, "fn complete(", "one-shot completion terminal")
    require(completion, "CanonicalPhysicalCompleteInvocationV1", "route-specific completion product")
    require(completion, "CollectedCanonicalSinglePhysicalV1", "single physical receipt retention")
    require(completion, "CollectedCanonicalCallablePhysicalV1", "callable physical receipt retention")
    require(completion, "CanonicalCallableCapabilityWitnessV1", "capability witness retention")
    if "canonical_root_completion" in completion or "Option<" in completion or ".take().expect" in completion:
        raise AssertionError("new completion reuses legacy scaffold or drop-only plan path")
    for fixture in (
        "compiler_bridge_completion_retains_single_physical_receipt",
        "compiler_bridge_completion_retains_acyclic_capability_and_receipt",
        "compiler_bridge_completion_retains_recursive_capability_and_receipt",
    ):
        require(completion_p0, fixture, f"completion fixture: {fixture}")
    if "canonical_root_completion::" in source or "canonical_root_completion::" in completion_p0:
        raise AssertionError("new compiler bridge calls legacy canonical completion")

    factory_callers = []
    for path in production_rust_files():
        text = path.read_text()
        relative = path.relative_to(ROOT).as_posix()
        canonical_surface = relative.startswith("src/mir/compiler/") or \
            relative.startswith("src/mir/builder/canonical")
        if canonical_surface and "TestInvocationPreflightFactoryV1::new(" in text:
            factory_callers.append(path.relative_to(ROOT))
    if factory_callers:
        raise AssertionError(f"canonical production must not use test identity factory: {factory_callers}")

    print(
        "[cut0-i0-root0-canon0-bridge-guard] ok "
        "shared_identity=1 issuer_callers=1 owner0=1 collect0_single=1 "
        "collect0_batch=1 completion=1 token_conversion=0 canonical_test_factory=0"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
