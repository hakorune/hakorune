---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-037-M8-OSVM-PAGE-SIZE-ROW
Scope: M8 hako.osvm page-size facade
---

# 293x-037 M8 OSVM Page-Size Row

## Decision

`hako.osvm` now exposes the page-size row beside the already-live
reserve/commit/decommit rows:

```text
OsVmCoreBox.page_size_i64()
externcall "hako_osvm_page_size_i64"()
```

The native keep owns platform page-size discovery. `.hako` owns only the
capability facade.

## Responsibility

- `lang/src/runtime/substrate/osvm/` owns OS VM capability vocabulary.
- `OsVmCoreBox` owns the `.hako` facade.
- `lang/c-abi/shims/hako_kernel.c` owns native page-size discovery.
- VM-hako subset owns deterministic acceptance for the current MIR JSON shape.
- Allocator policy remains outside `hako.osvm`.

## Live Surface

```text
page_size_i64()
reserve_bytes_i64(len_bytes)
commit_bytes_i64(base, len_bytes)
decommit_bytes_i64(base, len_bytes)
```

## Non-Goals

- No allocator policy in `hako.osvm`.
- No broad raw OS syscall surface.
- No final OS VM rewrite.
- No page-aligned allocator state machine.

## Acceptance

- `hako_osvm_page_size_i64(void)` is declared in the C ABI header and
  implemented by the C shim.
- `OsVmCoreBox.page_size_i64()` routes to `hako_osvm_page_size_i64`.
- VM-hako subset accepts `boxcall(OsVmCoreBox.page_size_i64)` with no args.
- VM-hako subset accepts `externcall(hako_osvm_page_size_i64/0)` with no args.
- compile v0 emits `mir_call(Extern:hako_osvm_page_size_i64)`.
- guards no longer treat page size as parked.

## Gates

```bash
bash tools/checks/k2_wide_osvm_first_row_guard.sh
bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
