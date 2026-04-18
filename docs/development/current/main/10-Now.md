---
Status: SSOT
Date: 2026-04-19
Scope: current lane / blocker / next pointer only.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md
  - docs/development/current/main/design/string-value-model-phased-rollout-ssot.md
  - docs/development/current/main/phases/phase-137x/phase137x-text-lane-rollout-checklist.md
---

# Self Current Task — Now (main)

## Current

- current optimization lane:
  - `phase-137x publication/source-capture reopen after compiler-known-length keeper`
  - execution mode:
    - phased value-model rollout
- background compiler lanes:
  - `phase-29bq loop owner seam cleanup landing`
  - `phase-163x primitive-family / user-box fast-path landing`
- blocker:
  - `none`

## Snapshot

- keeper front stays closed:
  - `kilo_micro_substring_concat = C 2 ms / Ny AOT 3 ms`
  - `kilo_micro_substring_only = C 3 ms / Ny AOT 3 ms`
- exact `store.array.str` front is now closed:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`
  - reading:
    - the shared-receiver `KernelTextSlot` bridge is a keeper
    - microasm top is startup/env dominated, so this exact front is no longer the active owner proof
- current bridge front:
  - `kilo_meso_substring_concat_array_set_loopcarry`
  - shape: `substring + concat + array.set + loopcarry`
  - role: adopted middle between exact micro and whole kilo
  - rule: use it to validate store/publication cuts without the whole-front `indexOf("line")` row-scan noise
- current bridge reread after the shared-receiver landing:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 57 ms`
  - reading:
    - still inside the prior `56-59 ms` band
    - producer-side unpublished outcome widening stays live, but this landing is not a meso keeper by itself
- current whole accept gate:
  - `kilo_kernel_small`
  - current reread result: `C 80 ms / Ny AOT 739 ms` (`repeat=3`)
  - reading:
    - pure-first AOT build shape stays reopened; direct/helper replay still compile after the helper declaration/need-flag fixes
    - loop-body `KernelTextSlot` allocas no longer crash the whole bench after `stacksave/stackrestore`
    - whole is still not a whole-front keeper, but the remaining owner is now pinned more tightly
    - emitted LLVM IR now proves the two hot whole-bench store sites already lower to:
      - direct-set-only `insert_hsi -> kernel_slot_insert_hsi -> kernel_slot_store_hi`
      - direct-set-only `current + "ln" -> kernel_slot_concat_hs -> kernel_slot_store_hi`
    - perf/asm reread says the next owner is materialization/copy tax, not compiler fallback:
      - `array_string_store_kernel_text_slot_at 5.99%`
      - `objectize_kernel_text_slot_stable_box 1.14%`
      - `insert_const_mid_into_slot 1.64%`
      - `nyash.string.kernel_slot_concat_hs 0.60%`
      - libc `memmove 19.48%` / `_int_malloc 5.05%`
    - observability split now pins the whole owner one step further upstream:
      - `const_suffix freeze_fallback = 479728 / 480000`
      - `materialize total = 539728` (`~4.5 GB`)
      - `publish_reason.generic_fallback = 539728`
      - `site.string_concat_hh.* = 0`
      - `site.string_substring_concat_hhii.* = 0`
      - reading:
        - the whole-front owner is still `const_suffix` freeze fallback, not a reopened generic concat/substr site
        - the next card is deferred `const_suffix` residence under the current `KernelTextSlot` ABI
- accepted phased rollout order:
  - semantic lock:
    - `String = value`
    - `publish = boundary effect`
    - `freeze.str = only birth sink`
  - `Phase 1`: producer outcome -> canonical sink with existing carriers
    - `VerifiedTextSource`
    - `TextPlan`
    - `OwnedBytes`
    - `KernelTextSlot`
  - `Phase 2`: cold publish effect
  - `Phase 2.5`: read-side alias lane split
  - `Phase 3`: future `TextLane` storage specialization
  - `Phase 4`: MIR legality and sink-aware AOT
