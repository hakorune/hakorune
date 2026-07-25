#!/usr/bin/env python3
"""S3-OWNER0 guard for the shared Raw compile owner."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "entry-result-projection0-s3-raw-vm-activation-execution-task-2026-07-25.md"
)
PROFILE_TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "post-s3-clean-retire-and-normal-entry-canary-task-map-2026-07-25.md"
)
KERNEL = ROOT / "src/mir/compiler/raw_published_compile.rs"
INGRESS = ROOT / "src/mir/compiler/raw_public_ingress.rs"
EXEC = ROOT / "src/mir/compiler/source_entry_vm_execution.rs"
PROFILE = ROOT / "src/runner/reference/raw_vm_reference_request.rs"
CANARY = ROOT / "src/runner/reference/raw_vm_reference.rs"
PARITY_PROOF = ROOT / "tools/checks/lib/entry_result_projection0_s3_canary_parity.py"
CLI = ROOT / "src/cli/mod.rs"
CLI_ARGS = ROOT / "src/cli/args.rs"
DISPATCH = ROOT / "src/runner/dispatch.rs"
FROZEN_ROUTES = (
    ROOT / "src/runner/core_executor.rs",
    ROOT / "src/runner/mir_json_v0.rs",
    ROOT / "src/runner/mir_json_emit/mod.rs",
    ROOT / "src/runtime/mirbuilder_emit.rs",
    ROOT / "src/runner/route_orchestrator.rs",
    ROOT / "src/runner/dispatch.rs",
    ROOT / "src/runner/product/llvm/mir_compiler.rs",
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    task = TASK.read_text()
    profile_task = PROFILE_TASK.read_text()
    kernel = KERNEL.read_text()
    ingress = INGRESS.read_text()
    execution = EXEC.read_text()
    profile = PROFILE.read_text()
    canary = CANARY.read_text()
    parity_proof = PARITY_PROOF.read_text()
    cli = CLI.read_text()
    cli_args = CLI_ARGS.read_text()
    dispatch = DISPATCH.read_text()
    execution_production = execution.split("#[cfg(test)]", 1)[0]

    require(task, "S3-OWNER0", "task contract")
    require(task, "compile_raw_published_v1", "typed compile kernel contract")
    for fragment in (
        "RejectedRawPublishedCompileV1",
        "pub(in crate::mir) fn compile_raw_published_v1(",
        "fn discard(self)",
        "fn into_public_string(self)",
    ):
        require(kernel, fragment, f"owner kernel {fragment}")
    require(ingress, ".compile_raw_published_v1(", "compatibility ingress consumer")
    require(execution, ".compile_raw_published_v1(", "VM-reference ingress consumer")
    require(execution, "pub(crate) fn run_raw_vm_reference(", "explicit VM-reference entry")
    require(parity_proof, "entry-result-projection0-s3-canary-parity", "real-binary parity proof")
    require(cli_args, '.default_value("mir")', "default backend remains mir")
    if "raw-vm-reference" in dispatch:
        raise AssertionError("general dispatch must not own the Raw VM-reference canary")
    for frozen in FROZEN_ROUTES:
        if "RawVmReference" in frozen.read_text() or "raw-vm-reference" in frozen.read_text():
            raise AssertionError(f"frozen route widened into Raw VM-reference: {frozen}")
    require(profile_task, "NORMAL-ENTRY-PROFILE0-S0", "passive profile task")
    for fragment in (
        "RawVmReferenceProductionRequestV1",
        "select_from_cli",
        "RawVmReferenceGrammarV1::Canonical",
        "RawVmReferenceSourceProfileV1::NarrowV1",
        "RawVmReferenceImportProfileV1::None",
        "RawVmReferenceCallableMainProfileV1::Omitted",
        "RawVmReferenceBackendV1::VmReference",
        "RawVmReferenceProcessProfileV1::CanonicalV1",
    ):
        require(profile, fragment, f"passive profile {fragment}")
    for fragment in (
        "pub macro_preexpand: bool",
        "pub macro_preexpand_auto: bool",
        "pub macro_top_level_allow: bool",
        "pub macro_profile: Option<String>",
        "pub script_args: Vec<String>",
    ):
        require(cli, fragment, f"CLI fact retention {fragment}")
        require(cli_args, fragment.split(":", 1)[0].replace("pub ", ""), f"CLI parser fact {fragment}")

    profile_callers = []
    for path in (ROOT / "src/runner").rglob("*.rs"):
        if path in (PROFILE, CANARY):
            continue
        if "RawVmReferenceProductionRequestV1" in path.read_text():
            profile_callers.append(path.relative_to(ROOT))
    if profile_callers:
        raise AssertionError(f"profile has duplicate production callers: {profile_callers}")
    require(canary, "RawVmReferenceProductionRequestV1", "canary profile consumer")
    require(canary, "select_from_cli", "canary selector consumer")
    require(canary, "pub(crate) fn select_and_run(", "canary runner entry")
    require(canary, "read_to_string(source_file)", "canary source read")
    require(canary, "parse_from_string_with_build_config", "canary canonical parse")
    runner = (ROOT / "src/runner/mod.rs").read_text()
    require(runner, "reference::raw_vm_reference::select_and_run", "early canary selector")
    if runner.index("reference::raw_vm_reference::select_and_run") > runner.index(
        "// Early: macro child"
    ):
        raise AssertionError("canary selector must precede compatibility runner effects")

    if ingress.count("compile_raw_published_v1(") != 1:
        raise AssertionError("compatibility ingress must have one typed-kernel consumer")
    if execution.count("compile_raw_published_v1(") != 1:
        raise AssertionError("VM-reference ingress must have one typed-kernel consumer")
    if canary.count("select_from_cli") != 1:
        raise AssertionError("canary must have one profile selector consumer")
    if canary.count("run_raw_vm_reference(") != 1:
        raise AssertionError("canary must have one VM-reference report consumer")
    if canary.count("read_to_string(source_file)") != 1:
        raise AssertionError("canary must have one source-file read")
    if canary.count("parse_from_string_with_build_config") != 1:
        raise AssertionError("canary must have one canonical parse")
    if canary.count("std::process::exit") != 1:
        raise AssertionError("canary must have one process terminal")
    if "Program(RawVmReferenceRunReportV1)" not in canary or "Program {" in canary:
        raise AssertionError("canary must retain the Raw process report until its terminal")
    for forbidden in (
        "compile_raw_with_source",
        "compile_with_source",
        "prepare_source_with_imports",
        "prepare_source_minimal",
        "NYASH_ENTRY",
        "fallback",
    ):
        if forbidden in canary:
            raise AssertionError(f"canary must not own legacy/fallback behavior: {forbidden}")
    runner_raw_callers = []
    for path in (ROOT / "src/runner").rglob("*.rs"):
        if path == CANARY:
            continue
        if "compile_raw_with_source" in path.read_text() or "run_raw_vm_reference" in path.read_text():
            runner_raw_callers.append(path.relative_to(ROOT))
    if runner_raw_callers:
        raise AssertionError(f"runner has unexpected Raw production callers: {runner_raw_callers}")
    if "run_raw_vm_reference_status(" in execution or "run_raw_vm_reference_status(" in canary:
        raise AssertionError("untyped VM-reference status adapter must remain absent")
    if execution.count("pub(crate) fn run_raw_vm_reference(") != 1:
        raise AssertionError("VM-reference production entry must be unique")

    for forbidden in (
        "bind_raw_source_for_public(",
        "prepare_public_eligibility(",
        "execute_module(",
        "run_vm_compiled_module(",
        "NYASH_ENTRY",
        "build_module(",
    ):
        if forbidden in ingress or forbidden in execution_production:
            raise AssertionError(f"new ingress must not duplicate/discover legacy work: {forbidden}")

    for path in (
        TASK,
        PROFILE_TASK,
        KERNEL,
        INGRESS,
        EXEC,
        PROFILE,
        CANARY,
        ROOT / "src/runner/mod.rs",
        CLI,
        CLI_ARGS,
        DISPATCH,
        Path(__file__),
        PARITY_PROOF,
    ):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[entry-result-projection0-s3-owner-guard] ok "
        "typed_kernel=1 profile=1 canary=1 duplicate_profile_callers=0 "
        "legacy_duplication=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
