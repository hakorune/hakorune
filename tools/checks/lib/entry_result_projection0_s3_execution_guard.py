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


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    task = TASK.read_text()
    execution = EXEC.read_text()
    publication = PUB.read_text()
    reference = REF.read_text()

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

    for forbidden in (
        "execute_module(",
        "NYASH_ENTRY",
        "to_nyash_box(",
        "as_integer(",
        "as_bool(",
    ):
        if forbidden in execution:
            raise AssertionError(f"S3 execution must not use legacy discovery/coercion: {forbidden}")

    for path in (TASK, EXEC, PUB, REF, Path(__file__)):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[entry-result-projection0-s3-execution-guard] ok "
        "exact_target=1 decode_plan=1 source_fault=1 owner_retained=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
