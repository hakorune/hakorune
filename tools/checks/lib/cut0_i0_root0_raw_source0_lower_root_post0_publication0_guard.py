#!/usr/bin/env python3
"""PUBLICATION0-S0 guard for the RawDirect publication boundary."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-post0-publication0-s0-execution-task-2026-07-24.md"
)
SOURCES = (
    ROOT / "src/mir/compiler/publication_kernel.rs",
    ROOT / "src/mir/compiler/raw_root_publication.rs",
    ROOT / "src/mir/compiler/external_commit.rs",
    ROOT / "src/mir/builder/builder_publication_target.rs",
    ROOT / "src/mir/builder/raw_root_physical/publication_terminal.rs",
    ROOT / "src/mir/builder/module_invocation_session.rs",
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = (ROOT / "docs/development/current/main/CURRENT_STATE.toml").read_text()
    task = TASK.read_text()
    if not any(
        row in state
        for row in (
            'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLICATION0-S0"',
            'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLICATION-ADAPTER-CONSULT0"',
        )
    ):
        raise AssertionError("PUBLICATION0-S0 is not the current execution row")

    for fragment in (
        "RAW-PUBLICATION-prime-r1",
        "MirCompiler::publish_raw_direct",
        "RawPublishedInvocationV1::{Script, App}",
        "PUBLICATION-KERNEL0",
        "PreparedBuilderExternalCommitV1::commit non-test direct caller = 1",
        "RawPublishedInvocationV1 -> MirCompileResult producer          = 0",
    ):
        require(task, fragment, f"task contract {fragment}")

    texts = {path: path.read_text() for path in (TASK, *SOURCES)}
    for path, text in texts.items():
        if len(text.splitlines()) >= 800:
            raise AssertionError(f"PUBLICATION0 file must remain below 800 lines: {path}")

    kernel = texts[SOURCES[0]]
    raw = texts[SOURCES[1]]
    legacy = texts[SOURCES[2]]
    target = texts[SOURCES[3]]
    carrier = texts[SOURCES[4]]
    session = texts[SOURCES[5]]

    for fragment in (
        "trait SealedPublicationPayloadV1",
        "fn publish_once",
        "builder.commit(target)",
        "BuilderPublicationReceiptV1",
    ):
        require(kernel, fragment, f"shared publication kernel {fragment}")
    require(legacy, "publish_once", "legacy publication kernel reuse")

    for fragment in (
        "RawPublishedInvocationV1",
        "publish_raw_direct",
        "RejectedRawPublicationInvocationV1",
        "RawPublicationSealV1",
        "RawPostprocessEvidenceV1",
        "RawExternalCommitModuleV1",
        "RawPublishedModuleV1",
        "PreparedRawPublicationV1",
        "ProgressNotSealed",
    ):
        require(raw, fragment, f"RawDirect publication surface {fragment}")

    for fragment in (
        "check_builder_external_commit_quiescence",
        "current_module",
        "current_slot_registry",
        "recursion_depth",
        "BuilderPublicationReceiptV1",
    ):
        require(target, fragment, f"target quiescence/receipt {fragment}")
    require(carrier, "module: MirModule", "published module owner")
    require(session, "check_builder_external_commit_quiescence", "candidate quiescence reuse")
    require(session, "fn replace_live_builder", "single Builder assignment helper")

    forbidden_new = (
        "MirCompileResult",
        "into_module",
        "DerefMut",
        "AsMut<MirModule>",
        "module_mut",
        "compile_with_source",
        "catch_unwind",
        "retry(",
        "fallback(",
    )
    for fragment in forbidden_new:
        if fragment in raw or fragment in carrier:
            raise AssertionError(f"PUBLICATION0 leaks forbidden authority: {fragment}")

    if raw.count("pub(in crate::mir) fn publish_raw_direct") != 1:
        raise AssertionError("RawDirect publication consumer must be exactly one")
    if raw.count("RawPublishedInvocationV1::Script") != 1 or raw.count(
        "RawPublishedInvocationV1::App"
    ) != 1:
        raise AssertionError("Script/App publication constructors must be unique")
    if kernel.count("builder.commit(target)") != 1:
        raise AssertionError("shared kernel must own exactly one low-level assignment call")
    if session.count("*current = candidate") != 1:
        raise AssertionError("live Builder replacement must have one assignment site")

    print(
        "[cut0-i0-root0-raw-source0-lower-root-post0-publication0-guard] ok "
        "kernel=1 target_quiescence=1 opaque=1 raw_consumer=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
