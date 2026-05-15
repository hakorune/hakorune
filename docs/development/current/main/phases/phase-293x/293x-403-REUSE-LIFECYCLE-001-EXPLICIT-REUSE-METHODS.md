# 293x-403 REUSE-LIFECYCLE-001 Explicit Reuse Methods

Status: landed
Date: 2026-05-15

## Decision

Allocator object reuse must stay explicit method surface such as `reset`,
`reactivate`, `configure`, `clear`, or `attach`.

This row follows the `birth` cleanup rows by preventing lifecycle reuse from
slipping back into direct `birth(...)` calls or hidden constructor re-entry.

## Scope

- Inventory allocator lifecycle reuse methods that are already explicit.
- Add a narrow guard that rejects new direct receiver `birth(...)` reuse
  workarounds in `hako_alloc`.
- Document the reuse-method naming boundary and contract/transitions owner.

## Stop Lines

- Do not accept source-level `obj.birth(...)`.
- Do not introduce named constructor arguments.
- Do not add allocator-provider activation, host allocator replacement, hooks,
  or `#[global_allocator]`.
- Do not rewrite existing allocator behavior unless the guard finds a concrete
  lifecycle ambiguity.

## Required Evidence

```text
bash tools/checks/k2_wide_reuse_lifecycle_explicit_methods_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Implementation

- Added the current `hako_alloc` explicit reuse method inventory to the
  constructor/new lifecycle SSOT.
- Fixed the current allowed reuse surfaces as `reactivate()`, `reuse()`,
  `reset()`, and `attachFreshPage(...)`.
- Added a guard that rejects new direct receiver `birth(...)` reuse
  workarounds under `lang/src/hako_alloc`, while preserving the single legacy
  `arc.birth(ptr)` host-facade exception.

## Evidence

```text
bash tools/checks/k2_wide_reuse_lifecycle_explicit_methods_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