- current phase-2 start is now landed structurally:
  - `string_handle_from_owned{,_concat_hh,_substring_concat_hhii,_const_suffix}` enter explicit cold publish adapters
  - `publish_owned_bytes_*_boundary` / `objectize_kernel_text_slot_stable_box` are outlined as cold boundaries
  - latest reread stays `exact closed / whole neutral`:
    - `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
    - `kilo_kernel_small = C 81 ms / Ny AOT 768 ms`
  - reading:
    - owner family is still publication/source-capture
    - next phase-2 card must reduce publish frequency, not only outline the same boundary
- latest phase-2 source-capture prework is now landed:
  - `with_array_store_str_source(...)` checks a latest-fresh stable-box cache before registry slot lookup
  - cache validity is guarded by `drop_epoch`
  - latest reread remains `exact closed / whole neutral`:
    - `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
    - `kilo_kernel_small = C 80 ms / Ny AOT 1068 ms`
  - reading:
    - same owner family remains live
    - treat this as valid prework, not a keeper
    - legacy coexistence is temporary; remove legacy dual routing after the new path proves out
- latest phase-2 store-side narrow cut is now landed:
  - `kernel_slot_store_hi` overwrites an existing `StringBox` array slot in place instead of replacing the outer box
  - latest reread stays `exact closed / whole neutral`:
    - `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
    - `kilo_kernel_small = C 80 ms / Ny AOT 781 ms`
  - reading:
    - this is a safe runtime-private cut, not a keeper by itself
    - the next card stays on producer materialization (`kernel_slot_concat_hs`, then `insert_const_mid_into_slot`)
- latest phase-2 materialize cut is now landed:
  - `kernel_slot_concat_hs` now prefers borrowed-text direct materialization under `with_text_read_session_ready(...)`
  - `insert_const_mid_into_slot` now takes the same borrowed-text direct path before owned fallback
  - latest reread:
    - `kilo_micro_array_string_store = C 9 ms / Ny AOT 3 ms`
    - `kilo_kernel_small = C 80 ms / Ny AOT 739 ms`
    - `kilo_kernel_small_hk = C 79 ms / Ny AOT 748 ms` (`strict`, parity ok)
  - reading:
    - exact stays closed
    - whole moved in the right direction on both plain and strict rereads
    - keep the lane open until that better band proves keeper-grade stability
- latest phase-2 deferred `const_suffix` slot cut is now landed:
  - `kernel_slot_concat_hs` can now leave a deferred `const_suffix` state inside the existing `KernelTextSlot` layout
  - `kernel_slot_store_hi` consumes that state before generic freeze/objectize
  - existing `StringBox` array slots append in place when the deferred source still matches the current slot text
  - latest reread:
    - `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
    - `kilo_kernel_small = C 79 ms / Ny AOT 726 ms`
    - `kilo_kernel_small_hk = C 81 ms / Ny AOT 808 ms` (`strict`, parity ok)
  - reading:
    - exact stays closed
    - plain whole improved again versus the prior `739 ms` reread
    - strict whole still needs a stability reread before this becomes a keeper
