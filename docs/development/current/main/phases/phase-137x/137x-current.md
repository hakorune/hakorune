# 137x Current Entry

This is the active entry for phase-137x. The long `README.md` keeps historical
ledger details; current implementation work should start here.

## Current Lane

- lane: `137x-H25 array text residence session contract`
- front: `kilo_meso_substring_concat_array_set_loopcarry`
- current blocker token: `137x-H25c.2c single-region executor contract`
- current benchmark state:
  - `C 3 ms / Ny AOT 6 ms`
  - `ny_aot_instr=40330160`
  - `ny_aot_cycles=12366672`
- active owner:
  - write-lock acquire/release guard mechanics inside the fused loopcarry helper
  - H24 IP evidence: acquire/release `lock cmpxchg` sites own the samples
- non-owners:
  - fallback/promotion: H23a observed `update_text_resident_hit=179999`
  - helper-local resident/fallback compaction: H23b regressed to `ny_aot_instr=45910743`
  - byte-edit/memmove body: H24 samples did not land there

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
  - status: active; first MIR metadata slice landed
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
  - status: blocked on H25c.2c implementation
  - require target transition evidence, not only `ny_aot_ms`.

## Next Slice

H25c.2b is closed as a clean non-keeper. The next keeper path is H25c.2c:
a MIR-proven single-region executor contract nested under
`array_text_residence_sessions`.

Required order:
1. Land nested `executor_contract` metadata in MIR docs/code.
   - status: done for H25c.2c-1.
2. Make `.inc` validate and consume that metadata without CFG/raw shape
   rediscovery.
3. Extend the MIR contract for any missing loop/PHI/exit semantics before
   replacing the loop region.
4. Add a runtime-private one-call RAII executor only if MIR fully owns legality,
   fallback/materialization policy, and publication boundary.
5. Rerun exact timing and asm after behavior change.

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
