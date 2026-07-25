#!/usr/bin/env python3
"""CUT0-I0-POST0 disconnected postprocess-owner census guard."""

from __future__ import annotations

import pathlib
import re


ROOT = pathlib.Path(__file__).resolve().parents[3]
FILES = {
    "task": ROOT / "docs/development/current/main/investigations/cut0-i0-prod-activation-execution-task-2026-07-23.md",
    "post": ROOT / "src/mir/compiler/module_postprocess.rs",
    "stages": ROOT / "src/mir/compiler/module_postprocess_stages.rs",
    "tests": ROOT / "src/mir/compiler/module_postprocess_p0.rs",
    "mod": ROOT / "src/mir/compiler/mod.rs",
    "final": ROOT / "src/mir/compiler/canonical_finalization.rs",
}
OLD_FILES = (
    ROOT / "src/mir/builder/raw_physical_finalization.rs",
    ROOT / "src/mir/compiler/raw_finalization.rs",
)
OLD_SYMBOLS = (
    "RawPhysicalCompleteInvocationV1",
    "RawModuleFinalizerV1",
    "RawFinalizationInputV1",
    "RawFinalizedModuleInvocationV1",
    "ModulePostprocessInputV1::Raw",
    "PostprocessEvidenceInputV1::Raw",
    "PostprocessEvidenceSealV1::Raw",
)

_RUST_IGNORED = re.compile(
    r"(?P<raw>r(?P<hash>#*)\".*?\"(?P=hash))"
    r"|(?P<string>(?:b|c)?\"(?:\\.|[^\"\\])*\")"
    r"|(?P<block>/\*.*?\*/)"
    r"|(?P<line>//[^\n]*)",
    re.S,
)
_CFG_TEST_MODULE = re.compile(r"#\[cfg\(test\)\]\s*mod\s+\w+")


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def production_code(path: pathlib.Path) -> str:
    text = _RUST_IGNORED.sub(
        lambda match: "".join("\n" if char == "\n" else " " for char in match.group()),
        path.read_text(),
    )
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
            raise AssertionError(f"cfg(test) module without body: {path}")
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
            raise AssertionError(f"unterminated cfg(test) module: {path}")


def main() -> int:
    texts = {name: path.read_text() for name, path in FILES.items()}
    for path in OLD_FILES:
        if path.exists():
            raise AssertionError(f"retired Raw source still exists: {path}")
    for name, path in FILES.items():
        if len(texts[name].splitlines()) >= 800:
            raise AssertionError(f"POST0 file must remain below 800 lines: {path}")

    require(
        texts["task"],
        "## POST0 — one postprocess owner (closed; production disconnected)",
        "POST0 boundary",
    )
    require(texts["mod"], "mod module_postprocess;", "POST0 module registration")
    require(
        texts["mod"],
        "#[cfg(test)]\nmod prod_activation_p0_r1;",
        "P0-R1 fixture must remain test-only",
    )
    for fragment in (
        "ModulePostprocessScheduleV1",
        "ModulePostprocessOwnerV1",
        "ModuleVerificationEvidenceV1",
        "ModulePostprocessInputV1",
        "PostprocessedModuleInvocationV1",
        "RejectedModulePostprocessV1",
        "input: ModulePostprocessInputV1<'a>",
        "schedule: ModulePostprocessScheduleV1",
        "stage: PostprocessFailureStageV1",
        "error: ModulePostprocessErrorV1",
        "PostprocessFailureStageV1",
        "fn discard(self)",
        "fn error(&self)",
        "fn stage(&self)",
        "ModulePostprocessInputV1::Canonical",
        "for_family(",
    ):
        require(texts["post"], fragment, f"postprocess owner: {fragment}")
    for fragment in (
        "ModuleVerificationEvidenceV1::Raw",
        "refresh_rune_plans",
        "optimize",
        "refresh_contracts",
        "verify",
        "insert_rc",
        "refresh_semantic_metadata",
        "canonicalize_callsites",
    ):
        require(texts["stages"], fragment, f"shared stage kernel: {fragment}")

    order = [
        "target.refresh_rune_plans()",
        "target.optimize()",
        "target.refresh_contracts()",
        ".verify(verifier)",
        "target.insert_rc()",
        "target.refresh_semantic_metadata()",
        "target.canonicalize_callsites()",
    ]
    positions = [texts["stages"].find(fragment) for fragment in order]
    if any(position < 0 for position in positions) or positions != sorted(positions):
        raise AssertionError(f"POST0 stage order drift: {positions}")

    for forbidden in (
        "current_module",
        "source AST",
        "ModuleInvocationPolicyV1",
        "ConditionFnPolicyV1::Optional",
        "DrainedModuleCandidateV1",
        "retry(",
        "fallback(",
    ):
        if forbidden in texts["post"]:
            raise AssertionError(f"POST0 leaks forbidden authority: {forbidden}")

    for fixture in (
        "postprocess_schedule_is_family_owned",
        "postprocess_consumes_finalized_single_without_publication",
        "postprocess_final_verifier_failure_retains_discard_only_owner",
    ):
        require(texts["tests"], fixture, f"POST0 fixture: {fixture}")

    production = []
    for path in ROOT.glob("src/**/*.rs"):
        if path in (FILES["post"], FILES["final"]):
            continue
        if path.name.endswith("_p0.rs") or path.name.endswith("_tests.rs"):
            continue
        if path.name == "prod_activation_p0_r1.rs":
            continue
        if "tests" in path.parts:
            continue
        text = production_code(path)
        if "run_raw(" in text or "run(finalized)" in text:
            production.append(path.relative_to(ROOT))
    if production:
        raise AssertionError(f"POST0 has production consumers: {production}")

    leaked = []
    for path in ROOT.glob("src/**/*.rs"):
        text = production_code(path)
        for symbol in OLD_SYMBOLS:
            if symbol in text:
                leaked.append((path.relative_to(ROOT), symbol))
    if leaked:
        raise AssertionError(f"retired Raw symbols remain: {leaked}")

    print(
        "[cut0-i0-prod-activation-post0-guard] ok "
        "schedule=1 stage_order=1 family_policy=1 old_chain=0 production_consumers=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
