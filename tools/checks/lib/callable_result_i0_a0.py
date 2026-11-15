#!/usr/bin/env python3
"""Guard source-proof-gated callable-result activation rows."""

from __future__ import annotations

import re
import sys
from pathlib import Path

def fail(message: str) -> None:
    raise SystemExit(f"[callable-result-i0-a0] {message}")

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
    emission = read(
        root, "src/mir/builder/control_flow/plan/lowerer/emission_port.rs"
    )
    located_loop = read(root, "src/mir/builder/control_flow/plan/located_loop.rs")
    solver = read(root, "src/mir/callable_result_representation/solver.rs")

    require_count(
        located_loop,
        "struct LocatedCoreLoopExecutionSessionV1",
        1,
        "claimed CorePlan execution session owner",
    )
    require_count(
        located_loop,
        "enum LocatedCoreLoopExecutionStateV1",
        1,
        "claimed CorePlan execution state owner",
    )
    require_count(
        located_loop,
        "into_claimed_execution",
        0,
        "claimed bundle production bypass",
    )
    if "LocatedLegacyLoweringSessionV1" in located_loop:
        fail("CorePlan session must not reuse the legacy AST lowering session")

    if "if !all_keys.contains_key(caller)" not in solver:
        fail("instance caller membership is not checked against the complete catalog")
    if "if !static_keys.contains_key(row.target())" not in solver:
        fail("target membership is not restricted to static result keys")
    if re.search(r"for \([^)]*\) in all_declarations \{[\s\S]{0,300}prove_function", solver):
        fail("instance declarations entered result inference")

    production_consumers = 0
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if "/tests/" in relative or path.name == "tests.rs" or path.name.endswith("_tests.rs"):
            continue
        production_consumers += path.read_text(encoding="utf-8").count(
            "VerifiedCallableResultActivationPlanV1::seal("
        )
    if production_consumers != 0:
        fail(f"production activation consumers: expected=0 actual={production_consumers}")

    production_session_starts = 0
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if "/tests/" in relative or path.name == "tests.rs" or path.name.endswith("_tests.rs"):
            continue
        count = path.read_text(encoding="utf-8").count(".start_execution(")
        production_session_starts += count
    if production_session_starts != 0:
        fail(
            "production claimed CorePlan execution-session callers: "
            f"expected=0 actual={production_session_starts}"
        )

    touched = [
        "src/mir/builder/callable_declaration_catalog/catalog.rs",
        "src/mir/builder/control_flow/plan/lowerer/emission_port.rs",
        "src/mir/builder/control_flow/plan/located_loop.rs",
        "src/mir/builder/control_flow/plan/located_loop_tests.rs",
        "src/mir/callable_result_representation/solver.rs",
        "src/mir/callable_result_representation/tests/support.rs",
        "tools/checks/lib/callable_result_i0_a0.py",
    ]
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        "[callable-result-i0-a0] ok: solver_membership_retained=1 "
        "retired_activation_consumers=0 "
        "core_loop_session_consumers=0"
    )

if __name__ == "__main__":
    main()
