"""NORMAL-SCRIPT0 shared terminal-kernel guard.

This child keeps Script's source-owned recipe and the builder-only terminal
vocabulary together without allowing either the normal source-plan parent or
the Raw lifecycle to become a second Script result authority.
"""

from pathlib import Path


def _require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def _require_count(text: str, fragment: str, expected: int, label: str) -> None:
    actual = text.count(fragment)
    if actual != expected:
        raise AssertionError(
            f"{label}: expected {expected} occurrences of {fragment!r}, got {actual}"
        )


def check_script_physical(root: Path) -> tuple[Path, ...]:
    """Validate the bounded SCRIPT-TERMINAL-KERNEL0 authority slice."""

    source_plan = root / "src/mir/compiler/normal_source_plan"
    builder = root / "src/mir/builder"
    recipe_handoff = source_plan / "script_recipe.rs"
    terminal_dir = builder / "script_physical_exit"
    terminal = terminal_dir / "terminal.rs"
    exit_kernel = terminal_dir / "exit.rs"
    terminal_mod = terminal_dir / "mod.rs"
    terminal_tests = terminal_dir / "tests.rs"
    lowering = builder / "raw_root_body_lowering.rs"
    raw_adapter = builder / "raw_root_environment_install/body_transaction.rs"

    handoff = recipe_handoff.read_text()
    terminal_text = terminal.read_text()
    exit_text = exit_kernel.read_text()
    terminal_module = terminal_mod.read_text()
    terminal_test_text = terminal_tests.read_text()
    lowering_text = lowering.read_text()
    raw_adapter_text = raw_adapter.read_text()

    _require(handoff, "source: RetainedNormalScriptSourceV1", "opaque Script source retention")
    _require(handoff, "recipe: RawScriptBodyRecipeV1", "exact Script recipe")
    _require(handoff, "fn into_recipe(self) -> RawScriptBodyRecipeV1", "sole Script recipe handoff")
    for forbidden in (
        "pub(in crate::mir) fn ast",
        "into_ast",
        "fn reclassify",
        "fn retry",
    ):
        if forbidden in handoff:
            raise AssertionError(f"Script recipe handoff gained forbidden escape: {forbidden}")

    _require_count(terminal_text, "enum LoweredScriptTerminalV1", 1, "sole terminal vocabulary")
    _require_count(
        terminal_text,
        "enum LoweredScriptUnitPayloadV1",
        1,
        "sole Unit physical payload vocabulary",
    )
    _require_count(
        terminal_text,
        "struct ScriptRecipeLoweringErrorV1",
        1,
        "sole typed Script lowering error",
    )
    for forbidden in ("MirInstruction", "FunctionSignature", "MirType", "ModuleInvocationBrandV1"):
        if forbidden in terminal_text:
            raise AssertionError(f"terminal kernel gained non-terminal authority: {forbidden}")
    _require(terminal_module, "mod terminal;", "terminal module registration")
    _require(terminal_module, "mod exit;", "physical exit module registration")

    _require_count(
        exit_text,
        "struct ScriptPhysicalExitCommitV1",
        1,
        "sole Script physical exit commit owner",
    )
    _require_count(
        exit_text,
        "fn commit_projected(",
        1,
        "sole Script physical Return commit terminal",
    )
    _require_count(
        exit_text,
        "MirInstruction::Return",
        1,
        "sole Script physical Return writer",
    )
    for forbidden in ("ModuleInvocationBrandV1", "RawRootBody", "RawPublished"):
        if forbidden in exit_text:
            raise AssertionError(f"shared Script exit kernel gained Raw authority: {forbidden}")

    _require(
        lowering_text,
        "Result<LoweredScriptTerminalV1, ScriptRecipeLoweringErrorV1>",
        "typed shared Script lowering terminal",
    )
    _require_count(
        lowering_text,
        "LoweredScriptTerminalV1::Unit",
        3,
        "exact Script Unit lowering paths",
    )
    _require(
        raw_adapter_text,
        "legacy_script_terminal_from_root_result",
        "legacy Raw recipe to exact terminal bridge",
    )
    _require(
        raw_adapter_text,
        "RAW-SCRIPT-EXIT-ADAPTER0",
        "legacy bridge retirement marker",
    )
    _require(
        raw_adapter_text,
        "commit_raw_script_exit_v1",
        "Raw lifecycle-only Script exit adapter",
    )
    if "legacy_root_body_result_from_script_terminal" in raw_adapter_text:
        raise AssertionError("Raw Script terminal must not project directly to RootBodyResultV1")
    _require(
        (builder / "raw_root_body_exit.rs").read_text(),
        "PreparedRawScriptCompletionAdapterV1",
        "Raw tracker-only completion adapter",
    )
    for prefix in (
        "script_terminal_kernel_classifies",
        "script_terminal_kernel_preserves",
        "script_terminal_kernel_reports",
        "script_physical_exit_kernel_commits",
        "script_physical_exit_kernel_materializes",
        "script_physical_exit_kernel_rejects",
    ):
        _require(terminal_test_text, prefix, f"terminal fixture {prefix}")
    _require(
        (builder / "raw_root_body_exit.rs").read_text(),
        "raw_script_exit_adapter_preserves",
        "Raw decode-origin adapter fixture",
    )

    files = (
        recipe_handoff,
        terminal_mod,
        terminal,
        exit_kernel,
        terminal_tests,
        lowering,
        raw_adapter,
    )
    for path in files:
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path.relative_to(root)}")
    return files
