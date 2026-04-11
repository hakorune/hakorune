# 168x-90: counter step_chain pure-first exact route refresh SSOT

Status: SSOT
Date: 2026-04-12
Scope: refresh the stale pure-first direct-route contract for `kilo_micro_userbox_counter_step_chain` so the exact backend seed matches the current canonical MIR shape produced after `phase-167x`.

## Goal

- keep `Counter.step_chain/0` on the current canonical known-receiver direct route
- align the pure-first seed and direct smoke with the current forwarding body shape
- recover exact build/asm evidence for `kilo_micro_userbox_counter_step_chain`

## Diagnosis

Current direct JSON is already stable after `phase-167x`:

- `Counter.step_chain/0` is always a canonical known-receiver `Method` call
- repeated release direct emit stays green (`6/6`)

The remaining failure is narrower:

- direct contract smoke still expects `Counter.step_chain/0` to have two receiver copies before the call
- `hako_llvmc_match_userbox_counter_step_chain_micro_seed()` enforces the same 4-instruction forwarding shape
- current direct JSON now emits the function as:
  - `copy`
  - `mir_call`
  - `ret`
- the seed therefore returns `-1`, the backend falls back to generic walk, sees `main:newbox`, and stops on `unsupported pure shape`

## Authority

1. direct canonical MIR contract
2. exact pure-first seed matcher
3. asm/perf keepers

This phase refreshes step 2 to follow step 1 again.

## Fix

### 1. Refresh direct contract truth

- `phase163x_direct_emit_user_box_counter_step_chain_contract.sh`
- accept the current narrow forwarding body:
  - one receiver copy minimum
  - canonical known-receiver `Method Counter.step`
  - return of call result

### 2. Refresh pure-first exact matcher

- `hako_llvmc_ffi_user_box_micro_seed.inc`
- accept the same narrow forwarding body for `Counter.step_chain/0`
- keep leaf/body checks for `Counter.step/0` unchanged

### 3. Re-lock exact evidence

- `bench_micro_aot_asm.sh kilo_micro_userbox_counter_step_chain 'ny_main' 1`
- `bench_micro_c_vs_aot_stat.sh kilo_micro_userbox_counter_step_chain 1 1`

## Non-Goals

- no broader recursive matcher widening
- no helper-retry absorb
- no backend generic-walk support for `newbox`

## Acceptance

- direct contract smoke passes on the current `Counter.step_chain/0` forwarding shape
- pure-first exact build/asm no longer falls back to generic walk on `main:newbox`
- exact `Counter.step_chain` probe is green again
