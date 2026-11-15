#!/usr/bin/env python3
"""Guard disconnected SITE0-L0 located legacy callable-result inputs."""

from __future__ import annotations

import re
import sys
from pathlib import Path

def fail(message: str) -> None:
    raise SystemExit(f"[callable-result-i0-site0-l0] {message}")

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
    policy = read(root, "src/mir/resolved_semantics/source_path_policy.rs")

    require_count(policy, "pub(crate) fn resolve<'source>", 2, "neutral child resolvers")
    require_count(
        policy,
        "ExprChildSyntaxV1::SyntheticName",
        2,
        "synthetic child classification plus fixture",
    )

    production_consumers = 0
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if (
            "/callable_result_representation/tests/" in relative
            or relative.endswith("/located_legacy.rs")
            or relative.endswith("/located_legacy_lowering.rs")
        ):
            continue
        production_consumers += path.read_text(encoding="utf-8").count(
            "VerifiedCallableResultLegacySourceViewV1::verify("
        )
    if production_consumers != 0:
        fail(f"production located consumers: expected=0 actual={production_consumers}")

    touched = [
        "src/mir/resolved_semantics/source_path_policy.rs",
        "src/mir/compiler/source_view.rs",
        "tools/checks/lib/callable_result_i0_site0_l0.py",
    ]
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        "[callable-result-i0-site0-l0] ok: role_policy_retained=1 "
        "role_resolvers=2 retired_consumers=0"
    )

if __name__ == "__main__":
    main()
