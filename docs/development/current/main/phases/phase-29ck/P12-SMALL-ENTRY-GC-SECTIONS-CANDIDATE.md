---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P11` close 後の small-entry startup/loader front を boundary link owner へ 1 本に絞り、Linux mainline `--gc-sections` link trim を landed truth として固定する。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P11-SMALL-ENTRY-STARTUP-INVENTORY.md
  - docs/development/current/main/phases/phase-29ck/P13-SMALL-ENTRY-RAW-NET-REFRESH.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - lang/c-abi/shims/hako_aot_shared_impl.inc
  - crates/nyash-llvm-compiler/src/link_driver.rs
  - tools/archive/legacy-selfhost/engineering/phase29ck_small_entry_gc_sections_experiment.sh
---

# P12: Small-Entry `gc-sections` Link Trim

## Purpose

- `small-entry` startup front の first code bucket を 1 owner に絞る。
- boundary mainline link owner に Linux-only `-Wl,--gc-sections` を入れ、size/relocation trim を current truth にする。

## Candidate Owner

1. landed mainline owner
   - `lang/c-abi/shims/hako_aot_shared_impl.inc`
   - exact seam: `hako_aot_build_link_command(...)`
2. keep-only mirror
   - `crates/nyash-llvm-compiler/src/link_driver.rs`

読み:
- mainline perf judge is boundary-owned, so the first landed code bucket stayed in `hako_aot` link command.
- harness/native link driver is a mirror/keep lane and should not drive the first edit.

## Evidence

1. `P11` startup probe is green
   - `method_call_only_small` and `box_create_destroy_small` both dump to pure-loop IR
   - runtime string/box leafs are absent
2. mainline boundary link trim is now green on `method_call_only_small`
   - current mainline exe size:
     - file size `5,375,880`
     - relocation count `61`
   - pre-trim historical baseline:
     - file size `18,000,192`
     - relocation count `181`
   - dynamic dependency set stayed the same (`libm`, `libgcc_s`, `libc`, ELF interpreter)
   - `tools/archive/legacy-selfhost/engineering/phase29ck_small_entry_startup_probe.sh` now rebuilds stale `libhako_llvmc_ffi` artifacts before checking the boundary mainline shape
3. current post-trim startup-subtracted evidence stays in the small-entry lane
   - `method_call_only_small`: `c_ms=2`, `ny_aot_ms=1`
   - `box_create_destroy_small`: `c_ms=2`, `ny_aot_ms=0`

## Current Next Exact Front

1. `P13-SMALL-ENTRY-RAW-NET-REFRESH.md`
2. purpose:
   - refresh the post-trim raw/net numbers for `method_call_only_small` and `box_create_destroy_small`
   - decide whether the small-entry lane closes monitor-only before any medium/full `kilo` widening

## Non-Goals

- runtime string/box leaf edits
- immediate `link_driver.rs` parity edit
- medium/full `kilo` reopen
- `llvmlite` keep-lane changes

## Exit Condition

- boundary link owner change is cut as a single exact slice
- size/relocation deltas are recorded
- runtime/perf smoke contracts stay green
- next exact front is named as `P13-SMALL-ENTRY-RAW-NET-REFRESH.md`
