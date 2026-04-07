# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-07
Scope: repo root から current lane / next lane / restart read order に最短で戻るための薄い anchor。

## Purpose

- root から current lane と current front を最短で読む
- landed history や implementation detail は phase docs を正本にする
- `CURRENT_TASK.md` は pointer に徹し、ledger にはしない

## Quick Restart Pointer

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `git status -sb`
4. `tools/checks/dev_gate.sh quick`

## Order At A Glance

1. `phase-132x vm default backend decision` (landed)
2. `phase-133x micro kilo reopen selection` (landed)
3. `phase-134x nyash_kernel layer recut selection` (landed)
4. `phase-138x nyash_kernel semantic owner cutover` (landed)
5. `phase-139x array owner pilot` (landed)
6. `phase-140x map owner pilot` (landed)
7. `phase-141x string semantic boundary review` (landed)
8. `phase-142x array owner cutover implementation` (landed)
9. `phase-143x map owner cutover implementation` (landed)
10. `phase-144x string semantic owner follow-up` (landed)
11. `phase-145x compat quarantine shrink` (landed)
12. `phase-146x string semantic boundary tighten` (landed)
13. `phase-137x main kilo reopen selection` (paused after reopen proof)
14. `phase-147x semantic optimization contract selection` (landed)
15. `phase-148x borrowed text and sink contract freeze` (landed)
16. `phase-149x concat const-suffix vertical slice` (landed)
17. `phase-150x array string-store vertical slice` (landed)
18. `phase-151x canonical lowering visibility lock` (landed)
19. `phase-137x main kilo reopen selection` (paused after reopen proof)
20. `phase-152x llvmlite object emit cutover` (landed)
21. `phase-153x ny_mir_builder harness drop` (landed)
22. `phase-154x llvmlite archive lock` (landed)
23. `phase-155x perf canonical visibility tighten` (landed)
24. `phase-156x perf counter instrumentation` (landed)
25. `phase-157x observe feature split` (landed)
26. `phase-158x observe tls backend` (landed)
27. `phase-159x observe trace split` (landed)
28. `phase-160x capability-family inventory` (landed)
29. `phase-161x hot-path capability seam freeze` (landed)
30. `phase-137x main kilo reopen selection` (active after capability map lock)
31. `phase-kx vm-hako small reference interpreter recut` (parked after optimization)

## Current Front

- Active lane: `phase-137x main kilo reopen selection`
- Active front: `store.array.str` と `SourceLifetimeKeep` の structural build を先に進め、bench は accept gate として使う
- Current blocker: transport-only / helper-local trim は regress しやすいので、contract split が先、bench reopen はその確認に留める
- Current next design slice:
  - keep `OwnedBytes` as backend-private carrier below `MaterializeOwned`
  - keep `TextReadSession` as backend-private read seam below pure string consumers
  - keep `SourceLifetimeKeep` as backend-private keep seam below source-preserving aliases
  - build structure before benchmark-driven widening:
    - freeze source-preserve / identity / publication policy above Rust
    - keep Rust changes no-behavior or narrowly contract-backed until the seam is fixed
  - reduce `StableBoxNow` demand before trimming `box_id` or registry issue again
  - freeze `store.array.str` source contract before widening Rust leaf helpers again
  - lifecycle policy belongs above Rust:
    - `.hako`: source-preserve / identity / publication demand
    - `MIR`: canonical contract + escalation visibility
    - `Rust`: runtime fact checks + source-lifetime mechanics only
  - current `store.array.str` visibility carrier is now fixed:
    - `GenericMethodRouteState`
    - `GenericMethodEmitPlan`
    - keep `store.array.str` as the public canonical row
    - do not promote `RetargetAlias` into a public MIR op
  - no-behavior lifecycle carrier is now landed above Rust for `store.array.str`:
    - `array_store_string_source_preserve`
    - `array_store_string_identity_demand_stable_object`
    - `array_store_string_publication_demand_publish_handle`
    - current default reading is:
      - source preserve: eligible
      - identity demand: none
      - publication demand: none
  - no-behavior-change `source_kind_check` split is now landed in `string_store.rs`
  - split `store.array.str` source contract into:
    - `SourceKindCheck`
    - `SourceLifetimeKeep`
    - `AliasUpdate`
    - `NeedStableObject`
  - only `NeedStableObject` may justify generic object entry
  - next structure-first freeze:
    - add lifecycle visibility above Rust for `store.array.str`:
      - `source_preserve`
      - `identity_demand`
      - `publication_demand`
    - keep runtime execution below Rust:
      - `StringLikeProof`
      - `TextKeep`
      - `AliasSourceMeta`
  - latest landed `RetargetAlias` slice:
    - full object API demand on `BorrowedHandleBox` stays closed on current workloads:
      - `to_string_box_latest_fresh=0`
      - `equals_latest_fresh=0`
      - `clone_box_latest_fresh=0`
    - retarget success path now moves the source `Arc` into alias keep instead of cloning it again
    - 3-run plain release:
      - `kilo_micro_array_string_store: 178 ms`
      - `kilo_micro_concat_hh_len: 65 ms`
      - `kilo_kernel_small_hk: 740 ms`
  - latest landed live-source alias slice:
    - `BorrowedHandleBox::as_str_fast()` now reads `drop_epoch()` only when `observe::enabled()`
    - plain release hot path no longer pays the live/stale probe when counters are off
    - 3-run plain release:
      - `kilo_micro_array_string_store: 169 ms`
      - `kilo_micro_concat_hh_len: 61 ms`
      - `kilo_kernel_small_hk: 717 ms`
  - latest landed source-lifetime keep split:
    - `ArrayStoreStrSource::StringLike(...)` now carries `SourceLifetimeKeep`
    - retarget path now consumes `try_retarget_borrowed_string_slot_take_keep(...)`
    - this is no-behavior-change; it narrows the next actual cut to keep semantics instead of keep transport
    - 3-run plain release reread:
      - `kilo_micro_array_string_store: 169 ms`
      - `kilo_micro_concat_hh_len: 63 ms`
      - `kilo_kernel_small_hk: 703 ms`
  - latest landed keep/meta split:
    - `BorrowedHandleBox` now separates:
      - `TextKeep`
      - `AliasSourceMeta`
    - `SourceLifetimeKeep` remains the backend-private keep carrier
    - this still keeps current representation on `StableBox(...)`
    - purpose is to narrow the next behavior change to keep semantics, not transport
    - accept-gate reread:
      - `kilo_micro_array_string_store: 175 ms`
      - `kilo_micro_concat_hh_len: 63 ms`
      - `kilo_kernel_small_hk: 703 ms`
  - close `TextSnapshot` keep for now:
    - `kilo_micro_array_string_store: 178 ms`
    - `kilo_micro_concat_hh_len: 65 ms`
    - `kilo_kernel_small_hk: 1792 ms`
    - exact improved but whole-kilo collapsed, so the behavior change is reverted
  - only remove generic object entry from the part that does not carry source-lifetime semantics
