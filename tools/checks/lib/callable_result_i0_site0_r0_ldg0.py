#!/usr/bin/env python3
"""Guard SITE0-R0-LDG0's disconnected exact caller ledger."""

from __future__ import annotations

import sys
from pathlib import Path


def fail(message: str) -> None:
    raise SystemExit(f"[callable-result-i0-site0-r0-ldg0] {message}")


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
    ledger = read(root, "src/mir/callable_result_representation/caller_ledger.rs")
    errors = read(root, "src/mir/callable_result_representation/caller_ledger_error.rs")
    located = read(root, "src/mir/callable_result_representation/located_legacy.rs")

    require_count(
        ledger,
        "struct VerifiedCallableResultCallerLedgerV1",
        1,
        "caller ledger owner",
    )
    require_count(ledger, "pub(crate) fn verify(", 1, "ledger constructor")
    require_count(ledger, "pub(crate) fn claim(", 1, "exact claim owner")
    require_count(ledger, "pub(crate) fn finish(", 1, "exact finish owner")
    require_count(
        ledger,
        "struct ClaimedCallableResultActivationSiteV1",
        1,
        "claimed-site token owner",
    )
    require_count(
        ledger,
        "struct VerifiedCallableResultInactivePrefixV1",
        1,
        "inactive-prefix proof owner",
    )
    require_count(ledger, "starts_with(prefix_segments)", 1, "prefix containment law")
    require_count(ledger, "BTreeSet<", 1, "claimed-site set")

    for forbidden, label in (
        ("#[derive(Clone", "Clone ledger/proof"),
        ("Arc<", "shared ledger authority"),
        ("Rc<", "shared ledger authority"),
        ("MirBuilder", "Builder coupling"),
        ("Callee::", "MIR publication"),
        ("FunctionCall.name", "name recovery"),
    ):
        if forbidden in ledger:
            fail(f"{label} is forbidden")

    for variant in (
        "UnknownCaller",
        "ForeignPlan",
        "ForeignCaller",
        "ClaimRequiresMethodCall",
        "Duplicate",
        "WrongOrder",
        "Unexpected",
        "RowsUnderPrefix",
        "Missing",
    ):
        if variant not in errors:
            fail(f"missing typed ledger error {variant}")

    require_count(located, "fn activation_claim_parts(", 1, "private claim carrier")
    require_count(located, "plan_identity: located.plan_identity", 4, "branded claim/prefix views")

    production_consumers = 0
    for path in (root / "src").rglob("*.rs"):
        if "callable_result_representation" in path.parts:
            continue
        if path.name == "located_legacy_lowering.rs":
            continue
        production_consumers += path.read_text(encoding="utf-8").count(
            "VerifiedCallableResultCallerLedgerV1"
        )
    if production_consumers != 0:
        fail(f"production ledger consumers: expected=0 actual={production_consumers}")

    touched = [
        "src/mir/callable_result_representation/caller_ledger.rs",
        "src/mir/callable_result_representation/caller_ledger_error.rs",
        "src/mir/callable_result_representation/located_legacy.rs",
        "src/mir/callable_result_representation/tests/caller_ledger.rs",
        "tools/checks/lib/callable_result_i0_site0_r0_ldg0.py",
        "tools/checks/lib/callable_result_i0_path0.py",
    ]
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        "[callable-result-i0-site0-r0-ldg0] ok: "
        "ledger=1 claim=1 finish=1 prefix=1 production_consumers=0"
    )


if __name__ == "__main__":
    main()
