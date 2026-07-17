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
    located = read(root, "src/mir/callable_result_representation/located_legacy.rs")
    errors = read(root, "src/mir/callable_result_representation/located_legacy_error.rs")
    policy = read(root, "src/mir/resolved_semantics/source_path_policy.rs")
    tests = read(root, "src/mir/callable_result_representation/tests/located_legacy.rs")

    require_count(located, "struct VerifiedCallableResultLegacySourceViewV1", 1, "source view")
    for carrier in ("Body", "Stmt", "Expr"):
        require_count(located, f"struct LocatedLegacy{carrier}V1", 1, f"located {carrier}")
        require_count(located, f"struct UnlocatedLegacy{carrier}V1", 1, f"unlocated {carrier}")
        require_count(located, f"enum Legacy{carrier}InputV1", 1, f"{carrier} input")

    require_count(located, "VerifiedCallableResultActivationPlanV1", 2, "activation plan boundary")
    require_count(located, "UnlocatedCannotClaimActivation", 1, "unlocated claim rejection")
    require_count(errors, "UnlocatedCannotClaimActivation", 1, "typed unlocated error")
    require_count(policy, "pub(crate) fn resolve<'source>", 2, "neutral child resolvers")
    require_count(
        policy,
        "ExprChildSyntaxV1::SyntheticName",
        2,
        "synthetic child classification plus fixture",
    )

    for forbidden in (
        "SourcePathSegmentV1::",
        "build_expression(",
        "MirBuilder",
        "MirInstruction",
        "Arc<",
        "Rc<",
        "ASTNode::clone",
        "span",
        "source_location",
    ):
        if forbidden in located:
            fail(f"located carrier contains forbidden authority: {forbidden}")
    if re.search(r"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(crate\) (struct|enum)", located):
        fail("located inputs/views must remain non-Clone")

    production_consumers = 0
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if relative.endswith("/tests/located_legacy.rs") or relative.endswith("/located_legacy.rs"):
            continue
        production_consumers += path.read_text(encoding="utf-8").count(
            "VerifiedCallableResultLegacySourceViewV1::verify("
        )
    if production_consumers != 0:
        fail(f"production located consumers: expected=0 actual={production_consumers}")

    for evidence in (
        "root_body_local_initializer_and_nested_arguments_keep_exact_sites",
        "if_body_uses_role_owned_body_and_item_segments",
        "unlocated_syntax_and_descendants_cannot_claim_activation",
        "equal_foreign_plan_carriers_are_rejected",
        "declaration_reorder_preserves_normalized_site",
    ):
        if evidence not in tests:
            fail(f"missing fixture evidence: {evidence}")

    touched = [
        "src/mir/callable_result_representation/located_legacy.rs",
        "src/mir/callable_result_representation/located_legacy_error.rs",
        "src/mir/callable_result_representation/tests/located_legacy.rs",
        "src/mir/resolved_semantics/source_path_policy.rs",
        "src/mir/compiler/source_view.rs",
        "tools/checks/lib/callable_result_i0_site0_l0.py",
    ]
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        "[callable-result-i0-site0-l0] ok: view=1 located=3 "
        "unlocated=3 role_resolvers=2 production_consumers=0"
    )


if __name__ == "__main__":
    main()
