#!/usr/bin/env python3
"""Guard the disconnected callable-result PATH0 structural observation."""

from __future__ import annotations

import re
import sys
from pathlib import Path


def fail(message: str) -> None:
    raise SystemExit(f"[callable-result-i0-path0] {message}")


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
    compiler_view = read(root, "src/mir/compiler/source_view.rs")
    compiler_located = read(root, "src/mir/compiler/located.rs")
    shadow_resolver = read(root, "src/mir/resolved_semantics/shadow/resolver.rs")
    shadow_expr = read(root, "src/mir/resolved_semantics/shadow/expr.rs")
    shadow_stmt = read(root, "src/mir/resolved_semantics/shadow/stmt.rs")
    shadow_block = read(root, "src/mir/resolved_semantics/shadow/block_expr.rs")
    result_expr = read(root, "src/mir/callable_result_representation/expression_proof.rs")
    result_function = read(root, "src/mir/callable_result_representation/function_proof.rs")

    require_count(policy, "enum ExprChildRoleV1", 1, "expression role owner")
    require_count(policy, "enum BodyChildRoleV1", 1, "body role owner")
    require_count(policy, "enum SourceBodyKindV1", 1, "body kind owner")
    require_count(compiler_view, "enum ExprChildRoleV1", 0, "compiler role duplicates")
    require_count(compiler_view, "enum BodyChildRoleV1", 0, "compiler body-role duplicates")
    require_count(compiler_located, "enum SourceBodyKindV1", 0, "compiler body-kind duplicates")

    for label, text in (
        ("shadow expr", shadow_expr),
        ("shadow stmt", shadow_stmt),
        ("shadow block", shadow_block),
        ("result expr", result_expr),
        ("result function", result_function),
    ):
        if "SourcePathSegmentV1::" in text:
            fail(f"{label} bypasses neutral child-role policy")

    require_count(
        shadow_resolver,
        "fn observe_method_calls_shadow_view_v0(",
        1,
        "all-MethodCall observer",
    )
    require_count(
        shadow_resolver,
        "ShadowMethodCallObservationModeV0::All",
        2,
        "all-observation selection and query",
    )

    production_consumers = 0
    for path in (root / "src").rglob("*.rs"):
        if path.name.endswith("tests.rs") or path == root / "src/mir/resolved_semantics/shadow/resolver.rs":
            continue
        production_consumers += path.read_text(encoding="utf-8").count(
            "observe_method_calls_shadow_view_v0("
        )
    if production_consumers != 0:
        fail(f"production observer consumers: expected=0 actual={production_consumers}")

    parser = read(root, "lang/src/compiler/parser/parser_box.hako")
    start = parser.index("\n  static_const_parse_add(text, pos) {")
    end = parser.index("\n  static_const_parse_mul(text, pos) {")
    method = parser[start:end]
    expected = {
        "current-owner me receivers": (r"\bme\.", 9),
        "bound text receivers": (r"\btext\.", 4),
        "proven-unbound skip_ws receivers": (r"\bParserStringUtilsBox\.skip_ws\(", 2),
    }
    for label, (pattern, count) in expected.items():
        actual = len(re.findall(pattern, method))
        if actual != count:
            fail(f"ParserBox {label}: expected={count} actual={actual}")

    touched = [
        "src/mir/resolved_semantics/source_path_policy.rs",
        "src/mir/resolved_semantics/shadow/resolver.rs",
        "src/mir/resolved_semantics/shadow/expr.rs",
        "src/mir/resolved_semantics/shadow/stmt.rs",
        "src/mir/resolved_semantics/shadow/product.rs",
        "src/mir/resolved_semantics/shadow/method_call_observation_tests.rs",
        "tools/checks/lib/callable_result_i0_path0.py",
    ]
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print("[callable-result-i0-path0] ok: policy=1 walker=1 calls=15 consumers=0")


if __name__ == "__main__":
    main()
