# 293x-400 LIFECYCLE-BIRTH-001 New-Only Birth Policy

Status: ready
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
- Add the smallest negative fixture or parser/verifier guard needed to prevent
  direct receiver `birth(...)` calls from becoming an allocator workaround.
- Keep this as lifecycle policy, not allocator behavior.

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
