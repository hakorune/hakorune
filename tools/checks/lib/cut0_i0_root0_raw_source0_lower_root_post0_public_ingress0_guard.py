#!/usr/bin/env python3
"""PUBLIC-INGRESS0-S0 guard for the explicit NarrowV1 Raw entry."""

from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-post0-public-ingress0-s0-"
    "execution-task-2026-07-24.md"
)
REPAIR_TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-post0-public-ingress0-closeout-repair0-s0-"
    "execution-task-2026-07-24.md"
)
CALLER_MANIFEST = ROOT / "tools/checks/manifests/raw_public_cutover_caller_manifest_v1.json"
SOURCES = (
    ROOT / "src/mir/compiler/raw_public_ingress.rs",
    ROOT / "src/mir/compiler/raw_public_ingress_p0.rs",
    ROOT / "src/mir/compiler/raw_root_publication_adapter.rs",
)

_RUST_IGNORED = re.compile(
    r"(?P<raw>r(?P<hash>#*)\".*?\"(?P=hash))"
    r"|(?P<string>(?:b|c)?\"(?:\\.|[^\"\\])*\")"
    r"|(?P<block>/\*.*?\*/)"
    r"|(?P<line>//[^\n]*)",
    re.S,
)
_CFG_TEST_MODULE = re.compile(r"#\[cfg\(test\)\]\s*mod\s+\w+")


def code_only(text: str) -> str:
    return _RUST_IGNORED.sub(
        lambda match: "".join("\n" if char == "\n" else " " for char in match.group()),
        text,
    )


def strip_cfg_test_modules(text: str) -> str:
    cursor = 0
    output: list[str] = []
    while True:
        match = _CFG_TEST_MODULE.search(text, cursor)
        if match is None:
            output.append(text[cursor:])
            return "".join(output)
        output.append(text[cursor : match.start()])
        brace = text.find("{", match.start())
        semicolon = text.find(";", match.start())
        if semicolon >= 0 and (brace < 0 or semicolon < brace):
            cursor = semicolon + 1
            continue
        if brace < 0:
            raise AssertionError("cfg(test) module without body or declaration terminator")
        depth = 0
        for end in range(brace, len(text)):
            if text[end] == "{":
                depth += 1
            elif text[end] == "}":
                depth -= 1
                if depth == 0:
                    cursor = end + 1
                    break
        else:
            raise AssertionError("unterminated cfg(test) module")


def production_paths() -> list[Path]:
    declared_test_modules: set[str] = set()
    for path in ROOT.glob("src/**/*.rs"):
        declared_test_modules.update(
            re.findall(
                r"#\[cfg\(test\)\]\s*mod\s+([A-Za-z0-9_]+)\s*;",
                path.read_text(),
            )
        )
    return sorted(
        path
        for path in ROOT.glob("src/**/*.rs")
        if path.stem not in declared_test_modules
        and not path.name.endswith("_tests.rs")
        and not path.name.endswith("_p0.rs")
        and "tests" not in path.parts
    )


def production_code(path: Path) -> str:
    return strip_cfg_test_modules(code_only(path.read_text()))


def count_by_manifest(rows: dict[str, int], token: str) -> None:
    if not rows:
        raise AssertionError(f"caller manifest has no rows for {token!r}")
    for relative, expected in rows.items():
        path = ROOT / relative
        if not path.is_file():
            raise AssertionError(f"caller manifest path missing: {relative}")
        count = production_code(path).count(token)
        if count != expected:
            raise AssertionError(
                f"caller drift in {relative}: token={token!r} expected={expected} actual={count}"
            )


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = (ROOT / "docs/development/current/main/CURRENT_STATE.toml").read_text()
    task = TASK.read_text()
    repair_task = REPAIR_TASK.read_text()
    caller_manifest = json.loads(CALLER_MANIFEST.read_text())
    require(
        state,
        "PUBLIC-INGRESS0-CLOSEOUT-REPAIR0-S0 are closed",
        "closed closeout-repair row",
    )
    require(
        state,
        "raw_post0_public_ingress0_closeout_repair0_task =",
        "closeout-repair pointer",
    )
    require(task, "Status: closed", "landed ingress row")
    require(repair_task, "Status: closed", "closed closeout-repair task")
    for fragment in (
        "RAW-PUBLIC-ADAPTER-prime-r1",
        "compile_raw_with_source",
        "NarrowV1",
        "compile_with_source cutover",
        "Program(JSON v0)",
        "RAW-PUBLICATION-SUNSET-001",
    ):
        require(task, fragment, f"task contract {fragment}")

    texts = {
        path: path.read_text()
        for path in (TASK, REPAIR_TASK, CALLER_MANIFEST, *SOURCES)
    }
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
    require(tests, "raw_public_ingress_reuses_one_compiler_for_two_successes", "reuse fixture")
    require(tests, "raw_public_ingress_failure_is_discarded_before_reuse", "failure/reuse fixture")
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

    if caller_manifest.get("schema_version") != 1:
        raise AssertionError("caller manifest schema drift")
    raw_public = caller_manifest.get("raw_public", {})
    definition_path = ROOT / raw_public.get("definition_file", "")
    definition_anchor = raw_public.get("definition_anchor", "")
    if definition_path != SOURCES[0] or definition_anchor not in ingress:
        raise AssertionError("Raw public definition manifest drift")
    production = {path: production_code(path) for path in production_paths()}
    raw_calls = [
        path.relative_to(ROOT)
        for path, text in production.items()
        if path != SOURCES[0] and "compile_raw_with_source(" in text
    ]
    if len(raw_calls) != raw_public.get("non_test_callers"):
        raise AssertionError(f"Raw public non-test caller drift: {raw_calls}")

    normal = caller_manifest.get("normal_source_hint", {})
    count_by_manifest(normal.get("no_import_callers", {}), "compile_with_source_hint(")
    count_by_manifest(
        normal.get("import_callers", {}),
        "compile_with_source_hint_and_imports(",
    )
    count_by_manifest(caller_manifest.get("normal_compile_adapters", {}), ".compile(")
    count_by_manifest(
        caller_manifest.get("direct_build_module_production", {}),
        ".build_module(",
    )
    test_bridge = ROOT / "src/host_providers/mir_builder/lowering.rs"
    require(test_bridge.read_text(), "#[cfg(test)]\nmod ast_json;", "cfg(test) AST-JSON bridge")
    for relative in caller_manifest.get("direct_build_module_cfg_test", {}):
        if not (ROOT / relative).is_file():
            raise AssertionError(f"cfg(test) AST-JSON bridge path missing: {relative}")

    old_calls = []
    old_definitions = {
        ROOT / "src/mir/builder/raw_physical_finalization.rs",
        ROOT / "src/mir/compiler/raw_finalization.rs",
        ROOT / "src/mir/compiler/module_postprocess.rs",
        ROOT / "src/mir/compiler/external_commit.rs",
    }
    for path, text in production.items():
        if path in old_definitions:
            continue
        if "RawModuleFinalizerV1::prepare(" in text or (
            "RawPhysicalCompleteInvocationV1" in text and ".prepare_finalization(" in text
        ):
            old_calls.append(path.relative_to(ROOT))
    if old_calls:
        raise AssertionError(f"old Raw non-test callers: {old_calls}")
    if caller_manifest.get("old_raw_non_test_callers") != 0:
        raise AssertionError("old Raw caller manifest drift")

    print(
        "[cut0-i0-root0-raw-source0-lower-root-post0-public-ingress0-guard] ok "
        "landed=1 closeout=1 raw_non_test=0 old_raw_non_test=0 json=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
