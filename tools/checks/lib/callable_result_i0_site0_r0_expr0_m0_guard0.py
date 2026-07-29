#!/usr/bin/env python3
"""Guard the exact raw-expression recursion-depth owner after M0-GUARD0."""

from __future__ import annotations

import sys
from pathlib import Path


def fail(message: str) -> None:
    raise SystemExit(f"[callable-result-i0-site0-r0-expr0-m0-guard0] {message}")


def read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        fail(f"missing {relative}")
    return path.read_text(encoding="utf-8")


def require_count(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        fail(f"{label}: expected={expected} actual={actual}")


def main() -> None:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    recursive = read(root, "src/mir/builder/recursive_child_lowering.rs")
    build = read(root, "src/mir/builder/builder_build.rs")
    tests = read(root, "src/mir/builder/recursive_child_lowering_tests.rs")
    readme = read(root, "src/mir/builder/calls/README.md")

    require_count(
        recursive,
        "const MAX_RAW_EXPRESSION_RECURSION_DEPTH: usize = 200;",
        1,
        "depth limit owner",
    )
    require_count(
        recursive,
        "fn lower_raw_expression_with_recursion_guard_v1<Port>(",
        1,
        "raw guard owner",
    )
    require_count(recursive, "builder.recursion_depth += 1;", 1, "guard increment")
    require_count(recursive, "builder.recursion_depth -= 1;", 2, "guard restoration")
    require_count(
        recursive,
        "lower_raw_expression_with_recursion_guard_v1(builder, input)",
        0,
        "retired unported raw guard consumer",
    )
    require_count(
        recursive,
        "lower_raw_expression_with_recursion_guard_v1(builder, self, input)",
        2,
        "raw child-port guard consumers",
    )
    require_count(
        build,
        "drive_raw_legacy_expression_v1(self, ast)",
        1,
        "public raw-port delegation",
    )
    for forbidden in (
        "MAX_RECURSION_DEPTH",
        "recursion_depth +=",
        "recursion_depth -=",
        "build_expression_impl(ast)",
    ):
        if forbidden in build:
            fail(f"public expression facade retains guard authority: {forbidden}")

    for evidence in (
        "expression_failure_restores_recursion_depth_for_reuse",
        "raw_expression_depth_limit_rejects_without_poisoning_the_session",
    ):
        if evidence not in tests:
            fail(f"missing guard fixture: {evidence}")

    for phrase in (
        "GUARD0 restores one exact recursion-depth guard",
        "nested raw expression descent both reach that owner exactly once",
    ):
        if phrase not in readme:
            fail(f"missing README boundary: {phrase}")

    touched = [
        "src/mir/builder/builder_build.rs",
        "src/mir/builder/recursive_child_lowering.rs",
        "src/mir/builder/recursive_child_lowering_tests.rs",
        "src/mir/builder/calls/README.md",
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_guard0.py",
    ]
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        "[callable-result-i0-site0-r0-expr0-m0-guard0] ok: "
        "guard_owner=1 raw_consumer=1 public_guard_owner=0"
    )


if __name__ == "__main__":
    main()
