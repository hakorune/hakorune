# Phase 172x: substring concat stable-length exact-route cut

- Status: Landed
- Purpose: make the pure-first exact route for `kilo_micro_substring_concat` consume the already-landed `stable_length_scalar` witness and collapse the current exact seed from text rotation to the existing length-only route.
- Scope:
  - `lang/c-abi/shims/hako_llvmc_ffi_common.inc`
  - `lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc`
  - exact `kilo_micro_substring_concat` perf / asm evidence
  - root/current/workstream pointers for the active string front
- Non-goals:
  - no new MIR metadata contract
  - no `phi_merge` widening
  - no new sink rewrite family
  - no runtime/helper leaf redesign
  - no DCE/escape/user-box work mixed into this cut

## Decision Now

- this is an exact-route local cut on top of the landed `stable_length_scalar` contract
- the current string proof already says:
  - `%21` keeps `stop_at_merge`
  - `%21` also carries `stable_length_scalar` with witness `%5`
- this cut only switches the exact seed consumer:
  - from `substring_concat_loop_ir(...)`
  - to the existing `substring_concat_len_ir(...)`
  - when the witness is present on the current header string-lane phi and matches the entry source-length lane

## Restart Handoff

- parent lane:
  - `docs/development/current/main/phases/phase-163x/README.md`
- sibling guardrail lane:
  - `docs/development/current/main/phases/phase-137x/README.md`
- previous exact-front cut:
  - `docs/development/current/main/phases/phase-171x/README.md`
- current snapshot:
  - `docs/development/current/main/10-Now.md`
- workstream map:
  - `docs/development/current/main/15-Workstream-Map.md`
- SSOT:
  - `docs/development/current/main/phases/phase-172x/172x-90-substring-concat-stable-length-exact-route-ssot.md`
  - `docs/development/current/main/phases/phase-172x/172x-91-task-board.md`

## Stop Line

- do not reopen broader publication/materialization work here
- do not change direct MIR shape or metadata
- do not relax `phi_merge` stop-lines
- do not introduce another remembered-side bridge

## Landed Result

- current direct emit contracts stay green
- `tools/checks/dev_gate.sh quick` stays green
- exact asm/perf on `kilo_micro_substring_concat` improved sharply on the same front:
  - before: `ny_aot_instr=5,565,470 / ny_aot_cycles=5,893,313 / ny_aot_ms=5`
  - after: `ny_aot_instr=1,666,187 / ny_aot_cycles=1,049,205 / ny_aot_ms=4`
- current `ny_main` keeps the same latch-only compare shape while the exact route itself is now length-only
- reading:
  - this cut clears the first `instr < 5.5M` keeper target by a wide margin
  - the next string work should return to broader `return` / `store` / host-boundary publication
