#!/usr/bin/env python3
"""Evidence guard for CUT0-I0-ROOT0-CANON0 LOWER0."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
PACKAGE = ROOT / "src/mir/compiler/source_bound_package.rs"
COMPILER = ROOT / "src/mir/compiler/mod.rs"
LOWER = ROOT / "src/mir/builder/resolved_lowering/mod.rs"
CALLABLE = ROOT / "src/mir/builder/resolved_lowering/callable_module_transaction.rs"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-root0-canon0-lower0-execution-task-2026-07-22.md"
)
SOURCE_TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-root0-canon0-source-binding-execution-task-2026-07-22.md"
)
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
MANIFEST = (PACKAGE, COMPILER, LOWER, CALLABLE, TASK, SOURCE_TASK, pathlib.Path(__file__))


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
    package = PACKAGE.read_text()
    package_production = package.split("#[cfg(test)]", 1)[0]
    compiler = COMPILER.read_text()
    lower = LOWER.read_text()
    callable_lower = CALLABLE.read_text()
    task = TASK.read_text()
    source_task = SOURCE_TASK.read_text()
    state = STATE.read_text()

    for path in MANIFEST:
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"LOWER0 file must remain below 800 lines: {path}")

    require(state, "RECEIPT0-20260722", "successor blocker")
    require(source_task, "Status: **Closed — SOURCE-BIND0", "source-bind predecessor")
    require(task, "private consume_lowering(package)", "LOWER0 terminal contract")
    require(compiler, "pub(in crate::mir) fn lower_canonical_source", "compiler lower terminal")
    require(package, "pub(super) fn consume", "package consumer")
    for fragment, label in (
        ("builder.lower_resolved_function_draft(plan)", "A+ draft consumer"),
        ("builder.lower_resolved_trivial_function_draft(plan)", "trivial draft consumer"),
        ("builder.lower_acyclic_callable_drafts(plan)", "acyclic draft consumer"),
        ("builder.lower_recursive_callable_drafts(plan)", "recursive draft consumer"),
        ("enum LoweredCanonicalPlanV1", "unpublished lowering product"),
        ("RejectedCanonicalLoweringV1", "typed lowering rejection"),
    ):
        require(package, fragment, label)
    for fragment, label in (
        ("lower_resolved_function_draft", "A+ draft seam"),
        ("with_resolved_function_draft_session", "A+ draft session"),
        ("lower_acyclic_callable_drafts", "acyclic draft seam"),
        ("lower_recursive_callable_drafts", "recursive draft seam"),
        ("VerifiedUnpublishedCallableDraftSetV1", "unpublished callable set"),
    ):
        require(lower + callable_lower, fragment, label)

    for forbidden, label in (
        ("finalize_module", "module finalization in package consumer"),
        ("try_add_function", "single publication in package consumer"),
        ("publish_callable", "callable publication in package consumer"),
        ("prepare_module", "module preparation in package consumer"),
        ("insert_rc_instructions", "postprocess in package consumer"),
        ("current_module", "ambient module observation in package consumer"),
    ):
        if forbidden in package_production:
            raise AssertionError(f"forbidden {label}: {forbidden}")

    consumers = []
    for path in production_rust_files():
        if path == PACKAGE:
            continue
        if "package.consume(" in path.read_text():
            consumers.append(path.relative_to(ROOT))
    if consumers != [COMPILER.relative_to(ROOT)]:
        raise AssertionError(f"expected one package consumer, got {consumers}")

    print(
        "[cut0-i0-root0-canon0-lower0-guard] ok "
        "package_consumer=1 draft_routes=4 publication=0 production_consumers=0"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
