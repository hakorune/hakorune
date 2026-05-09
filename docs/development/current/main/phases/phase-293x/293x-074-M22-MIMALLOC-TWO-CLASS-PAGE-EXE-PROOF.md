---
Status: done
Date: 2026-05-09
Scope: M22 mimalloc two-class page EXE proof
---

# 293x-074 M22 Mimalloc Two-Class Page EXE Proof

## Decision

`M22 mimalloc two-class page EXE proof` locks the next allocator-shaped app
slice after M21: one source `static const u16[]` size-class table seeds two raw
pages, and the pure-first EXE path proves allocation, full-page reject,
oversize reject, release, and reuse for small and medium classes.

This is an app proof over existing compiler facts. It adds no new source
syntax, table type, allocator policy, or app-specific `.inc` matcher.

## Owned

- `apps/mimalloc-two-class-page-proof/`
- A pure-first EXE guard for the fixture.
- MIR JSON checks that:
  - `MI_SIZE_CLASS` and `MI_CLASS_CAP` remain static `u16` data plans.
  - `main` reads both tables through `static_data_load`.
  - both raw pages route through RawBuf / RawArray generic-i64 facts.

## Not Owned

- New static table element types.
- Dynamic size-class lookup or computed bin indexing.
- TLS, atomics, OSVM, native pointer attrs, or allocator ownership proof.
- Runtime `ArrayBox` / `MapBox` size-class materialization.
- App-specific symbol matching in `.inc`.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_two_class_page_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result: green on 2026-05-09.

## Next Reading

After M22, the allocator lane should split the next concrete blocker before
adding more substrate vocabulary. Likely future rows are dynamic bin selection,
TLS cache slot, atomic remote-free primitive, or OSVM-backed page reservation.