- rejected follow-up probe:
  - replacing BorrowedHandleBox unpublished retarget objectization with an owned-string keep regressed whole:
    - `kilo_kernel_small = C 81 ms / Ny AOT 980 ms`
    - `kilo_kernel_small_hk = C 80 ms / Ny AOT 1015 ms`
  - reason:
    - `array.get` / borrowed-alias encode fallback began allocating a fresh stable object on every read
    - the store-side win was smaller than the new read-side loss
  - restored reread after reverting the probe:
    - `kilo_kernel_small = C 81 ms / Ny AOT 810 ms`
    - `kilo_kernel_small_hk = C 82 ms / Ny AOT 864 ms`
  - next seam must preserve cheap alias encode on read; `owned-string keep` is not the keeper
  - next card is read-side alias lane split:
    - `TextReadOnly`
    - `EncodedAlias`
    - `StableObject`
    - stable objectize stays cold and cache-backed, not per-read
  - first phase 2.5 slice is now landed:
    - `BorrowedHandleBox` caches the encoded runtime handle for unpublished keeps
    - `array.get` can reuse the cached stable handle instead of fresh-promoting on every read
    - latest strict reread: `kilo_kernel_small_hk = C 79 ms / Ny AOT 791 ms` (`repeat=3`, parity ok)
  - latest phase 2.5 follow-on slices are now landed:
    - map value stores preserve borrowed string aliases through `CodecProfile::MapValueBorrowString`
    - borrowed-alias runtime-handle cache is shared across alias lineage, so map reads do not drop the cached encoded handle when the read path clones the alias box
    - `perf-observe` now splits read-side alias outcomes by caller for both:
      - `array.get`
      - `runtime_data` map reads
    - locked outcomes:
      - `live source`
      - `cached handle`
      - `cold fallback`
  - latest strict reread on the updated lane:
    - `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
    - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 61 ms`
    - `kilo_kernel_small_hk = C 82 ms / Ny AOT 809 ms`
    - `kilo_kernel_small_hk = C 80 ms / Ny AOT 892 ms`
  - reading:
    - phase 2.5 runtime contract is now fixed more tightly than the first `array.get`-only slice
    - exact stays closed, but meso / strict whole reopened upward versus the prior keeper-candidate band
    - current read is reject-side, so the next step is BoxShape cleanup on this proven lane, not a new `TextLane` / MIR legality card
- phase/task anchors:
  - `docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md`
  - `docs/development/current/main/design/string-value-model-phased-rollout-ssot.md`
  - `docs/development/current/main/phases/phase-137x/phase137x-text-lane-rollout-checklist.md`
- `indexOf` separation:
  - keep as side diagnosis; reread only when the main card reopens it
