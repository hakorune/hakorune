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
CANONICAL_PUBLICATION = (
    ROOT / "src/mir/compiler/canonical_core_dispatch/publication.rs"
)
CANONICAL_DISPATCH = ROOT / "src/mir/compiler/canonical_core_dispatch.rs"
CANONICAL_CALLABLE_DISPATCH = (
    ROOT / "src/mir/compiler/canonical_core_dispatch/callable.rs"
)
NORMAL_MAIN_TX = (
    ROOT / "src/mir/builder/normal_module_transaction/main_transaction.rs"
)


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
    canonical_publication = CANONICAL_PUBLICATION.read_text()
    canonical_dispatch = CANONICAL_DISPATCH.read_text()
    canonical_callable_dispatch = CANONICAL_CALLABLE_DISPATCH.read_text()
    normal_main_tx = NORMAL_MAIN_TX.read_text()
    normal_main_tx_production = normal_main_tx

    for fragment in (
        "S3-EXECUTION0",
        "execute_function_with_args(module, \"main\", &[])",
        "ScriptUnitValue",
        "ProcessExitProjectionV1",
        "VMError",
    ):
        require(task, fragment, f"task contract {fragment}")
    for fragment in (
        "prepare_neutral_vm_reference",
        ".execute()",
        "complete_canonical_source_entry()",
        "RejectedRawPublishedVmAdapterV1",
    ):
        require(execution, fragment, f"execution owner {fragment}")
    require(publication, "fn execute_exact_vm_entry(", "consuming publication terminal")
    require(reference, "enum VmReferencePublishedOwnerV1", "published owner family")
    require(reference, "Raw(RawPublishedInvocationV1)", "retained Raw published owner")

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
    for fragment in (
        "trait VmReferenceExecutablePublishedOwnerV1",
        "struct CompletedVmReferenceSourceEntryInvocationV1<O>",
        "fn execute(self)",
        "decode_vm_value",
        "vm_error_to_source_fault",
        "ProcessExitProjectionV1::project_canonical",
        "from_published_vm_reference",
    ):
        require(vm_invocation_production, fragment, f"sole neutral VM executor {fragment}")
    for forbidden in (
        "NYASH_ENTRY",
        "execute_module",
        "module.functions",
        "as_integer(",
        "as_bool(",
    ):
        if forbidden in vm_invocation_production:
            raise AssertionError(f"neutral VM executor gained discovery/coercion: {forbidden}")

    external_rust = "\n".join(
        path.read_text()
        for path in (ROOT / "src").rglob("*.rs")
        if path not in (
            PUBLISHED,
            VM_INVOCATION,
            RAW_ADAPTER,
            CANONICAL_PUBLICATION,
            CANONICAL_DISPATCH,
        )
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
        "MirInterpreter",
        "ProcessExitProjection",
        "decode_vm_value",
        "vm_error_to_source_fault",
        "execute_module",
        "NYASH_ENTRY",
        "into_compatibility_module",
        "fallback",
        "retry",
    ):
        if forbidden in raw_adapter_production:
            raise AssertionError(f"Raw neutral adapter gained runtime policy: {forbidden}")
    for retired in (
        "PreparedRawVmReferenceActivationV1",
        "CompletedRawVmReferenceExecutionV1",
        "prepare_vm_reference_activation",
        "from_raw_vm_reference",
        "VmReferenceProjectedOwnerV1::Raw",
    ):
        combined = execution_production + vm_invocation_production + reference
        if retired in combined:
            raise AssertionError(f"old Raw-direct VM authority remains: {retired}")

    for fragment in (
        "struct PreparedCanonicalSourceEntryPublicationV1",
        "struct PublishedCanonicalSourceEntryOwnerV1",
        "enum PublishedCanonicalFamilyEvidenceV1",
        "CanonicalPublishedSourceEntryMembershipV1::Main",
        "CanonicalPublishedSourceEntryMembershipV1::Script",
        "CanonicalPublishedSourceEntryMembershipV1::Callable",
        "fn commit(self) -> PublishedCanonicalSourceEntryInvocationV1",
    ):
        require(canonical_publication, fragment, f"shared canonical publication {fragment}")
    for forbidden in (
        "ASTNode",
        "MirInstruction::Return",
        "NYASH_ENTRY",
        "execute_module",
        "module.functions.get",
        "fallback",
        "retry",
    ):
        if forbidden in canonical_publication:
            raise AssertionError(
                f"canonical publication gained re-inference/fallback policy: {forbidden}"
            )
    for fragment in (
        "VmReferencePublishedOwnerV1::Canonical",
        "impl VmReferenceExecutablePublishedOwnerV1 for PublishedCanonicalSourceEntryOwnerV1",
        "execute_function_with_args(self.module(), symbol, &[])",
    ):
        require(reference, fragment, f"sole canonical VM owner {fragment}")
    for fragment in (
        "const CANONICAL_CORE_SINGLE_FILE_UNIT_ORDINAL: u32 = 0;",
        "RejectedCanonicalCallableDispatchV1",
        "prepare_normal_helper_draft_prefix_v1",
        "prepare_normal_callable_main_physical_v1",
        "prepare_normal_callable_commit_v1",
    ):
        require(canonical_callable_dispatch, fragment, f"sole callable dispatch {fragment}")
    for forbidden in ("compile_with_source", "fallback", "retry", "RawPublished"):
        if forbidden in canonical_callable_dispatch:
            raise AssertionError(f"callable dispatch gained forbidden authority: {forbidden}")
    if "FamilyCapabilityPending(\n                        CanonicalCorePendingFamilyV1::CallableModule" in canonical_dispatch:
        raise AssertionError("canonical CallableModule must not remain a pending dispatch family")
    for retired in (
        "PublishedNormalMainInvocationV1",
        "CanonicalMain(",
        "source_entry_vm_normal_main_adapter",
    ):
        combined = reference + canonical_publication + normal_main_tx_production
        if retired in combined:
            raise AssertionError(f"retired Main-specific publication owner remains: {retired}")

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
        CANONICAL_PUBLICATION,
        CANONICAL_DISPATCH,
        CANONICAL_CALLABLE_DISPATCH,
        NORMAL_MAIN_TX,
        Path(__file__),
    ):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[entry-result-projection0-s3-execution-guard] ok "
        "exact_target=1 decode_plan=1 source_fault=1 owner_retained=1 "
        "neutral_owner=1 sole_vm_executor=1 raw_adapter=1 "
        "canonical_publication=1 old_raw_direct=0 neutral_discovery=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
