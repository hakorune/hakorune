---
Status: Active
Date: 2026-04-19
Scope: phase-137x で `public handle ABI` を維持したまま、text hot corridor を `producer -> sink -> publish` の値モデルへ段階導入する taskboard。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md
  - docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md
  - docs/development/current/main/design/string-value-model-phased-rollout-ssot.md
---

# Phase 137x Text Lane Rollout Checklist

- North Star:
  - `String` is semantic value; handle/object is boundary representation
  - `Public world`: `StringHandle` / `ArrayHandle` / `Box<dyn NyashBox>`
  - `Execution world`: `VerifiedTextSource -> TextPlan -> OwnedTextBuf -> TextCell`
  - rule: `publish` は escape 時の effect に限定し、`freeze.str` は唯一の birth sink として読む
  - rule: future `TextLane` は storage specialization であり semantic truth ではない
- Current locked reading:
  - exact front is closed by the shared-receiver `KernelTextSlot` bridge
  - middle contradiction guard remains `kilo_meso_substring_concat_array_set_loopcarry`
  - whole owner remains upstream producer publication plus array/string slot work
- current whole-side landed cuts are still narrow:
  - direct-set-only `const_suffix -> KernelTextSlot -> kernel_slot_store_hi`
  - shared-receiver `const_suffix` reuse on exact front
  - direct-set-only `insert_hsi`
  - direct-set-only deferred `Pieces3 substring`
- first phase 2.5 slice is now landed:
  - `BorrowedHandleBox` caches the encoded runtime handle for unpublished keeps
  - `array.get` can reuse the cached stable handle instead of fresh-promoting on every read
- latest phase 2.5 follow-on slices are now landed:
  - map value stores preserve borrowed string aliases instead of eagerly rebuilding stable `StringBox` values
  - borrowed-alias runtime-handle cache is shared per alias lineage, so read clones do not drop the cached encoded handle
  - `perf-observe` and end-to-end tests now lock all three read outcomes on both array/map routes:
    - `live source`
    - `cached handle`
    - `cold fallback`
- next card is read-side alias lane split, not full `TextLane`:
  - `TextReadOnly`
  - `EncodedAlias`
  - `StableObject`
  - keep stable objectize cold and cache-backed

## Goal

- stop treating text as `handle/object world` inside the same corridor
- promote `KernelTextSlot`-style unpublished residence from side path to canonical sink contract
- introduce `TextLane` only after producer and sink ownership are separated
- keep exact / meso / whole as separate accept fronts while moving toward one value model

## Accept Fronts

- exact keeper gate:
  - `kilo_micro_array_string_store`
  - must stay closed after each card
- middle contradiction gate:
  - `kilo_meso_substring_concat_array_set_loopcarry`
  - use to reject owner-shift-only cards
- whole progress gate:
  - `kilo_kernel_small`
  - use repeated `repeat=3` windows only

## Rollout Rules

- 1 card = 1 owner move = 1 keeper/revert decision
- do not mix `producer outcome`, `sink specialization`, and `publish legality` in one card
- do not widen shared generic helper ABI before the current corridor-local contract is proven
- do not introduce public `TextOutcome` / `TextLane` API on this lane
- do not let this checklist become a second design authority; semantic truth stays in design SSOT
- legacy helper coexistence is temporary; remove old string/array routing once the new card becomes keeper-grade
- prefer existing repo-local shapes first:
  - `VerifiedTextSource`
  - `TextPlan`
  - `OwnedBytes`
  - `KernelTextSlot`

## Card Map

- `Card 0`:
  - semantic / baseline lock
  - keep docs aligned on `String = value`, `publish = boundary effect`, `freeze.str = only birth sink`
- `Card 1`:
  - producer outcome contract
  - `const_suffix` / `freeze_text_plan(Pieces3)` to `KernelTextSlot`
- `Card 2`:
  - cold publish effect
  - isolate `objectize_stable_string_box` and `issue_fresh_handle`