- Exact focus:
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/design/birth-placement-ssot.md`
  - `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
  - `docs/development/current/main/phases/phase-160x/README.md`
  - `docs/development/current/main/phases/phase-161x/README.md`
  - `crates/nyash_kernel/src/plugin/array_handle_cache.rs`
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`

## Successor Corridor

1. `phase-137x main kilo reopen selection`
2. `phase-kx vm-hako small reference interpreter recut`

## Parked After Optimization

- `phase-kx vm-hako small reference interpreter recut`
  - keep `vm-hako` as reference/conformance only
  - do not promote to product/mainline
  - revisit after the optimization corridor, not before

## Structural Stop Lines

- `rust-vm`
  - mainline retirement: achieved
  - residual explicit keep: frozen
- `vm-hako`
  - reference/conformance keep
- `nyash_kernel`
  - keep `Rust host microkernel`, ABI thin facade, lifetime-sensitive hot leaf, and native accelerator leaves in Rust
  - move semantic ownership, collection owner policy, and route semantics toward `.hako`
  - do not turn compat quarantine into a permanent owner layer

## Long-Term Direction Lock

- design truth lives in:
  - `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
  - `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
- fixed wording:
  - mid-term stop-line: `Rust = semantics-free runtime mechanics kernel`
  - long-term asymptote: `Rust -> OS / ABI / host boundary`
  - layer borrowing rule:
    - `.hako` borrows Rust-like ownership vocabulary as meaning
    - `MIR` borrows delayed-materialization reading as canonical contract
    - `Rust` borrows C-like storage/lifetime discipline as runtime mechanics
    - `LLVM` keeps generic optimization/codegen only
- do not let this long-term target disappear behind phase churn or perf-only notes

## Read Next

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-137x/README.md`
4. `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
5. `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
6. `docs/development/current/main/design/canonical-lowering-visibility-ssot.md`
7. `docs/development/current/main/phases/phase-160x/README.md`
8. `docs/development/current/main/phases/phase-161x/README.md`

## Notes

- `phase-132x` landed:
  - remove `vm` from the default backend
  - keep explicit `vm` / `vm-hako` proof-debug callers alive
  - do not wait for full vm source retirement before resuming mainline work
- llvmlite retreat order is fixed:
  1. runner object emit cutover
  2. `ny_mir_builder` harness drop
  3. llvmlite keep/archive lock
  4. perf reopen
- `phase-154x` landed:
  - `tools/selfhost/lib/selfhost_build_exe.sh` no longer forces `NYASH_LLVM_USE_HARNESS=1`
  - `tools/selfhost/README.md` and `src/host_providers/llvm_codegen/README.md` now read `ny-llvmc` as the daily owner and llvmlite as explicit keep
  - `tools/build_llvm.sh` harness keep now routes through `ny-llvmc --driver harness`
  - `tools/llvm_smoke.sh` is explicit llvmlite compat/probe keep, not daily mainline evidence
  - `docs/guides/exe-first-wsl.md` now treats `ny-llvmc` as the daily EXE-first route
  - `docs/guides/selfhost-pilot.md` no longer requires llvmlite for daily selfhost/product flows
  - `docs/reference/environment-variables.md` labels `NYASH_LLVM_USE_HARNESS=1` examples as explicit keep-lane
- current perf reopen truth:
  - `kilo_kernel_small_hk`: latest reread `ny_aot_ms=703`
  - `kilo_micro_concat_const_suffix`: `ny_aot_ms=84`
  - `kilo_micro_concat_hh_len`: `ny_aot_ms=63`
  - `kilo_micro_array_string_store`: `ny_aot_ms=169`
- long-range direction lock:
  - `docs/development/current/main/design/birth-placement-ssot.md` is now the SSOT for Birth / Placement outcome vocabulary
  - read string hot paths through:
    - `ReturnHandle`
    - `BorrowView`
    - `FreezeOwned`
    - `FreshHandle`
    - `MaterializeOwned`
    - `StoreFromSource`
  - backend-only second axis is now locked in SSOT:
    - `Objectization = None | StableBoxNow | DeferredStableBox`
    - `RegistryIssue = None | ReuseSourceHandle | FreshRegistryHandle`
  - lifecycle policy lock:
    - `.hako` owns source-preserve / identity / publication demand
    - `MIR` owns canonical contract and escalation visibility
    - `Rust` owns source kind checks, source-lifetime keep, alias survival, and registry/drop-epoch mechanics only
  - `box_id` belongs to Rust-side `Objectization::StableBoxNow`, not to top-level Birth / Placement vocabulary
  - do not promote helper names such as `string_handle_from_owned(...)` or `freeze_text_plan(...)` into public optimization vocabulary
