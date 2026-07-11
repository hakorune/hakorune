# 171x-90: substring concat exact-seed loop-shape SSOT

Status: SSOT
Date: 2026-04-12
Scope: move the `kilo_micro_substring_concat` pure-first exact seed to a bottom-tested loop shape without changing the current MIR/string semantic contract.

## Goal

- keep canonical MIR and string corridor metadata unchanged
- keep the current exact seed matcher contract unchanged on the MIR side
- reduce the exact front instruction count by removing the redundant loop-head compare in the generated pure IR
- preserve the existing exact asm/perf/smoke route as the proving lane

## Diagnosis

Current direct exact evidence says:

- `kilo_micro_substring_concat` remains the active exact front
- latest reread is still slightly above the first keeper target:
  - `ny_aot_instr=5,565,845`
  - target `instr < 5.5M`
- current `ny_main` asm still shows a top-tested loop:
  - one compare before the loop body
  - one compare at the latch

That means the current front is no longer blocked by missing string corridor metadata. It is blocked by the current exact seed loop shape.

## Fix

### 1. Keep MIR and seed matching stable

Do not change:

- emitted MIR shape
- `phase137x_direct_emit_substring_concat_post_sink_shape.sh`
- `phase137x_direct_emit_substring_concat_phi_merge_contract.sh`
- the current exact matcher input contract in `hako_llvmc_match_substring_concat_loop_ascii_seed(...)`

### 2. Recut only the generated pure IR

`hako_llvmc_emit_substring_concat_loop_ir(...)` may change from:

- top-tested loop

to:

- bottom-tested loop

as long as it preserves:

- the same number of loop iterations
- the same rotated text update
- the same `acc += seed_len + middle_len`
- the same exit return `acc + seed_len`

### 3. Keep the cut structural, not semantic

This cut is about loop shape only. It must not:

- add a new exact seed family
- add new helper contracts
- add new metadata reads
- reopen `phi_merge` proof carry

## Acceptance

- `tools/checks/dev_gate.sh quick` stays green
- current direct emit/string contract smokes stay green:
  - `phase137x_direct_emit_substring_concat_post_sink_shape.sh`
  - `phase137x_direct_emit_substring_concat_phi_merge_contract.sh`
- exact asm stays on `ny_main` and shows the bottom-tested loop shape
- `bench_micro_c_vs_aot_stat.sh kilo_micro_substring_concat 1 3` improves enough to clear the first keeper target or, at minimum, shows a clear instruction win on the same front

## Current Reading

- the first loop-shape recut is green and should stay
- current evidence after the cut:
  - `ny_aot_instr=5,565,470`
  - `ny_aot_cycles=5,893,313`
  - `ny_aot_ms=5`
- this is an asm-visible and exact-visible improvement over the pre-cut reread
- but the first keeper target `instr < 5.5M` remains open
- therefore this SSOT remains active until the next exact-route-local cut is chosen

## Non-Goals

- no broader publication sink wave
- no host-boundary publication wave
- no `return` rewrite wave
- no runtime leaf/helper redesign
