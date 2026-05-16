# 293x-501 OSVM-EXPORT-VALIDATION-HELPER-001

Status: selected current
Date: 2026-05-16

## Decision

`OSVM-EXPORT-VALIDATION-HELPER-001` is a BoxShape cleanup for OSVM export
validation. It factors repeated base/len validation in the kernel OSVM export
surface without changing any export behavior.

## Scope

- Update `crates/nyash_kernel/src/exports/osvm.rs` only.
- Factor repeated `base` / `len` validation shared by commit/decommit/unreserve.
- Preserve exact status-code mapping and platform calls.

## Stop Lines

- Do not add new exports.
- Do not change page-size behavior.
- Do not change mmap/mprotect/munmap flags or reserve/commit/decommit/unreserve
  lifetime semantics.
- Do not touch allocator provider activation, hooks, host allocator replacement,
  or `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `OSVM.1` | Add a small local validation helper. | code compiles. | no ABI/status change |
| `OSVM.2` | Use it from commit/decommit/unreserve. | focused OSVM tests are green. | no platform flag change |
| `OSVM.3` | Verify and close out. | required evidence is green. | no provider/allocator work |

## Required Evidence

```text
cargo test -q -p nyash_kernel osvm
bash tools/checks/k2_wide_mimalloc_osvm_page_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_osvm_unreserve_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## Closeout

This row closes when OSVM export validation has a small shared helper and the
existing OSVM behavior is unchanged.
