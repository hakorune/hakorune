---
Status: done
Date: 2026-05-09
Scope: M20 mimalloc raw-page EXE parity guard
---

# 293x-072 M20 Mimalloc Raw-Page EXE Parity Guard

## Decision

`M20 mimalloc raw-page EXE parity guard` locks the first raw-page allocator
fixture that now compiles and runs under pure-first EXE after M14-M19:

```text
RawBufCoreBox.alloc/free
RawArrayCoreBox.slot_append_any
RawArrayCoreBox.slot_len_i64
RawArrayCoreBox.slot_load_i64
RawArrayCoreBox.slot_store_i64
```

This row does not add a new accepted shape. It freezes the composed route
surface as a regression guard and verifies that `.inc` remains a route-fact
reader rather than an app-specific planner.

## Owned

- A pure-first EXE guard for `apps/mimalloc-raw-page-proof/main.hako`.
- Trace checks for the route facts that make the fixture executable:
  `hako_mem_alloc/free`, RawArray append/len/load/store, and same-module
  generic-i64 calls.
- Runtime output check for the fixture summary.

## Not Owned

- New RawArray vocabulary beyond M16-M19.
- Broad ArrayBox generic method parity.
- Store-handle/string parity.
- Allocator policy, TLS, atomics, OSVM, or native pointer attrs.
- Backend symbol-name guessing for `MiRawPage*` boxes.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_raw_page_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result: green on 2026-05-09.

## Next Reading

After this guard, the allocator lane can move from raw-page proof toward the
next allocator app slice, but new blockers must be split into fresh rows.