- capability-family lock before perf reopen: landed
  1. inventory current Rust helpers by future capability family
  2. freeze hot-path mapping for `store.array.str` / `const_suffix` / observer backend
  3. resume micro + main kilo tuning with seam names as first reading
- `phase-155x` current perf order is fixed as canonical reading first:
  1. generic string concat/len fast path under the current `kilo_micro_concat_const_suffix` workload
     - current AOT consumer: `nyash.string.concat_hh` + `nyash.string.len_h`
     - executor detail: `string_concat_hh_export_impl(...)` + `string_len_from_handle(...)`
     - exact micro: `kilo_micro_concat_const_suffix`
     - isolated exact micro: `kilo_micro_concat_hh_len`
     - Birth / Placement reading:
       - current next seam is `FreezeOwned / FreshHandle / MaterializeOwned`
       - read-side small seams have been tried and reverted
  2. `store.array.str`
     - executor detail: `array_string_store_handle_at(...)`
     - exact micro: `kilo_micro_array_string_store`
  3. `const_suffix`
     - canonical reading: `thaw.str + lit.str + str.concat2 + freeze.str`
     - executor detail: `concat_const_suffix_fallback(...)`
     - exact micro consumer is not the current `kilo_micro_concat_const_suffix` AOT path
- `phase-156x` landed:
  - route-tagged counters exist for `store.array.str` and `const_suffix`
  - generic string consumer counters now also exist for:
    - `str.concat2.route`: `total / dispatch_hit / fast_str_owned / fast_str_return_handle / span_freeze / span_return_handle / materialize_fallback / unclassified`
    - `str.len.route`: `total / dispatch_hit / fast_str_hit / fallback_hit / miss / latest_fresh_handle_fast_str_hit / latest_fresh_handle_fallback_hit / unclassified`
  - direct probe truth for the current exact string fronts:
    - `kilo_micro_concat_birth`
      - `str.concat2.route`: `fast_str_owned=800000`, all other classified routes `0`, `unclassified=0`
      - `str.len.route`: `fast_str_hit=1`, `latest_fresh_handle_fast_str_hit=1`, all other classified routes `0`, `unclassified=0`
    - `kilo_micro_concat_hh_len`
      - `str.concat2.route`: `fast_str_owned=800000`, all other classified routes `0`, `unclassified=0`
      - `str.len.route`: `fast_str_hit=800002`, `latest_fresh_handle_fast_str_hit=800000`, all other classified routes `0`, `unclassified=0`
  - current problem is not route misclassification:
    - the active exact fronts already stay on `fast_str_owned -> fast_str_hit`
    - `len_h` is usually reading the latest freshly issued handle immediately after concat birth
    - the remaining gap lives under Birth / Placement backend:
      - `materialize_owned_bytes`
      - `objectize_stable_box_now`
      - `issue_fresh_handle`
  - latest design lock:
    - treat birth as three backend events, not one:
      - byte birth = `MaterializeOwned`
      - object birth = `StableBoxNow`
      - publication birth = `FreshRegistryHandle`
    - next backend-private carriers are:
      - `OwnedBytes`
      - `TextReadSession`
    - reduce `StableBoxNow` demand before trying to make `box_id` cheaper again
  - source-backed seam status:
    - `OwnedBytes` now exists as a private carrier in `string_store.rs`
    - `TextReadSession` now exists in `host_handles.rs`
  - current string fast readers (`len_h`, `is_empty`, pair/triple fast concat readers) can already consume that seam
  - no delayed objectization behavior change is reintroduced by this slice
  - `StableBoxNow` demand probe truth on the current exact fronts:
    - `kilo_micro_concat_birth`
      - `stable_box_demand`:
        - `object_get_latest_fresh=0`
        - `object_with_handle_latest_fresh=0`
        - `object_pair_latest_fresh=0`
        - `object_triple_latest_fresh=0`
        - `text_read_handle_latest_fresh=1`
        - `text_read_pair_latest_fresh=0`
        - `text_read_triple_latest_fresh=0`
    - `kilo_micro_concat_hh_len`
      - `stable_box_demand`:
        - `object_get_latest_fresh=0`
        - `object_with_handle_latest_fresh=0`
        - `object_pair_latest_fresh=0`
        - `object_triple_latest_fresh=0`
        - `text_read_handle_latest_fresh=800000`
        - `text_read_pair_latest_fresh=0`
        - `text_read_triple_latest_fresh=0`
    - current exact problem is therefore not fresh-handle leakage into object APIs
    - the next structural slice should stay narrow:
      - single-handle text-read consumer
      - delayed `StableBoxNow` only where whole-kilo does not regress
  - delayed `StableBoxNow` retry truth:
    - exact micro improved again:
      - `kilo_micro_concat_birth: 50 -> 37 ms`
      - `kilo_micro_concat_hh_len: 67 -> 57 ms`
    - whole-kilo still regressed:
      - `kilo_kernel_small_hk: 764 ms`
    - observe whole probe says latest fresh handles are not staying in the same narrow seam:
      - `stable_box_demand.object_with_handle_latest_fresh=540000`
      - `stable_box_demand.text_read_handle_latest_fresh=0`
      - `stable_box_demand.text_read_pair_latest_fresh=938`
    - current explanation:
      - exact micro stays on single-handle text-read
      - whole-kilo quickly escalates latest fresh handles into generic object `with_handle(...)`
      - do not reland deferred objectization before that consumer is widened or bypassed
    - caller-attributed whole-kilo truth:
      - `stable_box_demand.object_with_handle_array_store_str_source_latest_fresh=540000`
      - `stable_box_demand.object_with_handle_substring_plan_latest_fresh=0`
      - `stable_box_demand.object_with_handle_decode_array_fast_latest_fresh=0`
      - `stable_box_demand.object_with_handle_decode_any_arg_latest_fresh=0`
      - `stable_box_demand.object_with_handle_decode_any_index_latest_fresh=0`
    - source-backed `store.array.str` split now confirms the whole-kilo latest-fresh demand is retarget-only:
      - `store.array.str latest_fresh_retarget_hit=540000`
      - `store.array.str latest_fresh_source_store=0`
    - no-behavior-change planner truth now confirms the hot path mismatch:
      - whole-kilo:
        - `plan.source_kind_string_like=540000`
        - `plan.source_kind_other_object=0`
        - `plan.source_kind_missing=0`
        - `plan.slot_kind_borrowed_alias=540000`
        - `plan.slot_kind_other=0`
        - `plan.action_retarget_alias=540000`
        - `plan.action_store_from_source=0`
        - `plan.action_need_stable_object=0`
      - exact `kilo_micro_array_string_store`:
        - `plan.source_kind_string_like=800000`
        - `plan.slot_kind_borrowed_alias=800000`
        - `plan.action_retarget_alias=800000`
        - `plan.action_store_from_source=0`
        - `plan.action_need_stable_object=0`
    - no-behavior-change reason truth now clarifies why object world is still entered:
      - whole-kilo:
        - `reason.source_kind_via_object=540000`
        - `reason.retarget_keep_source_arc=540000`
        - `reason.retarget_alias_update=540000`
      - exact `kilo_micro_array_string_store`:
        - `reason.source_kind_via_object=800000`
        - `reason.retarget_keep_source_arc=800000`
        - `reason.retarget_alias_update=800000`
    - current read:
      - planner says the hot path is pure `RetargetAlias`
      - executor still escalates through `with_handle(ArrayStoreStrSource)` before reaching that action
      - but the current contract is not only source-kind probing:
        - retarget also keeps the source `Arc` alive so the alias survives source-handle drop
      - next slice must separate `source_kind_check` from `keep_source_arc`, not assume object entry is fully removable
      - target assembly shape:
        - hot `RetargetAlias` should spend cycles on source metadata update and slot mutation, not generic object fetch/downcast
        - object-world entry should remain only for source-lifetime keep semantics, not for source-kind probing itself
    - borrowed alias whole-kilo truth:
      - `borrowed.alias.borrowed_source_fast=540000`
      - `borrowed.alias.as_str_fast=540064`
      - `borrowed.alias.as_str_fast_live_source=540064`
      - `borrowed.alias.as_str_fast_stale_source=0`
      - `borrowed.alias.array_len_by_index_latest_fresh=1`
      - `borrowed.alias.array_indexof_by_index_latest_fresh=938`
      - `borrowed.alias.encode_epoch_hit=0`
      - `borrowed.alias.encode_ptr_eq_hit=0`
      - `borrowed.alias.encode_to_handle_arc=0`
      - `borrowed.alias.encode_to_handle_arc_array_get_index=0`
      - `borrowed.alias.encode_to_handle_arc_map_runtime_data_get_any=0`
    - current read:
      - retargeted latest-fresh aliases are not escaping through encoder fallback
      - caller-attributed encode-to-handle paths are also closed in current behavior
      - `BorrowedHandleBox::as_str_fast()` stays entirely on the live-source side in whole-kilo
      - `array_string_len_by_index(...)` / `array_string_indexof_by_index(...)` are not the 540k latest-fresh culprit
      - the remaining stable object pressure stays on `store.array.str -> with_handle(ArrayStoreStrSource)` itself, not alias runtime encode
    - landed structural slice:
      - `ArrayStoreStrSource` now owns the source `Arc`
      - `with_array_store_str_source(...)` finishes host-handle source read before `arr.with_items_write(...)`
      - `store.array.str` no longer nests host-handle read-lock across planner/retarget execution
    - current 3-run plain-release recheck on the landed slice:
      - `kilo_micro_array_string_store: 189 ms`
      - `kilo_micro_concat_hh_len: 67 ms`
      - `kilo_kernel_small_hk: 745 ms`
    - current read:
      - this is not a large exact-front win
      - but it is a cleaner source-contract split and keeps whole-kilo near the good end of the current band
      - target assembly shape remains:
        - `with_handle(ArrayStoreStrSource)` should stop dominating the hot path
        - planner-proved string-like retarget should approach metadata-heavy code instead of object-world transport
    - closed follow-up:
      - replacing `with_handle(ArrayStoreStrSource)` with direct `get()` source load regressed slightly
      - 3-run plain release:
        - `kilo_micro_array_string_store: 192 ms`
        - `kilo_micro_concat_hh_len: 69 ms`
        - `kilo_kernel_small_hk: 747 ms`
      - revert the behavior change; keep `with_handle_caller(...)` for now
    - current first widening target is:
      - `store.array.str` source read under `array_string_slot.rs`
    - attempted widening truth:
      - moving `store.array.str` source read into `TextReadSession` did redirect latest fresh demand out of `object_with_handle(...)`
      - but plain release regressed:
        - `kilo_micro_array_string_store: 181 -> 187 ms`
        - `kilo_kernel_small_hk: 757 -> 916 ms`
      - actual behavior change is reverted
    - narrow `retarget` retry truth:
      - a no-op `try_retarget_borrowed_string_slot_verified(...)` guard on unchanged `(source_handle, drop_epoch)` did not materially move the front
      - plain release recheck:
        - `kilo_micro_array_string_store: 183 ms`
        - `kilo_kernel_small_hk: 746 ms`
      - keep only the counter truth; behavior change is reverted
    - latest-fresh stable object cache truth:
      - caching the newest `Arc<dyn NyashBox>` in TLS and short-circuiting `with_handle(ArrayStoreStrSource)` regressed both exact and whole
      - plain release 3-run:
        - `kilo_micro_array_string_store: 210 ms`
        - `kilo_micro_concat_hh_len: 78 ms`
        - `kilo_kernel_small_hk: 760 ms`
      - the behavior change is reverted
    - borrowed alias raw string cache truth:
      - caching source string addr/len inside `BorrowedHandleBox` and bypassing `inner.as_str_fast()` regressed exact and whole
      - plain release 3-run:
        - `kilo_micro_array_string_store: 196 ms`
        - `kilo_micro_concat_hh_len: 69 ms`
        - `kilo_kernel_small_hk: 798 ms`
      - the behavior change is reverted
    - typed string payload truth:
      - issuing fresh string handles through a typed `StringBox` payload and using a typed retarget fast path regressed the exact fronts immediately
      - plain release 3-run:
        - `kilo_micro_array_string_store: 201 ms`
        - `kilo_micro_concat_hh_len: 72 ms`
      - whole-kilo was not pursued; the behavior change is reverted
    - cloned source-arc retarget truth:
      - hot `RetargetAlias` was retried with a narrow `clone source Arc first, then retarget` slice
      - plain release 3-run regressed across exact and whole:
        - `kilo_micro_array_string_store: 205 ms`
        - `kilo_micro_concat_hh_len: 91 ms`
        - `kilo_kernel_small_hk: 959 ms`
      - the behavior change is reverted
      - keep only the no-behavior structural split:
        - `StoreArrayStrPlan` separates planner from executor
        - borrowed retarget now exposes `keep_source_arc` / `alias_update` helpers
    - typed `ArrayStoreStrSource` helper is now landed:
      - `with_array_store_str_source(...)` wraps `with_handle_caller(ArrayStoreStrSource)`
      - `store.array.str` now consumes a typed source contract instead of open-coding generic object entry at the executor callsite
      - this remains the landed no-behavior seam
    - typed helper internal bypass truth:
      - trying `with_str_handle(...)` first inside `with_array_store_str_source(...)` regressed exact and whole
      - plain release 3-run:
        - `kilo_micro_array_string_store: 190 ms`
        - `kilo_micro_concat_hh_len: 67 ms`
        - `kilo_kernel_small_hk: 783 ms`
      - the behavior change is reverted
    - `keep_source_arc` ptr-eq truth:
      - observe direct probe now runs again via sync-stamp aligned perf-observe lane
      - exact `kilo_micro_array_string_store`:
        - `reason.retarget_keep_source_arc_ptr_eq_hit=0`
        - `reason.retarget_keep_source_arc_ptr_eq_miss=800000`
      - whole `kilo_kernel_small_hk`:
        - `reason.retarget_keep_source_arc_ptr_eq_hit=0`
        - `reason.retarget_keep_source_arc_ptr_eq_miss=540000`
      - `keep_source_arc` is always seeing a different source object on the current hot path
      - clone-elision / ptr-eq guard ideas are therefore closed
    - borrowed string keep seam is now landed:
      - `BorrowedHandleBox` no longer treats stable source object keep as an anonymous field
      - `BorrowedStringKeep` is the backend-private seam under alias source-lifetime
      - current behavior still uses `StableBox(...)` only
      - this is still no-behavior-change; it narrows the next structural cut away from generic object transport
    - closed follow-up:
      - a typed `BorrowedStringKeep::StringBox` fast path regressed on both exact and whole
      - 3-run plain release:
        - `kilo_micro_array_string_store: 198 ms`
        - `kilo_micro_concat_hh_len: 71 ms`
        - `kilo_kernel_small_hk: 777 ms`
      - revert the behavior change; keep `BorrowedStringKeep` on `StableBox(...)` until source-lifetime keep semantics change
  - immediate next observation order is fixed:
    1. split the `store.array.str -> with_handle(ArrayStoreStrSource)` object contract again before changing behavior
    2. keep borrowed alias string-read trimming closed; live-source fast read was not enough
    3. keep typed `StringBox` payload widening closed at the host-handle layer
    4. keep `keep_source_arc` clone-elision ideas closed; ptr-eq never hits on the current culprit
    5. keep typed `BorrowedStringKeep::StringBox` fast path closed; transport-only specialization still loses
    6. do not add more typed-helper transport; move the next cut to the source-lifetime contract side
    7. use `BorrowedStringKeep` as the backend-private seam, but change keep semantics before changing keep representation again
    8. only then retry delayed `StableBoxNow`
  - `DeferredString` experiment truth:
    - exact micro improved:
      - `kilo_micro_concat_hh_len`: `57 -> 51 ms`
      - `kilo_micro_concat_birth`: `47 -> 35 ms`
    - whole-kilo probe regressed:
      - `kilo_kernel_small_hk`: `741 -> 952 ms`
    - code was reverted
    - next widening choice is now:
      1. explain the whole-kilo regression first
      2. only then reconsider pair/span widening
  - payload seam scaffolding is now source-backed:
    - `host_handles` slot storage now reads through `HandlePayload::StableBox(...)`
    - public registry API is unchanged
    - this is not `DeferredStableBox` yet; it only fixes the narrow widening seam so future `DeferredString` does not have to start from raw `Arc<dyn NyashBox>` slots
    - single-handle string-only access now also has its own seam:
      - `host_handles::with_str_handle(...)`
      - `string_len_from_handle(...)` and `string_is_empty_from_handle(...)` now read through that path instead of generic `with_handle(...)`
  - Birth / Placement counters now also exist for:
    - `ReturnHandle / BorrowView / FreezeOwned / FreshHandle / MaterializeOwned / StoreFromSource`
  - birth backend counters now also exist for:
    - `freeze_text_plan_total`
    - `freeze_text_plan_view1 / pieces2 / pieces3 / pieces4 / owned_tmp`
    - `materialize_owned_total / materialize_owned_bytes`
    - `string_box_new_total / string_box_new_bytes`
    - `string_box_ctor_total / string_box_ctor_bytes`
    - `arc_wrap_total`
    - `handle_issue_total`
    - `gc_alloc_called / gc_alloc_bytes / gc_alloc_skipped`
  - drill-down counters now exist for:
    - `store.array.str`: `existing_slot / append_slot / source_string_box / source_string_view / source_missing`
    - `const_suffix`: `empty_return / cached_fast_str_hit / cached_span_hit`
  - first exact probe:
    - `bench_kilo_micro_array_string_store.hako` -> `cache_hit=800000`, `cache_miss_epoch=0`
    - current cache-churn hypothesis is not supported on that exact micro
  - deeper exact probe:
    - `bench_kilo_micro_array_string_store.hako` -> `retarget_hit=800000`, `existing_slot=800000`, `source_string_box=800000`
    - `bench_kilo_micro_concat_const_suffix.hako` AOT run does not hit `const_suffix`; it currently lowers through `nyash.string.concat_hh` + `nyash.string.len_h`
  - Birth / Placement direct probe:
    - `bench_kilo_micro_concat_hh_len.hako` AOT run is currently:
      - `birth.placement`: `fresh_handle=800000`
      - `birth.backend`: `materialize_owned_total=800000`, `materialize_owned_bytes=14400000`, `gc_alloc_called=800000`, `gc_alloc_bytes=14400000`
      - `return_handle / borrow_view / freeze_owned = 0`
    - current exact backend front is:
      - `FreshHandle`
      - `MaterializeOwned`
    - target string-chain assembly shape:
      - `concat_hh + len_h` should spend most hot-path cycles in text/materialize work, not registry/object machinery
      - ideal fast path is `OwnedBytes` / text-read first, with `StableBoxNow` and `FreshRegistryHandle` pushed to sink/object boundaries only
  - new exact split:
    - `bench_kilo_micro_concat_hh_len.hako` isolates `concat_hh + len_h` without substring carry
    - latest exact read: `c_ms=3 / ny_aot_ms=57`
  - new birth-only exact split:
    - `bench_kilo_micro_concat_birth.hako` isolates fresh concat birth/materialize with only final `len`
    - latest exact read: `c_ms=6 / ny_aot_ms=47`
    - direct probe:
      - `birth.placement`: `fresh_handle=800000`
      - `birth.backend`: `materialize_owned_total=800000`, `materialize_owned_bytes=14400000`, `gc_alloc_called=800000`, `gc_alloc_bytes=14400000`
  - `NYASH_PERF_BYPASS_GC_ALLOC=1` diagnostic observe lane:
    - `bench_kilo_micro_concat_birth.hako`: `50 -> 51 ms`
    - `bench_kilo_micro_concat_hh_len.hako`: `72 -> 70 ms`
    - observe-build `kilo_kernel_small_hk`: `1077 -> 1084 ms`
    - direct probe cleanly flips:
      - `gc_alloc_called=800000 -> 0`
      - `gc_alloc_skipped=0 -> 800000`
    - current evidence does not support `gc_alloc(...)` call overhead as the next main driver
    - next backend front remains `StringBox` birth / host handle registry issue
  - observe-build birth split:
    - direct probe now also shows:
      - `string_box_new_total=800000`
      - `string_box_ctor_total=800000`
      - `arc_wrap_total=800000`
      - `handle_issue_total=800000`
    - release observe direct probe now confirms second-axis counters too:
      - `objectize_stable_box_now_total=800000`
      - `objectize_stable_box_now_bytes=14400000`
      - `issue_fresh_handle_total=800000`
    - `kilo_micro_concat_birth` observe-build microasm after backend split reads:
      - `materialize_owned_bytes`: `25.81%`
      - `issue_fresh_handle`: `24.62%`
      - `StringBox::perf_observe_from_owned`: `21.27%`
      - `__memmove_avx512_unaligned_erms`: `14.67%`
      - `nyash.string.concat_hh`: `5.81%`
    - annotate of `issue_fresh_handle(...)` shows the dominant leaf is registry unlock/release:
      - final `lock cmpxchg` in `host_handles::REG` release path dominates local samples
    - current backend order is therefore:
      1. `materialize_owned_bytes(...)`
      2. `issue_fresh_handle(...)`
      3. `StringBox::perf_observe_from_owned(...)`
      4. `objectize_stable_string_box(...)` remains a naming seam, but most cost sits in the ctor/issue leaves
    - structural reading lock:
        - do not treat `box_id` as a top-level Birth outcome
        - first split the backend read into:
          1. `materialize_owned_bytes`
          2. `objectize_stable_string_box`
          3. `issue_fresh_handle`
        - current implementation still couples:
          - `FreshHandle`
          - `MaterializeOwned`
          - `StableBoxNow`
          - `FreshRegistryHandle`
        - current source now exposes these names in `string_store.rs`
        - second-axis observe counters also exist for:
          - `objectize_stable_box_now_total / bytes`
          - `issue_fresh_handle_total`
        - observe lane contract lock:
          - default perf AOT lane now also fails fast unless `target/release/.perf_release_sync` is newer than both `target/release/libnyash_kernel.a` and `target/release/hakorune`
          - observe lane still requires `target/release/.perf_observe_release_sync`
          - canonical rebuild orders are fixed in `tools/perf/build_perf_release.sh` and `tools/perf/build_perf_observe_release.sh`
          - helper-local perf ranking must use the matching lane:
            - plain release asm = cost ranking
            - `perf-observe` = counter totals and symbol split
            - do not promote a helper from observe annotate alone when the body is dominated by TLS counter work
