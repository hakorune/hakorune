# 293x-359 MIMAP-014C Allocation Fast-Path Stats Observers

Status: ready
Date: 2026-05-15

## Decision

`MIMAP-014C` is the next primary allocator row after `MIMAP-014B`. It exposes
read-only scalar allocation counters for the facade-owned small allocation
fast-path without changing queue selection behavior.

## Scope

- Keep allocation routed through `HakoAllocObjectLifecycleFacade`.
- Count small allocation attempts.
- Count allocation successes.
- Count allocation misses/failures.
- Count reusable-page allocation successes.
- Count active-page allocation successes.
- Add one proof app and one LLVM/EXE-primary guard.

## Non-goals

- No release/free route.
- No realloc route.
- No alignment route.
- No OSVM/page-source route.
- No provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- No selected-object return through the facade.
- No queue selection semantic change.
- No backend-specific matcher shortcuts.

## Expected Files

```text
apps/mimalloc-facade-small-alloc-stats-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_small_alloc_stats_exe_guard.sh
```

Likely implementation owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
```

## Acceptance

Expected proof output shape:

```text
mimalloc-facade-small-alloc-stats-proof
attempts=<count>
successes=<count>
failures=<count>
by_kind=<reusable successes>,<active successes>
summary=ok
```

Required guard evidence:

```text
[mimap014c-mir-json] ok
[k2-wide-mimalloc-facade-small-alloc-stats-exe] ok
```

## Stop Lines

If stats cannot stay as facade-local scalar observers, stop and split a smaller
stats-storage row.

If stats require queue behavior changes, stop and make the selection policy row
explicit before continuing.

If dense observer reads block MIR emit, stop this row and land `MIR-ROW-D` with
a minimized fixture.

## Follow-up

After `MIMAP-014C` lands:

```text
MIMAP-015A:
  release/free one known block through the facade
```
