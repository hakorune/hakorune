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
7. `docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md`
8. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
9. `docs/development/current/main/design/string-birth-sink-ssot.md`
10. `docs/development/current/main/15-Workstream-Map.md`
11. `git status -sb`
12. `tools/checks/dev_gate.sh quick`
13. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md` (`phase-29bq` に戻るときだけ)

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
- current bridge front is now adopted:
  - `kilo_meso_substring_concat_array_set_loopcarry`
    - shape: `substring + concat + array.set + loopcarry`
    - role: natural middle between exact micro and whole kilo on this lane
    - rule: use it to confirm store/publication cuts without the whole-front `indexOf("line")` row-scan noise
- current broad gap is no longer substring:
  - `kilo_micro_array_string_store`
    - `C: 10 ms`
    - `Ny AOT: 131 ms`
  - `kilo_kernel_small`
    - `C: 80 ms`
    - `Ny AOT: 741 ms`
- latest completed audit lock (confirmed evidence; prefer this block if older notes below differ):
  - exact asm/perf audit on `kilo_micro_array_string_store`:
    - top samples: `substring_concat_hhii_export_impl 22.38%`, `string_concat_hh_export_impl 21.70%`, array string-store closure `17.34%`, `from_i8_string_const 13.07%`, `LocalKey::with 6.07%`, `memmove 3.51%`, `_int_malloc 1.75%`
    - hottest instructions carry host-handle atomics (`lock xadd/cmpxchg/inc/dec`), TLS publish stores, alloc shim calls, and array-store handle/publication branches
    - extra loop-hot calls per iter vs C: `from_i8_string_const` x2, `concat_hh` x1, `set_his` x1, `substring_concat_hhii` x1
    - wrapper names are not the live owner; current evidence points to inner publication / object-world entry
  - whole asm/perf audit on `kilo_kernel_small`:
    - top user symbols: `nyash.string.concat_hs 11.19%`, `execute_store_array_str_contract` closure `7.01%`, `insert_const_mid_fallback` closure `3.89%`, `array_get_index_encoded_i64` closure `3.62%`, `from_i8_string_const 3.52%`, libc `memmove 14.92%`, `_int_malloc 4.65%`
    - `concat_hs` hot instructions are TLS/helper-entry, not the copy body
    - `insert_const_mid_fallback` and array store/read closures spend samples on registry fetch, `lock cmpxchg`, vtable probes, and handle/cache publication
    - whole-path cost still crosses many helper boundaries before store completion
  - observability-gap audit is complete:
    - prior evidence was not enough to split generic-fallback boundary cost from its children
    - landed observability-only patch in `crates/nyash_kernel/src/plugin/value_codec/string_materialize.rs`
    - new site-specific noinline generic-fallback boundary symbols: `string_concat_hh`, `string_substring_concat_hhii`, `const_suffix`, `freeze_text_plan_pieces3`
    - tests passed with and without `perf-observe`
    - perf/asm is now sufficient to choose the next keeper without another broad observability round
- current reading:
  - current main owner family is `array/string-store`, not `substring`
  - hot-corridor carrier design anchor is now:
    - `docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md`
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
    - `site.const_suffix.materialize_owned_total / bytes`
    - `site.const_suffix.objectize_box_total`
    - `site.const_suffix.publish_handle_total`
    - `site.freeze_text_plan_pieces3.materialize_owned_total / bytes`
    - `site.freeze_text_plan_pieces3.objectize_box_total`
    - `site.freeze_text_plan_pieces3.publish_handle_total`
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
    - `site.const_suffix.*=0`
    - `site.freeze_text_plan_pieces3.*=0`
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
  - `site.const_suffix.materialize_owned_total=479728`
  - `site.const_suffix.materialize_owned_bytes=4054750776`
  - `site.const_suffix.objectize_box_total=479728`
  - `site.const_suffix.publish_handle_total=479728`
  - `site.freeze_text_plan_pieces3.materialize_owned_total=60000`
  - `site.freeze_text_plan_pieces3.materialize_owned_bytes=506895016`
  - `site.freeze_text_plan_pieces3.objectize_box_total=60000`
  - `site.freeze_text_plan_pieces3.publish_handle_total=60000`
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
- latest lazy published-string handle seam is **not** a keeper and has been reverted:
  - it removed eager objectize counters as intended, but did not reduce real whole-front cost
  - exact micro regressed to `137-139 ms`
  - whole kilo regressed to `1888-2250 ms`
  - current conclusion: delaying `StableBoxNow` alone is insufficient; do not reopen this seam without new producer-boundary evidence
- latest runtime-fix-only reread stays in the same owner family:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 132 ms`
  - `kilo_kernel_small_hk = C 80 ms / Ny AOT 731 ms`