- `phase-157x` current rule:
  - observer is out-of-band only
  - default build compiles observer out
  - `perf-observe` build + `NYASH_PERF_COUNTERS=1` is the only supported counter lane
  - landed: observer module is split into contract / config / sink / backend
  - landed: default release no longer compiles observer machinery in
- `phase-158x` current rule:
  - exact counter backend is TLS-first inside `perf-observe`
  - current-thread flush owns end-of-run summary
  - shared atomic cost should stay out of hot path unless a future fallback backend explicitly asks for it
- `phase-159x` landed rule:
  - exact counter remains `perf-observe`
  - heavy trace / sampled probe must move to a separate feature lane
  - do not mix trace semantics into exact counter identity or sink
  - first trace lane is `perf-trace` + `NYASH_PERF_TRACE=1`
- latest bundle anchor:
  - `target/trace_logs/kilo-string-trace-asm/20260406-024104/summary.txt`
  - `target/trace_logs/kilo-string-trace-asm/20260406-024104/asm/perf_report.txt`
- fixed perf reopen order remains:
  - `leaf-proof micro`
  - `micro kilo`
  - `main kilo`
- `phase-133x` is landed:
  - `kilo_micro_substring_concat`: parity locked
  - `kilo_micro_array_getset`: parity locked
  - `kilo_micro_indexof_line`: frozen faster than C