- `Card 2.5`:
  - read-side alias lane
  - keep `array.get` on `TextReadOnly` / `EncodedAlias`, cold-cache `StableObject`
  - current landed sub-cards:
    - `2.5a`: cached runtime handle reuse on `array.get`
    - `2.5b`: map borrowed-string store + alias-lineage cache continuity
    - `2.5c`: array/map read-outcome observability and contract tests
- `Card 3`:
  - future `TextLane` storage
  - array internal specialization only
- `Card 4`:
  - MIR contract / verifier
  - publication boundary becomes verifier-visible

## Phase 0. Semantic / Baseline Lock

- Goal:
  - fix the semantic truth and the current evidence before widening the value model
- Touched areas:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md`
  - perf artifacts under `target/perf_state/phase137x-*`
- Expected evidence:
  - docs agree on:
    - `String = value`
    - `publish = boundary effect`
    - `freeze.str = only birth sink`
    - `TextLane = future storage`
  - exact closed truth remains documented
  - meso band remains around the current `56-59 ms` band unless a real keeper lands
  - whole proof front remains `kilo_kernel_small`
- Keeper criteria:
  - evidence is consistent across `CURRENT_TASK.md`, `10-Now.md`, and phase README
- Revert criteria:
  - docs disagree on active owner or current next seam
- Stop-line:
  - if exact, meso, and whole are not explicitly separated in docs, do not start the next card

## Phase 1. Producer Outcome Contract

- Goal:
  - make producer result ownership explicit without changing public ABI
- Cards:
  - `const_suffix` producer returns unpublished outcome to the corridor-local sink
  - `freeze_text_plan(Pieces3)` follows the same unpublished outcome contract
- Touched areas:
  - `crates/nyash_kernel/src/exports/string_helpers/concat/const_adapter.rs`
  - `crates/nyash_kernel/src/exports/string_helpers/materialize.rs`
  - `crates/nyash_kernel/src/exports/string_plan.rs`
  - compiler lowering under `lang/c-abi/shims/`
- Expected evidence:
  - site-local producer paths can terminate in `KernelTextSlot` without immediate handle issue
  - exact front stays closed
  - whole top report shifts away from producer-side eager publish where the card lands
- Keeper criteria:
  - exact does not reopen
  - middle is flat-to-better
  - whole improves or owner visibly moves toward the next narrower seam
- Revert criteria:
  - producer still returns to handle world before the store on the same corridor
  - exact regresses materially
  - whole gets worse while hot symbols remain on the same publication chain
- Stop-line:
  - if the only observable change is owner shift from one helper name to another with no boundary change, revert and do not continue widening

## Phase 2. Cold Publish Effect

- Goal:
  - move publish to one explicit cold effect instead of letting producer/sink helpers materialize implicitly
- Cards:
  - concentrate `objectize_stable_string_box` and `issue_fresh_handle` under explicit publish boundary helpers
  - reduce direct calls into shared materialize sinks from hot helpers
- Touched areas:
  - `crates/nyash_kernel/src/plugin/value_codec/string_materialize.rs`
  - `crates/nyash_kernel/src/exports/string_helpers/materialize.rs`
  - related `perf-observe` counters in `crates/nyash_kernel/src/observe/backend/tls/`
- Expected evidence:
  - publish reason becomes visible as a cold boundary event
  - hot top report loses `objectize` / `issue_fresh_handle` from same-corridor helpers where the card lands
- Keeper criteria:
  - whole improves on repeated windows
  - exact does not regress
  - publish counters move to explicit boundary sites instead of producer helpers
- Revert criteria:
  - publish path is centralized in code only, but hot path still reaches it at the same frequency
  - middle or exact worsens without a whole win
- Stop-line:
  - if boundary reasons are still ambiguous after the counter split, stop and add observability before another behavior card

## Phase 2.5. Read-Side Alias Lane

- Goal:
  - keep `array.get` on a cache-backed alias lane instead of promoting to stable/public on every read
- Cards:
  - split read demand into:
    - `TextReadOnly`
    - `EncodedAlias`
    - `StableObject`
  - keep stable objectize one-shot and cache-backed per cell
  - preserve cheap alias encode across read-heavy whole fronts
- Touched areas:
  - `crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs`
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs`
  - `crates/nyash_kernel/src/plugin/array_runtime_facade.rs`
  - `crates/nyash_kernel/src/plugin/array_handle_cache.rs`
  - `crates/nyash_kernel/src/plugin/runtime_data.rs`
  - `crates/nyash_kernel/src/plugin/map_runtime_facade.rs`
  - `crates/nyash_kernel/src/observe/backend/tls/`
