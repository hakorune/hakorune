---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-033-M3-RAWARRAY-CAP-SHAPE-LOCK
Scope: M3 RawBuf + RawArray allocator fixture cleanup
---

# 293x-033 M3 RawArray Cap Shape Lock

## Decision

`RawArray` owns a substrate-level capacity observer row before allocator-shaped
fixtures depend on the array shape.

`RawArrayCoreBox.slot_cap_i64(handle)` is the accepted narrow row:

```text
OwnershipCoreBox.ensure_handle_readable_i64
-> BufCoreBox.cap_i64
```

The route does not expose user-visible `ArrayBox.capacity` and does not widen
allocator policy.

## Responsibility

- `RawArrayCoreBox` owns the algorithm-substrate `slot_cap_i64` vocabulary.
- `BufCoreBox` owns the buffer-shape facade.
- `PtrCoreBox` owns the direct slot-cap backend symbol below `BufCoreBox`.

## Non-Goals

- No public `ArrayBox.capacity` method.
- No `set_len` / `shrink`.
- No `RawBuf` length/capacity state.
- No `MaybeInit`.
- No allocator policy/state migration.
- No TLS/atomic/OSVM dependency.

## Acceptance

- `RawArrayCoreBox.slot_cap_i64(handle)` exists.
- The row is readable ownership-gated.
- The row delegates to `BufCoreBox.cap_i64(handle)`.
- Guard/smoke lock the route and trace tag.

## Gates

```bash
bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh
bash tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
