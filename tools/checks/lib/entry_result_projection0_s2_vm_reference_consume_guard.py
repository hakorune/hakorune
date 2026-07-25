#!/usr/bin/env python3
"""S2 one-shot VM-reference carrier guard."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "entry-result-projection0-s2-vm-reference-consume-execution-task-2026-07-25.md"
)
RESULT = ROOT / "src/mir/compiler/source_entry_result.rs"
ADAPTER = ROOT / "src/mir/compiler/source_entry_vm_reference.rs"
TESTS = ROOT / "src/mir/compiler/source_entry_vm_reference_p0.rs"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def production_consumer_count() -> int:
    count = 0
    for path in (ROOT / "src").rglob("*.rs"):
        if path == ADAPTER or path.name.endswith("_p0.rs"):
            continue
        count += path.read_text().count(".consume_vm_reference()")
    return count


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    result = RESULT.read_text()
    adapter = ADAPTER.read_text()
    tests = TESTS.read_text()

    if not any(
        row in state
        for row in (
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S2-FAULT-STATUS0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S2-VM-REFERENCE-CONSUME0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S3-RUNTIME-ACTIVATION-DESIGN-STOP"',
        )
    ):
        raise AssertionError("missing active or retained S2 execution row")

    for fragment in (
        "S2-FAULT-STATUS0",
        "S2-VM-CARRIER0",
        "S2-P0/G0",
        "production consumer count zero",
        "S3-RUNTIME-ACTIVATION-DESIGN-STOP",
    ):
        require(task, fragment, f"task contract {fragment}")

    for fragment in (
        "Fault {",
        "status: ProcessExitCodeV1",
        "fault: ProcessFaultV1",
        "fn status_code(&self)",
        "fn fault(&self)",
    ):
        require(result, fragment, f"normalized termination {fragment}")
    if "SourceFault {\n        status:" in result:
        raise AssertionError("source fault must not own process-status policy")

    for fragment in (
        "struct VmReferenceProcessOutcomeV1",
        "projected: ProjectedSourceEntryV1",
        "fn consume_vm_reference(self)",
        "fn status(&self)",
        "fn fault(&self)",
        "fn discard(self)",
    ):
        require(adapter, fragment, f"VM-reference carrier {fragment}")
    if adapter.count("pub(in crate::mir) fn consume_vm_reference(self)") != 1:
        raise AssertionError("VM-reference carrier must have one consuming entry")
    if production_consumer_count() != 0:
        raise AssertionError("VM-reference carrier must have zero production consumers")

    for forbidden in (
        "SourceEntryResultV1",
        "PhysicalSourceEntryCarrierV1",
        "ProcessExitProjectionV1",
        "MirModule",
        "MirInterpreter",
        "NyashBox",
        "NYASH_ENTRY",
        "run_vm_compiled_module",
        "execute_mir_module",
        "process::exit",
        "reserved_fault",
        "70",
        "to_string",
    ):
        if forbidden in adapter:
            raise AssertionError(f"adapter must not add authority: {forbidden}")

    for root in (
        ROOT / "src/runner",
        ROOT / "src/backend",
        ROOT / "src/llvm_py",
    ):
        if not root.exists():
            continue
        for path in root.rglob("*.rs"):
            if "ProjectedSourceEntryV1" in path.read_text():
                raise AssertionError(f"production runtime consumed projected owner: {path}")

    for fragment in (
        "Integer(-1 / 256)",
        "Bool / Float / String / Object",
        "source Fault",
        "actual VM execution           = 0",
    ):
        require(task, fragment, f"acceptance matrix {fragment}")
    for fragment in (
        "out_of_range_status_is_reserved_and_keeps_exact_value",
        "unsupported_results_keep_exact_kind_without_success_fallback",
        "source_fault_keeps_code_and_detail_with_reserved_status",
        "route_for_test",
    ):
        require(tests, fragment, f"focused fixture {fragment}")

    for path in (STATE, TASK, RESULT, ADAPTER, TESTS, Path(__file__)):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")

    print(
        "[entry-result-projection0-s2-vm-reference-consume-guard] ok "
        "normalized_fault=1 carrier=1 typed_fault=1 production_callers=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
