# 293x-234 M194 Purge Decommit Execution Fail-Fast

Status: Complete

## Purpose

M194 adds the first purge/decommit execution entry, but keeps it fail-fast and
blocked. The row lets later code call an explicit execution attempt owner
without silently opening OSVM decommit, unreserve, or release behavior.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/purge_execution_box.hako
```

`HakoAllocPurgeExecutionFailFastEntry.attempt(decision)` returns a
`HakoAllocPurgeExecutionReport`:

```text
missing decision -> status 1
ineligible decision -> status 2
eligible decision -> status 3 blocked execution inactive
```

All execution result fields stay false:

```text
decommit_executed = 0
unreserve_executed = 0
os_release_executed = 0
```

## Stop Lines

- Do not call `HakoAllocPageSourcePolicy`.
- Do not call OSVM decommit/unreserve/release.
- Do not mutate heap/page state.
- Do not change allocation, release, realloc, aligned, or huge behavior.
- Do not add provider activation, hooks, or process allocator replacement.
- Do not add env toggles or mutable allocator options.

## Acceptance

- Missing decision, ineligible decision, and eligible decision all return
  blocked reports.
- Eligible decision reaches the execution entry and stops at
  `execution inactive`.
- VM and pure-first EXE proof output match the fail-fast matrix.
- The guard confirms no page-source or `.inc` matcher leak.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_purge_execution_failfast_guard.sh
```
