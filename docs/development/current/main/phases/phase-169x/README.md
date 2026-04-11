# Phase 169x: substring concat stable-length phi contract

- Status: Landed
- Purpose: widen the current `kilo_micro_substring_concat` exact front through the merged header `phi` stop-line by adding a narrow `loop-stable source length` relation and using it to collapse the complementary `substring_len_hii + const + substring_len_hii` loop path.
- Scope:
  - string relation metadata for the live merged header route
  - `string_corridor_sink` complementary-length fusion on the same route
  - direct/post-sink exact contract refresh for `kilo_micro_substring_concat`
  - pure-first exact seed refresh for the same live body shape
- Non-goals:
  - no generic merged-plan-window carry
  - no new public MIR dialect or `.hako` syntax
  - no DCE/escape/user-box work mixed into this cut

## Decision Now

- landed as a metadata-contract follow-on, not another leaf helper tweak
- canonical MIR stayed the only truth; the new proof lives in string relation metadata
- `%21 = stop_at_merge` stayed untouched for plan windows; only the narrow scalar-length witness was added
- direct/post-sink and pure-first contracts were refreshed together in the same cut

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
  - `docs/development/current/main/phases/phase-169x/169x-90-substring-concat-stable-length-phi-ssot.md`
  - `docs/development/current/main/phases/phase-169x/169x-91-task-board.md`
- code owner seam:
  - `src/mir/string_corridor_relation.rs`
  - `src/mir/passes/string_corridor_sink.rs`
  - `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_post_sink_shape.sh`
  - `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_phi_merge_contract.sh`
  - `lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc`

## Landed Result

- live direct MIR on `bench_kilo_micro_substring_concat.hako` still carries:
  - merged header `%21 = phi([4,0], [22,20])`
  - backedge `%22 = phi([36,19])`
  - helper `%36 = substring_concat3_hhhii(...)`
- relation metadata now carries:
  - `%22 = preserve_plan_window`
  - `%21 = stop_at_merge`
  - `%21 = stable_length_scalar` with the entry length witness
- the live post-sink loop body is now the collapsed shape:
  - no loop `substring_len_hii`
  - `source_len + const_len`
  - current direct probe reads `interesting_n = 14`
- verification is green:
  - `phase137x_direct_emit_substring_concat_post_sink_shape.sh`
  - `phase137x_direct_emit_substring_concat_phi_merge_contract.sh`
  - `phase29x_backend_owner_daily_substring_concat_loop_min.sh`
  - `bench_micro_aot_asm.sh kilo_micro_substring_concat 'ny_main' 1`
  - `bench_micro_c_vs_aot_stat.sh kilo_micro_substring_concat 1 3`
  - `tools/checks/dev_gate.sh quick`

## Stop Line

- do not widen proof-bearing plan windows across merged `%21`
- do not retire the existing header/latch semantic guard beyond the length witness added here
- do not reopen substring runtime/helper leaf tuning in this phase
