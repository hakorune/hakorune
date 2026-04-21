# 137x Current Entry

This is the active entry for phase-137x. The long `README.md` keeps historical
ledger details; current implementation work should start here.

## Current Lane

- lane: `137x-H25 array text residence session contract`
- front: `kilo_meso_substring_concat_array_set_loopcarry`
- current blocker token: `137x-H25e post-parity owner refresh`
- current benchmark state:
  - `C 3 ms / Ny AOT 3 ms`
  - `ny_aot_instr=16570267`
  - `ny_aot_cycles=3471656`
- active owner:
  - no new code owner selected yet; re-baseline exact/middle/whole first
  - H25d closed with the runtime-private single-region executor at C parity
  - latest asm top:
    - region store mutation closure: `52.65%`
    - `__memmove_avx512_unaligned_erms`: `35.67%`
- non-owners:
  - fallback/promotion: H23a observed `update_text_resident_hit=179999`
  - helper-local resident/fallback compaction: H23b regressed to `ny_aot_instr=45910743`
  - per-iteration exported fused helper call: removed by H25c.2c-4
  - write-lock acquire/release in emitted AOT loop: moved inside one Rust call

## Active Contract

- MIR owns:
  - residence-session eligibility
  - loop/session lifetime
  - alias/publication boundary
  - covered route facts
- `.inc` owns:
  - metadata consumption
  - later begin/update/end emit shape only
  - no raw MIR shape rediscovery
- Rust runtime owns:
  - write guard mechanics
  - text storage/slot access
  - mutation execution
  - no legality/provenance inference

## H25a Landed

- Added metadata-only `array_text_residence_sessions`.
- The loopcarry benchmark exposes exactly one session route:
  - `scope=loop_backedge_single_body`
  - `proof=loopcarry_len_store_only`
  - `consumer_capability=slot_text_len_store_session`
  - `publication_boundary=none`
- Behavior is unchanged:
  - no lowering change
  - no runtime/session helper change
  - no `.inc` route change

## H25b Landed

- Worker design check rejected a direct long-lived runtime begin/end ABI as the
  next step:
  - `ArrayStorage` and write guards are private runtime mechanics.
  - exporting/storing guards across C ABI calls would require unsafe lifetime
    or self-referential state.
  - `.inc` cannot infer preheader/exit placement from CFG without becoming a
    planner again.
- Extended `array_text_residence_sessions` as MIR-owned placement metadata:
  - `begin_block` + `begin_placement=before_preheader_jump`
  - `update_block` + `update_instruction_index` +
    `update_placement=route_instruction`
  - `end_block` + `end_placement=exit_block_entry`
  - `skip_instruction_indices`
- H25b remains behavior-preserving. The backend can now lower begin/update/end
  later without rediscovering loop shape from raw MIR.

## H25c.1 Landed

- Renamed active `.inc` array/text reader seams from `*_route_plan` to
  `*_route_metadata` so `plan` stays MIR-internal.
- Added `array_text_residence_sessions` metadata consumption in
  `hako_llvmc_ffi_generic_method_get_window.inc`.
- The current lowering consumes the residence-session metadata first, then maps
  it to the existing loopcarry update helper. This is still behavior-preserving:
  no runtime session helper and no begin/end calls yet.

## H25c.2 Task Split

H25c.2 is split so the clean substrate and the perf keeper decision do not
collapse into one risky change.

