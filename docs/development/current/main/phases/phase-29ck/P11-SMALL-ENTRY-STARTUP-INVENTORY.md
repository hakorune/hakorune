---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P10` close 後の small-entry perf lane を startup/loader inventory として固定し、runtime leaf が存在しない current truth を 1 本の front に落とす。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P10-SMALL-PERF-REENTRY-TASK-PACK.md
  - docs/development/current/main/phases/phase-29ck/P12-SMALL-ENTRY-GC-SECTIONS-CANDIDATE.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - benchmarks/baselines/method_call_only_small.latest.json
  - benchmarks/baselines/box_create_destroy_small.latest.json
  - tools/archive/legacy-selfhost/engineering/phase29ck_small_entry_startup_probe.sh
---

# P11: Small-Entry Startup Inventory

## Purpose

- `small-entry` lane を runtime leaf tuning に誤読しない。
- current small-entry cost が startup/loader shape から来ていることを固定する。
- next exact code bucket を startup/loader owner へ narrow に切る。

## Fixed Inputs

1. target
   - `method_call_only_small`
2. control
   - `box_create_destroy_small`
3. evidence probe
   - `tools/archive/legacy-selfhost/engineering/phase29ck_small_entry_startup_probe.sh`

## Current Truth

1. `method_call_only_small` mainline AOT IR
   - `ny_main` pure loop only
   - body increment is `+5`
   - runtime string length helpers are absent
2. `box_create_destroy_small` mainline AOT IR
   - `ny_main` pure loop only
   - body increment is `+1`
   - runtime box/string birth helpers are absent
3. `target/perf_ny_method_call_only_small.exe`
   - dynamically linked
   - not stripped
   - current dynamic surface includes `libm.so.6`, `libgcc_s.so.1`, `libc.so.6`, and the ELF interpreter
   - current relocation shape is non-trivial (`.rela.dyn` current count is 179, `.rela.plt` current count is 2 on the measured Linux host)
4. short microasm evidence is dominated by loader symbols such as `_dl_relocate_object` / `_dl_lookup_symbol_x`
5. startup-subtracted bench evidence
   - `method_call_only_small`: `ny_aot_ms=1`
   - `box_create_destroy_small`: `ny_aot_ms=0`

読み:
- current small-entry lane has no live runtime leaf to trim.
- the first exact perf owner is now startup/loader shape, not kernel string/box routes.

## Next Exact Front

1. startup/loader inventory is now named
   - dynamic link surface
   - relocation density
   - startup-subtract evidence
2. current next exact front is `P12-SMALL-ENTRY-GC-SECTIONS-CANDIDATE.md`
3. do not reopen runtime string/box leaf edits in the same series
4. do not widen to medium/full `kilo` until the boundary link owner slice is judged

## Expected Owners

- first owners:
  - `tools/perf/lib/aot_helpers.sh`
  - `tools/ny_mir_builder.sh`
  - `lang/c-abi/shims/hako_aot.c`
  - `lang/c-abi/shims/hako_aot_shared_impl.inc`
- keep-only:
  - `src/llvm_py/**`
  - `crates/nyash_kernel/src/exports/string.rs`
  - `src/runtime/host_handles.rs`

## Non-Goals

- `string.len` runtime retune
- `newbox` / `Registry::alloc` runtime retune
- medium/full `kilo` reopen
- `llvmlite` keep-lane widening

## Acceptance

1. refreshed small baselines are recorded
2. pure-loop IR evidence is pinned for target and control
3. dynamic/startup executable shape is pinned
4. the next exact code bucket names one startup/loader owner

## Exit Condition

この task pack は close したよ。

- small-entry startup inventory is reproducible
- runtime-leaf false positives are ruled out
- next exact code bucket is named as `P12-SMALL-ENTRY-GC-SECTIONS-CANDIDATE.md`