- Expected evidence:
  - common read path no longer allocates a fresh stable object
  - array/map read routes agree on the same three-way outcome contract:
    - `live source`
    - `cached handle`
    - `cold fallback`
  - whole owner moves away from read-side publication/objectize tax
  - exact and middle do not regress while read continuity improves
- Keeper criteria:
  - stable objectize stays cold and cache-backed
  - `array.get` common path stays on `TextReadOnly` / `EncodedAlias`
  - whole improves without reopening exact or middle
- Revert criteria:
  - store-side win is replaced by per-read stable object creation
  - alias encode becomes more expensive than the old public/stable path
  - read contract becomes runtime re-recognition instead of explicit lane split
- Stop-line:
  - if a card cannot explain which reads stay alias-only and which reads demand `StableObject`, stop and fix the contract first

## Phase 3. TextLane Storage

- Goal:
  - specialize array internal residence for text-heavy corridors without changing public array semantics
- Cards:
  - introduce runtime-only `ArrayStorage::TextLane` / equivalent specialized storage
  - degrade to generic storage only when non-text or stable object semantics require it
- Touched areas:
  - `crates/nyash_kernel/src/plugin/array_runtime_facade.rs`
  - `crates/nyash_kernel/src/plugin/array_slot_store.rs`
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs`
  - any new runtime-private array storage module added under `crates/nyash_kernel/src/plugin/`
- Expected evidence:
  - whole top report loses some generic array indirection
  - `array_string_store_kernel_text_slot_at` and related array path become simpler owners
- Keeper criteria:
  - whole gets a real step change, not just noise-band movement
  - exact and middle remain neutral-to-better
  - generic array semantics stay unchanged at the public boundary
- Revert criteria:
  - specialized storage adds branch tax to exact/middle without a whole win
  - degrade path becomes the common path
- Stop-line:
  - if array specialization starts requiring producer-specific by-name routing, stop and redesign the storage boundary first

## Phase 4. MIR Contract / Verifier

- Goal:
  - move legality from helper-name convention to MIR/lowering contract
- Cards:
  - transient/unpublished text result class is represented in MIR or recipe metadata
  - transient-capable consumers are explicit
  - publish boundary is explicit and verifier-visible
- Touched areas:
  - MIR/JoinIR recipe and lowering docs first
  - relevant MIR builder / lowerer code only after the docs and runtime contract are stable
- Expected evidence:
  - lowering can reject early publish on same-corridor store/loopcarry cases
  - helper names no longer carry legality by convention
- Keeper criteria:
  - runtime contract is already proven by earlier phases
  - MIR legality simplifies, rather than re-encoding helper-specific exceptions
- Revert criteria:
  - verifier arrives before runtime consume/publish boundaries are stable
  - legality is encoded as helper-name allowlists
- Stop-line:
  - if MIR work starts inventing a second truth for runtime ownership, stop and fold the contract back into one SSOT

## Evidence Checklist Per Card

- perf:
  - `tools/checks/dev_gate.sh quick`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_string_store 1 3`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_kernel_small 1 3`
- asm:
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_micro_array_string_store 'ny_main' 1`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small 'ny_main' 1`
- counters:
  - producer publication counters for `const_suffix` / `freeze_text_plan_pieces3`

## Post-Proof Cleanup Queue

- gate:
  - the current strict whole reread is now taken and reads reject-side on the updated phase-2.5 lane
  - these are `BoxShape` cleanup cards, not new acceptance-shape cards
- `Cleanup 1`: collapse no-policy `runtime_data` forwarding
  - review `runtime_data.rs -> runtime_data_array_dispatch.rs -> array_runtime_any.rs`
  - prefer `handle_cache::with_map_box` over a second map-only fetch shell when the route is already proven
- `Cleanup 2`: observe counter registration SSOT
  - after the landed borrowed-alias slice, move the next active family only:
    - `store.array.str`
  - remove raw snapshot index knowledge from sink/test mirrors where possible
- `Cleanup 3`: split `BorrowedHandleBox` responsibilities
  - separate only after the owner family points back at read encode again
  - if not, keep this card parked
- `Cleanup 4`: typed handle-cache consolidation
  - decide one owner for typed cache lookup
  - centralize only the live typed-cache routes; do not invent dead-path cleanup that is no longer present in checkout
- `Cleanup 5`: map-key codec SSOT
  - status: landed
  - map-key coercion now uses `CodecProfile::MapKeyBorrowString`
  - array profile naming no longer owns map-key decode policy
- `Cleanup 6`: `MapBox` raw helper boundary
  - status: landed
  - pull `clear` / `delete` style raw mutations behind narrow `MapBox` helpers instead of mutating `get_data().write()` from runtime facade code
  - `clear` / `delete` now share `MapBox::clear_entries` / `MapBox::remove_key_str` across public `MapBox` methods and raw slot mutation leaves
- `Cleanup 7`: legacy map compat surface retirement
  - status: partial
  - landed: `MapBox.size/len/length` lowering now targets canonical `nyash.map.entry_count_i64`; `nyash.map.entry_count_h` remains only as compat export/test/archive residue
  - landed: LL emit map i64-key `get` / `has` routes now target `nyash.map.slot_load_hi` / `nyash.map.probe_hi`; `nyash.map.get_h` / `nyash.map.has_h` remain compat export/test/archive residue
  - landed: RuntimeData field fallback now targets `nyash.map.slot_load_hh` / `nyash.map.slot_store_hhh`; `nyash.map.get_hh` / `nyash.map.set_hh` remain compat export/test/archive residue
  - landed: C-shim map size emission now targets `nyash.map.entry_count_i64`; dead `get_h` / `has_h` C declarations were removed from the active shim
  - landed: Rust `map_compat` exports are no longer re-exported through public `map::*`; compat ABI exports/tests, including `entry_count_h`, live inside `map_compat.rs`
  - landed: `NewBox(ArrayBox)` construction now goes through the ring1 array provider seam; the deprecated builtin ArrayBox fallback is removed
  - landed: `NewBox(MapBox)` construction now goes through the ring1 map provider seam; the deprecated builtin MapBox fallback is removed
  - landed: `NewBox(PathBox)` construction now goes through the ring1 path provider seam; the deprecated builtin PathBox fallback is removed
  - landed: `NewBox(ConsoleBox)` construction now goes through the ring1 console seam; the selfhost fallback remains but the standalone builtin wrapper is removed
  - landed: remaining `builtin_impls` are documented as a fallback quarantine; File/Null/primitive fallbacks are deferred until a separate SSOT owns their removal
  - landed: `DeferredConstSuffix -> kernel_slot_store_hi` regression tests now cover append, existing `StringBox`, and existing borrowed-alias retarget routes
  - retarget remaining lowering/runtime users off deprecated compat map exports
  - then collapse:
    - `map_compat.rs`
    - compat alias surface
    - deprecated builtin factory path (done)
  - slot publish-boundary counters
  - any new explicit publish-effect counters added by the card
- guards:
  - keep direct-set and shared-receiver phase-137x smokes green
  - add one new fixture/smoke only when the card expands acceptance shape

## Rollback Rules

- revert immediately if:
  - exact reopens materially
  - middle contradicts the supposed owner move
  - whole only changes inside the current WSL noise band with no owner shift
- do not keep speculative substrate:
  - no registry-backed transient carrier
  - no string-specialized handle payload retry
  - no generic helper ABI widening before site-local carrier proof
- do not keep permanent legacy/new dual routing after phase-2/3 keepers land

## Stop-Line Summary

- stop after 2 non-keeper cards in the same owner family
- stop if exact / middle / whole are being judged with different baselines
- stop if docs drift from the actual landed seam
- stop if `TextLane` work begins before Phase 1 and Phase 2 are both proven
- stop if publish centralization is attempted before the corridor can already stay unpublished through store
