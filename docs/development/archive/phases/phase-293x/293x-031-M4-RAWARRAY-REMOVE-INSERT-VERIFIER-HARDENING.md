# 293x-031 M4 RAWARRAY-REMOVE-INSERT-VERIFIER-HARDENING

Status: Landed
Date: 2026-05-08

## Decision

The first M4 minimum-verifier hardening slice extends existing RawArray verifier
coverage to `remove` and `insert` routes.

This is not a full sanitizer and not a new ownership model. It closes the
specific gap where RawArray remove/insert reached the pointer substrate after
ownership checks only.

## Scope

Accepted in this card:

- `BoundsCoreBox.ensure_insert_index_i64(handle, idx)` for insert-style bounds
  where `idx == len` is valid.
- `RawArrayCoreBox.slot_remove_any(handle, idx)` now gates through:
  - ownership writable
  - bounds existing-index
  - initialized-range existing-index
  - pointer substrate
- `RawArrayCoreBox.slot_insert_any(handle, idx, value_any)` now gates through:
  - ownership writable
  - bounds insert-index
  - ownership readable for positive `any` values
  - pointer substrate
- runtime v0 ABI slice guard checks the new gates.

Deferred:

- `slice` verifier hardening, because visible slice semantics currently clamp
  negative and oversized bounds instead of strict-failing.
- `set_len` / `shrink` initialized-range widening.
- double-free and use-after-free detection.
- borrowed alias expiry.
- allocator policy and raw pointer lifetime proof.

## Owner Boundary

- Bounds verifier owns index validity.
- Initialized-range verifier owns readable existing-index validity.
- Ownership verifier owns live handle / positive-any carrier validity.
- RawArray composes verifier answers but does not own verifier policy.
- PtrCoreBox remains the thin pointer-substrate call leaf.

## Gates

```bash
bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

Continue M4 with the next smallest verifier hardening row. The current open
items are `slice` semantics documentation before hardening and the deferred
double-free/use-after-free proof split.
