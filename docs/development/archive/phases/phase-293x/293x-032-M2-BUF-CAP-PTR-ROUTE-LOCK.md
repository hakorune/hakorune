---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-032-M2-BUF-CAP-PTR-ROUTE-LOCK
Scope: M2 hako.mem/buf/ptr widening cleanup
---

# 293x-032 M2 Buf Cap Ptr Route Lock

## Decision

`BufCoreBox.cap_i64` is buffer-shape vocabulary, not direct backend ABI
ownership.

The direct `nyash.array.slot_cap_h` route lives under `PtrCoreBox` as
`slot_cap_i64`. `BufCoreBox.cap_i64` delegates to that route, matching the
existing `len/reserve/grow` structure.

## Responsibility

- `hako.buf` owns the public shape/control facade:
  - `len_i64`
  - `cap_i64`
  - `reserve_i64`
  - `grow_i64`
- `hako.ptr` owns the current direct array-slot backend route names:
  - `slot_len_i64`
  - `slot_cap_i64`
  - slot load/store/append/remove/insert/slice/reserve/grow rows

## Non-Goals

- No new source syntax.
- No parser change.
- No allocator policy.
- No unrestricted pointer arithmetic.
- No `shrink` / `set_len` widening.
- No verifier semantic widening.

## Acceptance

- `BufCoreBox.cap_i64(handle)` calls `PtrCoreBox.slot_cap_i64(handle)`.
- `BufCoreBox` does not contain `externcall "nyash.array.slot_cap_h"`.
- `PtrCoreBox` contains the direct `nyash.array.slot_cap_h` route.
- Runtime substrate docs and the user-facing substrate capability manual name
  the route owner.

## Gates

```bash
bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh
bash tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
