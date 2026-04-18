# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-18
Scope: current lane / next lane / restart order only.

## Purpose

- root から active lane / next lane に最短で戻る
- landed history と rejected history は phase docs / investigations を正本にする
- `CURRENT_TASK.md` 自体は ledger にしない

## Quick Restart Pointer

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
4. `docs/development/current/main/phases/phase-137x/README.md`
5. `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
6. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
7. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
8. `docs/development/current/main/design/string-birth-sink-ssot.md`
9. `docs/development/current/main/15-Workstream-Map.md`
10. `git status -sb`
11. `tools/checks/dev_gate.sh quick`
12. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md` (`phase-29bq` に戻るときだけ)

## Current Lane

- expected worktree:
  - clean is expected right now
  - rejected slot-store boundary probe is parked separately in `stash@{0}` as `wip/concat-slot-store-window-probe`
- active lane:
  - `phase-137x publication/source-capture reopen after compiler-known-length keeper`
- background lanes:
  - `phase-29bq loop owner seam cleanup landing`
  - `phase-163x primitive-family / user-box fast-path landing`
- current blocker:
  - `none`

## Current Snapshot

- keeper front is still closed:
  - `kilo_micro_substring_concat`
    - `C: 2 ms`
    - `Ny AOT: 3 ms`
  - `kilo_micro_substring_only`
    - `C: 3 ms`
    - `Ny AOT: 3 ms`
- current broad gap is no longer substring:
  - `kilo_micro_array_string_store`
    - `C: 10 ms`
    - `Ny AOT: 132 ms`
  - `kilo_kernel_small_hk`
    - `C: 80 ms`
    - `Ny AOT: 731 ms`
- current reading:
  - current main owner family is `array/string-store`, not `substring`
  - trusted direct MIR no longer duplicates the `text + "xy"` producer across `set(...)` and trailing `substring(...)`
  - runtime gap stayed open after the compiler-side placement fix, so duplicated birth is no longer the live owner
  - latest keeper slice is compiler-side known string-length propagation across const / substring-window / same-length string `phi`
  - active AOT entry IR on this front no longer emits `nyash.string.len_h` inside `ny_main`
  - `Stage A` narrow owner slice is landed on the VM/reference lane:
    - `.hako` `ArrayCoreBox` now routes proven string-handle `set(...)` through `nyash.array.set_his`
    - cold tail stays in Rust
  - `Stage A` exact reread is now closed and parked on the active AOT front:
    - active AOT already reaches the current concrete `store.array.str` lowering without that VM/reference pilot
  - latest locked exact `perf-observe` counters on `kilo_micro_array_string_store` show:
    - `store.array.str total=800000`
    - `cache_hit=800000`
    - `plan.action_retarget_alias=800000`
    - `plan.action_store_from_source=0`
    - `plan.action_need_stable_object=0`
    - `carrier_kind.source_keep=0`
    - `carrier_kind.owned_bytes=1600000`
    - `carrier_kind.stable_box=1600000`
    - `carrier_kind.handle=1600000`
    - `publish_reason.generic_fallback=1600000`
  - trusted direct MIR on the same benchmark still carries generic `RuntimeDataBox.set(...)` / `substring(...)` calls
  - active AOT lowering is now confirmed separately:
    - direct MIR stays generic
    - entry LLVM IR still concretizes the array string-store call to `nyash.array.set_his`
    - guard: `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh`
  - therefore the landed `.hako` owner pilot is still VM/reference-lane only; active AOT already reaches the current concrete `store.array.str` lowering without that pilot
  - slot-store boundary delayed-publication probes were tried and rejected:
    - active slot route v1:
      - `kilo_micro_array_string_store = 252 ms`
      - `kilo_kernel_small_hk = 765 ms`
    - active slot route v2:
      - `kilo_micro_array_string_store = 211 ms`
      - `kilo_kernel_small_hk = 1807 ms`
    - producer-side unpublished-outcome active probe:
      - `kilo_micro_array_string_store = 236 ms`
      - `kilo_kernel_small_hk = 2173 ms`
    - the bad cut was the array-store boundary itself; it bypassed the existing `set_his` fast path / alias-retarget behavior
    - the same producer-side slot route is now rejected on this lane too; it regressed exact and whole while altering the active boundary shape
  - helper-only keepers on this lane are:
    - `b35382cf9 feat: add kernel text slot store helpers`
    - runtime-side alias-retarget repair for kernel-slot store into existing string slots
