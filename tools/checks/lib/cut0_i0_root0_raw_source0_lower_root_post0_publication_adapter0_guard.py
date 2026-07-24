#!/usr/bin/env python3
"""PUBLICATION-ADAPTER0-S0 guard for the Raw compatibility boundary."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-post0-publication-adapter0-s0-"
    "execution-task-2026-07-24.md"
)
SOURCES = (
    ROOT / "src/mir/compiler/raw_root_publication_adapter.rs",
    ROOT / "src/mir/compiler/raw_root_publication.rs",
    ROOT / "src/mir/builder/raw_root_physical/publication_terminal.rs",
    ROOT / "src/mir/builder/raw_root_physical/postprocess_terminal.rs",
    ROOT / "src/mir/builder/module_invocation_session.rs",
    ROOT / "src/mir/compiler/raw_root_publication_adapter_p0.rs",
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = (ROOT / "docs/development/current/main/CURRENT_STATE.toml").read_text()
    task = TASK.read_text()
    require(
        state,
        'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLICATION-ADAPTER0-S0"',
        "active adapter row",
    )
    for fragment in (
        "RAW-PUBLIC-ADAPTER-prime-r1",
        "RawPublishedInvocationV1",
        "RawPublicationCompatibilityEnvelopeV1",
        "compile_raw_with_source",
        "JSON",
        "RAW-PUBLICATION-SUNSET-001",
    ):
        require(task, fragment, f"task contract {fragment}")

    texts = {path: path.read_text() for path in (TASK, *SOURCES)}
    for path, text in texts.items():
        if len(text.splitlines()) >= 800:
            raise AssertionError(f"adapter file must remain below 800 lines: {path}")

    adapter = texts[SOURCES[0]]
    publication = texts[SOURCES[1]]
    carrier = texts[SOURCES[2]]
    physical = texts[SOURCES[3]]
    session = texts[SOURCES[4]]
    tests = texts[SOURCES[5]]

    for fragment in (
        "RawPublicationCompatibilityEnvelopeV1",
        "RawPublicationToResultEvidenceV1",
        "into_compatibility_envelope",
        "into_compatibility(self)",
        "project_verification",
        "into_compatibility_module",
        "RawVerificationProjectionSealV1",
    ):
        require(adapter, fragment, f"adapter surface {fragment}")
    require(carrier, "module: MirModule", "published module owner")
    require(carrier, "into_compatibility_module", "sole module opener")
    require(physical, "into_published_module", "physical carrier handoff")
    require(session, "commit_raw_direct", "Raw publication terminal")
    require(tests, "raw_adapter_moves_reportable_verifier_errors_once", "error fixture")

    if adapter.count("pub(in crate::mir) fn into_compatibility_envelope") != 1:
        raise AssertionError("compatibility envelope producer must be exactly one")
    if adapter.count("pub(in crate::mir) fn into_compatibility(self)") != 1:
        raise AssertionError("authority-erasure terminal must be exactly one")
    if carrier.count("pub(in crate::mir) fn into_compatibility_module") != 1:
        raise AssertionError("published module opener must be exactly one")
    if "compile_raw_with_source" in "".join(texts[path] for path in SOURCES):
        raise AssertionError("public ingress must remain disconnected in adapter row")
    if "compile_with_source" in adapter or "runtime/mirbuilder_emit" in adapter:
        raise AssertionError("adapter must not touch normal or JSON ingress")
    for forbidden in (
        "MirBuilder.builder.current_module",
        "current_module.take",
        "ledger.reproject",
        "clone()",
        "retry(",
        "fallback(",
        "catch_unwind",
    ):
        if forbidden in adapter:
            raise AssertionError(f"adapter leaks forbidden authority: {forbidden}")
    require(publication, "RawPublishedModuleV1", "publication result carrier")
    require(session, "replace_live_builder(self.session.candidate, current)", "one-shot co-publication")

    print(
        "[cut0-i0-root0-raw-source0-lower-root-post0-publication-adapter0-guard] ok "
        "carrier=1 evidence=1 erasure=1 ingress=0 json=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