- completed audit lock (confirmed evidence):
  - exact audit: top samples are `substring_concat_hhii_export_impl 22.38%`, `string_concat_hh_export_impl 21.70%`, array string-store closure `17.34%`, `from_i8_string_const 13.07%`, `LocalKey::with 6.07%`, `memmove 3.51%`, `_int_malloc 1.75%`; wrapper names are not the live owner, current evidence points to inner publication / object-world entry
  - whole audit: top user symbols are `nyash.string.concat_hs 11.19%`, `execute_store_array_str_contract` closure `7.01%`, `insert_const_mid_fallback` closure `3.89%`, `array_get_index_encoded_i64` closure `3.62%`, `from_i8_string_const 3.52%`, libc `memmove 14.92%`, `_int_malloc 4.65%`; `concat_hs` hot instructions are TLS/helper-entry, not copy body
  - observability audit: the generic-fallback split is now locked by site-specific noinline symbols in `crates/nyash_kernel/src/plugin/value_codec/string_materialize.rs`; tests passed with and without `perf-observe`
  - choice rule: perf/asm is now sufficient to choose the next keeper without another broad observability round
  - current owner reading:
  - exact `array/string-store` is now closed
  - live next owner family is upstream producer publication on whole
  - duplicated `text + "xy"` producer is already removed in trusted direct MIR
  - compiler-side known string-length propagation is now landed for const / substring-window / same-length string `phi`
  - active AOT entry IR on this front no longer emits `nyash.string.len_h` in `ny_main`
  - current exact owner is still publication/source-capture
  - current exact/meso/whole split is now explicit:
    - `kilo_micro_array_string_store` is dominated by the shared generic publish/objectize corridor behind `string_concat_hh` + `string_substring_concat_hhii`
    - `kilo_meso_substring_concat_array_set_loopcarry` is the adopted bridge front for the same store/publication corridor without whole-front `indexOf("line")` noise
    - `kilo_kernel_small` is dominated by `const_suffix` fallback plus `freeze_text_plan(Pieces3)` publication
  - hot-corridor carrier design is now fixed separately:
    - `docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md`
  - current code pick stays producer-first:
    - active owner is still upstream producer publication on whole
    - first implementation target stays corridor-local:
      - `const_suffix -> KernelTextSlot`
      - `KernelTextSlot -> store.array.str`
      - same producer contract may also feed trailing `substring(...)` before any publish boundary
    - keep this landing corridor-local; do not widen generic helper ABI
    - compiler/backend consumption is landed for:
      - direct-set-only `const_suffix -> set(...)`
      - narrow shared-receiver exact widening:
        - `text + "xy"` reused by `set(...)` + known-length observer + trailing `substring(...)`
    - producer stays specialized; only the internal contract to sink/reuse is widened
    - next widening target is fixed:
      - direct-set-only `insert_const_mid_fallback` / `insert_hsi` is now landed on the same unpublished contract
      - direct-set-only deferred `Pieces3 substring` is now also landed on the same unpublished contract
      - next widening, if needed, is post-store reuse / non-direct-set `Pieces3`
      - keep the same unpublished contract and do not reopen generic helper ABI widening
    - before Card A/B, slot publish-boundary verifier/counters are now landed:
      - `publish_boundary.slot_publish_handle_total`
      - `publish_boundary.slot_objectize_stable_box_total`
      - `publish_boundary.slot_empty`
      - `publish_boundary.slot_already_published`
      - `objectize_kernel_text_slot_stable_box` now records `publish_reason.need_stable_object`
      - latest exact / meso / whole reread shows these slot-boundary counters remain `0`; slot exit is observed and inactive on the live fronts
    - middle remains the contradiction guard; if producer-side unpublished outcome does not lift meso, reopen `substring_hii -> borrowed_substring_plan_from_handle`
    - latest local probe after landing the cold retirement sink:
      - `kilo_meso_substring_concat_array_set_loopcarry = 53 ms` (`repeat=3`)
      - `kilo_kernel_small_hk = 733 ms`, `736 ms` (`repeat=3` x2)
      - treat this as `neutral whole / slight meso` until a wider reread proves the whole-front win
    - rejected probe:
      - direct `StringBox -> handle` publish plus string-specialized host-handle payload
      - `kilo_meso_substring_concat_array_set_loopcarry = 68 ms`
      - `kilo_kernel_small = 950 ms`
      - reverted; this seam is not the owner
  - exact hot instructions carry host-handle atomics, TLS publish stores, alloc shim calls, and array-store handle/publication branches
  - exact loop still pays extra per-iter helper calls vs C: `from_i8_string_const` x2, `concat_hh` x1, `set_his` x1, `substring_concat_hhii` x1
  - whole hot closures still pay registry fetch, `lock cmpxchg`, vtable probes, and handle/cache publication before store completion
  - `Stage A` narrow owner slice is now landed on the VM/reference lane:
    - `.hako` `ArrayCoreBox` routes proven string-handle `set(...)` through `nyash.array.set_his`
    - same protocol, same cold Rust tail
  - `Stage A` exact reread is now closed on the active AOT front:
    - `store.array.str total=800000`
    - `plan.action_retarget_alias=800000`
    - `plan.action_store_from_source=0`
    - `carrier_kind.source_keep=0`
    - `publish_reason.generic_fallback=1600000`
  - trusted direct MIR still carries generic `RuntimeDataBox.set(...)` / `substring(...)`
  - active AOT lowering is separately locked:
    - direct MIR stays generic
    - entry LLVM IR still calls `nyash.array.set_his`
    - guard: `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh`
  - new boundary direct-set-only guard is now locked for the narrow bridge:
    - fixture: `apps/tests/mir_shape_guard/string_const_suffix_kernel_slot_direct_set_min_v1.mir.json`
    - guard: `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_const_suffix_kernel_slot_store_contract.sh`
  - active AOT exact is therefore not the `.hako` owner pilot itself; the live owner stays publication/source-capture
  - slot-store boundary probes are now a rejected card:
    - v1 exact/whole: `252 ms / 765 ms`
    - v2 exact/whole: `211 ms / 1807 ms`
    - they cut at the wrong seam and broke the existing `set_his` fast path
  - a producer-side unpublished-outcome active probe is also rejected:
    - exact/whole: `236 ms / 2173 ms`
    - it regressed both fronts while changing the active boundary shape
  - helper-side keepers from these rejected cards are:
    - `b35382cf9`
    - runtime-side alias-retarget repair for kernel-slot store into existing string slots
  - latest `perf-observe` reread no longer ranks `string_len_export_slow_path`; the live top stays on `issue_fresh_handle` / `freeze_owned_bytes` / `capture_store_array_str_source` / `StringBox::perf_observe_from_owned`
  - latest observability split landed and is now pinned:
    - `lookup.registry_slot_read`
    - `lookup.caller_latest_fresh_tag`
    - `site.string_concat_hh.{materialize_owned_total/materialize_owned_bytes/objectize_box_total/publish_handle_total}`
    - `site.string_substring_concat_hhii.{materialize_owned_total/materialize_owned_bytes/objectize_box_total/publish_handle_total}`
    - `publish_boundary.slot_{publish_handle_total,objectize_stable_box_total,empty,already_published}`
  - latest raw exact observe reread on `kilo_micro_array_string_store` shows:
    - `lookup.registry_slot_read=800000`
    - `lookup.caller_latest_fresh_tag=800000`
    - `site.string_concat_hh.materialize_owned_total=800000`
    - `site.string_substring_concat_hhii.materialize_owned_total=800000`
  - latest raw whole observe reread on `kilo_kernel_small_hk` shows:
    - `const_suffix freeze_fallback=479728`
    - `freeze_text_plan_pieces3=60000`
    - `publish_reason.generic_fallback=539728`
    - `site.string_concat_hh.*=0`
    - `site.string_substring_concat_hhii.*=0`
  - latest runtime-fix-only reread stays on the same owner family:
    - `kilo_micro_array_string_store = C 10 ms / Ny AOT 127 ms`
    - `kilo_kernel_small_hk = C 81 ms / Ny AOT 755 ms`
  - current whole-owner reread is now pinned:
    - first owner = `const_suffix` / `nyash.string.concat_hs`
    - secondary guard = `freeze_text_plan(Pieces3)` / `insert_hsi`
  - latest asm read:
    - `ny_main` loop shape is already close to C
    - the remaining gap is helper-entry branch / TLS / generic publication tail inside helper bodies
  - next first slice is no longer `len_h` removal; it is publication/source-capture reopen with the compiler-known-length lane fixed
  - evidence points to publication/object-world entry as the live owner on both fronts; this does not yet prove any representation / ABI change
  - latest design consult is accepted in narrowed form:
    - no syntax expansion
    - no public raw string / mutable bytes
    - keep the next widening inside runtime-private `const_suffix` / `TextPlan(Pieces3)` publication only
    - if publication timing wins, reuse existing runtime-private `TextPlan` / `OwnedBytes` seams first

## Next

1. continue `Phase 2`
   - keep publish as an explicit cold effect
   - next slice must reduce publish/source-capture frequency, not just outline it
2. preserve the current guards during phase 2
   - exact stays closed
   - middle stays the contradiction gate
   - preserve the existing `set_his` fast path
3. defer `Phase 3`
   - do not introduce `TextLane` before producer/sink split is proven
4. defer `Phase 4`
   - do not raise MIR legality before runtime consume/publish boundaries are stable

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
3. `docs/development/current/main/phases/phase-137x/README.md`
4. `docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md`
5. `docs/development/current/main/design/string-value-model-phased-rollout-ssot.md`
6. `docs/development/current/main/phases/phase-137x/phase137x-text-lane-rollout-checklist.md`
7. `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
8. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
9. `docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md`
10. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
11. `docs/development/current/main/design/string-birth-sink-ssot.md`
12. `docs/development/current/main/15-Workstream-Map.md`

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
cargo test -p nyash_kernel --lib --tests --no-run
```