- latest reverted-baseline validation on the current tree:
  - probe reread #1: `kilo_micro_array_string_store = 143 ms`, `kilo_kernel_small_hk = 862 ms`
  - probe reread #2: `kilo_micro_array_string_store = 130 ms`, `kilo_kernel_small_hk = 750 ms`
  - treat the lazy-handle branch as removed; current tree is back in the prior release range, with normal bench noise
- latest 3-run release reread on the reverted baseline:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 127 ms`
  - `kilo_kernel_small_hk = C 81 ms / Ny AOT 755 ms`
  - current WSL noise band is still real, so judge future keepers on repeated 3-run windows rather than single probes
- latest narrow `const_suffix` hot/cold split is now landed as an executor-local cut:
  - public ABI stays unchanged
  - `const_adapter.rs` now keeps the source-read / concat work on the helper edge and sends owned-result publication through a dedicated cold adapter
  - shared generic materialize paths stay unchanged; the `const_suffix` split is isolated to its site-specific sink
  - release validation on the landed cut:
    - `cargo check --features perf-observe -p nyash_kernel`
    - `cargo test -q -p nyash_kernel --lib -- --test-threads=1`
    - `tools/checks/dev_gate.sh quick`
    - `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh`
  - latest 3-run release numbers on the landed cut:
    - `kilo_micro_array_string_store = C 9 ms / Ny AOT 131-132 ms` (repeat window on this tree)
    - `kilo_kernel_small_hk = C 82 ms / Ny AOT 737 ms`
  - latest whole asm/perf reread shows the owner moved:
    - `nyash.string.concat_hs` is down to ~2-3% in whole
    - new top user-space owner is `materialize::publish_const_suffix_owned_cold` at ~7-8%
    - `memmove` still dominates (~18-19%), allocator stays visible (~4-5%)
  - current read: this cut shrank the `concat_hs` entry tax and improved whole-kilo, but the next whole-front owner is now the `const_suffix` publish/materialize tail rather than the helper entry itself
  - `phase137x-publish-const-suffix-tail-split` tried and **not** a keeper:
    - cut: replaced single `materialize_owned_string_generic_fallback_for_site(...)` call in `publish_const_suffix_owned_cold` with explicit `freeze_owned_bytes_with_site` → `publish_owned_bytes_generic_fallback_for_site` 2-step
    - asm/perf: hot owner moved from `publish_const_suffix_owned_cold` to `publish_owned_bytes_generic_fallback_for_site` (jump trampoline only)
    - 3-run numbers:
      - `kilo_micro_array_string_store = C 9 ms / Ny AOT 136 ms` (baseline 127-132 ms, **exact regressed**)
      - `kilo_kernel_small = C 81 ms / Ny AOT 735 ms` (baseline 737 ms, noise-level whole improvement)
    - conclusion: splitting the call site alone does not shrink the object-world entry cost; the two stages are still fused by the optimizer, and exact regressed — **do not retry this cut without new producer-boundary evidence**
- latest exact/whole asm + perf reread sharpens the keeper choice:
  - exact top report still clusters on:
    - `string_concat_hh_export_impl`
    - `string_substring_concat_hhii_export_impl`
    - array string-store closure
  - whole top report now clusters on:
    - `nyash.string.concat_hs` (~11-13%)
    - `insert_const_mid_fallback` closure / `nyash.string.insert_hsi` (~2-3%)
    - array string-store closure (~5-6%)
    - libc `memmove` (~19-21%) and allocator (`malloc` / `_int_malloc`)
  - `nyash.string.insert_hsi` itself is a thin TLS trampoline; it is not the first keeper owner on whole
  - `nyash.string.concat_hs` is the first whole-front helper owner; the active whole card is now the const_suffix publish/materialize tail, not `pieces3`
- latest C-vs-AOT loop shape comparison:
  - both exact and whole `ny_main` are already structurally close to C (`get/len/edit/set` style loop with direct helper calls)
  - the remaining mismatch is **inside the helper bodies**, not in the top-level lowered loop
  - current bad shape is:
    - helper entry branches / TLS init / dispatch checks
    - generic publication/objectize tail on the returned handle path
  - target shape is:
    - hot path = source read -> size calc -> one alloc/copy leaf -> sink / cold publish adapter
    - cold path = trace / bridge / TLS init / generic publication fallback
  - conclusion: the ideal asm shape is still reachable, and the landed split confirms the first whole-kilo win comes from shrinking the `const_suffix` helper entry; the next keeper must now attack the `const_suffix` publish/materialize tail while keeping publication mechanics off the hot edge
- current live owner remains publication/source-capture around the string births, not array-set route selection
- current reread order is now:
  - exact micro -> adopted middle -> whole kilo
- current next-cut reading from the latest audit bundle is:
  - first narrow cut candidate stays in the store/publication corridor
  - prioritize:
    - `execute_store_array_str_contract`
    - `array_get_index_encoded_i64`
    - `insert_const_mid_fallback`
  - treat allocator / GC (`memmove` / `gc_alloc` / `_int_malloc`) as secondary diagnosis until that corridor is disproved
- current implementation pick for the next keeper card is:
  - keep whole-first as the tie-breaker
  - cut the retarget/publication tail under `execute_store_array_str_contract`
  - first code seam:
    - `try_retarget_borrowed_string_slot_take_verified_text_source`
    - `keep_borrowed_string_slot_source_keep`
  - aim:
    - move old keep retirement off the hot edge while preserving the existing `set_his` / alias-retarget contract
  - latest probe read after landing the cold retirement sink:
    - `kilo_meso_substring_concat_array_set_loopcarry = 53 ms` (`repeat=3`, prior local reread `56 ms`)
    - `kilo_kernel_small_hk = 733 ms`, `736 ms` (`repeat=3` x2)
    - read it as `slight meso lift / whole gain not yet proven`
  - keep adopted middle as the contradiction guard:
    - if the whole-first seam lands but `kilo_meso_substring_concat_array_set_loopcarry` stays flat-to-worse, the next card reopens `substring_hii -> borrowed_substring_plan_from_handle`
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
5. next slice stays evidence-first, and exact / meso / whole are now split:
   - exact micro owner: common generic publish/objectize corridor shared by `string_concat_hh` and `string_substring_concat_hhii`
   - middle bridge: `kilo_meso_substring_concat_array_set_loopcarry` keeps `substring + concat + array.set + loopcarry` while dropping whole-front `indexOf("line")` row-scan noise
   - whole kilo owner: `const_suffix` fallback plus `freeze_text_plan_pieces3` publication
6. do not spend the next keeper card on pair/substring helper specialization; whole-kilo counters prove those sites are inactive there
7. the whole-kilo publish split is now landed:
   - `const_suffix` owns `479728 / 4054750776 bytes`
   - `freeze_text_plan(Pieces3)` owns `60000 / 506895016 bytes`
   - do not reopen any helper-site keeper; the whole front is now pinned to these two producer families
8. keep the lazy published-string handle seam parked as a non-keeper; it changed counters but exploded whole-kilo time
9. next:
   - use the adopted middle front first when judging the next keeper card
   - first narrow cut candidate is the store/publication corridor around `execute_store_array_str_contract` / `array_get_index_encoded_i64` / `insert_const_mid_fallback`
   - do not promote allocator / GC as the first keeper until that corridor loses on exact + meso + whole
   - keep `publish_const_suffix_owned_cold` call-site-only splitting parked as a non-keeper
10. after that keeper selection:
   - `kilo_micro_array_string_store`
   - `kilo_meso_substring_concat_array_set_loopcarry`
   - `kilo_kernel_small_hk`
   - top asm on the active whole producer helper (`const_suffix` / pieces3 path) plus the exact-front publish tail
11. only after that comparison proves one specific producer stage, reopen a new narrow keeper candidate on that stage alone

## Guardrails

- MIR/lowering still owns legality, `proof_region`, and `publication_boundary`
- keep carrier/publication split physically narrow
- do not widen this card into a generic slot API or helper substrate
- keep public ABI stable
- do not add syntax or public raw-string carriers on this card
- compare `Rust vs .hako` only under:
  - same protocol
  - same public ABI with different internal seam
- `kilo_*` is still a small app on this lane; if exact and whole disagree, treat it as missing observability and add counters before choosing a keeper
- do not select a keeper from helper names alone; require producer-kind × stage × bytes evidence on the active whole front first

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
