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
- `crates/nyash_kernel/src/plugin/array_string_slot_write.rs`
  - owns string-slot write executor glue.
  - may orchestrate handle acquisition and call array text mechanics.
  - must not own storage layout, route legality, or publication policy.
- `crates/nyash_kernel/src/plugin/array_handle_cache.rs`
  - owns handle-to-`ArrayBox` access mechanics.
  - must stay runtime-private.

## Backend Files

- `lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_window.inc`
  - may read route metadata.
  - must not rediscover route legality by scanning raw shape.
  - active array/text readers use `*_route_metadata` naming; do not add new
    cross-boundary `*_route_plan` names.
- `lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_lowering.inc`
  - may emit the selected helper calls and skip covered instructions.
  - H25c.1 consumes residence-session metadata first, but still maps it to the
    existing loopcarry update helper.
  - H25c.2 may add begin/update/end emission against H25b placement metadata.

## Forbidden Drift

- helper names as MIR truth
- runtime legality/provenance inference
- `.inc` exact matcher revival for loopcarry session
- semantic/search-result caches
- loop-wide session without explicit MIR lifetime/alias/publication contract
