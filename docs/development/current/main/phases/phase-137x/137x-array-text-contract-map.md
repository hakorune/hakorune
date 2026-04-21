# 137x Array Text Contract Map

Purpose: keep the active array/text route files readable. This is a navigation
and ownership map, not a second semantic source.

## Ownership Rule

- `.hako`: user meaning only.
- MIR: route legality, provenance, consumer capability, publication boundary,
  and residence/session eligibility.
- `.inc`: metadata-to-call emit only.
- Rust runtime: storage, guard, mutation, and helper mechanics only.

## MIR Plan Files

- `src/mir/array_text_loopcarry_plan.rs`
  - owns the existing loopcarry len-store route.
  - emits `array_text_loopcarry_len_store_routes`.
  - current active proof: `insert_mid_subrange_trailing_len`.
- `src/mir/array_text_residence_session_plan.rs`
  - owns H25 residence-session eligibility derived from MIR route metadata.
  - emits `array_text_residence_sessions`.
  - current active proof: `loopcarry_len_store_only`.
  - H25c.2c nested executor contract fields:
    - `executor_contract.execution_mode=single_region_executor`
    - `proof_region=loop_backedge_single_body`
    - `publication_boundary=none`
    - `carrier=array_lane_text_cell`
    - `effects=[store.cell, length_only_result_carry]`
    - `consumer_capabilities=[sink_store, length_only]`
    - `materialization_policy=text_resident_or_stringlike_slot`
    - `region_mapping={loop_index_phi/init/next/bound, accumulator_phi/init/next,
       loop_index_initial_const=0, accumulator_initial_const=0,
       exit_accumulator, row_index, row_modulus}`
  - H25b placement fields:
    - `begin_block` / `begin_placement=before_preheader_jump`
    - `update_block` / `update_instruction_index`
    - `end_block` / `end_placement=exit_block_entry`
    - `skip_instruction_indices`
  - backend must consume these fields instead of deriving preheader/exit shape.
- `src/mir/array_text_observer_plan.rs`
  - owns generic read-side observer legality.
  - current use: `array_text_observer_routes`.
- `src/mir/array_text_state_residence_plan.rs`
  - owns generic state-residence route inventory for existing exact/front
    bridge cleanup.
  - do not grow it into loop session legality; H25 uses the session plan.

## Runtime Files

- `src/boxes/array/ops/text.rs`
  - owns `ArrayStorage::Text` read/write mechanics.
  - may expose runtime-private helpers, but must not decide MIR legality.
  - H25c.2a added an `ArrayBox`-local closure-scoped
    `ArrayTextSlotSession` substrate here.
  - H25c.2c added `slot_text_region_update_sum_raw(...)` as a one-call
    runtime executor for the MIR-proven loop region.
  - any write guard or slot borrow must be created and dropped inside one Rust
    call stack.
- `src/boxes/array/ops/text_session.rs`
  - optional split file if the H25c.2a substrate grows beyond a few helpers.
  - must stay mechanism-only: no route proof, provenance, or publication policy.
- `crates/nyash_kernel/src/plugin/array_string_slot_write.rs`
  - owns string-slot write executor glue.
  - may orchestrate handle acquisition and call array text mechanics.
  - must not own storage layout, route legality, or publication policy.
  - H25c.2a may add a thin substrate consumer only; H25c.2b may add a
    metadata-selected single-call executor only after docs acceptance.
- `crates/nyash_kernel/src/plugin/array_text_write_txn.rs`
  - H25c.2a proposed file for kernel-private transaction glue.
  - may own handle lookup, existing demand markers, observe accounting, and
    resident-first/fallback mode selection.
  - must not own route legality, storage layout, public ABI names, or session
    lifetime beyond one Rust call stack.
- `crates/nyash_kernel/src/plugin/array_runtime_substrate.rs`
  - may receive private forwarding only if needed by H25c.2a.
  - must not become a second policy owner or exported ABI surface.
- `crates/nyash_kernel/src/plugin/array_handle_cache.rs`
  - owns handle-to-`ArrayBox` access mechanics.
  - must stay runtime-private.
- `crates/nyash_kernel/src/plugin/array_runtime_facade.rs`
  - forwarding-only; do not grow ownership or mutation policy here.
- `crates/nyash_kernel/src/plugin/array_runtime_aliases.rs`
  - compatibility ABI aliases only.
  - H25c.2c exposes only the metadata-selected region executor alias; it must
    stay executor-only and must not become a route legality owner.

## Backend Files

- `lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_window.inc`
  - may read route metadata.
  - must not rediscover route legality by scanning raw shape.
  - active array/text readers use `*_route_metadata` naming; do not add new
    cross-boundary `*_route_plan` names.
  - H25c.2c validates `executor_contract`; it rejects missing/mismatched nested
    contract fields instead of inferring them from CFG.
  - H25c.2c also validates `region_mapping` presence and minimum cross-field
    invariants, including zero initial loop/accumulator constants; it must not
    derive loop/PHI/exit facts from raw blocks.
- `lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_lowering.inc`
  - may emit the selected helper calls and skip covered instructions.
  - H25c.1 consumes residence-session metadata first, but still maps it to the
    existing loopcarry update helper.
  - H25c.2c emits one metadata-selected executor call from the MIR-selected
    begin site and skips the covered header/body region.
  - The executor keeps the guard lifetime inside Rust and returns the
    accumulator value selected by MIR metadata.
  - must not emit guard-bearing begin/end handle plumbing in H25c.2.

## Forbidden Drift

- helper names as MIR truth
- runtime legality/provenance inference
- `.inc` exact matcher revival for loopcarry session
- semantic/search-result caches
- loop-wide session without explicit MIR lifetime/alias/publication contract
- session handle tables that store write guards or borrowed slot references
- guard/session lifetime crossing a C ABI call boundary
- benchmark-named whole-loop helpers