- H25c.2a `runtime-private session substrate`
  - status: landed
  - intent: add an `ArrayBox`-local closure-scoped `ArrayTextSlotSession`
    substrate, plus an optional kernel-private `ArrayTextWriteTxn` wrapper.
  - landed:
    - `ArrayTextSlotSession` now owns text-slot update mechanics inside one
      `ArrayBox` write-lock frame.
    - `slot_update_text_resident_raw(...)` and `slot_update_text_raw(...)`
      are adapters over that substrate.
    - `slot_update_text_resident_first_raw(...)` reports whether the update
      hit an already text-resident lane without exposing a guard or slot borrow.
    - kernel-private `array_text_write_txn.rs` wraps handle lookup and
      resident-first/fallback outcome mapping.
    - same-slot string write helpers call the transaction wrapper without
      adding public ABI names.
  - files:
    - `src/boxes/array/ops/text.rs`, or `src/boxes/array/ops/text_session.rs`
    - `src/boxes/array/ops.rs` if a new module is split out
    - `src/boxes/array/tests.rs`
    - `crates/nyash_kernel/src/plugin/array_text_write_txn.rs`
    - `crates/nyash_kernel/src/plugin/mod.rs`
    - optional thin consumer in `crates/nyash_kernel/src/plugin/array_string_slot_write.rs`
    - optional private forwarding in `crates/nyash_kernel/src/plugin/array_runtime_substrate.rs`
  - allowed:
    - hold `RwLockWriteGuard` only inside one Rust stack frame
    - expose only closure-scoped methods such as `with_text_slot_session(...)`
      or `update_text_slot_session(...)`
    - keep existing `slot_update_text_resident_raw(...)` and
      `slot_update_text_raw(...)` as compatibility adapters
    - keep resident and fallback paths distinct
    - add unit tests for text-lane mutation and mixed-array fallback behavior
  - forbidden:
    - public begin/end ABI
    - session handle table
    - storing guard/slot references outside the call stack
    - runtime legality/provenance inference
    - new `nyash.array.*` exported ABI symbols
    - new environment variables
  - keeper expectation: none. This is substrate-only unless later perf evidence
    shows a safe executor boundary can use it.
  - task granularity:
    - H25c.2a-1: add ArrayBox-local `ArrayTextSlotSession` and outcome/kind
      enums.
    - H25c.2a-2: rebuild existing raw update methods as adapters over that
      session substrate.
    - H25c.2a-3: add array unit tests for resident hit, boxed string hit,
      boxed non-string miss, negative index miss, and resident-only non-promotion.
    - H25c.2a-4: add kernel-private `ArrayTextWriteTxn` wrapper for handle
      lookup and resident-first/fallback outcome mapping; observe accounting
      stays at existing helper call sites.
    - H25c.2a-5: refactor existing slot write helpers to call the transaction
      wrapper without changing exported ABI names.
- H25c.2b `single-call executor decision`
  - status: closed as clean non-keeper
  - decide whether `slot_text_len_store_session` can be executed as one
    capability-generic runtime call whose entire proven region stays inside one
    Rust call stack.
  - verdict:
    - accepted as a contract-cleaning boundary only.
    - existing metadata can select the update instruction without `.inc`
      re-scanning CFG or raw neighboring instructions.
    - current one-call executor shape is still one Rust call per iteration, so
      it cannot remove the measured per-iteration write-lock acquire/release
      owner.
    - keep this as clean closeout, not as a perf keeper.
  - rejected for keeper:
    - begin/update/end ABI with session handle table.
    - guard or slot borrow crossing C ABI.
    - runtime-owned legality from residence state.
    - benchmark-named whole-loop helper.
