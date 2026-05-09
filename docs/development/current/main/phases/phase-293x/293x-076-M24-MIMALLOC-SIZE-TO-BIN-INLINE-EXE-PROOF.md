---
Status: done
Date: 2026-05-10
Scope: M24 mimalloc size_to_bin inline EXE proof
---

# 293x-076 M24 Mimalloc Size-To-Bin Inline EXE Proof

## Decision

`M24 mimalloc size_to_bin inline EXE proof` locks the first allocator-shaped
source helper that computes a bin before static table lookup:

```hako
@rune Profile(allocator.fast)
size_to_bin(size) { ... }
```

The helper stays idiomatic source, but MIR verifies the required inline plan
and consumes it before pure-first backend lowering. The backend sees expanded
scalar MIR plus `static_data_load`; it must not route or inline by profile name.

## Owned

- `apps/mimalloc-size-to-bin-inline-proof/`
- A pure-first EXE guard for the fixture.
- MIR JSON checks that:
  - `MiBinSelector.size_to_bin/1` has verified `allocator.fast` InlinePlan.
  - `main` no longer calls or routes to the helper after MIR optimization.
  - table reads use dynamic `static_data_load` indices.
  - raw page operations still route through RawBuf / RawArray generic-i64 facts.

## Not Owned

- General mimalloc size-class algorithm.
- Wider inline body shapes or backend inline decisions.
- TLS, atomics, OSVM, native pointer attrs, or allocator ownership proof.
- Dynamic table bounds proof.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_size_to_bin_inline_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result: green on 2026-05-10.

## Next Reading

After M24, the static-table + inline selector path is covered. The next new
substrate row should be driven by a concrete allocator fixture, likely TLS
cache slot, atomic remote-free primitive, or OSVM-backed page reservation.
