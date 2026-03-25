---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P11` close 後の small-entry startup/loader front を boundary link owner へ 1 本に絞り、`--gc-sections` candidate を next exact code bucket として固定する。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P11-SMALL-ENTRY-STARTUP-INVENTORY.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - lang/c-abi/shims/hako_aot_shared_impl.inc
  - crates/nyash-llvm-compiler/src/link_driver.rs
  - tools/dev/phase29ck_small_entry_gc_sections_experiment.sh
---

# P12: Small-Entry `gc-sections` Candidate

## Purpose

- `small-entry` startup front の first code bucket を 1 owner に絞る。
- current evidence を boundary link command へ集約し、runtime leaf や medium/full `kilo` widening へ逸れないようにする。

## Candidate Owner

1. mainline first owner
   - `lang/c-abi/shims/hako_aot_shared_impl.inc`
   - exact seam: `hako_aot_build_link_command(...)`
2. keep-only mirror
   - `crates/nyash-llvm-compiler/src/link_driver.rs`

読み:
- mainline perf judge is boundary-owned, so first code bucket is `hako_aot` link command.
- harness/native link driver is a mirror/keep lane and should not drive the first edit.

## Evidence

1. `P11` startup probe is green
   - `method_call_only_small` and `box_create_destroy_small` both dump to pure-loop IR
   - runtime string/box leafs are absent
2. one-off manual link experiment with `method_call_only_small`
   - current link command:
     - file size `18,000,192`
     - relocation count `181`
   - `-Wl,--gc-sections` variant:
     - file size `5,375,880`
     - relocation count `61`
   - dynamic dependency set stayed the same (`libm`, `libgcc_s`, `libc`, ELF interpreter)
   - coarse 5-run wallclock stayed flat at `3 ms` / `3 ms`, so this is a startup-shape candidate, not yet a throughput proof

## Next Exact Code Bucket

1. add `-Wl,--gc-sections` to the boundary mainline link command only
2. re-run:
   - small-entry startup probe
   - `phase21_5_perf_loop_integer_hotspot_contract_vm.sh`
   - `phase21_5_perf_strlen_ir_contract_vm.sh`
   - `tools/checks/dev_gate.sh quick`
3. measure:
   - executable size
   - relocation count
   - small-entry raw AOT ms
   - small-entry startup-subtracted AOT ms

## Non-Goals

- runtime string/box leaf edits
- immediate `link_driver.rs` parity edit
- medium/full `kilo` reopen
- `llvmlite` keep-lane changes

## Exit Condition

- boundary link owner change is cut as a single exact slice
- size/relocation deltas are recorded
- runtime/perf smoke contracts stay green
