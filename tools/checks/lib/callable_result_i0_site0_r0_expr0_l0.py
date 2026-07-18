#!/usr/bin/env python3
"""Guard SITE0-R0-EXPR0-L0's disconnected located lowering session."""

from __future__ import annotations

import re
import sys
from pathlib import Path


TAG = "[callable-result-i0-site0-r0-expr0-l0]"


def fail(message: str) -> None:
    raise SystemExit(f"{TAG} {message}")


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
    session_path = "src/mir/builder/located_legacy_lowering.rs"
    session = read(root, session_path)
    tests = read(
        root,
        "src/mir/callable_result_representation/tests/located_legacy_lowering.rs",
    )
    builder_root = read(root, "src/mir/builder.rs")
    recursion = read(root, "src/mir/builder/recursive_child_lowering.rs")
    readme = read(root, "src/mir/builder/calls/README.md")

    require_count(
        session,
        "struct LocatedLegacyLoweringSessionV1",
        1,
        "located session owner",
    )
    require_count(session, "state: LocatedLegacyLoweringStateV1", 2, "failure state")
    require_count(
        session,
        "source: VerifiedCallableResultLegacySourceViewV1",
        1,
        "source-view owner",
    )
    require_count(
        session,
        "ledger: VerifiedCallableResultCallerLedgerV1",
        1,
        "caller-ledger owner",
    )
    require_count(session, ".claim(&input)", 1, "MethodCall entry claim")
    require_count(session, ".finish()", 1, "ledger finish")
    require_count(session, "LocatedLegacyLoweringErrorV1::Poisoned", 2, "poison law")

    claim_at = session.index(".claim(&input)")
    guard_at = session.index("with_legacy_expression_recursion_guard_v1(", claim_at)
    route_at = session.index("build_method_call_from_input_v1", claim_at)
    if not claim_at < guard_at < route_at:
        fail("claim must precede expression guard and MethodCall route")

    for kind in ("body", "statement", "expression"):
        require_count(
            session,
            f"fn delegate_inactive_{kind}(",
            1,
            f"inactive {kind} delegate",
        )
    require_count(
        session,
        "_proof: VerifiedCallableResultInactivePrefixV1",
        3,
        "proof-required raw delegates",
    )
    require_count(
        session,
        "ExprChildRoleV1::Receiver",
        1,
        "PATH0 receiver projection",
    )
    require_count(
        session,
        "ExprChildRoleV1::CallArgument(index)",
        1,
        "PATH0 argument projection",
    )

    for helper in (
        "emit_typeop_value_terminal_raw_v1",
        "emit_global_value_terminal_raw_v1",
        "emit_env_value_terminal_raw_v1",
        "emit_standard_value_terminal_raw_v1",
    ):
        if helper not in session:
            fail(f"missing thin V0 raw terminal forwarding: {helper}")

    if re.search(
        r"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(in crate::mir\) struct LocatedLegacyLoweringSessionV1",
        session,
    ):
        fail("located session must remain non-Clone")
    for forbidden in (
        "Arc<",
        "Rc<",
        "thread_local!",
        "current_claim",
        "SourcePathV1",
        "activation_site()",
        "value_types",
        "next_value_id",
        "MirInstruction",
        "CallTarget",
        "retry",
        "fallback",
    ):
        if forbidden in session:
            fail(f"session owns forbidden authority: {forbidden}")

    require_count(
        recursion,
        "fn with_legacy_expression_recursion_guard_v1",
        1,
        "shared expression recursion guard",
    )
    require_count(
        builder_root,
        "mod located_legacy_lowering;",
        1,
        "private session module",
    )
    if "LocatedLegacyLoweringSessionV1<'" in builder_root:
        fail("MirBuilder must not store the located session")

    production_constructor_callers = 0
    for path in (root / "src").rglob("*.rs"):
        if path.resolve() == (root / session_path).resolve() or "tests" in path.parts:
            continue
        production_constructor_callers += path.read_text(encoding="utf-8").count(
            "LocatedLegacyLoweringSessionV1::verify("
        )
    if production_constructor_callers != 0:
        fail(
            "production session constructors: "
            f"expected=0 actual={production_constructor_callers}"
        )

    for evidence in (
        "selected_nested_and_unselected_method_rows_are_claimed_before_descent",
        "inactive_expression_delegates_once_and_finish_reports_missing_rows",
        "active_row_under_non_method_prefix_never_reaches_raw_lowering",
        "wrong_order_and_duplicate_claims_fail_before_new_child_effects",
        "route_failure_after_claim_poisons_session_and_fresh_session_is_independent",
    ):
        if evidence not in tests:
            fail(f"missing L0 fixture: {evidence}")

    for phrase in (
        "disconnected EXPR0-L0 session",
        "Every selected",
        "or unselected MethodCall row is claimed",
        "exact inactive-prefix",
        "zero production callers",
        "active non-MethodCall spines remain a separate",
    ):
        if phrase not in readme:
            fail(f"missing README boundary: {phrase}")

    touched = (
        session_path,
        "src/mir/callable_result_representation/located_legacy.rs",
        "src/mir/callable_result_representation/tests/located_legacy_lowering.rs",
        "src/mir/builder/recursive_child_lowering.rs",
        "src/mir/builder/calls/method_call_terminal.rs",
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_l0.py",
    )
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        f"{TAG} ok: session=1 claim-before-child=1 inactive-proofs=3 "
        "production_callers=0 result_publication=0"
    )


if __name__ == "__main__":
    main()
