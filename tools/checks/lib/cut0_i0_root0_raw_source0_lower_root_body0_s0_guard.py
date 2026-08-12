#!/usr/bin/env python3
"""BODY0-S0 owner, recipe, and unpublished-root boundary guard."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "script-result-tail0-s0-execution-task-2026-07-25.md"
)
SOURCE = tuple(
    ROOT / path
    for path in (
        "src/mir/raw_root_body_recipe.rs",
        "src/mir/compiler/raw_root_source_facts.rs",
        "src/mir/compiler/raw_root_source_facts/recipe_projection.rs",
        "src/mir/compiler/raw_root_source_facts/script_result_p0.rs",
        "src/mir/compiler/raw_root_environment_manifest.rs",
        "src/mir/compiler/raw_root_decl_access.rs",
        "src/mir/compiler/raw_root_decl_access_p0.rs",
        "src/mir/builder/raw_root_environment_install.rs",
        "src/mir/builder/raw_root_environment_install/body_transaction.rs",
        "src/mir/builder/raw_root_body_exit.rs",
        "src/mir/builder/raw_root_physical.rs",
        "src/mir/builder/raw_root_physical/root_batch_terminal.rs",
        "src/mir/builder/raw_root_completion.rs",
        "src/mir/builder/raw_root_body_lowering.rs",
        "src/mir/builder/root_body_completion.rs",
    )
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    task = TASK.read_text()
    joined = "\n".join(path.read_text() for path in SOURCE)
    for fragment in (
        "Decision: `SCRIPT-RESULT-TAIL-prime-r1`",
        "Implementation authorization: `SCRIPT-RESULT-TAIL0-S0` only",
        "RawScriptResultContractV1",
        "RawScriptBodyRecipeV1",
        "one prepared BODY commit",
        "all modified/new source/check files",
        "RawRootBodyExitWitnessV1",
    ):
        require(task, fragment, f"task contract {fragment}")

    for fragment in (
        "fn begin_body(",
        "fn drive_root_body(",
        "lower_linear_scalar_recipe_v1",
        "lower_script_body_recipe_v1",
        "begin_root_body_preserving",
        "prepare_root_body_completion",
        "seal_root_body_prepared",
        "script_contract(",
        "collector_and_ledger_untouched",
        "body_entry_consumes_declared_script_into_unpublished_completion",
        "begin_raw_root_function_v1",
        "prepare_raw_root_exit_v1",
        "commit_raw_root_exit_v1",
        "RawRootBatchPhysicalErrorV1::ExitWitness",
        "RawScriptTerminalRecipeV1",
        "RawScriptUnitOriginV1",
    ):
        require(joined, fragment, f"implementation {fragment}")

    body_lowerer = (ROOT / "src/mir/builder/raw_root_body_lowering.rs").read_text()
    recipe = (ROOT / "src/mir/raw_root_body_recipe.rs").read_text()
    for forbidden in (
        "ASTNode",
        "OwnedRawSourceV1",
        "current_module",
        "build_module(",
        "lower_root(",
        "finalize_module(",
        "RawDraftInvocationV1",
        "ModuleLoweringInvocationStateV1",
        "MainPending",
        "MainCaptured",
        "catch_unwind",
    ):
        if forbidden in body_lowerer or forbidden in recipe:
            raise AssertionError(f"forbidden BODY0 authority: {forbidden}")

    decl_access = (ROOT / "src/mir/compiler/raw_root_decl_access.rs").read_text()
    for forbidden in (
        "current_module",
        "OwnedRawSourceV1::ast",
        "sorted_method_entries",
        "MainPending",
        "MainCaptured",
        "finalize_module",
        "catch_unwind",
        "retry(",
        "resume(",
        "fallback(",
    ):
        if forbidden in decl_access:
            raise AssertionError(f"forbidden BODY0 compiler authority: {forbidden}")

    if decl_access.count("fn begin_body(") != 1:
        raise AssertionError("BODY0 must have one compiler begin_body terminal")
    if joined.count("fn drive_root_body(") != 1:
        raise AssertionError("BODY0 must have one paired drive_root_body terminal")
    if joined.count("fn begin_raw_root_function_v1(") != 1:
        raise AssertionError("BODY-RETURN must have one root skeleton producer")
    if joined.count("fn prepare_raw_root_exit_v1(") != 1:
        raise AssertionError("BODY-RETURN must have one exit prepare producer")
    if joined.count("fn commit_raw_root_exit_v1(") != 1:
        raise AssertionError("BODY-RETURN must have one exit commit producer")
    if "finish_raw_root_function_v1" in joined:
        raise AssertionError("legacy split root finalizer must be absent")
    if "ScriptLastValueOrVoid" in joined:
        raise AssertionError("legacy Script last-ValueId policy must be absent")
    if "seal_root_body_preserving" in (
        ROOT / "src/mir/builder/raw_root_environment_install/body_transaction.rs"
    ).read_text():
        raise AssertionError("active BODY path must use prepared completion commit")

    for path in (ROOT / "docs/development/current/main/CURRENT_STATE.toml", TASK, *SOURCE):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")

    print(
        "[cut0-i0-root0-raw-source0-lower-root-body0-s0-guard] ok "
        "one_entry=1 one_paired_terminal=1 recipe=1 no_ast_rescan=1 "
        "unpublished_root=1 production_consumer=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
