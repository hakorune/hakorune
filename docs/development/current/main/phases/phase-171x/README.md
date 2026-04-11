# Phase 171x: substring concat exact-seed loop-shape cut

- Status: Active
- Purpose: trim the current `kilo_micro_substring_concat` exact front by recutting the pure-first exact seed from a top-tested loop to a bottom-tested loop while keeping the same string semantic contract.
- Scope:
  - `lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc`
  - exact `kilo_micro_substring_concat` perf / asm evidence
  - root/current/workstream pointers for the active string front
- Non-goals:
  - no new string corridor metadata contract
  - no `phi_merge` widening
  - no new sink rewrite family
  - no runtime/helper leaf retune outside the current exact seed
  - no DCE/escape/user-box work mixed into this cut

## Decision Now

- this is an exact-front keeper cut inside the sibling string lane, not a new generic string-family widening
- current exact front remains:
  - `kilo_micro_substring_concat`
  - latest reread after the current cut: `ny_aot_instr=5,565,470 / ny_aot_cycles=5,893,313 / ny_aot_ms=5`
- current exact asm now shows the loop-shape win:
  - the entry/head compare is gone
  - only the latch compare remains on the backedge
- the current keeper target stays:
  - `instr < 5.5M`
- this cut touched only the pure-first seed loop shape
- current result: asm-visible win is landed, but the exact keeper target is still open

## Restart Handoff

- parent lane:
  - `docs/development/current/main/phases/phase-163x/README.md`
- sibling guardrail lane:
  - `docs/development/current/main/phases/phase-137x/README.md`
- current snapshot:
  - `docs/development/current/main/10-Now.md`
- workstream map:
  - `docs/development/current/main/15-Workstream-Map.md`
- SSOT:
  - `docs/development/current/main/phases/phase-171x/171x-90-substring-concat-exact-seed-loop-shape-ssot.md`
  - `docs/development/current/main/phases/phase-171x/171x-91-task-board.md`
- code owner seam:
  - `lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc`
  - `tools/perf/bench_micro_aot_asm.sh`
  - `tools/perf/bench_micro_c_vs_aot_stat.sh`
  - `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_post_sink_shape.sh`
  - `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_phi_merge_contract.sh`

## Stop Line

- do not widen this into a broader publication/materialization phase
- do not mix `return` / `store` / host-boundary publication genericization into this cut
- do not reopen builder-local string shape logic
- do not relax `phi_merge` stop-lines in the same commit series

## Current Result

- current direct emit contracts stay green
- `tools/checks/dev_gate.sh quick` stays green
- exact asm now uses the bottom-tested loop shape
- exact reread improved slightly:
  - before: `ny_aot_instr=5,565,845 / ny_aot_cycles=5,943,591 / ny_aot_ms=5`
  - after: `ny_aot_instr=5,565,470 / ny_aot_cycles=5,893,313 / ny_aot_ms=5`
- reading:
  - this cut is valid and worth keeping
  - it does not clear the `instr < 5.5M` keeper by itself
  - the remaining work should move to body compaction or another exact-route-local cut, not back to metadata widening
