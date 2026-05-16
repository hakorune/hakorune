# 293x-501 OSVM-EXPORT-VALIDATION-HELPER-001

Status: landed
Date: 2026-05-17

## Decision

`OSVM-EXPORT-VALIDATION-HELPER-001` is a BoxShape cleanup for OSVM export
validation. It factors repeated base/len validation in the kernel OSVM export
surface without changing any export behavior.

## Scope

- Update `crates/nyash_kernel/src/exports/osvm.rs` as the primary owner.
- Factor repeated `base` / `len` validation shared by commit/decommit/unreserve.
- Preserve exact status-code mapping and platform calls.
- Keep any evidence-script adjustment limited to current-state gating for the
  required guards.

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

## Result

Landed:

- Added a shared Unix `normalize_region_len(base, len_bytes)` helper for
  commit/decommit/unreserve validation and page-size rounding.
- Added a non-Unix `region_input_is_positive(base, len_bytes)` helper so the
  unsupported-platform status mapping remains unchanged.
- Updated the historical MIMAP-032A unreserve EXE guard to skip only the
  pre-page-source owner no-growth scan once MIMAP-033A has landed. The guard
  still proves the OSVM unreserve substrate route and MIR route metadata.

No OSVM ABI, export name, page-size behavior, mmap/mprotect/munmap flags,
status-code mapping, allocator provider activation, hooks, host allocator
replacement, or `#[global_allocator]` behavior changed.

## Evidence

```text
cargo test -q -p nyash_kernel osvm
bash tools/checks/k2_wide_mimalloc_osvm_page_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_osvm_unreserve_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```
