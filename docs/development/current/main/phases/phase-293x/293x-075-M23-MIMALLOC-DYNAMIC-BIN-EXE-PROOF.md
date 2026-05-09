---
Status: done
Date: 2026-05-09
Scope: M23 mimalloc dynamic bin EXE proof
---

# 293x-075 M23 Mimalloc Dynamic Bin EXE Proof

## Decision

`M23 mimalloc dynamic bin EXE proof` locks the first non-constant static table
index used by an allocator-shaped app. A request size chooses class index `1`,
then `MI_SIZE_CLASS[class_idx]` and `MI_CLASS_CAP[class_idx]` seed a raw-page
fixture under pure-first EXE.

This row validates an already-supported backend shape from M21:
`static_data_load` may use a runtime `i64` index. It adds no new source syntax,
table type, allocator policy, or `.inc` app matcher.

## Owned

- `apps/mimalloc-dynamic-bin-proof/`
- A pure-first EXE guard for the fixture.
- MIR JSON checks that:
  - static table plans stay `u16`.
  - both tables have at least one `static_data_load` with a non-constant index.
  - raw page operations still route through RawBuf / RawArray generic-i64 facts.

## Not Owned

- General `size_to_bin` algorithm.
- Static table bounds proof.
- TLS, atomics, OSVM, native pointer attrs, or allocator ownership proof.
- Dynamic allocator policy or cross-page ownership.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_dynamic_bin_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result: green on 2026-05-09.

## Next Reading

After M23, the allocator lane has static-table dynamic indexing covered. The
next row should only add new substrate vocabulary when a concrete fixture
requires it, likely TLS cache slot, atomic remote-free, or OSVM page reservation.
