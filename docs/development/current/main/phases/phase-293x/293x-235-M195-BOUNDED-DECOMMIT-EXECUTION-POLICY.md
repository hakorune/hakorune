# 293x-235 M195 Bounded Decommit Execution Policy

Status: Complete

## Purpose

M195 opens the first bounded decommit execution policy without directly calling
OSVM/page-source APIs. The row accepts a purge decision, validates base and
byte bounds, and calls a caller-provided `decommitPage(base, bytes)` executor at
most once.

Unreserve and OS release remain closed.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/purge_bounded_decommit_box.hako
```

`HakoAllocBoundedDecommitPolicy.attemptDecommit(decision, source, base, bytes)`
returns `HakoAllocBoundedDecommitReport`.

Blocked statuses:

```text
1 missing decision
2 ineligible decision
3 invalid base
4 invalid bytes
5 bytes over max_decommit_bytes
6 source decommit rejected
```

Success status:

```text
0 bounded decommit executed
```

## Stop Lines

- Do not directly call `HakoAllocPageSourcePolicy`.
- Do not directly call `OsVmCoreBox`.
- Do not unreserve pages.
- Do not release OSVM pages.
- Do not mutate heap/page state.
- Do not change allocation, release, realloc, aligned, or huge behavior.
- Do not add provider activation, hooks, or process allocator replacement.
- Do not add env toggles or mutable allocator options.

## Acceptance

- Missing and ineligible decisions do not call the source executor.
- Invalid base and over-bound byte requests do not call the source executor.
- Eligible in-bound requests call the provided source exactly once.
- Source reject is reported without claiming execution success.
- Successful source decommit sets `decommit_executed = 1`.
- `unreserve_executed` and `os_release_executed` remain `0`.
- VM and pure-first EXE proof output match the bounded decision matrix.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_bounded_decommit_policy_guard.sh
```
