#!/usr/bin/env python3
"""PUBLIC-INGRESS0-S0 guard for the explicit NarrowV1 Raw entry."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-post0-public-ingress0-s0-"
    "execution-task-2026-07-24.md"
)
SOURCES = (
    ROOT / "src/mir/compiler/raw_public_ingress.rs",
    ROOT / "src/mir/compiler/raw_public_ingress_p0.rs",
    ROOT / "src/mir/compiler/raw_root_publication_adapter.rs",
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = (ROOT / "docs/development/current/main/CURRENT_STATE.toml").read_text()
    task = TASK.read_text()
    require(
        state,
        'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-INGRESS0-S0"',
        "active ingress row",
    )
    for fragment in (
        "RAW-PUBLIC-ADAPTER-prime-r1",
        "compile_raw_with_source",
        "NarrowV1",
        "compile_with_source cutover",
        "Program(JSON v0)",
        "RAW-PUBLICATION-SUNSET-001",
    ):
        require(task, fragment, f"task contract {fragment}")

    texts = {path: path.read_text() for path in (TASK, *SOURCES)}
    for path, text in texts.items():
        if len(text.splitlines()) >= 800:
            raise AssertionError(f"ingress file must remain below 800 lines: {path}")

    ingress = texts[SOURCES[0]]
    tests = texts[SOURCES[1]]
    adapter = texts[SOURCES[2]]
    for fragment in (
        "compile_raw_with_source",
        "RawPublicIngressPolicyV1",
        "RawCallableMainSelectionV1::Omitted",
        '"main"',
        "bind_raw_source",
        "into_root_package",
        "prepare_eligibility",
        "prepare_root_batch",
        "prepare_drain",
        "prepare_finalization",
        "prepare_external_commit",
        "publish_raw_direct",
        "into_compatibility_envelope",
        "fn reject<",
        "discard(rejection)",
    ):
        require(ingress, fragment, f"ingress chain {fragment}")
    require(tests, "raw_public_ingress_compiles_empty_script_without_legacy_fallback", "success fixture")
    require(tests, "raw_public_ingress_rejects_repl_before_source_binding", "REPL fixture")
    require(adapter, "into_compatibility", "adapter handoff")

    if ingress.count("pub fn compile_raw_with_source") != 1:
        raise AssertionError("explicit Raw ingress producer must be exactly one")
    for forbidden in (
        "compile_legacy(",
        "compile_legacy_request(",
        "build_module(",
        "compile_with_source(",
        "ProgramV0Compatibility",
        "catch_unwind",
        "retry(",
        "fallback(",
    ):
        if forbidden in ingress:
            raise AssertionError(f"Raw ingress leaks forbidden route: {forbidden}")
    if "runtime/mirbuilder_emit" in ingress or "Program(JSON" in ingress:
        raise AssertionError("Raw ingress must not alter JSON/runtime bridges")

    print(
        "[cut0-i0-root0-raw-source0-lower-root-post0-public-ingress0-guard] ok "
        "consumer=1 chain=1 fallback=0 json=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