- `phase-134x` landed the refactor split:
  - `keep`
  - `thin keep`
  - `compat glue`
  - `substrate candidate`
- `phase-138x` is the next design corridor:
  - landed: final shape is `Rust host microkernel` + `.hako semantic kernel` + `native accelerators`
  - landed: `ABI facade` is thin keep
  - landed: `compat quarantine` is non-owner and shrink-only
  - landed: `Array owner` is the first cutover pilot
- `phase-139x` current seam:
  - landed: owner = `lang/src/runtime/collections/array_core_box.hako`
  - landed: substrate = `lang/src/runtime/substrate/raw_array/raw_array_core_box.hako`
  - landed: ABI facade = `crates/nyash_kernel/src/plugin/array_substrate.rs`
  - landed: compat/runtime forwarders = `crates/nyash_kernel/src/plugin/array_runtime_facade.rs`
  - landed: accelerators = `crates/nyash_kernel/src/plugin/array_handle_cache.rs`, `crates/nyash_kernel/src/plugin/array_string_slot.rs`
- `phase-140x` landed seam:
  - landed: owner = `lang/src/runtime/collections/map_core_box.hako`, `lang/src/runtime/collections/map_state_core_box.hako`
  - landed: substrate = `lang/src/runtime/substrate/raw_map/raw_map_core_box.hako`
  - landed: thin facade = `crates/nyash_kernel/src/plugin/map_aliases.rs`
  - landed: observer shim = `crates/nyash_kernel/src/plugin/map_substrate.rs`
  - landed: compat/runtime forwarding = `crates/nyash_kernel/src/plugin/map_runtime_facade.rs`
  - landed: accelerators = `crates/nyash_kernel/src/plugin/map_probe.rs`, `crates/nyash_kernel/src/plugin/map_slot_load.rs`, `crates/nyash_kernel/src/plugin/map_slot_store.rs`
