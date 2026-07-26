#!/usr/bin/env python3
"""S3-EXECUTION0 structural guard for the exact Raw VM lane."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "entry-result-projection0-s3-raw-vm-activation-execution-task-2026-07-25.md"
)
EXEC = ROOT / "src/mir/compiler/source_entry_vm_execution.rs"
PUB = ROOT / "src/mir/compiler/raw_root_publication.rs"
REF = ROOT / "src/mir/compiler/source_entry_vm_reference.rs"
NEUTRAL_TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "source-entry-vmref-neutral0-l0-execution-task-2026-07-26.md"
)
PUBLISHED = ROOT / "src/mir/compiler/source_entry_published_invocation.rs"
VM_INVOCATION = ROOT / "src/mir/compiler/source_entry_vm_invocation.rs"
RAW_ADAPTER = ROOT / "src/mir/compiler/source_entry_vm_raw_adapter.rs"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    task = TASK.read_text()
    execution = EXEC.read_text()
    execution_production = execution.split("#[cfg(test)]", 1)[0]
    publication = PUB.read_text()
    reference = REF.read_text()
    neutral_task = NEUTRAL_TASK.read_text()
    published = PUBLISHED.read_text()
    published_production = published.split("#[cfg(test)]", 1)[0]
    vm_invocation = VM_INVOCATION.read_text()
    vm_invocation_production = vm_invocation.split("#[cfg(test)]", 1)[0]
    raw_adapter = RAW_ADAPTER.read_text()
    raw_adapter_production = raw_adapter.split("#[cfg(test)]", 1)[0]

    for fragment in (
        "S3-EXECUTION0",
        "execute_function_with_args(module, \"main\", &[])",
        "ScriptUnitValue",
        "ProcessExitProjectionV1",
        "VMError",
    ):
        require(task, fragment, f"task contract {fragment}")
    for fragment in (
        "prepare_vm_reference_activation",
        "execute_exact_vm_entry",
        "VmSourceEntryDecodePlanV1",
        "ProcessExitProjectionV1::project_borrowed",
        "vm_error_to_source_fault",
        "source_result: SourceEntryResultV1",
    ):
        require(execution, fragment, f"execution owner {fragment}")
    require(publication, "fn execute_exact_vm_entry(", "consuming publication terminal")
    require(reference, "published: RawPublishedInvocationV1", "retained published owner")

    for fragment in (
        "SOURCE-ENTRY-VMREF-NEUTRAL0-L0",
        "PublishedSourceEntryInvocationV1",
        "PreparedVmReferenceSourceEntryInvocationV1",
        "L0 production producer                             = 0",
        "L0 production consumer                             = 0",
    ):
        require(neutral_task, fragment, f"neutral L0 contract {fragment}")
    for fragment in (
        "struct PublishedSourceEntryInvocationV1<O>",
        "struct VerifiedPublishedSourceEntryTargetV1",
        "enum PublishedSourceEntryResultContractV1",
        "enum PublishedSourceEntryMembershipV1",
        "enum PublishedUnitPhysicalContractV1",
        "struct RejectedPublishedSourceEntryTargetV1",
    ):
        require(published, fragment, f"neutral published owner {fragment}")
    for fragment in (
        "struct PreparedVmReferenceSourceEntryInvocationV1<O>",
        "fn prepare_vm_reference(",
        "VmSourceEntryDecodePlanV1::Unit",
        "PublishedUnitPhysicalContractV1::ExactVoid",
    ):
        require(vm_invocation, fragment, f"passive VM projection {fragment}")

    for forbidden in (
        "VMValue",
        "MirInterpreter",
        "ProcessExitProjection",
        "ProcessExitCode",
        "Diagnostic",
        "NYASH_ENTRY",
        "execute_module",
        "module.functions",
    ):
        if forbidden in published_production:
            raise AssertionError(
                f"backend-neutral published owner gained runtime policy: {forbidden}"
            )
    for forbidden in (
        "VMValue",
        "MirInterpreter",
        "ProcessExitProjection",
        "ProcessExitCode",
        "Diagnostic",
        "NYASH_ENTRY",
        "execute_module",
        "module.functions",
    ):
        if forbidden in vm_invocation_production:
            raise AssertionError(
                f"passive VM projection gained execution/process policy: {forbidden}"
            )

    external_rust = "\n".join(
        path.read_text()
        for path in (ROOT / "src").rglob("*.rs")
        if path not in (PUBLISHED, VM_INVOCATION, RAW_ADAPTER)
    )
    for fragment in (
        "PublishedSourceEntryInvocationV1::from_verified_parts(",
        ".prepare_vm_reference()",
    ):
        if fragment in external_rust:
            raise AssertionError(
                f"neutral L0 gained a production/test consumer outside its fixture: {fragment}"
            )
    for fragment in (
        "fn prepare_neutral_vm_reference(",
        "PublishedSourceEntryInvocationV1::from_verified_parts(",
        ".prepare_vm_reference()",
        "self.invocation_brand() != self.selected_entry().brand()",
        "self.main_entry_target_matches()",
        "DecodeRoundTripMismatch",
    ):
        require(raw_adapter_production, fragment, f"Raw neutral adapter {fragment}")
    for forbidden in (
        "VMValue",
        "MirInterpreter",
        "ProcessExitProjection",
        "execute_module",
        "NYASH_ENTRY",
        "into_compatibility_module",
        "fallback",
        "retry",
    ):
        if forbidden in raw_adapter_production:
            raise AssertionError(f"Raw neutral adapter gained runtime policy: {forbidden}")

    for forbidden in (
        "execute_module(",
        "NYASH_ENTRY",
        "to_nyash_box(",
        "as_integer(",
        "as_bool(",
    ):
        if forbidden in execution_production:
            raise AssertionError(f"S3 execution must not use legacy discovery/coercion: {forbidden}")

    for path in (
        TASK,
        EXEC,
        PUB,
        REF,
        NEUTRAL_TASK,
        PUBLISHED,
        VM_INVOCATION,
        RAW_ADAPTER,
        Path(__file__),
    ):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[entry-result-projection0-s3-execution-guard] ok "
        "exact_target=1 decode_plan=1 source_fault=1 owner_retained=1 "
        "neutral_owner=1 passive_vm_projection=1 raw_adapter=1 "
        "neutral_runtime_policy=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
