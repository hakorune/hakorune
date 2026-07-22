#!/usr/bin/env python3
"""HDR0-P0 legacy method-tail compatibility guard."""

from __future__ import annotations

from pathlib import Path


def _need(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def _ban(text: str, fragment: str, label: str) -> None:
    if fragment in text:
        raise AssertionError(f"forbidden {label}: {fragment!r}")


def verify_method_tail_compat(root: Path) -> None:
    index = (root / "src/mir/builder/builder_method_index.rs").read_text()
    lifecycle = (root / "src/mir/builder/module_lifecycle.rs").read_text()
    context = (root / "src/mir/builder/compilation_context.rs").read_text()
    tests = (root / "src/mir/builder/compilation_context/tests.rs").read_text()

    _need(index, "names.sort();", "sorted legacy source inventory")
    _need(index, "candidates.sort();", "sorted legacy candidate lists")
    _ban(index, "rebuild_method_tail_index_with_headers", "ambient explicit-header cache writer")
    _need(lifecycle, "self.comp_ctx.clear_method_tail_index();", "prepare module cache reset")
    _need(context, "self.method_tail_index_source_len = 0;", "freshness witness reset")
    _need(tests, "clearing_method_tail_index_resets_freshness_witness", "freshness fixture")

    for path in (
        root / "tools/checks/lib/headerport_method_tail_compat_guard.py",
        root / "src/mir/builder/builder_method_index.rs",
        root / "src/mir/builder/module_lifecycle.rs",
        root / "src/mir/builder/compilation_context.rs",
        root / "src/mir/builder/compilation_context/tests.rs",
    ):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"800-line boundary exceeded: {path.relative_to(root)}")


if __name__ == "__main__":
    verify_method_tail_compat(Path(__file__).resolve().parents[3])
    print("[headerport-method-tail-compat-guard] ok")
