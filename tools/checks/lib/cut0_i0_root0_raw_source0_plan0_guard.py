#!/usr/bin/env python3
"""RAW-SOURCE0-PLAN0 owned source-projection proof guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
CARD = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-consultation-2026-07-23.md"
)
MODULE = ROOT / "src/mir/builder/raw_source_projection.rs"
BUILDER = ROOT / "src/mir/builder.rs"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    card = CARD.read_text()
    module = MODULE.read_text()
    builder = BUILDER.read_text()

    for fragment in (
        "RAW-SOURCE0-PLAN0",
        "owned source projection",
        "borrowed self-referential expansion = 0",
        "Program(JSON v0) remains outside this row",
    ):
        require(card, fragment, f"RAW-SOURCE0 boundary {fragment}")
    require(builder, "mod raw_source_projection;", "Builder module registration")
    for fragment in (
        "RawSourceOriginV1",
        "RawSourceLocatorV1",
        "OwnedRawRootProjectionV1",
        "OwnedRawSourceV1",
        "projection_owns_locators_without_borrowing_the_ast",
        "script_projection_has_no_synthetic_root_locator",
    ):
        require(module, fragment, f"projection product {fragment}")

    if "MirCompiler" in module or "ModuleInvocationTokenV1" in module:
        raise AssertionError("PLAN0 projection must not own compiler identity")
    if "RawExpansionReceiptLedgerV1" in module or "ModuleDraftCollectorV1" in module:
        raise AssertionError("PLAN0 projection must not own physical evidence")

    consumers = []
    for path in ROOT.glob("src/**/*.rs"):
        if path == MODULE or "tests" in path.parts:
            continue
        if "OwnedRawSourceV1::bind(" in path.read_text():
            consumers.append(path.relative_to(ROOT))
    if consumers:
        raise AssertionError(f"PLAN0 has production projection consumers: {consumers}")

    for path in (MODULE, CARD):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"PLAN0 file must remain below 800 lines: {path}")

    print(
        "[cut0-i0-root0-raw-source0-plan0-guard] ok "
        "owned_projection=1 production_consumers=0 token=0 physical_evidence=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
