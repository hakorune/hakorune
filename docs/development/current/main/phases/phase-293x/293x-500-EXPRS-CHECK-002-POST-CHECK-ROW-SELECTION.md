# 293x-500 EXPRS-CHECK-002 Post-Check Row Selection

Status: landed
Date: 2026-05-16

## Decision

`EXPRS-CHECK-001` closed the CheckExpr owner split.

Select exactly one next cleanup row:

```text
OSVM-EXPORT-VALIDATION-HELPER-001:
  factor repeated base/len validation in OSVM commit/decommit/unreserve exports
```

## Why This Row

The expression-dispatcher cleanup burst is closed through indexing, collection
literals, and CheckExpr. The next smallest remaining concrete cleanup from the
worker inventory is OSVM export validation boilerplate. It is kernel-local and
does not add allocator/provider behavior.

## Selected Row

```text
row:
  OSVM-EXPORT-VALIDATION-HELPER-001
owner:
  crates/nyash_kernel/src/exports/osvm.rs
scope:
  factor repeated base/len validation in commit/decommit/unreserve
stop_line:
  no new exports
  no page-size behavior change
  no mmap/mprotect/munmap flag changes
  no provider/hook/global allocator work
evidence:
  cargo test -q -p nyash_kernel osvm
  bash tools/checks/k2_wide_mimalloc_osvm_page_exe_guard.sh
  bash tools/checks/k2_wide_mimalloc_osvm_unreserve_exe_guard.sh
  bash tools/checks/current_state_pointer_guard.sh
  tools/checks/dev_gate.sh quick
```

## Stop Lines

- Do not add OSVM exports or change ABI/status codes.
- Do not change page size, reserve/commit/decommit/unreserve platform flags, or
  ownership/lifetime semantics.
- Do not touch allocator provider activation, hooks, host allocator replacement,
  or `#[global_allocator]`.

## Closeout

This row closes when `OSVM-EXPORT-VALIDATION-HELPER-001` has a selected current
card with owner, scope, stop lines, and evidence.