- latest `perf-observe` reread on the active array-store front no longer ranks `string_len_export_slow_path`; top samples stay on:
  - `issue_fresh_handle`
  - `freeze_owned_bytes`
  - `capture_store_array_str_source`
  - `StringBox::perf_observe_from_owned`
- latest observability split made `lookup_array_store_str_source_obj` visible as its own hot symbol; source-capture is now split enough to compare lookup vs proof-shaping vs publish tail
- latest clean-baseline evidence split is now pinned:
  - source lane is `BoxShape`, not `BoxCount`
  - `lookup_array_store_str_source_obj = 9.60%`
  - `materialize_verified_text_source = 2.05%`
  - proof classify is now noise on this front
  - remaining source-capture blind spot is *inside* `lookup_array_store_str_source_obj`:
    - registry read / slot lookup
    - caller / latest-fresh tagging
- latest clean-baseline publish evidence is now pinned:
  - publish lane is `BoxShape`, not `BoxCount`
  - `freeze_owned_bytes = 15.98%`
  - `issue_fresh_handle = 14.53%`
  - `StringBox::perf_observe_from_owned = 7.25%`
  - current `carrier_kind` / `publish_reason` counters still collapse the producer sites together
- observation-only split is now landed on the perf-observe lane:
  - source lookup now exposes:
    - `lookup.registry_slot_read`
    - `lookup.caller_latest_fresh_tag`
  - publish tail now exposes:
    - `site.string_concat_hh.materialize_owned_total / bytes`
    - `site.string_concat_hh.objectize_box_total`
    - `site.string_concat_hh.publish_handle_total`
    - `site.string_substring_concat_hhii.materialize_owned_total / bytes`
    - `site.string_substring_concat_hhii.objectize_box_total`
    - `site.string_substring_concat_hhii.publish_handle_total`
  - latest raw exact reread on `kilo_micro_array_string_store` (observe build; split-only, not release truth) shows:
    - `lookup.registry_slot_read=800000`
    - `lookup.caller_latest_fresh_tag=800000`
    - `site.string_concat_hh.materialize_owned_total=800000`
    - `site.string_concat_hh.materialize_owned_bytes=14400000`
    - `site.string_concat_hh.objectize_box_total=800000`
    - `site.string_concat_hh.publish_handle_total=800000`
    - `site.string_substring_concat_hhii.materialize_owned_total=800000`
    - `site.string_substring_concat_hhii.materialize_owned_bytes=12800000`
    - `site.string_substring_concat_hhii.objectize_box_total=800000`
    - `site.string_substring_concat_hhii.publish_handle_total=800000`
- latest raw whole reread on `kilo_kernel_small_hk` / `bench_kilo_kernel_small.hako` (observe build; split-only, not release truth) shows a different owner family:
  - `Result: 1140576`
  - `store.array.str total=540000`
  - `lookup.registry_slot_read=540000`
  - `lookup.caller_latest_fresh_tag=540000`
  - `const_suffix total=480000`
  - `const_suffix freeze_fallback=479728`
  - `const_suffix cached_fast_str_hit=272`
  - `freeze_text_plan_pieces3=60000`
  - `publish_reason.generic_fallback=539728`
  - `site.string_concat_hh.*=0`
  - `site.string_substring_concat_hhii.*=0`
  - current whole-kilo owner is therefore **const_suffix / pieces3 producer publication**, not the pair/substring helper sites from the exact micro front
