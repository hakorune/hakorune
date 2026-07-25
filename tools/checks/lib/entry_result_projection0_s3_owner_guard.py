#!/usr/bin/env python3
"""S3-OWNER0 guard for the shared Raw compile owner."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "entry-result-projection0-s3-raw-vm-activation-execution-task-2026-07-25.md"
)
KERNEL = ROOT / "src/mir/compiler/raw_published_compile.rs"
INGRESS = ROOT / "src/mir/compiler/raw_public_ingress.rs"
EXEC = ROOT / "src/mir/compiler/source_entry_vm_execution.rs"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    task = TASK.read_text()
    kernel = KERNEL.read_text()
    ingress = INGRESS.read_text()
    execution = EXEC.read_text()

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
    require(execution, "pub fn run_raw_vm_reference(", "explicit VM-reference entry")

    if ingress.count("compile_raw_published_v1(") != 1:
        raise AssertionError("compatibility ingress must have one typed-kernel consumer")
    if execution.count("compile_raw_published_v1(") != 1:
        raise AssertionError("VM-reference ingress must have one typed-kernel consumer")
    if execution.count("pub fn run_raw_vm_reference(") != 1:
        raise AssertionError("VM-reference production entry must be unique")

    for forbidden in (
        "bind_raw_source_for_public(",
        "prepare_public_eligibility(",
        "execute_module(",
        "run_vm_compiled_module(",
        "NYASH_ENTRY",
        "build_module(",
    ):
        if forbidden in ingress or forbidden in execution:
            raise AssertionError(f"new ingress must not duplicate/discover legacy work: {forbidden}")

    for path in (TASK, KERNEL, INGRESS, EXEC, Path(__file__)):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[entry-result-projection0-s3-owner-guard] ok "
        "typed_kernel=1 ingress_consumers=2 legacy_duplication=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
