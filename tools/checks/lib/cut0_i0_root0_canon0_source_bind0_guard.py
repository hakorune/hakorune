#!/usr/bin/env python3
"""Evidence guard for CUT0-I0-ROOT0-CANON0 SOURCE-BIND0."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
PACKAGE = ROOT / "src/mir/compiler/source_bound_package.rs"
COMPILER = ROOT / "src/mir/compiler/mod.rs"
CAPABILITY = ROOT / "src/mir/compiler/capability.rs"
HEADER = ROOT / "src/mir/compiler/capability/resolved_owner_header.rs"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-root0-canon0-source-binding-execution-task-2026-07-22.md"
)
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
MANIFEST = (PACKAGE, COMPILER, CAPABILITY, HEADER, pathlib.Path(__file__))


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def production_rust_files() -> list[pathlib.Path]:
    files = []
    for path in ROOT.glob("src/**/*.rs"):
        if path.name.endswith("_tests.rs") or path.name.endswith("_p0.rs"):
            continue
        if "tests" in path.parts:
            continue
        files.append(path)
    return files


def main() -> int:
    package = PACKAGE.read_text()
    package_production = package.split("#[cfg(test)]", 1)[0]
    compiler = COMPILER.read_text()
    capability = CAPABILITY.read_text()
    header = HEADER.read_text()
    task = TASK.read_text()
    state = STATE.read_text()

    for path in MANIFEST:
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"SOURCE-BIND0 file must remain below 800 lines: {path}")

    require(state, "CANON-FIXTURE0-DESIGN-STOP-20260722", "successor blocker")
    require(task, "Status: **Closed — SOURCE-BIND0", "SOURCE-BIND0 closed card")
    require(task, "SOURCE-BIND0 implementation and evidence gate passed", "SOURCE-BIND0 closeout")
    require(task, "LOWER0", "later lowering row")
    require(task, "production lowering/drain/finalizer/external commit = 0", "stop line")
    require(compiler, "mod source_bound_package", "compiler package registration")
    require(compiler, "invocation_identity: InvocationIdentityIssuerV1", "compiler issuer owner")
    require(compiler, "pub(in crate::mir) fn bind_canonical_source", "sole compiler constructor")
    require(capability, "pub(crate) fn seal_resolved_owner_header_v1", "header seal helper")
    require(header, "pub(super) fn seal_input", "input-bound header seal")

    for fragment, label in (
        ("enum ExactCanonicalPreflightPlanV1", "exact plan enum"),
        ("APlus(CanonicalCurrentAPlusPlanV1", "A+ plan variant"),
        ("BindingSsaTrivial(CanonicalTrivialBindingSsaPlanV1", "trivial plan variant"),
        ("BindingSsaAcyclic(VerifiedAcyclicCallableModulePlanV1", "acyclic plan variant"),
        ("BindingSsaRecursive(VerifiedRecursiveCallableModulePlanV1", "recursive plan variant"),
        ("struct SourceBoundCanonicalPackageV1", "source-bound package"),
        ("struct RejectedCanonicalSourceBindingV1", "rejected owner"),
        ("struct InvocationIdentityIssuerV1", "sole issuer"),
        ("process-scoped compiler domain", "domain contract"),
        ("pub(super) fn bind(", "private package terminal"),
        ("mod tests", "focused fixture registration"),
    ):
        require(package, fragment, label)

    if "Option<ExactCanonicalPreflightPlanV1" in package:
        raise AssertionError("SOURCE-BIND0 package may not use Option<Plan>")
    for forbidden, label in (
        ("pub(crate) fn split", "public package split"),
        ("Arc<", "authority clone"),
        ("prepare(token", "caller token package"),
        ("current_module", "ambient source reacquisition"),
    ):
        if forbidden in package_production:
            raise AssertionError(f"forbidden {label}: {forbidden}")

    producer_calls = []
    for path in production_rust_files():
        if path == PACKAGE:
            continue
        text = path.read_text()
        if "SourceBoundCanonicalPackageV1::bind(" in text:
            producer_calls.append(path.relative_to(ROOT))
    if producer_calls != [COMPILER.relative_to(ROOT)]:
        raise AssertionError(f"expected one compiler package producer, got {producer_calls}")

    factory_calls = []
    for path in production_rust_files():
        if path in {
            ROOT / "src/mir/builder/module_invocation_identity.rs",
            ROOT / "src/mir/builder/raw_root_completion.rs",
        }:
            continue
        if "TestInvocationPreflightFactoryV1" in path.read_text():
            factory_calls.append(path.relative_to(ROOT))
    if factory_calls:
        raise AssertionError("test-only identity factory has production callers: " + ", ".join(map(str, factory_calls)))

    print(
        "[cut0-i0-root0-canon0-source-bind0-guard] ok "
        "package=1 issuer=1 domain=1 rejected_owner=1 production_consumers=0"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
