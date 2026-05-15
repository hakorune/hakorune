# 293x-400 LIFECYCLE-BIRTH-001 New-Only Birth Policy

Status: landed
Date: 2026-05-15

## Decision

`birth` is a constructor lifecycle hook, not a normal source-level receiver
method.

This row should lock the policy that `birth` fires only through `new Box(...)`
construction. Direct source calls such as `obj.birth(...)` must stay rejected or
fail-fast diagnosed; allocator lifecycle reuse must use explicit methods such
as `reset`, `reactivate`, `configure`, `clear`, or `attach`.

## Scope

- Pin the docs and guard for the new-only `birth` policy.
- Add the smallest guard needed to prevent direct receiver `birth(...)` calls
  from becoming an allocator workaround in `hako_alloc`.
- Keep this as lifecycle policy, not allocator behavior.
- Keep parser-negative fixture work in the follow-up `PARSER-BIRTH-001` row.

Known exception:

```text
lang/src/hako_alloc/memory/arc_box.hako:
  arc.birth(ptr)
```

This is classified as a legacy non-constructor host facade exception, not a
user-box constructor hook. The guard allows exactly this one existing call and
prevents new direct receiver `birth(...)` calls in `hako_alloc`.

## Stop Lines

- Do not accept source-level `obj.birth(...)`.
- Do not route allocator reuse through constructor hooks.
- Do not mix named constructor arguments, parser widening, or reuse-method
  semantics into this row.
- Do not alter mimalloc facade allocation/release behavior.

## Required Evidence

```text
bash tools/checks/k2_wide_lifecycle_birth_new_only_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Implementation

- Added `tools/checks/k2_wide_lifecycle_birth_new_only_guard.sh`.
- The guard checks the lifecycle SSOT, language lifecycle reference, taskboard,
  and card for the new-only birth policy.
- The guard rejects new `.birth(...)` receiver calls in `lang/src/hako_alloc`
  while preserving the single documented `arc.birth(ptr)` legacy host-facade
  exception.

## Evidence

```text
bash tools/checks/k2_wide_lifecycle_birth_new_only_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
