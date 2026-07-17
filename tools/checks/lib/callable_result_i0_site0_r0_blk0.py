#!/usr/bin/env python3
"""Guard SITE0-R0-BLK0's behavior-neutral legacy block driver."""

from __future__ import annotations

import sys
from pathlib import Path


def fail(message: str) -> None:
    raise SystemExit(f"[callable-result-i0-site0-r0-blk0] {message}")


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
    driver = read(root, "src/mir/builder/stmts/block_driver.rs")
    raw = read(root, "src/mir/builder/stmts/block_stmt.rs")
    tests = read(root, "src/mir/builder/stmts/block_driver_tests.rs")
    readme = read(root, "src/mir/builder/README.md")

    require_count(driver, "trait LegacyBlockDescentPortV1", 1, "block port owner")
    require_count(driver, "fn drive_legacy_block_v1", 1, "block driver owner")
    require_count(raw, "struct OwnedLegacyBlockPortV1", 1, "raw port owner")
    require_count(raw, "impl LegacyBlockDescentPortV1", 1, "raw port implementation")
    require_count(raw, "drive_legacy_block_v1(builder, &mut port)", 1, "selected driver caller")

    require_count(
        driver,
        "NormalizedShadowSuffixRouterBox::try_lower_loop_suffix(",
        1,
        "suffix-router owner",
    )
    require_count(raw, "NormalizedShadowSuffixRouterBox::try_lower_loop_suffix", 0, "raw suffix policy")
    require_count(driver, "builder.hint_scope_enter(scope_id)", 1, "scope enter")
    require_count(driver, "LexicalScopeGuard::new(builder)", 1, "lexical scope owner")
    require_count(driver, "is_current_block_terminated(builder)?", 1, "fallible termination")
    require_count(driver, "builder.is_current_block_terminated()", 1, "scope-leave termination")
    require_count(driver, "emit_void(builder)?", 1, "empty-block Void")
    require_count(driver, "builder.hint_scope_leave(scope_id)", 1, "scope leave")
    require_count(driver, "joinir_dev_enabled()", 1, "existing suffix selector")

    combined = driver + raw
    for forbidden in (
        "VerifiedCallableResult",
        "CallerLedger",
        "LegacyBodyInputV1",
        "ActivationDisposition",
        "value_origin_newbox",
        "retry",
        "fallback",
    ):
        if forbidden in combined:
            fail(f"selected raw block boundary contains forbidden authority: {forbidden}")

    production_driver_consumers = 0
    port_implementations = 0
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if relative.endswith("/block_driver.rs") or relative.endswith("/block_driver_tests.rs"):
            continue
        text = path.read_text(encoding="utf-8")
        production_driver_consumers += text.count("drive_legacy_block_v1(")
        port_implementations += text.count("impl LegacyBlockDescentPortV1")
    if production_driver_consumers != 1:
        fail(
            "production driver consumers: "
            f"expected=1 actual={production_driver_consumers}"
        )
    if port_implementations != 1:
        fail(f"selected port implementations: expected=1 actual={port_implementations}")

    for evidence in (
        "empty_block_emits_one_void_and_restores_lexical_scope",
        "statements_lower_once_in_source_order_and_return_the_last_value",
        "termination_stops_before_an_invalid_trailing_statement",
        "successful_local_scope_restores_variable_and_binding_views",
        "failure_after_local_restores_scope_state_without_retry",
    ):
        if evidence not in tests:
            fail(f"missing block-driver parity fixture: {evidence}")

    for phrase in (
        "legacy block descent boundary",
        "LegacyBlockDescentPortV1",
        "must not decide suffix policy",
    ):
        if phrase not in readme:
            fail(f"missing README boundary: {phrase}")

    touched = [
        "src/mir/builder/README.md",
        "src/mir/builder/stmts/block_driver.rs",
        "src/mir/builder/stmts/block_driver_tests.rs",
        "src/mir/builder/stmts/block_stmt.rs",
        "src/mir/builder/stmts/mod.rs",
        "tools/checks/lib/callable_result_i0_site0_r0_blk0.py",
    ]
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        "[callable-result-i0-site0-r0-blk0] ok: "
        "driver=1 raw_port=1 production_callers=1 result_publishers=0"
    )


if __name__ == "__main__":
    main()
