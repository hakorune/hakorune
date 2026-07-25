#!/usr/bin/env python3
"""Aggregate the accepted bounded MirBuilder core-complete proof."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
FUNCTION_EXIT_DOC = ROOT / "docs/reference/language/function-exit-and-entry-result.md"
FUNCTION_EXIT_OWNER = ROOT / "src/mir/resolved_control_flow/function_control.rs"
DRAFT_SEAL = ROOT / "src/mir/builder/resolved_lowering/draft_seal_owner.rs"
SCRIPT_RECIPE = ROOT / "src/mir/raw_root_body_recipe.rs"
ENTRY_RESULT = ROOT / "src/mir/compiler/source_entry_result.rs"
RAW_COMPILE = ROOT / "src/mir/compiler/raw_published_compile.rs"
RAW_EXEC = ROOT / "src/mir/compiler/source_entry_vm_execution.rs"
NORMAL_RUNNER = ROOT / "src/runner/reference/normal_file_vm.rs"
SELECTOR = ROOT / "src/runner/reference/mod.rs"
ROUTE_GUARD = ROOT / "tools/checks/lib/normal_file_vm0_frontdoor_forge_guard.py"
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
    entry_result = ENTRY_RESULT.read_text()
    raw_compile = RAW_COMPILE.read_text()
    raw_exec = RAW_EXEC.read_text()
    normal_runner = NORMAL_RUNNER.read_text()
    selector = SELECTOR.read_text()

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

    for fragment in (
        "pub(in crate::mir) enum SourceEntryResultV1",
        "pub(in crate::mir) struct ProcessExitProjectionV1",
        "project_canonical",
    ):
        require(entry_result, fragment, f"entry/result owner {fragment}")
    require(raw_compile, "pub(in crate::mir) fn compile_raw_published_v1(", "Raw publication kernel")
    require(raw_exec, "pub(in crate::mir) fn run_raw_vm_reference_owned_v1(", "Raw VM execution owner")
    require(normal_runner, "pub(crate) fn run(request: NormalFileVmReferenceProductionRequestV1)", "bounded normal lane")
    for fragment in (
        "pub(crate) fn select_and_run(config: &CliConfig)",
        "Some(raw_vm_reference::run(request))",
        "Some(normal_file_vm::run(request))",
    ):
        require(selector, fragment, f"single explicit selector {fragment}")

    route_guard = ROUTE_GUARD.read_text()
    require(route_guard, "central_selector=1 normal_caller=1", "production route guard")
    if "std::process::exit" in normal_runner or "compile_with_source" in normal_runner:
        raise AssertionError("bounded normal lane owns neither process exit nor Legacy compilation")
    if "normal_file_vm_reference" in selector.lower() and "fallback" in selector.lower():
        raise AssertionError("explicit selector must not add fallback policy")

    checked = (
        FUNCTION_EXIT_DOC,
        FUNCTION_EXIT_OWNER,
        DRAFT_SEAL,
        SCRIPT_RECIPE,
        ENTRY_RESULT,
        RAW_COMPILE,
        RAW_EXEC,
        NORMAL_RUNNER,
        SELECTOR,
        ROUTE_GUARD,
        TASK,
        Path(__file__),
    )
    for path in checked:
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path.relative_to(ROOT)}")
    print(
        "[mirbuilder-core-complete0-guard] ok "
        "function_exit=1 script_result=1 entry_result=1 publication=1 "
        "raw_vm=1 normal_file=1 central_selector=1 default_cutover=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
