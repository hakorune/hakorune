---
Status: Active
Date: 2026-04-18
Scope: phase-137x で `public handle ABI` を維持したまま、text hot corridor を `producer -> sink -> publish` の値モデルへ段階導入する taskboard。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md
  - docs/development/current/main/design/string-value-model-phased-rollout-ssot.md
---

# Phase 137x Text Lane Rollout Checklist

- North Star:
  - `Public world`: `StringHandle` / `ArrayHandle` / `Box<dyn NyashBox>`
  - `Execution world`: `VerifiedTextSource -> TextPlan -> OwnedTextBuf -> TextCell/TextLane`
  - rule: publish は escape 時の effect に限定する
- Current locked reading:
  - exact front is closed by the shared-receiver `KernelTextSlot` bridge
  - middle contradiction guard remains `kilo_meso_substring_concat_array_set_loopcarry`
  - whole owner remains upstream producer publication plus array/string slot work
  - current whole-side landed cuts are still narrow:
    - direct-set-only `const_suffix -> KernelTextSlot -> kernel_slot_store_hi`
    - shared-receiver `const_suffix` reuse on exact front
    - direct-set-only `insert_hsi`
    - direct-set-only deferred `Pieces3 substring`

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
- prefer existing repo-local shapes first:
  - `VerifiedTextSource`
  - `TextPlan`
  - `OwnedBytes`
  - `KernelTextSlot`

## Phase 0. Baseline Lock

- Goal:
  - fix the current truth before widening the value model
- Touched areas:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
  - perf artifacts under `target/perf_state/phase137x-*`
- Expected evidence:
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

## Phase 2. Canonical Sink Contract

- Goal:
  - promote `KernelTextSlot` from special case to canonical text sink residence
- Cards:
  - `store.array.str` accepts unpublished producer outcome as the normal sink path
  - shared-receiver reuse continues to read from slot/local residence before publish
- Touched areas:
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs`
  - `crates/nyash_kernel/src/plugin/value_codec/string_store.rs`
  - `crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs`
  - `crates/nyash_kernel/src/plugin/value_codec/string_materialize.rs`
  - compiler lowering under `lang/c-abi/shims/`
- Expected evidence:
  - same-corridor `const_suffix` / `Pieces3` store routes terminate at slot residence
  - `set_his` fast path and alias-retarget legality stay intact
  - exact shared-receiver reuse remains green
- Keeper criteria:
  - exact remains closed
  - middle improves through fewer transitions or at minimum stays neutral
  - whole hot path shows more `kernel_slot_*` and less eager materialize/publish
- Revert criteria:
  - slot bridge only moves cost into a later forced publish without reducing total work
  - `set_his` fast path or alias-retarget contract breaks
- Stop-line:
  - if slot-boundary counters stay zero and whole owner still sits entirely upstream, do not widen the sink contract further before re-reading the producer side

## Phase 3. Cold Publish Effect

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

## Phase 4. TextLane Storage

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

## Phase 5. MIR Contract / Verifier

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

## Stop-Line Summary

- stop after 2 non-keeper cards in the same owner family
- stop if exact / middle / whole are being judged with different baselines
- stop if docs drift from the actual landed seam
- stop if `TextLane` work begins before `producer outcome` and `canonical sink` are both proven
- stop if publish centralization is attempted before the corridor can already stay unpublished through store
