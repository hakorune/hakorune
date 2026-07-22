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
MIR_MOD = ROOT / "src/mir/mod.rs"
BUILDER = ROOT / "src/mir/builder.rs"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-root0-canon0-bridge-execution-task-2026-07-23.md"
)
MANIFEST = (SHARED, BUILDER_ID, ROUTE, SOURCE, MIR_MOD, BUILDER, TASK, pathlib.Path(__file__))


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

    for path in MANIFEST:
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"CANON-BRIDGE0 file must remain below 800 lines: {path}")

    require(task, "CANON-BRIDGE0", "CANON-BRIDGE0 lane card")
    require(task, "Shared guard policy", "shared bridge guard policy")
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

    completion = (ROOT / "src/mir/builder/canonical_root_completion.rs").read_text()
    if "TestInvocationPreflightFactoryV1" in completion or "ModuleInvocationTokenV1::from_test" in completion:
        raise AssertionError("canonical completion still mints test identity")
    if "ModuleInvocationBrandV1 {" in builder_id or "ModuleInvocationTokenV1 {" in builder_id:
        raise AssertionError("Builder shim reconstructs shared identity")
    if "ordinal copy" in source.lower() or "from_source(brand" in source:
        raise AssertionError("post-hoc identity conversion/rebrand remains in compiler source")
    require(builder, "mod module_invocation_identity;", "Builder identity module registration")

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
        "shared_identity=1 issuer_callers=1 token_conversion=0 canonical_test_factory=0"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
