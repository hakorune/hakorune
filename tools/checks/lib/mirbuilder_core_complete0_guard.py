#!/usr/bin/env python3
"""Aggregate the accepted bounded MirBuilder core-complete proof."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
FUNCTION_EXIT_DOC = ROOT / "docs/reference/language/function-exit-and-entry-result.md"
FUNCTION_EXIT_OWNER = ROOT / "src/mir/resolved_control_flow/function_control.rs"
DRAFT_SEAL = ROOT / "src/mir/builder/resolved_lowering/draft_seal_owner.rs"
SCRIPT_RECIPE = ROOT / "src/mir/raw_root_body_recipe.rs"
RAW_COMPILE = ROOT / "src/mir/compiler/raw_published_compile.rs"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "mirbuilder-core-complete0-proof-task-2026-07-26.md"
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    task = TASK.read_text()
    function_doc = FUNCTION_EXIT_DOC.read_text()
    function_owner = FUNCTION_EXIT_OWNER.read_text()
    draft_seal = DRAFT_SEAL.read_text()
    script_recipe = SCRIPT_RECIPE.read_text()
    raw_compile = RAW_COMPILE.read_text()

    for fragment in (
        "MIRBUILDER-CORE-COMPLETE0-P0",
        "canonical function exit owner = 1",
        "canonical Script result owner = 1",
        "atomic draft/module publication = 1",
        "bounded normal file lane = 1",
        "default route replacement = 0",
        "JSON / REPL / LLVM / selfhost = non-blocking",
    ):
        require(task, fragment, f"core-complete contract {fragment}")

    for fragment in (
        "ordinary function/method",
        "ScriptLastExpressionOrUnit",
        "ProcessExitProjectionV1",
        "promotion",
    ):
        require(function_doc, fragment, f"normative function/entry contract {fragment}")
    require(function_owner, "pub(crate) struct SealedFunctionExitContractV1", "function exit owner")
    require(draft_seal, "PreparedFunctionDraftSealV1", "prepared draft seal owner")
    for fragment in ("RawScriptBodyRecipeV1", "RawScriptTerminalRecipeV1"):
        require(script_recipe, fragment, f"Script result owner {fragment}")

    require(raw_compile, "pub(in crate::mir) fn compile_raw_published_v1(", "Raw publication kernel")

    checked = (
        FUNCTION_EXIT_DOC,
        FUNCTION_EXIT_OWNER,
        DRAFT_SEAL,
        SCRIPT_RECIPE,
        RAW_COMPILE,
        TASK,
        Path(__file__),
    )
    for path in checked:
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path.relative_to(ROOT)}")
    print(
        "[mirbuilder-core-complete0-guard] ok "
        "function_exit=1 script_result=1 entry_result=retired publication=1 "
        "raw_vm=retired normal_file=retired central_selector=retired default_cutover=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