- `phase-141x` landed seam:
  - semantic owner: `lang/src/runtime/kernel/string/README.md`, `lang/src/runtime/kernel/string/chain_policy.hako`, `lang/src/runtime/kernel/string/search.hako`
  - VM-facing wrapper: `lang/src/runtime/collections/string_core_box.hako`
  - thin facade: `crates/nyash_kernel/src/exports/string.rs`
  - Rust keep: `crates/nyash_kernel/src/exports/string_view.rs`, `crates/nyash_kernel/src/exports/string_helpers.rs`, `crates/nyash_kernel/src/exports/string_plan.rs`
  - quarantine: `crates/nyash_kernel/src/plugin/module_string_dispatch/**`
- `phase-142x` landed cutover:
  - `ArrayBox.{push,get,set,len/length/size,pop}` visible semantics now sit on `.hako` owner helpers
  - Rust array surface is split into compat aliases, runtime any-key shell, idx forwarding, substrate forwarding, and accelerators
- `phase-143x` landed cutover:
  - visible `MapBox.{set,get,has,len/length/size}` behavior now reads through `.hako` owner helpers
  - Rust map surface remains thin facade / observer shim / forwarding / raw leaves
- `phase-144x` landed follow-up:
  - `StringCoreBox.{size,indexOf,lastIndexOf,substring}` now reads through helperized wrapper paths
  - `lastIndexOf` now delegates to `.hako` search owner helper instead of wrapper-local search
  - `indexOf(search, fromIndex)` now delegates to `.hako` search owner via `StringSearchKernelBox.find_index_from(...)`