- H25c.2c `single-region executor contract`
  - status: landed
  - intent: open the next keeper path as a MIR-proven region replacement nested
    under `array_text_residence_sessions`, not as a new sibling plan family.
  - H25c.2c-1 landed:
    - `ArrayTextResidenceSessionRoute.executor_contract` is now emitted as
      nested metadata.
    - The current loopcarry benchmark exports:
      - `execution_mode=single_region_executor`
      - `proof_region=loop_backedge_single_body`
      - `publication_boundary=none`
      - `carrier=array_lane_text_cell`
      - `effects=[store.cell, length_only_result_carry]`
      - `consumer_capabilities=[sink_store, length_only]`
      - `materialization_policy=text_resident_or_stringlike_slot`
    - Route tests assert the contract and MIR JSON emission preserves it.
    - Behavior is unchanged; no backend execution replacement yet.
  - H25c.2c-2 landed:
    - `hako_llvmc_ffi_generic_method_get_window.inc` validates the nested
      `executor_contract` before accepting `array_text_residence_sessions`.
    - Missing or mismatched executor contract fields reject the route instead
      of letting `.inc` infer the contract from CFG/raw shape.
    - Active lowering still maps the validated session to the existing
      per-iteration loopcarry update helper; no region replacement yet.
    - Probe trace hit:
      `stage=array_text_residence_session result=hit reason=mir_route_metadata`.
  - H25c.2c-3 landed:
    - `executor_contract.region_mapping` now carries the loop index PHI,
      loop bound, accumulator PHI, accumulator exit use, row index, and row
      modulus needed by a later region replacement.
    - `.inc` validates that `region_mapping` exists, that numeric fields are
      present, that `row_index_value` matches the top-level `index_value`, and
      that the exit accumulator aliases the accumulator PHI.
    - Active backend trace still hits `array_text_residence_session` through
      `mir_route_metadata`; lowering behavior remains unchanged.
  - H25c.2c-4 landed:
    - MIR `region_mapping` now also proves the loop index and accumulator
      initial constants are `0`; runtime does not infer that fact.
    - `.inc` matches the MIR-selected begin block and emits one
      `nyash.array.string_insert_mid_subrange_len_store_region_hiisi` call,
      then skips the covered header/body region without redefining PHI values.
    - Runtime executes the proven loop inside
      `ArrayBox::slot_text_region_update_sum_raw(...)`; the write guard stays
      inside one Rust call stack and no session table or begin/end ABI is used.
    - Probe trace hit:
      `stage=array_text_residence_region_begin result=hit reason=mir_region_mapping`.
  - contract shape:
    - `executor_contract.execution_mode = single_region_executor`
    - `proof_region = loop_backedge_single_body`
    - `publication_boundary = none`
    - `effects = store.cell + LengthOnly result carry`
    - `carrier = ArrayLane(Text) / Cell`
    - `consumer_capability = { SinkStore, LengthOnly }`
    - `materialization_policy` must be MIR-owned if fallback behavior is needed.
  - backend rule:
    - `.inc` emits one call from the MIR-selected begin site and skips the
      covered region.
    - `.inc` must not infer loop legality, preheader, exit, PHI mapping, or
      fallback policy.
  - runtime rule:
    - one-call RAII executor may acquire the array write guard once, resolve the
      target slot once, run the proven region internally, return final length,
      and drop the guard before returning.
    - no guard/session table, TLS continuity, hidden legality, or public
      begin/end ABI.
