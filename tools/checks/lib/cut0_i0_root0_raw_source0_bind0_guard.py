#!/usr/bin/env python3
"""RAW-SOURCE0-BIND0 compiler-owned Raw source-binding guard."""

from __future__ import annotations

import pathlib
import re


ROOT = pathlib.Path(__file__).resolve().parents[3]
CARD = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-consultation-2026-07-23.md"
)
MODULE = ROOT / "src/mir/compiler/raw_source_binding.rs"
TEST = ROOT / "src/mir/compiler/raw_source_binding_p0.rs"
LOWERING_INPUT = ROOT / "src/mir/compiler/lowering_input.rs"
PROGRAM_V0_LOADER = ROOT / "src/runner/json_artifact/program_json_v0_loader.rs"
ISSUER = ROOT / "src/mir/compiler/source_bound_package.rs"
SESSION = ROOT / "src/mir/builder/module_invocation_session.rs"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    card = CARD.read_text()
    module = MODULE.read_text()
    test = TEST.read_text()
    issuer = ISSUER.read_text()
    session = SESSION.read_text()

    for fragment in (
        "RAW-SOURCE0-BIND0",
        "compiler-owned Raw token issuance",
        "Program(JSON v0) remains outside this row",
        "source continuation lifetime reaches paired evidence",
    ):
        require(card, fragment, f"BIND0 boundary {fragment}")
    for fragment in (
        "RawIngressRequestV1",
        "SourceBoundRawPackageV1",
        "RawSourceContinuationV1",
        "RawCallableMainSelectionV1",
        "issue_raw",
        "snapshot_for_raw",
    ):
        require(module + issuer + session, fragment, f"BIND0 product {fragment}")
    for fragment in (
        "raw_bind_mints_one_compiler_owned_raw_token_after_projection",
        "raw_bind_selected_callable_main_requires_app_source",
        "raw_bind_rejects_required_callable_main_for_script",
    ):
        require(test, fragment, f"BIND0 fixture {fragment}")
    retired = module + test + LOWERING_INPUT.read_text() + PROGRAM_V0_LOADER.read_text()
    for fragment in ("ProgramV0Compatibility", "program_v0_compatibility", "ProgramV0OutsideRawSource0"):
        if fragment in retired:
            raise AssertionError(f"retired Program-v0 Raw compatibility residue: {fragment}")

    for forbidden in (
        "ModuleDraftCollectorV1",
        "RawExpansionReceiptLedgerV1",
        "ModuleBuilderInvocationSessionV1",
        "MirModule",
        "build_module",
        "execute_preflighted_module_invocation",
    ):
        if forbidden in module:
            raise AssertionError(f"BIND0 must not own physical/executor surface: {forbidden}")

    issue_raw_count = issuer.count("pub(super) fn issue_raw")
    if issue_raw_count != 1:
        raise AssertionError(f"expected one compiler-owned issue_raw terminal, got {issue_raw_count}")
    bind_consumers = []
    for path in ROOT.glob("src/**/*.rs"):
        if path == MODULE or "tests" in path.parts or path.name.endswith("_p0.rs"):
            continue
        production = re.split(
            r"(?m)^#\[cfg\(test\)\]\s*\nmod\s+\w+\s*\{", path.read_text(), maxsplit=1
        )[0]
        if "bind_raw_source(" in production:
            bind_consumers.append(path.relative_to(ROOT))
    if bind_consumers != [pathlib.Path("src/mir/compiler/mod.rs")]:
        raise AssertionError(f"unexpected Raw binding consumers: {bind_consumers}")

    for path in (MODULE, TEST, CARD, ISSUER, SESSION):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"BIND0 file must remain below 800 lines: {path}")

    print(
        "[cut0-i0-root0-raw-source0-bind0-guard] ok "
        "source_binding=1 token_issuer=1 physical_consumer=0 executor=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