- `phase-145x` landed target:
  - host-side glue:
    - `crates/nyash_kernel/src/hako_forward_bridge.rs`
    - `crates/nyash_kernel/src/plugin/future.rs`
    - `crates/nyash_kernel/src/plugin/invoke_core.rs`
  - quarantine:
    - `crates/nyash_kernel/src/plugin/module_string_dispatch/**`
  - goal:
    - host service contract と compat quarantine を source 上で取り違えない状態にする
- `phase-146x` landed target:
  - string semantic owner / wrapper / native substrate の stop-line を source 上で tighten
  - target files:
    - `lang/src/runtime/kernel/string/README.md`
    - `lang/src/runtime/collections/string_core_box.hako`
    - `crates/nyash_kernel/src/exports/string_view.rs`
    - `crates/nyash_kernel/src/exports/string_plan.rs`
    - `crates/nyash_kernel/src/exports/string_helpers.rs`
- `phase-147x` landed design lock:
  - authority order is `.hako owner / policy -> MIR canonical contract -> Rust executor / accelerator -> LLVM generic optimization / codegen`
  - `BorrowedText` / `TextSink` may exist only as Rust internal executor protocol
  - first canonical-op candidates:
    - `lit.str`
    - `str.concat2`
    - `store.array.str`
    - `store.map.value`
  - first vertical slice stays `concat const-suffix`
