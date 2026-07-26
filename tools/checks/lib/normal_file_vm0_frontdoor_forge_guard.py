#!/usr/bin/env python3
"""NormalFile VM-reference route guard from Forge0 through one explicit caller."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "normal-file-vm0-frontdoor-forge-task-2026-07-26.md"
)
FRONTDOOR = ROOT / "src/runner/reference/normal_file_vm_frontdoor.rs"
NORMAL_REQUEST = ROOT / "src/runner/reference/normal_file_vm_request.rs"
CANONICAL_CORE_REQUEST = (
    ROOT / "src/runner/reference/normal_file_canonical_core_request.rs"
)
NORMAL_REPORT_RUNNER = ROOT / "src/runner/reference/normal_file_vm.rs"
PARITY_P0A = ROOT / "src/runner/reference/normal_file_vm/parity_p0a.rs"
TEST_ONLY_RAW_TERMINAL_CONSUMERS = (
    ROOT / "src/runner/reference/normal_file_vm_frontdoor/result_carrier_p0.rs",
)
SOURCE_PLAN_PROOF_CONSUMERS = (
    ROOT
    / "src/runner/reference/normal_file_vm_frontdoor/source_plan_input_tests.rs",
)
CANONICAL_CORE_DISPATCH = ROOT / "src/mir/compiler/canonical_core_dispatch.rs"
CANONICAL_CORE_DISPATCH_CARD = ROOT / (
    "docs/development/current/main/investigations/"
    "normal-file-canonical-core0-dispatch-series-execution-task-2026-07-26.md"
)
REFERENCE_MOD = ROOT / "src/runner/reference/mod.rs"
RAW_CONTRACT = ROOT / "src/mir/raw_vm_reference_contract.rs"
RUNNER = ROOT / "src/runner/mod.rs"
FROZEN_ROUTES = (
    ROOT / "src/runner/dispatch.rs",
    ROOT / "src/runner/route_orchestrator.rs",
    ROOT / "src/runner/core_executor.rs",
    ROOT / "src/runner/mir_json_v0.rs",
    ROOT / "src/runtime/mirbuilder_emit.rs",
)
S3_GUARDS = (
    ROOT / "tools/checks/lib/entry_result_projection0_s3_owner_guard.py",
    ROOT / "tools/checks/lib/entry_result_projection0_s3_execution_guard.py",
    ROOT / "tools/checks/lib/entry_result_projection0_s3_entry_carry_guard.py",
    ROOT / "tools/checks/lib/raw_vm_reference_conformance.py",
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    task = TASK.read_text()
    frontdoor = FRONTDOOR.read_text()
    production = frontdoor.split("#[cfg(test)]", 1)[0]
    normal_request = NORMAL_REQUEST.read_text()
    normal_report_runner = NORMAL_REPORT_RUNNER.read_text()
    parity_p0a = PARITY_P0A.read_text()
    reference_mod = REFERENCE_MOD.read_text()
    raw_contract = RAW_CONTRACT.read_text()
    frontdoor_input = (
        ROOT / "src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs"
    ).read_text()
    canonical_dispatch = CANONICAL_CORE_DISPATCH.read_text()
    canonical_dispatch_card = CANONICAL_CORE_DISPATCH_CARD.read_text()

    for fragment in (
        "NORMAL-FILE-VM0-FRONTDOOR-FORGE0-S0",
        "one UTF-8 file read",
        "one canonical parse",
        "fallback / retry         = zero",
        "< 800 lines",
    ):
        require(task, fragment, f"Forge0 task contract {fragment}")
    for fragment in (
        "NormalFileVmFrontDoorV1",
        "NormalEntryProfileV1",
        "FileNoImportVmReferenceV1",
        "FileCanonicalCoreVmReferenceV1",
        "SealedNormalEntryProfileV1",
        "NormalFileRequestV1",
        "PreparedNormalFileRequestV1",
        "LoadedNormalFileSourceV1",
        "PreparedNormalFileSourceV1",
        "NormalFileSourceReceiptV1",
        "PreparedNormalFileVmHandoffV1",
        "RejectedNormalFileSourceV1",
        "RejectedNormalFileVmHandoffV1",
        "ProfileExcludesRawVmReference",
        "file_canonical_core_request",
        "fn prepare_raw_vm_handoff(",
        "fn into_raw_vm_reference_invocation(self)",
        "std::fs::read_to_string(&source_file)",
        "parse_from_string_with_build_config",
        "GrammarProfile::Canonical",
        "find_no_import_violation",
        "ASTNode::UsingStatement",
        "ASTNode::ImportStatement",
    ):
        require(production, fragment, f"front-door owner {fragment}")
    for fragment in (
        "RawVmReferenceSupportProfileV1",
        "fn into_invocation(",
    ):
        require(raw_contract, fragment, f"downstream Raw contract {fragment}")
    require(
        reference_mod,
        "pub(crate) mod normal_file_vm_frontdoor;",
        "disconnected front-door module declaration",
    )
    for path in TEST_ONLY_RAW_TERMINAL_CONSUMERS:
        text = path.read_text()
        for fragment in (
            "//! RESULT-CARRIER-NORMAL-CAPABILITY0 S2 source-text evidence.",
            "prepare_raw_vm_handoff()",
            "into_raw_vm_reference_invocation()",
            "run_raw_vm_reference_v1",
            "front_door_rejections_leave_the_compiler_reusable",
            "canonical_process_and_vm_faults_leave_the_compiler_reusable",
        ):
            require(text, fragment, f"test-only Forge evidence {fragment}")
    for fragment in (
        "NORMAL-FILE-CANONICAL-CORE0-DISPATCH0-S0",
        "runner family match             = 0",
        "compiler -> runner import       = 0",
        "Script / CallableModule",
    ):
        require(canonical_dispatch_card, fragment, f"canonical dispatch contract {fragment}")
    for fragment in (
        "into_canonical_core_compile_request",
        "CanonicalCoreSourcePlanCompileRequestV1",
        "VerifiedCanonicalCoreSourcePlanAdmissionV1",
    ):
        require(frontdoor_input, fragment, f"canonical front-door handoff {fragment}")
    for forbidden in (
        "SealedNormalScalarRootV1::",
        "SealedNormalSourcePlanV1::CallableModule",
        "compile_raw_with_source",
        "compile_with_source",
        "build_module",
        "fallback",
        "retry",
    ):
        if forbidden in frontdoor_input:
            raise AssertionError(f"front door must not select source family or fallback: {forbidden}")
    for fragment in (
        "CanonicalCoreSourcePlanCompileRequestV1",
        "NormalCanonicalCoreSourcePlanCompilerV1",
        "CompletedCanonicalCoreSourceEntryCandidateV1",
        "compile_canonical_core_source_plan",
        "compile_main0",
        "CompletedCanonicalCoreSourceEntryFamilyV1::Callable",
    ):
        require(canonical_dispatch, fragment, f"canonical compiler dispatch {fragment}")
    if canonical_dispatch.count("match plan") != 1:
        raise AssertionError("canonical-core source family must have one compiler-layer match")
    if (
        canonical_dispatch.count("seal_from_frontdoor_profile") != 1
        or frontdoor_input.count("seal_from_frontdoor_profile") != 1
    ):
        raise AssertionError("canonical-core admission must have one front-door producer")
    if frontdoor_input.count("CanonicalCoreSourcePlanCompileRequestV1::new") != 1:
        raise AssertionError("canonical-core request must have one front-door producer")
    for forbidden in (
        "crate::runner",
        "RawPublishedCompileRequestV1",
        "RawPublishedInvocationV1",
        "RawVmReferenceInvocationV1",
        "NYASH_ENTRY",
        "execute_module(",
        "ProcessExitProjectionV1",
        "fallback",
        "retry",
    ):
        if forbidden in canonical_dispatch:
            raise AssertionError(f"canonical dispatch must stay unpublished and Raw-free: {forbidden}")
    for fragment in (
        "NormalFileVmReferenceProductionRequestV1",
        "NormalFileNoImportVmReferenceV1",
        "into_frontdoor_request(self)",
        "NonDefaultOptimizationRequested",
    ):
        require(normal_request, fragment, f"unconnected normal request {fragment}")
    canonical_core_request = CANONICAL_CORE_REQUEST.read_text()
    for fragment in (
        "NormalFileCanonicalCoreVmReferenceProductionRequestV1",
        "normal-file-canonical-core-vm-reference",
        "file_canonical_core_request",
        "into_frontdoor_request(self)",
        "NonDefaultOptimizationRequested",
    ):
        require(canonical_core_request, fragment, f"unconnected canonical-core request {fragment}")
    if ".prepare()" in canonical_core_request or "select_from_cli" in canonical_core_request:
        raise AssertionError("REQUEST0 must not select or execute the canonical-core front door")
    if ".prepare()" in normal_request or "run_raw_vm_reference" in normal_request:
        raise AssertionError("REQUEST0 must not execute the NormalFile front door")
    for fragment in (
        "NormalFileVmReferenceProductionRequestV1",
        "into_frontdoor_request().prepare()",
        "run_raw_vm_reference_for_runner_v1",
        "ReferenceRunOutcomeV1",
    ):
        require(normal_report_runner, fragment, f"REPORT0 bounded runner {fragment}")
    for forbidden in (
        "select_from_cli",
        "std::process::exit",
        "compile_raw_with_source",
        "compile_with_source",
        "compile_legacy",
        "build_module",
        "fallback",
        "retry",
    ):
        if forbidden in normal_report_runner:
            raise AssertionError(f"REPORT0 must not select or widen the normal route: {forbidden}")
    for fragment in (
        "NORMAL-FILE-VM0 PARITY0-P0a",
        "normal_program_projection_matches_raw_in_the_common_scalar_unit_subset",
        "normal_run_preserves_usage_invocation_and_program_boundaries_without_retry",
        "raw_vm_reference::run(request)",
        "run(request)",
    ):
        require(parity_p0a, fragment, f"pre-caller parity evidence {fragment}")
    if "std::process::exit" in parity_p0a or "select_from_cli" in parity_p0a:
        raise AssertionError("PARITY0-P0a must exercise owners directly, not select or terminate")

    if production.count("std::fs::read_to_string(&source_file)") != 1:
        raise AssertionError("Forge0 must read its source file exactly once")
    if production.count("parse_from_string_with_build_config") != 1:
        raise AssertionError("Forge0 must invoke the canonical parser exactly once")
    for forbidden in (
        "RawPublishedCompileProfileV1::narrow_v1",
        "RawVmReferenceSourceProfileV1",
        "RawVmReferenceImportProfileV1",
        "RawVmReferenceCallableMainProfileV1",
        "RawVmReferenceExecutionProfileV1::CanonicalV1",
        "compile_raw_with_source",
        "compile_with_source",
        "compile_legacy",
        "build_module",
        "run_raw_vm_reference_v1",
        "run_raw_vm_reference(",
        "execute_module(",
        "NYASH_ENTRY",
        "ProcessExitProjectionV1",
        "std::process::exit",
        "prepare_source_with_imports",
        "prepare_source_minimal",
        "fallback",
        "retry",
        "preexpand",
        "strip_",
        "merge",
    ):
        if forbidden in production:
            raise AssertionError(f"Forge0 must not own alternate policy/work: {forbidden}")

    for path in FROZEN_ROUTES:
        text = path.read_text()
        for token in (
            "NormalFileVmFrontDoorV1",
            "NormalFileRequestV1",
            "PreparedNormalFileVmHandoffV1",
        ):
            if token in text:
                raise AssertionError(f"frozen route widened into Forge0: {path.relative_to(ROOT)}")
    for path in (ROOT / "src/runner").rglob("*.rs"):
        if (
            path in (FRONTDOOR, NORMAL_REQUEST, CANONICAL_CORE_REQUEST, NORMAL_REPORT_RUNNER)
            or path in TEST_ONLY_RAW_TERMINAL_CONSUMERS
            or path in SOURCE_PLAN_PROOF_CONSUMERS
        ):
            continue
        text = path.read_text()
        if "NormalFileVmFrontDoorV1" in text or "PreparedNormalFileVmHandoffV1" in text:
            raise AssertionError(
                "front-door type escaped its one production owner: "
                f"{path.relative_to(ROOT)}"
            )
    for fragment in (
        "pub(crate) fn select_and_run(config: &CliConfig)",
        "ExplicitReferenceRunnerRequestV1::RawVmReference(request)",
        "ExplicitReferenceRunnerRequestV1::NormalFileVmReference(request)",
        "Some(raw_vm_reference::run(request))",
        "Some(normal_file_vm::run(request))",
    ):
        require(reference_mod, fragment, f"one central explicit selector {fragment}")
    runner = RUNNER.read_text()
    require(runner, "reference::select_and_run(&self.config)", "one production route caller")
    if runner.count("reference::select_and_run(&self.config)") != 1:
        raise AssertionError("normal-file VM route must have exactly one production caller")
    if "normal_file_vm_frontdoor" in runner:
        raise AssertionError("default runner must not reach through the typed front door")

    for path in (
        TASK,
        FRONTDOOR,
        NORMAL_REQUEST,
        CANONICAL_CORE_REQUEST,
        NORMAL_REPORT_RUNNER,
        PARITY_P0A,
        *TEST_ONLY_RAW_TERMINAL_CONSUMERS,
        *SOURCE_PLAN_PROOF_CONSUMERS,
        RAW_CONTRACT,
        CANONICAL_CORE_DISPATCH,
        CANONICAL_CORE_DISPATCH_CARD,
        Path(__file__),
        *S3_GUARDS,
    ):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path.relative_to(ROOT)}")
    print(
        "[normal-file-vm0-frontdoor-forge-guard] ok "
        "profile=1 read=1 parse=1 handoff=1 central_selector=1 normal_caller=1 "
        "default_delta=0 alternate_policy=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
