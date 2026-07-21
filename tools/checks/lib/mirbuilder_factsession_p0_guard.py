#!/usr/bin/env python3
"""Guard FACTSESSION0-P0's disconnected-only boundary."""

from __future__ import annotations

import sys
from pathlib import Path


TAG = "mirbuilder-factsession-p0-guard"
LIMITED_SYMBOLS = {
    "FactSessionP0HarnessV1": {
        "src/mir/builder/fact_session.rs",
        "src/mir/builder/fact_session_p0_tests.rs",
    },
    "FactSessionIssuerV1": {
        "src/mir/builder/fact_session.rs",
        "src/mir/builder/fact_session_p0_tests.rs",
    },
    "FunctionSessionP0TerminalV1": {
        "src/mir/builder/calls/function_session.rs",
        "src/mir/builder/calls/mod.rs",
        "src/mir/builder/fact_session_p0_tests.rs",
    },
    "observe_function_terminal_before_restore_for_p0_test": {
        "src/mir/builder/calls/function_session.rs",
        "src/mir/builder/fact_session_p0_tests.rs",
    },
}


def fail(message: str) -> None:
    raise SystemExit(f"[{TAG}] ERROR: {message}")


def source(root: Path, relative: str) -> str:
    try:
        return (root / relative).read_text(encoding="utf-8")
    except OSError as error:
        fail(f"cannot read {relative}: {error}")


def require_exactly_once(text: str, needle: str, relative: str) -> None:
    count = text.count(needle)
    if count != 1:
        fail(f"expected one {needle!r} in {relative}, found {count}")


def symbol_paths(root: Path, symbol: str) -> set[str]:
    return {
        str(path.relative_to(root))
        for path in (root / "src").rglob("*.rs")
        if symbol in path.read_text(encoding="utf-8")
    }


def main() -> None:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    builder = source(root, "src/mir/builder.rs")
    facts = source(root, "src/mir/builder/fact_session.rs")
    calls = source(root, "src/mir/builder/calls/function_session.rs")
    call_mod = source(root, "src/mir/builder/calls/mod.rs")
    tests = source(root, "src/mir/builder/fact_session_p0_tests.rs")

    require_exactly_once(
        builder,
        "#[cfg(test)]\nmod fact_session_p0_tests;",
        "src/mir/builder.rs",
    )
    require_exactly_once(
        facts,
        "#[cfg(test)]\npub(super) mod p0_test_support {",
        "src/mir/builder/fact_session.rs",
    )
    require_exactly_once(
        calls,
        "#[cfg(test)]\npub(in crate::mir::builder) enum FunctionSessionP0TerminalV1 {",
        "src/mir/builder/calls/function_session.rs",
    )
    require_exactly_once(
        calls,
        "#[cfg(test)]\n    pub(in crate::mir::builder) fn observe_function_terminal_before_restore_for_p0_test(",
        "src/mir/builder/calls/function_session.rs",
    )
    require_exactly_once(
        call_mod,
        "#[cfg(test)]\npub(in crate::mir::builder) use function_session::FunctionSessionP0TerminalV1;",
        "src/mir/builder/calls/mod.rs",
    )

    for symbol, expected_paths in LIMITED_SYMBOLS.items():
        actual_paths = symbol_paths(root, symbol)
        if actual_paths != expected_paths:
            fail(
                f"disconnected symbol paths drift symbol={symbol} "
                f"expected={sorted(expected_paths)} actual={sorted(actual_paths)}"
            )

    terminal_calls = tests.count(".observe_function_terminal_before_restore_for_p0_test(")
    if terminal_calls != 4:
        fail(f"expected four P0 terminal observations, found {terminal_calls}")
    for terminal in ("Success", "Primary", "Cleanup", "Panicked"):
        if f"FunctionSessionP0TerminalV1::{terminal}" not in tests:
            fail(f"P0 test matrix is missing terminal={terminal}")

    guarded = [
        root / "tools/checks/lib/mirbuilder_factsession_p0_guard.py",
        root / "src/mir/builder/fact_session.rs",
        root / "src/mir/builder/fact_session_p0_tests.rs",
        root / "src/mir/builder/calls/function_session.rs",
    ]
    oversized = [
        str(path.relative_to(root))
        for path in guarded
        if len(path.read_text(encoding="utf-8").splitlines()) >= 800
    ]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        f"[{TAG}] ok production_p0_consumers=0 terminal_observations={terminal_calls} "
        "terminal_matrix=success,primary,cleanup,panic"
    )


if __name__ == "__main__":
    main()
