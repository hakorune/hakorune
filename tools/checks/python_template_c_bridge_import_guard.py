#!/usr/bin/env python3
"""Guard retired Python-template C replacement-front bridge imports.

Normal allocator tools must not import the diagnostic bridge payload modules
directly. The only accepted imports are the payload modules wiring themselves,
the focused diagnostic smoke runner, and the guarded build helper that requires
`--allow-python-template-c-bridge-baseline`.
"""

from __future__ import annotations

import ast
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SCAN_ROOTS = (
    ROOT / "tools" / "allocator",
    ROOT / "tools" / "hako_check",
)

DIAGNOSTIC_MODULES = {
    "replacement_front_templates",
    "replacement_front_shim_templates",
    "replacement_front_bins_templates",
    "replacement_front_smoke_templates",
    "replacement_front_shim_report_source",
    "replacement_front_bins_report_source",
}

ALLOWED_IMPORTS: dict[str, set[str]] = {
    "tools/allocator/replacement_front_templates.py": {
        "replacement_front_bins_templates",
        "replacement_front_shim_templates",
        "replacement_front_smoke_templates",
    },
    "tools/allocator/replacement_front_shim_templates.py": {
        "replacement_front_shim_report_source",
    },
    "tools/allocator/replacement_front_bins_templates.py": {
        "replacement_front_bins_report_source",
    },
    "tools/allocator/replacement_front_smokes.py": {
        "replacement_front_smoke_templates",
    },
    "tools/allocator/hakozuna_mixed_ws_build_support.py": {
        "replacement_front_templates",
    },
}


def repo_path(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def imported_modules(tree: ast.AST) -> set[str]:
    modules: set[str] = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            for alias in node.names:
                modules.add(alias.name.split(".", 1)[0])
        elif isinstance(node, ast.ImportFrom):
            if node.module:
                modules.add(node.module.split(".", 1)[0])
    return modules


def main() -> int:
    failures: list[str] = []
    checked = 0
    for scan_root in SCAN_ROOTS:
        for path in sorted(scan_root.rglob("*.py")):
            if "__pycache__" in path.parts:
                continue
            checked += 1
            try:
                tree = ast.parse(path.read_text(encoding="utf-8"), filename=str(path))
            except SyntaxError as exc:
                failures.append(f"{repo_path(path)}: syntax error while checking imports: {exc}")
                continue
            imported = imported_modules(tree) & DIAGNOSTIC_MODULES
            if not imported:
                continue
            allowed = ALLOWED_IMPORTS.get(repo_path(path), set())
            disallowed = sorted(imported - allowed)
            if disallowed:
                failures.append(
                    f"{repo_path(path)} imports retired bridge payload(s): "
                    + ", ".join(disallowed)
                )
    if failures:
        print("[python-template-c-bridge-import-guard] failed", file=sys.stderr)
        for failure in failures:
            print(f"  {failure}", file=sys.stderr)
        return 1
    print(
        "[python-template-c-bridge-import-guard] ok "
        f"checked={checked} diagnostic_modules={len(DIAGNOSTIC_MODULES)}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
