#!/usr/bin/env python3
"""DRAIN0-S0 Raw manifest, keyed drain, and one-shot boundary guard."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-drain0-s0-execution-task-2026-07-24.md"
)
SOURCES = tuple(
    ROOT / path
    for path in (
        "src/mir/raw_physical_drain.rs",
        "src/mir/builder/raw_root_physical/drain_manifest.rs",
        "src/mir/builder/module_draft_collector/raw_drain.rs",
        "src/mir/builder/raw_root_physical/drain_terminal.rs",
        "src/mir/compiler/raw_root_drain.rs",
        "src/mir/compiler/raw_root_decl_access.rs",
    )
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    task = TASK.read_text()
    for path in SOURCES:
        if not path.exists():
            raise AssertionError(f"missing DRAIN0 source: {path}")
    joined = "\n".join(path.read_text() for path in SOURCES)

    require(task, "DRAIN-prime-r1", "decision lock")
    for fragment in (
        "RawPhysicalDrainManifestV1",
        "final_event_by_key",
        "prepare_drain(self)",
        "PreparedRawDrainInvocationV1",
        "PreparedRawDrainInvocationV1::drain",
        "RawDrainedInvocationV1",
        "external commit",
    ):
        require(task, fragment, f"task contract {fragment}")

    for fragment in (
        "final_events_in_ordinal_order",
        "RawPhysicalDrainManifestV1::new",
        "prepare_raw_drain",
        "PreparedRawCollectorDrainV1",
        "RawUnfinalizedModuleV1",
        "RawDrainWitnessV1",
        "RawDrainedInvocationV1",
    ):
        require(joined, fragment, f"DRAIN0 authority {fragment}")

    compiler = (ROOT / "src/mir/compiler/raw_root_drain.rs").read_text()
    for forbidden in (
        "current_module",
        "OwnedRawSourceV1",
        "DrainedModuleCandidateV1",
        "retry",
        "fallback",
    ):
        if forbidden in compiler:
            raise AssertionError(f"compiler DRAIN0 re-observation/escape: {forbidden}")

    collector = (ROOT / "src/mir/builder/module_draft_collector/raw_drain.rs").read_text()
    if "into_draft_functions" in collector:
        raise AssertionError("Raw DRAIN0 must retain keyed collector order")

    for path in (TASK, *SOURCES):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")

    print(
        "[cut0-i0-root0-raw-source0-lower-root-drain0-guard] ok "
        "manifest=1 keyed_collector=1 opaque_carrier=1 one_shot=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