- H25c.3 `keeper probe`
  - status: passed as partial keeper
  - timing: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 5 ms`
  - instruction/cycle transition:
    - before H25c.2c: `ny_aot_instr=40330160`, `ny_aot_cycles=12366672`
    - after H25c.2c: `ny_aot_instr=28630426`, `ny_aot_cycles=7033574`
  - target transition: hot path no longer emits the per-iteration
    `string_insert_mid_subrange_len_store_hisi` helper; owner moved into the
    runtime-private region executor and its text mutation/copy body.

## Next Slice

H25c.2c/H25c.3 are closed. The next owner-first slice is H25d:
`region executor inner mutation owner`.

Required order:
1. Re-run `bench_micro_aot_asm.sh` and annotate
   `slot_text_region_update_sum_raw` before editing runtime internals.
2. Fix only the sampled block owner.
3. Keep MIR contract unchanged unless a new legality/materialization fact is
   genuinely required.
4. Reject helper-name or benchmark-name dispatch.

H25d.1 implementation target:

- owner evidence:
  - `slot_text_region_update_sum_raw` is the top sampled symbol after H25c.
  - the runtime region executor still calls the generic
    `ArrayTextSlotSession::update` path for every iteration, even when storage
    is already `ArrayStorage::Text`.
- change scope:
  - make the text-resident region executor loop directly over
    `Text(Vec<String>)` after taking the single write guard
  - keep the existing compatible fallback for boxed/stringlike arrays
  - do not change MIR metadata, `.inc` lowering, public ABI, or helper names
- keeper gate:
  - behavior tests remain green
  - `kilo_meso_substring_concat_array_set_loopcarry` must not regress from the
    H25c partial keeper baseline (`C 3 ms / Ny AOT 5 ms`)

H25d.1 probe:

- result: `ny_aot_instr=24851120`, `ny_aot_cycles=6700078`, `Ny AOT 5 ms`
- verdict: keeper as instruction/cycles reduction, not a new ms keeper
- next owner:
  - `array_string_insert_const_mid_subrange_len_region_store_len` inlined
    mutation closure (`66.75%`)
  - `__memmove_avx512_unaligned_erms` (`18.52%`)

H25d.2 implementation target:

- split `update_insert_const_mid_subrange_len_value` into:
  - hot in-place fixed len-store path
  - cold semantic fallback for generic UTF-8/materialization cases
- keep UTF-8 boundary checks in the hot path; do not assume ASCII without MIR
  proof
- do not change MIR metadata, `.inc` lowering, public ABI, or helper names

H25d.2 probe:

- result: `ny_aot_instr=16570239`, `ny_aot_cycles=3459091`, `Ny AOT 4 ms`
- verdict: keeper; the cold fallback split removed the generic materialization
  body from the active hot path
- next owner:
  - hot mutation closure (`63.01%`)
  - `__memmove_avx512_unaligned_erms` (`23.20%`)

H25d.3 implementation target:

- replace the fixed in-place path's small overlapping shifts with manual byte
  moves for short text cells
- keep `ptr::copy` fallback for larger cells
- preserve UTF-8 boundary checks and keep MIR/ABI unchanged

H25d.3 probe:

- result: `ny_aot_instr=22511003`, `ny_aot_cycles=4765539`, `Ny AOT 4 ms`
- verdict: rejected; manual byte moves increase instruction/cycle count versus
  H25d.2, so keep the existing `ptr::copy` path

H25d.4 implementation target:

- hoist `observe::enabled()` out of the per-iteration region mutation closure
- keep all semantics and MIR/ABI unchanged

H25d.4 probe:

- result: `ny_aot_instr=22510404`, `ny_aot_cycles=4773551`, `Ny AOT 4 ms`
- verdict: rejected; instruction/cycle regression versus H25d.2, reverted

H25d final state:

- accepted code: H25d.1 + H25d.2 only
- final repeated stat: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 3 ms`,
  `ny_aot_instr=16570267`, `ny_aot_cycles=3471656`
- final asm top:
  - region store mutation closure: `52.65%`
  - `__memmove_avx512_unaligned_erms`: `35.67%`
- H25d.5 closeout:
  - residual `memmove` / mutation surgery is not reopened from the current
    percentage alone
  - H25d.3 manual byte moves and H25d.4 observe hoist both regressed, so
    H25d accepted code remains H25d.1 + H25d.2
- next slice:
  - H25e post-parity owner refresh
  - re-baseline exact/middle/whole with stat + asm before opening any new code
  - do not add source-length or ASCII assumptions unless MIR provides an
    explicit generic proof

Reject immediately if the implementation requires:
- runtime deciding session legality from residence state
- `.inc` scanning raw MIR JSON for session shape
- benchmark-specific whole-loop helper
- session across publish/objectize/generic fallback/unknown side-effect boundary
- any session handle table that stores `RwLockWriteGuard` or borrowed slot data
- moving loop semantics into runtime without a MIR-owned region contract

## Validation Anchor

- `cargo check -q`
- `cargo test -q benchmark_meso_substring_concat_array_set_loopcarry_has_len_store_route -- --nocapture`
- `cargo run -q --bin hakorune -- --emit-mir-json target/perf_state/h25_loopcarry.mir.json benchmarks/bench_kilo_meso_substring_concat_array_set_loopcarry.hako`
- H25c.2a substrate:
  - `cargo test -q slot_update_text --lib`
  - `cargo check -q -p nyash_kernel`
- after behavior change:
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry '' 20`

## Temporary Counters

H23 split counters are evidence-only and may stay while H25 session work is
active:
- `update_text_resident_hit`
- `update_text_resident_miss`
- `update_text_fallback_hit`
- `update_text_fallback_miss`

Retire them after H25 either lands a session keeper or rejects the session
hypothesis. Do not add a new env var for them.
