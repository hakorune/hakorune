#!/usr/bin/env python3
"""S3-PARITY0/G0 closeout guard for the explicit Raw VM lane."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "entry-result-projection0-s3-raw-vm-activation-execution-task-2026-07-25.md"
)
EXEC = ROOT / "src/mir/compiler/source_entry_vm_execution.rs"
KERNEL = ROOT / "src/mir/compiler/raw_published_compile.rs"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    task = TASK.read_text()
    execution = EXEC.read_text()
    kernel = KERNEL.read_text()

    for fragment in (
        "S3-PARITY0/G0",
        "Closeout evidence",
        "Route-scoped G0 census snapshot",
        "decode ABI mismatch",
        "decoy `NYASH_ENTRY`",
    ):
        require(task, fragment, f"task closeout {fragment}")
    for fragment in (
        "raw_vm_entry_executes_empty_app_main_without_symbol_discovery",
        "raw_vm_entry_keeps_app_scalar_fallthrough_as_unit",
        "raw_vm_entry_preserves_integer_process_status_boundaries",
        "raw_vm_entry_reports_out_of_range_integer_without_zero_fallback",
        "raw_vm_entry_reports_unsupported_process_result_kinds",
        "raw_vm_entry_maps_division_fault_to_source_diagnostic",
        "raw_vm_entry_ignores_decoy_nyash_entry_environment",
        "raw_vm_reference_reuses_compiler_after_entry_rejection",
        "fn abi_mismatch(",
    ):
        require(execution, fragment, f"parity fixture {fragment}")
    require(kernel, "compile_raw_published_v1", "shared compile owner")

    for fragment in (
        "general VM/MIR runner status-law replacement",
        "LLVM/native ny_main activation",
        "normal compile_with_source cutover",
        "JSON / Program(JSON v0)",
        "CUT0",
    ):
        require(task, fragment, f"non-claim {fragment}")

    for path in (TASK, EXEC, KERNEL, Path(__file__)):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[entry-result-projection0-s3-parity-guard] ok "
        "actual_execution=1 fault_projection=1 decoy_isolation=1 caller_census=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
