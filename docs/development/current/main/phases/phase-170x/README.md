# Phase 170x: direct-kernel substring plan proof

- Status: Landed
- Purpose: shrink the remaining boundary-only bridge on helper-result `substring()` by making boundary `pure-first` read concat-triplet piece carriers from MIR `direct_kernel_entry.plan.proof` instead of remembered concat-chain side state.
- Scope:
  - `StringCorridorCandidateProof::ConcatTriplet` metadata contract widening
  - MIR JSON emission of the same proof payload
  - boundary `substring()` lowering on the `direct_kernel_entry` lane
  - targeted fixture/smoke for the new plan-proof route
- Non-goals:
  - no new sink rewrite family
  - no `phi_merge` widening
  - no runtime/helper leaf retune
  - no DCE/escape/user-box work mixed into this cut

## Decision Now

- landed as a bridge-shrink follow-on inside the sibling string lane, not as a new runtime helper wave
- canonical MIR stays the only truth; boundary lowering now reads piece carriers from metadata proof instead of remembered concat-chain state
- the existing `length()` direct-kernel route stays unchanged and green
- exact `kilo_micro_substring_concat` evidence is kept as a no-regression check, not treated as a new keeper by itself

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
  - `docs/development/current/main/phases/phase-170x/170x-90-direct-kernel-substring-plan-proof-ssot.md`
  - `docs/development/current/main/phases/phase-170x/170x-91-task-board.md`
- code owner seam:
  - `src/mir/string_corridor_placement.rs`
  - `src/runner/mir_json_emit/mod.rs`
  - `lang/c-abi/shims/hako_llvmc_ffi_string_chain_terms.inc`
  - `lang/c-abi/shims/hako_llvmc_ffi_generic_method_substring_policy.inc`
  - `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_direct_kernel_plan_substring_min.sh`

## Landed Result

- `ConcatTriplet` proof now carries:
  - `left_value`
  - `middle`
  - `right_value`
  - plus the existing shared-source/source-window proof fields
- MIR JSON exports the same carrier values on `string_corridor_candidates[*].plan.proof`
- boundary `pure-first` now has a narrow plan-selected substring route:
  - helper-result receiver with `direct_kernel_entry`
  - concat-triplet proof with piece carriers
  - lowers to `substring_concat3_hhhii`
  - avoids `remember_string_concat_*` as the proof source on that lane
- verification is green:
  - `phase137x_boundary_string_direct_kernel_plan_len_min.sh`
  - `phase137x_boundary_string_direct_kernel_plan_substring_min.sh`
  - `phase137x_direct_emit_substring_concat_post_sink_shape.sh`
  - `phase137x_direct_emit_substring_concat_phi_merge_contract.sh`
  - `bench_micro_aot_asm.sh kilo_micro_substring_concat 'ny_main' 1`
  - `bench_micro_c_vs_aot_stat.sh kilo_micro_substring_concat 1 3`
  - `tools/checks/dev_gate.sh quick`

## Stop Line

- do not widen this into generic lifecycle/boundary extraction
- do not reopen builder-local string shape logic
- do not mix host-boundary publication or loop-carried `phi_merge` widening into this cut
