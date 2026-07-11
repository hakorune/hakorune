# 172x-90: substring concat stable-length exact-route SSOT

Status: SSOT
Date: 2026-04-12
Scope: make the exact seed for `kilo_micro_substring_concat` consume the landed `stable_length_scalar` witness and route the current benchmark through the existing length-only pure IR emitter.

## Goal

- keep canonical MIR and direct-emit shape unchanged
- keep the current `phase137x_direct_emit_substring_concat_phi_merge_contract.sh` contract unchanged
- consume the existing `%21 stable_length_scalar -> witness %5` relation in the exact seed matcher
- remove the remaining byte-rotation work from the exact route

## Diagnosis

After `phase-171x`, the current exact front improved, but remained above the first keeper target:

- `ny_aot_instr=5,565,470`
- target `instr < 5.5M`

The remaining gap is no longer a loop-head compare. It is the fact that the exact seed still emits the full substring-rotation body even though the current metadata contract already proves:

- merged `%21` does not preserve the plan window
- merged `%21` does preserve the stable source-length scalar `%5`

That is enough for the current benchmark because the observable result is length-only.

## Fix

### 1. Add a narrow relation reader in the shim helper layer

Add one generic helper that can read:

- `string_corridor_relations`
- `value`
- `kind`
- `base_value`
- `witness_value`
- `window_contract`

from emitted MIR JSON.

### 2. Switch only the exact seed consumer

Inside `hako_llvmc_match_substring_concat_loop_ascii_seed(...)`:

- keep all existing structure checks
- keep the current `publication_sink` proof checks
- additionally require:
  - `%21 stable_length_scalar`
  - `base_value == helper_result`
  - `witness_value == source_len_value`
  - `window_contract == stop_at_merge`

If that holds, lower through:

- `hako_llvmc_emit_substring_concat_len_ir(...)`

instead of:

- `hako_llvmc_emit_substring_concat_loop_ir(...)`

### 3. Keep the old loop route as fallback

If the stable-length witness is absent, keep the previous exact route.

### 4. Keep the phi owner structural

Do not pin the relation to a literal value id like `%21` in the seed consumer.

Instead:

- require the current header `phi/phi/phi` contract
- read the current string-lane merged phi from that header shape
- consume `stable_length_scalar` on that phi

## Acceptance

- current direct-emit shape smoke stays green
- current phi-merge metadata-contract smoke stays green
- exact asm/perf on `kilo_micro_substring_concat` improve on the same front
- `tools/checks/dev_gate.sh quick` stays green
- landed reread:
  - `ny_aot_instr=1,666,187`
  - `ny_aot_cycles=1,049,205`
  - `ny_aot_ms=4`

## Non-Goals

- no new string relation kind
- no new direct-kernel proof kind
- no host-boundary publication widening
- no runtime leaf tuning