- `phase-148x` landed contract freeze:
  - owner route `const_suffix` now freezes the canonical MIR reading `thaw.str + lit.str + str.concat2 + freeze.str`
  - owner route `ArrayStoreString` now freezes the canonical MIR reading `store.array.str`
  - owner route `MapStoreAny` now freezes the canonical MIR reading `store.map.value`
  - current concrete executor paths remain `nyash.string.concat_hs`, `nyash.array.set_his`, and `nyash.map.slot_store_hhh`
- `phase-149x` landed first vertical slice:
  - current concrete helper `concat_const_suffix_fallback(...)` now reads as executor detail under `.hako` route `const_suffix`
  - `nyash.string.concat_hs` is no longer treated as semantic authority
- `phase-150x` landed second vertical slice:
  - current concrete symbol `nyash.array.set_his` now reads as ABI/executor detail under `.hako` route `ArrayStoreString`
  - Rust forwarding now exposes `array_runtime_store_array_string(...)` as the current contract-shaped facade
- `phase-151x` landed visibility lock:
  - `const_suffix`
  - `ArrayStoreString`
  - `MapStoreAny`
  are all now readable as:
  - `.hako owner`
  - canonical MIR reading
  - current concrete lowering
  - Rust executor
- final optimization form is fixed:
  - `.hako` owns route / retained-form / boundary / visible contract
  - MIR owns canonical optimization names
  - Rust owns executor / accelerator only
  - perf reopen is blocked until canonical readings are visible against current concrete lowering for both string const-suffix and array string-store
- `phase-137x` current baseline and first reopen wins:
  - baseline: `kilo_kernel_small_hk`: `c_ms=81 / ny_aot_ms=1529`
  - after string const-path branch collapse: `c_ms=82 / ny_aot_ms=775`
  - after const-handle cache follow-up: `c_ms=84 / ny_aot_ms=731`
  - after const empty-flag cache: `c_ms=81 / ny_aot_ms=723`
  - after shared text-based const-handle helper: `c_ms=80 / ny_aot_ms=903`
  - after single-closure const suffix fast path: `c_ms=83 / ny_aot_ms=820`
  - latest whole-kilo reread after visibility lock: `c_ms=83 / ny_aot_ms=762`
  - latest array-string-store executor trim: exact micro `kilo_micro_array_string_store`: `c_ms=10 / ny_aot_ms=207`
  - whole-kilo recheck after array-string-store trim: `c_ms=81 / ny_aot_ms=745`
  - exact micro `kilo_micro_concat_const_suffix`: `c_ms=2 / ny_aot_ms=85`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4`
- latest bundle read:
  - string trace contract unchanged for `concat_hs` / `insert_hsi`
  - `20260406-024104` bundle still has `concat_const_suffix_fallback` as the top explicit hot symbol (`11.70%`)
  - `array_string_store_handle_at` remains second (`5.68%`)
- `phase-137x` is reopened:
  - perf consumer resumes only after the contract corridor landed
  - do not let new perf work invent a parallel owner or canonical contract
- `phase-152x` current seam:
  - `--backend llvm` / `--emit-exe` daily mainline is already `ny-llvmc`
  - remaining mismatch is `.o` emit:
    - `src/runner/modes/common_util/exec.rs::llvmlite_emit_obj_lib(...)`
    - `src/runner/modes/common_util/exec.rs::ny_llvmc_emit_obj_lib(...)` compatibility alias still routes to llvmlite
    - `src/runner/product/llvm/mod.rs::emit_requested_object_or_exit(...)`
    - `src/bin/ny_mir_builder.rs` `obj` / `exe` still force `NYASH_LLVM_USE_HARNESS=1`
  - cut goal:
    - current object emit reads `ny-llvmc --emit obj`
    - llvmlite becomes explicit compat/archive keep only
  - current landed slice:
    - `src/runner/product/llvm/mod.rs::emit_requested_object_or_exit(...)` now routes object emit to `ny_llvmc_emit_obj_lib(...)`
    - `src/runner/modes/common_util/exec.rs::ny_llvmc_emit_obj_lib(...)` now uses `ny-llvmc --emit obj`
    - `src/bin/ny_mir_builder.rs` `obj|exe` no longer force `NYASH_LLVM_USE_HARNESS=1`
- first exact slices:
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/plugin/map_substrate.rs`
