#!/usr/bin/env python3
"""Evidence guard for CUT0-I0-ROOT0-CANON0 RECURSIVE0."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
SHELL = ROOT / "src/mir/builder/module_lowering_shell.rs"
COMPLETION = ROOT / "src/mir/builder/canonical_root_completion.rs"
FIXTURE = ROOT / "src/mir/builder/canonical_root_completion_recursive0_p0.rs"
BUILDER = ROOT / "src/mir/builder.rs"
LEGACY = ROOT / "src/mir/builder/resolved_lowering/callable_module_transaction.rs"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-root0-canon0-recursive0-execution-task-2026-07-22.md"
)
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
MANIFEST = (SHELL, COMPLETION, FIXTURE, BUILDER, LEGACY, TASK, pathlib.Path(__file__))


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def production_rust_files() -> list[pathlib.Path]:
    return [
        path
        for path in ROOT.glob("src/**/*.rs")
        if not path.name.endswith("_tests.rs")
        and not path.name.endswith("_p0.rs")
        and "tests" not in path.parts
    ]


def main() -> int:
    shell = SHELL.read_text()
    completion = COMPLETION.read_text()
    fixture = FIXTURE.read_text()
    builder = BUILDER.read_text()
    legacy = LEGACY.read_text()
    task = TASK.read_text()
    state = STATE.read_text()

    for path in MANIFEST:
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"RECURSIVE0 file must remain below 800 lines: {path}")

    require(state, "CANON-FIXTURE0-DESIGN-STOP-20260722", "successor blocker")
    require(task, "Status: **Closed — RECURSIVE0 branded capability provenance complete", "closed recursive task")
    for fragment, label in (
        ("brand: ModuleInvocationBrandV1", "receipt brand field"),
        ("family: ModuleInvocationFamilyV1", "receipt family field"),
        ("install_callable_batch_shell_fact(", "private shell install terminal"),
        ("install_callable_batch_capability(", "branded shell wrapper"),
        ("CanonicalRecursiveCallableModuleCapabilityV1::install_for_module", "marker install"),
        ("CapabilityBrandMismatch", "brand co-seal rejection"),
        ("CapabilityWitnessFamilyMismatch", "family co-seal rejection"),
    ):
        require(shell + completion, fragment, label)
    require(completion, ".install_callable_batch_capability(family)", "source-driven production caller")
    require(completion, "CallableBatchCapabilityDispositionV1::Recursive(receipt)", "recursive completion witness")
    require(completion, "CallableBatchCapabilityDispositionV1::Acyclic(absence)", "acyclic completion witness")
    require(builder, "mod canonical_root_completion_recursive0_p0", "focused fixture registration")
    for fragment, label in (
        ("recursive_install_returns_exact_brand_and_family_once", "recursive fixture"),
        ("acyclic_install_returns_branded_absence_witness", "acyclic fixture"),
        ("acyclic_route_rejects_a_preexisting_recursive_marker", "unexpected marker fixture"),
    ):
        require(fixture, fragment, label)

    production_calls = []
    for path in production_rust_files():
        if ".install_callable_batch_capability(" in path.read_text():
            production_calls.append(path.relative_to(ROOT))
    if production_calls != [COMPLETION.relative_to(ROOT)]:
        raise AssertionError(f"expected one branded production caller, got {production_calls}")

    require(legacy, "publish_recursive_callable_drafts", "legacy recursive path census")
    if "install_callable_batch_shell_fact(family)" in shell:
        raise AssertionError("old unbranded shell install signature remains")
    if "metadata().canonical_recursive_callable_module_capability" in completion:
        raise AssertionError("completion re-observes capability metadata")

    direct_install = []
    for path in production_rust_files():
        if "::install_for_module(" in path.read_text():
            direct_install.append(path.relative_to(ROOT))
    allowed_install = {
        pathlib.Path("src/mir/canonical_recursive_callable_module_capability.rs"),
        pathlib.Path("src/mir/canonical_recursive_callable_module_backend_capability.rs"),
        pathlib.Path("src/mir/builder/module_lowering_shell.rs"),
        pathlib.Path("src/mir/builder/resolved_lowering/callable_module_transaction.rs"),
    }
    if set(direct_install) != allowed_install:
        raise AssertionError(
            f"recursive marker install escaped shell/legacy allowlist: {direct_install}"
        )

    print(
        "[cut0-i0-root0-canon0-recursive0-guard] ok "
        "branded_terminal=1 production_caller=1 legacy_path=allowlisted witness_checks=2"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
