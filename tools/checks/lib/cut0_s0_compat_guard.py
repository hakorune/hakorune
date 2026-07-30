#!/usr/bin/env python3
"""CUT0-S0-COMPAT0 policy and selected-call failure guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
SRC = ROOT / "src/mir/builder"
CARD = ROOT / (
    "docs/development/current/main/investigations/"
    "mirbuilder-headerport-i0-hdr0-p0-execution-task-2026-07-22.md"
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    names = (
        "main_expansion.rs",
        "module_compat_policy.rs",
        "module_compat_policy_p0.rs",
        "decls.rs",
        "raw_static_main_compat_batch.rs",
        "recursive_child_lowering.rs",
        "calls/function_session.rs",
        "calls/lowering.rs",
        "builder_build.rs",
        "module_lifecycle.rs",
        "normal_default_root_catalog_lifecycle.rs",
    )
    files = {name: (SRC / name).read_text() for name in names}
    for name, text in files.items():
        if len(text.splitlines()) >= 800:
            raise AssertionError(f"CUT0-S0-COMPAT0 source must remain below 800 lines: {name}")

    require(files["main_expansion.rs"], "VerifiedRawRootExpansionV1", "raw root selector")
    require(files["main_expansion.rs"], "DuplicateMainBox", "duplicate Main rejection")
    require(files["module_compat_policy.rs"], "snapshot_from_legacy_ingress", "sealed policy snapshot")
    require(files["decls.rs"], "CallableMainCompatibilityLoweringErrorV1", "typed callable-Main error")
    require(files["calls/lowering.rs"], "lower_static_method_as_function_typed", "typed selected lowering")
    if "let _ = self.lower_static_method_as_function" in files["calls/lowering.rs"]:
        raise AssertionError("selected callable Main lowering must not discard its error")
    require(
        files["normal_default_root_catalog_lifecycle.rs"],
        "VerifiedRawRootExpansionV1::from_program",
        "preflight selector",
    )
    require(
        files["raw_static_main_compat_batch.rs"],
        "PreparedRawStaticMainBoxCompatibilityV1",
        "prepared raw static-Main owner",
    )
    require(
        files["recursive_child_lowering.rs"],
        "PreparedRawStaticMainBoxCompatibilityV1::prepare(box_name, methods)",
        "direct prepared raw static-Main handoff",
    )
    if "build_static_main_box_typed" in files["decls.rs"]:
        raise AssertionError("retired typed static-Main facade returned")
    require(files["module_compat_policy_p0.rs"], "not_a_missing_receipt", "typed failure fixture")
    if (SRC / "module_compat_raw_ledger_p0.rs").exists():
        raise AssertionError("retired disconnected receipt bridge returned")
    if "mod module_compat_raw_ledger_p0;" in (SRC.parent / "builder.rs").read_text():
        raise AssertionError("retired receipt bridge registration returned")
    callable_main = (SRC / "raw_root_physical/callable_main_terminal.rs").read_text()
    for fragment in (
        "fn complete_callable_main(",
        "RawExpansionDraftRequestV1::callable_main_compatibility",
        "ledger.abort(reservation",
    ):
        require(callable_main, fragment, "live callable-Main receipt owner")
    for fragment in ("retry", "fallback"):
        if fragment in callable_main:
            raise AssertionError(f"live callable-Main owner must not contain {fragment}")

    policy_readers = [
        name for name, text in files.items() if "builder_build_static_main_entry()" in text
    ]
    if policy_readers != ["module_compat_policy.rs"]:
        raise AssertionError(f"ambient compatibility toggle has extra readers: {policy_readers}")

    card = CARD.read_text()
    require(card, "CUT0-S0-COMPAT0", "compatibility task row")
    require(card, "selected callable-Main typed failures", "compatibility acceptance")

    print("[cut0-s0-compat-guard] ok policy=sealed duplicate_main=preflight receipt=abort-proof")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