- latest exact asm diff is now pinned:
  - C side is still a tight `main` loop plus `strlen`
  - Ny AOT hot symbols are:
    - `string_substring_concat_hhii_export_impl`
    - `string_concat_hh_export_impl`
    - array string-store closure
    - `from_i8_string_const`
  - hottest Ny AOT blocks sit inside helper-side branch / indirect-call / sync-heavy paths, not inside the array store leaf itself
- latest publish-asm mapping is now pinned:
  - loop shape is `concat_hh -> set_his -> substring_concat_hhii`
  - producer side still pays:
    - `freeze_owned_bytes`
    - generic publish boundary
    - `issue_fresh_handle`
  - current largest C-vs-Ny mismatch is **producer helper publication**, not array-store mutation
- latest AOT-only `const_suffix` direct-store probe is **not** a keeper and has been reverted:
  - direct MIR / direct-emit contract stayed green
  - `kilo_micro_array_string_store = 138 ms` (baseline 132 ms)
  - `kilo_kernel_small_hk = 1141 ms` (baseline 731 ms, probe-only 1x reread)
  - current conclusion: this route widens hot-path cost elsewhere and should stay parked
- latest runtime-fix-only reread stays in the same owner family:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 132 ms`
  - `kilo_kernel_small_hk = C 80 ms / Ny AOT 731 ms`
- current live owner remains publication/source-capture around the string births, not array-set route selection
- next comparison must split:
    - implementation language cost
    - protocol / seam cost
  - compiler-known-length keeper is landed; next slice is no longer `len_h` removal but publication/source-capture reopen while keeping that lane fixed
  - latest design consult is accepted in narrowed form:
    - no syntax expansion
    - no public raw string / mutable bytes
    - keep `const_suffix` as a future narrow probe, not the immediate active widening
    - reuse existing runtime-private `TextPlan` / `OwnedBytes` seams if the next probe needs them
  - `indexOf` stays a side diagnostic lane and is not the current keeper card

## Next

1. keep `Stage A` parked as VM/reference-only and stop spending exact-front time on owner-route widening
2. keep the compiler-known-length lane fixed and guarded on `kilo_micro_array_string_store`
3. keep array-store route selection parked; the exact-front owner is still the **producer helper publish tail**
4. keep the reverted AOT-only `const_suffix` direct-store corridor parked as a non-keeper; do not reopen it before new evidence
5. next slice stays evidence-first, but exact and whole are now split:
   - exact micro owner: common generic publish/objectize corridor shared by `string_concat_hh` and `string_substring_concat_hhii`
   - whole kilo owner: `const_suffix` fallback plus `freeze_text_plan_pieces3` publication
6. do not spend the next keeper card on pair/substring helper specialization; whole-kilo counters prove those sites are inactive there
7. next observation gap, before a keeper probe on whole kilo:
   - split publish bytes/stages for `const_suffix` vs `freeze_text_plan(Pieces3)` so the whole front can choose the first structural keeper without guessing from counts alone
8. after that reread:
   - `kilo_micro_array_string_store`
   - `kilo_kernel_small_hk`
   - top asm on the active whole producer helper (`const_suffix` / pieces3 path) plus the exact-front publish tail
9. only after the observation split proves one specific producer stage, reopen a new narrow keeper candidate on that stage alone

## Guardrails

- MIR/lowering still owns legality, `proof_region`, and `publication_boundary`
- keep carrier/publication split physically narrow
- do not widen this card into a generic slot API or helper substrate
- keep public ABI stable
- do not add syntax or public raw-string carriers on this card
- compare `Rust vs .hako` only under:
  - same protocol
  - same public ABI with different internal seam

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
cargo test -p nyash_kernel --lib --tests --no-run
```

## Detail Pointers

- current evidence snapshot:
  - `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
- history / rejects / longer ledger:
  - `docs/development/current/main/phases/phase-137x/README.md`
- design anchors:
  - `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
  - `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
  - `docs/development/current/main/design/string-birth-sink-ssot.md`
