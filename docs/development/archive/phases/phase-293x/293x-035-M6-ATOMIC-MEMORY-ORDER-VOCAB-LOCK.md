---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-035-M6-ATOMIC-MEMORY-ORDER-VOCAB-LOCK
Scope: M6 hako.atomic memory-order vocabulary and ordered fence
---

# 293x-035 M6 Atomic Memory-Order Vocabulary Lock

## Decision

`hako.atomic` now has a narrow memory-order vocabulary and an ordered fence
row. This is still not a generic atomic API.

Accepted live surface:

```text
AtomicCoreBox.order_relaxed_i64()  -> 0
AtomicCoreBox.order_acquire_i64()  -> 1
AtomicCoreBox.order_release_i64()  -> 2
AtomicCoreBox.order_acq_rel_i64()  -> 3
AtomicCoreBox.order_seq_cst_i64()  -> 4
AtomicCoreBox.is_valid_order_i64(order)
AtomicCoreBox.fence_order_i64(order)
```

`fence_i64()` remains as the compatibility first row.

## Responsibility

- `lang/src/runtime/substrate/atomic/` owns atomic capability vocabulary.
- `AtomicCoreBox` owns the narrow `.hako` facade.
- VM-hako subset owns acceptance and fail-fast diagnostics for the current
  MIR JSON shape.
- Native keep remains `hako_barrier_touch_i64`; no allocator policy moves into
  `hako.atomic`.

## Non-Goals

- No `load`.
- No `store`.
- No CAS.
- No `fetch_add`.
- No pause/yield hint.
- No TLS cache policy.
- No allocator state policy.
- No final platform atomics fallback.

## VM-Hako Contract

Accepted:

```text
boxcall(AtomicCoreBox.fence_i64)
boxcall(AtomicCoreBox.fence_order_i64, order_reg)
boxcall(AtomicCoreBox.order_*_i64)
boxcall(AtomicCoreBox.is_valid_order_i64, order_reg)
```

Invalid ordered-fence values fail-fast with:

```text
[vm-hako/contract][boxcall-fence_order_i64-invalid-order]
```

## Acceptance

- `AtomicCoreBox.fence_order_i64(order)` routes to
  `hako_barrier_touch_i64(order)`.
- `AtomicCoreBox.fence_i64()` keeps the old `hako_barrier_touch_i64(0)` route.
- VM-hako subset accepts ordered fence with one register argument.
- VM-hako subset rejects order constants with accidental arguments.
- Reference docs state that generic load/store/CAS/fetch_add remain future
  splits.

## Gates

```bash
bash tools/checks/k2_wide_atomic_first_row_guard.sh
bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
