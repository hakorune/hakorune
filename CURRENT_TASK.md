# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-22
Scope: current lane / next lane / restart order only.

## Purpose

- root から active lane / next lane に最短で戻る
- landed history と rejected history は phase docs / investigations を正本にする
- `CURRENT_TASK.md` 自体は ledger にしない

## Quick Restart Pointer

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/05-Restart-Quick-Resume.md`
3. `docs/development/current/main/10-Now.md`
4. `docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md`
5. `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
6. `docs/development/current/main/design/value-corridor-generic-optimization-contract.md` (`137x-H` generic contract vocabulary)
7. `docs/development/current/main/phases/phase-289x/README.md` (`runtime-wide value/object` vocabulary; `137x-F` consumes it as the constrained bridge)
8. `docs/development/current/main/phases/phase-289x/289x-90-runtime-value-object-design-brief.md`
9. `docs/development/current/main/phases/phase-289x/289x-91-runtime-value-object-task-board.md`
10. `docs/development/current/main/phases/phase-289x/289x-92-value-boundary-inventory-ledger.md`
11. `docs/development/current/main/phases/phase-289x/289x-93-demand-vocabulary-ledger.md`
12. `docs/development/current/main/phases/phase-289x/289x-94-container-demand-table.md`
13. `docs/development/current/main/phases/phase-289x/289x-95-array-text-residence-pilot.md`
14. `docs/development/current/main/phases/phase-289x/289x-96-demand-backed-cutover-inventory.md`
15. `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
16. `docs/development/current/main/phases/phase-137x/README.md`
17. `docs/development/current/main/phases/phase-292x/README.md`
18. `docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md`
19. `docs/development/current/main/phases/phase-292x/292x-91-task-board.md`
20. `docs/development/current/main/phases/phase-292x/292x-92-inc-codegen-analysis-debt-ledger.md`
21. `docs/development/current/main/phases/phase-292x/292x-93-array-rmw-window-route-card.md`
22. `docs/development/current/main/phases/phase-292x/292x-94-array-string-len-window-route-card.md`
23. `docs/development/current/main/phases/phase-292x/292x-95-array-string-len-keep-live-route-card.md`
24. `docs/development/current/main/phases/phase-292x/292x-96-array-string-len-source-only-route-card.md`
25. `docs/development/current/main/phases/phase-292x/292x-97-array-string-len-c-analyzer-deletion-card.md`
26. `docs/development/current/main/phases/phase-292x/292x-98-array-rmw-c-analyzer-deletion-card.md`
27. `docs/development/current/main/phases/phase-292x/292x-99-string-direct-set-window-metadata-card.md`
28. `docs/development/current/main/phases/phase-292x/292x-100-generic-method-route-policy-metadata-card.md`
29. `docs/development/current/main/phases/phase-292x/292x-101-exact-seed-ladder-function-route-tags-card.md`
30. `docs/development/current/main/investigations/phase137x-inc-codegen-thin-tag-inventory-2026-04-22.md`
31. `docs/development/current/main/phases/phase-291x/README.md` (`CoreBox surface catalog` landed reference)
32. `docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md`
33. `docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md`
34. `docs/development/current/main/phases/phase-137x/137x-94-textlane-value-allocator-implementation-gate.md`
35. `docs/development/current/main/phases/phase-137x/137x-95-mir-backend-seam-closeout-before-textlane.md`
36. `docs/development/current/main/phases/phase-137x/137x-93-container-primitive-design-cleanout.md`
37. `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
38. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
39. `docs/development/current/main/design/perf-owner-first-optimization-ssot.md` (`137x-H` owner-first optimization に戻るとき)
40. `docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md`
41. `docs/development/current/main/design/string-value-model-phased-rollout-ssot.md`
42. `docs/development/current/main/phases/phase-137x/phase137x-text-lane-rollout-checklist.md`
43. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
44. `docs/development/current/main/design/string-birth-sink-ssot.md`
45. `docs/development/current/main/15-Workstream-Map.md`
46. `git status -sb`
47. `tools/checks/dev_gate.sh quick`
48. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md` (`phase-29bq` に戻るときだけ)

## Current Lane

- expected worktree:
  - clean is expected right now
  - rejected slot-store boundary probe is parked separately in `stash@{0}` as `wip/concat-slot-store-window-probe`
- active lane:
  - compiler cleanup lane is primary now
  - current-state token:
    - `phase-292x .inc codegen thin tag cleanup`
  - active phase:
    - `docs/development/current/main/phases/phase-292x/README.md`
  - method anchor:
    - `docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md`
  - taskboard:
    - `docs/development/current/main/phases/phase-292x/292x-101-exact-seed-ladder-function-route-tags-card.md`
  - current app slice:
    - phase-291x CoreBox surface catalog is landed
    - StringBox / MapBox catalog seams are pinned
    - safe legacy std/debt deletions are landed
  - phase-137x remains observe-only
  - current perf blocker stays recorded as `137x-H46 text-cell residence/materialization design`, but it does not preempt app work unless app implementation is actually blocked
  - current phase goal:
    - make `.inc` a thin boundary glue layer
    - move route legality and shape ownership to MIR-owned metadata
    - keep `.inc` on metadata read / field validation / emit / skip / fail-fast only
    - prevent new `.inc` raw MIR analysis debt with `tools/checks/inc_codegen_thin_shim_guard.sh`
    - first implementation card `array_rmw_window` is landed as metadata-first lowering
    - `array_string_len_window` len-only route is landed as metadata-first lowering
    - `array_string_len_window` keep-live source reuse is landed as metadata-first lowering
    - `array_string_len_window` source-only direct-set reuse is landed as metadata-first lowering
    - legacy `array_string_len_window` C analyzer deletion is landed
    - legacy `array_rmw_window` C analyzer deletion is landed
    - string direct-set source-window metadata is landed
    - `generic_method.has` route policy metadata is landed
    - next implementation card is exact seed ladders to function-level backend route tags
  - current app/runtime gap read:
    - ArrayBox surface catalog is landed and phase-290x is closed
    - StringBox surface catalog is landed for the first stable rows and pinned by `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh`
    - `apps/std/string.hako` is std sugar; the old diagnostic `apps/std/string2.hako` stub was deleted during cleanup triage
    - first StringBox stable target is `length/len/size/substr/substring/concat/indexOf/find/replace/trim/lastIndexOf/contains`
    - CoreBox router follow-up has moved `StringBox.length/len/size`, `StringBox.substring/substr`, `StringBox.concat`, `StringBox.trim`, `StringBox.contains`, one-arg `StringBox.lastIndexOf`, `StringBox.replace`, `StringBox.indexOf/find`, `ArrayBox.length/size/len`, `ArrayBox.push`, `ArrayBox.slice`, `MapBox.size`, and `MapBox.len` to the Unified value path; remaining cleanup is ArrayBox `get/set/pop/remove/insert` and remaining MapBox rows
    - MapBox Rust vtable surface is now cataloged; legacy `apps/std/map_std.hako`, unused `map_keys_values_bridge.hako`, and live `apps/lib/boxes/map_std.hako` prelude scaffold were deleted, while compat ABI, MIR lowering, and `.hako` extended routes remain separate cleanup cards
    - static-box receiver friction remains a semantics/diagnostics issue
    - two-arg `lastIndexOf` remains a separate runtime gap
  - current blocker token:
    - `move exact seed ladders to function-level backend route tags`
  - stop rule:
    - app lane is primary; phase-137x is observe-only unless app work is actually blocked
    - helper-local perf reopen is closed; new perf cards need one-family owner pin plus one-card rollback
  - implementation mode:
    - `137x-E0 MIR / backend seam closeout` is closed
    - `137x-E0.1 legacy seam shrink` is closed enough to unblock `137x-E1`
      - removed the old `9-block` `kilo_micro_array_string_store` exact seed branch; the compact `8-block` direct producer is now the only accepted seed shape
      - attempted shared-receiver metadata-only deletion and exposed the remaining metadata gap for direct/front fixtures
      - keep the exact seed bridge itself until `137x-E1` gives array-string store a keeper TextLane / `ArrayStorage::Text` route
    - `137x-E0.2 shared-receiver alias metadata coverage` is closed enough to unblock `137x-E1`
      - active const-suffix / insert-mid shared-receiver fixtures now carry MIR-owned `read_alias.shared_receiver`
      - backend `.inc` consumes metadata only and no longer scans later instructions to rediscover shared receiver legality
    - `137x-E1 minimal TextLane / ArrayStorage::Text` is landed
      - implemented as array-internal storage/residence only: `String = value`, public Array/String ABI, and MIR legality stay unchanged
      - array-string kernel read/store/mutate routes now use text raw APIs; generic/mixed ArrayBox routes degrade to Boxed instead of making `TextLane` semantic truth
      - retired the array-string store `BorrowedHandleBox` retarget executor path; runtime now stores text residence or degrades mixed values without re-planning alias legality
    - closed CURRENT_STATE token: `137x-F Value Lane bridge`
    - `137x-F` Value Lane bridge is closed; `137x-F1 demand-to-lane executor bridge` and `137x-F2 producer outcome manifest split` are landed
    - `137x-H` runtime cleanup: removed dead `ValueLaneAction::PublishBoundary`; array string store now selects `TextCellResidence` or `GenericBoxResidence` once and the executor path only consumes the preselected action
    - `137x-H` backend cleanup: `string_concat_emit_routes` now uses `kernel_plan_read_publication_boundary_window` for publication-boundary checks and no longer replays the corridor fallback in the insert-mid shared-receiver branch
    - `137x-H` backend cleanup: `match_piecewise_slot_hop_substring_consumer` is retired; slot-hop substring continuation is now MIR-owned `StringKernelPlan.slot_hop_substring` metadata
    - `137x-H` backend cleanup: the exact array-string seed bridge no longer rescans raw 8-block MIR JSON; it consumes MIR-owned `array_string_store_micro_seed_route` metadata and only selects the existing temporary specialized emitter
    - `137x-H` backend cleanup: removed unused `hako_llvmc_string_corridor_read_insert_mid_window_plan_values`; kernel-plan reader is now the only insert-mid window SSOT
    - `137x-H` backend cleanup: removed the standalone corridor triplet reader; `direct_kernel_entry` substring proof now goes through centralized `hako_llvmc_string_kernel_plan_read_concat_triplet_values` (kernel-first, corridor compat fallback)
    - `137x-H1` MIR string value lowering cleanup is closed
      - first cut: string constants and string `Add` results stay `MirType::String` instead of creating `StringBox` origin metadata
      - method dispatch may still route `MirType::String` to `StringBox` runtime methods as a compatibility boundary; it must not mark the value as born object-world text
      - `text.ref` / `text.owned` / `publish` remain plan metadata and verifier vocabulary in this cut; first-class MIR instructions are deferred until the metadata contract needs a broader representation
      - Rust fresh-handle reissue sites must go through named publication/cache boundary helpers instead of calling the raw handle issuer from export helpers
    - `137x-H2` MIR JSON text object boundary shrink is closed
      - retire the active `compat_text_primitive.rs` module name from Rust callers
      - keep the remaining Rust-side `MIR(JSON text) -> object path` route as an explicit backend boundary
      - exact seed bridge deletion remains blocked until active array-string store coverage is proven outside the exact matcher
      - verification: targeted `rustfmt --check`, `git diff --check`, `cargo check -q`, `cargo check -q -p nyash_kernel`, `phase137x_direct_emit_array_store_string_contract.sh`, and `tools/checks/dev_gate.sh quick` passed
    - `137x-H3` exact array-string-store emitted length seam is closed
      - perf-first baseline:
        - `kilo_micro_array_string_store = C 10 ms / Ny AOT 10 ms`
        - `ny_aot_instr=26917228`, `ny_aot_cycles=34122757`
        - `ny_main` asm/top owner: `__strlen_evex 53.84%`, `ny_main 45.10%`
      - implementation: the temporary exact seed bridge now emits the shape-known stored text length (`seed_len + 2`) instead of per-iteration `strlen(slot)`
      - result:
        - `kilo_micro_array_string_store = C 10 ms / Ny AOT 9 ms`
        - `ny_aot_instr=18866112`, `ny_aot_cycles=27213979`
        - regenerated exact asm has no `strlen@plt` call in `ny_main`
      - guard held: no bridge widening, no new route legality, and no keeper-architecture promotion
    - `137x-H4` exact array-string-store out-buffer seam is closed
      - perf-first baseline after H3:
        - `kilo_micro_array_string_store = C 10 ms / Ny AOT 9 ms`
        - exact `ny_main` top owner is the remaining stack-copy loop; annotated samples sit on the `out` temp copy and slot tail store
      - implementation: the exact seed bridge writes `text + "xy"` directly into the selected array slot, then updates loop-carried `text` from `slot + 2`
      - result:
        - `kilo_micro_array_string_store = C 10 ms / Ny AOT 8 ms`
        - `ny_aot_instr=11666577`, `ny_aot_cycles=20300845`
        - regenerated exact asm has no separate `out` stack buffer and no runtime/public helper call in `ny_main`
      - guard held: no route widening, no new MIR legality, no runtime helper, and no public ABI change
    - `137x-H5` middle same-slot subrange store materialization seam is closed
      - perf-first baseline after H4:
        - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 10 ms`
        - top owners: `array_string_len_by_index` closure, same-slot subrange store closure, and Rust dealloc
      - implementation: the existing `nyash.array.string_insert_mid_subrange_store_hisiiii` runtime-private helper now mutates the safe same-length subrange shape in place after checking only required UTF-8 byte boundaries
      - result:
        - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 8 ms`
        - `ny_aot_instr=83021976`, `ny_aot_cycles=26509766`
        - `kilo_kernel_small = C 81 ms / Ny AOT 19 ms`
      - guard held: no MIR route widening, no public ABI, and no helper-name semantic ownership
    - `137x-H6` array text length substrate seam is closed
      - perf-first baseline after H5:
        - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 8 ms`
        - top owner is still `array_string_len_by_index` closure
      - implementation: `ArrayBox::slot_text_len_raw(...)` gives the runtime-private len helper a direct text-lane length substrate instead of the generic `slot_with_text_raw` closure path
      - result:
        - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 9 ms` in the rerun noise band
        - `ny_aot_instr=80862657`, `ny_aot_cycles=26265409`
        - `kilo_kernel_small = C 84 ms / Ny AOT 19 ms`
      - guard held: no MIR route widening, no known-length inference, and no public ABI change
      - next seam: eliminate the `nyash.array.string_len_hi` call from lowering when MIR proves same-length loop-carried text
    - `137x-H7` same-length loop-carry length call seam is closed
      - baseline:
        - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 8 ms`
        - `ny_aot_instr=80862956`, `ny_aot_cycles=26254374`
        - asm top owner remains `nyash.array.string_len_hi`
      - implementation: backend lowering fuses only the proven same-slot loop-carry window into runtime-private `nyash.array.string_insert_mid_subrange_len_store_hisi`
      - result:
        - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`
        - `ny_aot_instr=58004175`, `ny_aot_cycles=17079682`
        - generated `ny_main` no longer calls `nyash.array.string_len_hi`; the new top owner is inside the fused slot mutation helper plus `memmove` / `String::Drain`
      - guard held: no array-wide seed length inference, no public ABI, and no runtime legality ownership
    - `137x-H8` same-length loop-carry byte rewrite seam is closed
      - baseline after H7:
        - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`
        - `ny_aot_instr=58004175`, `ny_aot_cycles=17079682`
        - helper-symbol asm/top owner is the fused helper closure with secondary `memmove` / `String::Drain`
      - implementation: replace only the proven same-length `insert_str -> drain -> truncate` path with a fixed-length byte rewrite
      - result:
        - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`
        - `ny_aot_instr=49691974`, `ny_aot_cycles=12941203`
        - `String::Drain` and libc `memmove` no longer appear as top owners
      - guard held: no allocator/arena work, no public ABI, and no new MIR legality
    - `137x-H9` same-length loop-carry small shift seam is rejected
      - baseline after H8:
        - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`
        - `ny_aot_instr=49693471`, `ny_aot_cycles=13039022`
        - fused helper closure remains top owner; libc `memmove` is still visible as secondary owner in the saved optimization bundle
      - trial: replace the runtime-private small byte shifts with bytewise overlap loops while keeping large strings on the existing `ptr::copy` path
      - result: `ny_aot_instr=54372282`, `ny_aot_cycles=13785721`; worse than H8, so code was reverted
      - guard held: no benchmark-name dispatch, no public ABI, no new MIR legality
    - `137x-H10` text-resident slot update fast path seam is closed
      - baseline remains H8: `ny_aot_instr=49693471`, `ny_aot_cycles=13039022`
      - owner: fused helper closure, with hot annotate pointing at text slot write-lock / fast-path entry
      - implementation: add a text-resident-only ArrayBox update path and keep mixed/boxed fallback cold
      - result: `ny_aot_instr=40152332`, `ny_aot_cycles=12636090`
      - guard held: no public ArrayBox semantic change, no MIR legality, no public ABI
    - `137x-H11` exclusive text-region lock owner is blocked
      - evidence: saved bundle `target/perf_state/optimization_bundle/137x-h11-loopcarry-owner` shows the remaining dominant sample on the `ArrayBox` text slot write-lock fast path
      - decision: do not remove the lock in runtime based on helper name or benchmark shape
      - required next contract: MIR-owned exclusive text-region / proof-region contract that can justify lock hoist or lock elision
      - side evidence: `kilo_kernel_small_hk = C 81 ms / Ny AOT 26 ms`, parity ok; split-string exact fronts are no longer the immediate blocker
    - `137x-H12` MIR-owned loopcarry route SSOT is active
      - problem: `hako_llvmc_ffi_generic_method_get_window.inc` still rediscovers the active `array.get -> string edit -> array.set -> trailing length` route from raw MIR JSON
      - decision: MIR must own the fused loopcarry len-store route plan; `.inc` may only consume the plan and emit/skip
      - first slice: landed backend-consumable MIR metadata for the active loopcarry len-store window and made `.inc` consume it without the legacy matcher
      - evidence: route trace hits `array_string_loopcarry_len_store_window reason=mir_route_plan`; `tools/checks/dev_gate.sh quick` is green
      - deletion gate: legacy C-side window matcher removed from the active lowering path; extend the same metadata-first contract to any remaining direct/front loopcarry windows
    - `137x-H13` MIR-owned piecewise direct-set consumer is active
      - problem: direct-front `Pieces3` emit routes still decide direct `array.set` consumer legality in `.inc`
      - decision: `StringKernelPlan.read_alias.direct_set_consumer` is the piecewise plan fact; generic `FunctionMetadata.value_consumer_facts` owns single direct `set` sink facts; `.inc` only consumes metadata
      - first slice: moved string concat / insert direct-set consumer checks to MIR metadata
      - second slice: retired backend `has_direct_array_set_consumer(...)`; all remaining direct-set checks read `hako_llvmc_value_has_direct_set_consumer(...)`
      - third slice: `StringKernelPlan.slot_hop_substring` now owns the same-block slot-hop substring consumer, hop window, and skip indices; `.inc` only reads the route metadata before emitting/skipping
      - fourth slice: `FunctionMetadata.array_string_store_micro_seed_route` now owns the current compact 8-block exact seed proof; `.inc` reads this route metadata and no longer carries the raw JSON shape scanner
      - fifth slice: retired dead `kilo_micro_concat_hh_len` exact `.inc` bridge; current direct MIR already stays on the generic/metadata route and the specialized 5-block matcher was not active
      - sixth slice: `FunctionMetadata.concat_const_suffix_micro_seed_route` now owns the still-active `kilo_micro_concat_const_suffix` exact route proof; `.inc` reads metadata and keeps only the temporary emitter
      - verification: direct MIR metadata probes export the array-store and const-suffix routes; `cargo test array_string_store_micro_seed --lib`, `cargo test concat_const_suffix_micro_seed --lib`, `bash tools/perf/build_perf_release.sh`, `phase137x_direct_emit_array_store_string_contract.sh`, `tools/checks/dev_gate.sh quick`, `git diff --check`, exact `kilo_micro_array_string_store` microstat (`C 10 ms / Ny AOT 6 ms`), post-retirement `kilo_micro_concat_hh_len` microstat (`C 3 ms / Ny AOT 3 ms`), and const-suffix metadata-route microstat (`C 3 ms / Ny AOT 4 ms`) passed
      - seventh slice: `hako_llvmc_ffi_string_loop_seed_substring_concat.inc` now consumes existing MIR `StringKernelPlan.loop_payload` + `stable_length_scalar` metadata and no longer scans raw block/op JSON
      - seventh slice verification: `bash tools/perf/build_perf_release.sh`, `phase137x_direct_emit_substring_concat_phi_merge_contract.sh`, `phase137x_direct_emit_substring_concat_post_sink_shape.sh`, `tools/checks/dev_gate.sh quick`, `git diff --check`, and exact `kilo_micro_substring_concat` microstat (`C 3 ms / Ny AOT 4 ms`) passed
      - eighth slice: `FunctionMetadata.substring_views_micro_seed_route` now owns the active `kilo_micro_substring_views_only` source literal/length and loop-bound proof; `.inc` reads this route metadata and no longer scans raw block/op JSON
      - eighth slice verification: direct MIR metadata probe exports `substring_views_micro_seed_route` with `source_len=16`, `loop_bound=300000`, and proof `kilo_micro_substring_views_only_5block`; `cargo test substring_views_micro_seed --lib`, `bash tools/perf/build_perf_release.sh`, `tools/checks/dev_gate.sh quick`, `git diff --check`, exact `kilo_micro_substring_views_only` microstat (`C 3 ms / Ny AOT 3 ms`), and adjacent `kilo_micro_len_substring_views` microstat (`C 3 ms / Ny AOT 3 ms`) passed
      - ninth slice: retired the dead length-hot exact matcher family; current `kilo_micro_len_substring_views` and `method_call_only_small` direct MIR are 4-block generic/metadata routes, so the obsolete 5/6-block raw scanners and their dedicated emitter are deleted
      - ninth slice verification: `bash tools/perf/build_perf_release.sh` passed after removing the hidden include-order brace dependency; `tools/checks/dev_gate.sh quick`, `git diff --check`, `kilo_micro_len_substring_views` microstat (`C 3 ms / Ny AOT 3 ms`), `method_call_only_small` microstat (`C 2 ms / Ny AOT 2 ms`), and `kilo_micro_substring_concat` microstat (`C 3 ms / Ny AOT 3 ms`) passed
    - `137x-H14` MIR-owned string search seed route is closed
      - problem: `hako_llvmc_ffi_string_search_seed.inc` still proves the exact `indexOf` leaf/line micro routes by rescanning raw MIR JSON in C
      - decision: MIR owns the route proof (`variant`, `rows`, `ops`, literal lengths, optional `flip_period`, and proof name); `.inc` may only consume metadata and select the existing temporary emitter
      - first slice: add `FunctionMetadata.indexof_search_micro_seed_route` for `kilo_leaf_array_string_indexof_const` and `kilo_micro_indexof_line`, then delete the C-side raw block/op scanners
      - initial guard: preserve `NYASH_LLVM_SKIP_INDEXOF_LINE_SEED` while generic lowering was slower; retired in H15.7 after text-state residence reached keeper speed
      - first slice result: `FunctionMetadata.indexof_search_micro_seed_route` now exports `variant=leaf` / `proof=kilo_leaf_array_string_indexof_const_10block` and `variant=line` / `proof=kilo_micro_indexof_line_15block`; `hako_llvmc_match_indexof_{leaf,line}_ascii_seed(...)` are metadata consumers and no longer scan raw blocks/instructions
      - verification: `cargo test indexof_search_micro_seed --lib`, touched-file `rustfmt --check`, `bash tools/perf/build_perf_release.sh`, direct MIR metadata probes for both fronts, `tools/checks/dev_gate.sh quick`, and manual `NYASH_LLVM_ROUTE_TRACE=1 tools/ny_mir_builder.sh ...` leaf route trace passed
      - perf note: `kilo_micro_indexof_line` remains `C 4 ms / Ny AOT 4 ms`; `kilo_leaf_array_string_indexof_const` emits the existing exact leaf bridge but remains slow (`C 4-5 ms / Ny AOT 64-73 ms`) because that emitter still calls `nyash.string.indexOf_ss` in the loop; treat that as the next performance seam, not part of this SSOT cleanup
    - `137x-H14.1` MIR-owned indexOf predicate action is closed
      - problem: the leaf exact route is selected from MIR metadata, but the backend leaf emitter still replays runtime string search in the loop
      - decision: MIR route metadata must also own the result-use/action contract: `found_predicate`, `literal_membership_predicate`, and per-candidate outcomes (`line_seed => found`, `none_seed => not_found`)
      - first slice: export the predicate action metadata, make `.inc` validate it, and replace the leaf loop runtime `nyash.string.indexOf_ss` call with the metadata-owned literal membership predicate
      - guard: `.inc` must not infer the predicate from literal spelling or helper names; it may only emit the membership predicate after the MIR metadata contract matches
      - result: leaf exact emitter no longer declares/calls `nyash.string.indexOf_ss`; it uses the metadata-owned literal membership predicate like the line bridge
      - verification: `cargo test indexof_search_micro_seed --lib`, touched-file `rustfmt --check`, `bash tools/perf/build_perf_release.sh`, direct MIR metadata probes for leaf/line, manual route trace, `nm -u`/`objdump` search for runtime search calls, `tools/checks/dev_gate.sh quick`, and `git diff --check` passed
      - perf result: `kilo_leaf_array_string_indexof_const = C 4 ms / Ny AOT 4 ms`; `kilo_micro_indexof_line = C 5 ms / Ny AOT 4 ms`
    - `137x-H14.2` exact search emitter surface shrink is closed
      - problem: the remaining temporary exact search bridge still carries two near-duplicate C emitters for leaf and line
      - decision: keep the MIR-owned route proof/action, but collapse backend emission into one optional-flip emitter; `.inc` must not regain route legality or predicate ownership
      - first slice: replace `hako_llvmc_emit_indexof_leaf_ir(...)` and `hako_llvmc_emit_indexof_line_ir(...)` with one `hako_llvmc_emit_indexof_seed_ir(...)` that consumes `flip_period=0|16`
      - result: leaf and line now share `hako_llvmc_emit_indexof_seed_ir(...)`; leaf passes `flip_period=0`, line passes MIR-owned `flip_period=16`, and matcher functions remain metadata consumers only
      - verification: `git diff --check`, `bash tools/perf/build_perf_release.sh`, `tools/checks/dev_gate.sh quick`, exact `kilo_leaf_array_string_indexof_const` microstat (`C 4 ms / Ny AOT 3 ms`), and exact `kilo_micro_indexof_line` microstat (`C 4 ms / Ny AOT 4 ms`) passed
    - `137x-H14.3` exact search matcher surface shrink is closed
      - problem: leaf/line dispatch wrappers still duplicate the same metadata parse, route validation, trace, and emitter call
      - historical decision: keep public wrapper names for dispatch and `NYASH_LLVM_SKIP_INDEXOF_LINE_SEED` while generic lowering was slower; superseded by H15.7 retirement
      - first slice: add one `hako_llvmc_match_indexof_ascii_seed_variant(...)` helper; wrappers provide only variant/proof/trace constants
      - result: leaf/line wrappers are now thin constants-only dispatch surfaces; shared parse/validation/trace/emitter mechanics live in `hako_llvmc_match_indexof_ascii_seed_variant(...)`
      - verification: `git diff --check`, `bash tools/perf/build_perf_release.sh`, `tools/checks/dev_gate.sh quick`, exact `kilo_leaf_array_string_indexof_const` microstat (`C 5 ms / Ny AOT 4 ms`), and exact `kilo_micro_indexof_line` microstat (`C 4 ms / Ny AOT 4 ms`) passed
    - `137x-G` allocator / arena pilot is rejected for now because allocator/copy samples are secondary, not the dominant owner
    - `137x-H15` generic array/text observer route is closed
      - problem: the remaining exact search bridge is thin, but the generic path still lacks a MIR-owned read-side observer contract for `array.get(i).indexOf(needle)`
      - decision: MIR owns `array_text_observer_routes` with legality/provenance/consumer facts; first consumer is `observer_kind=indexof`
      - guard: `.inc` may consume route metadata and map it to helper calls, but it must not rediscover raw `indexOf` windows or make helper symbols the MIR truth
      - first slice: add metadata + JSON export + unit tests; keep the exact search bridge until metadata-driven generic lowering reaches keeper speed
      - first slice result: direct MIR metadata probes for `kilo_leaf_array_string_indexof_const` and `kilo_micro_indexof_line` now export `array_text_observer_routes[0]` with `observer_kind=indexof`, `consumer_shape=found_predicate`, `publication_boundary=none`, and `result_repr=scalar_i64`
      - second slice result: active indexOf observer prepass/get lowering now consumes `array_text_observer_routes` metadata; raw C window scanners are no longer called from those active surfaces
      - investigation result: seed-off fell on `array.get(row).indexOf("line")` in the hot 400k loop where `array_text_observer_routes[0].keep_get_live=false`, but H15 generic get lowering still emitted an unused `nyash.array.slot_load_hi`; perf owner was repeated Text-lane object materialization (`ArrayBox::boxed_from_text` plus malloc/free), not `indexOf` search itself
      - third slice result: observer get lowering now honors `keep_get_live=false`; it suppresses the unused public get/materialize path while still remembering the array/index source and emitting the metadata-owned observer helper
      - verification: `cargo test array_text_observer --lib`, `cargo build --release -j 24`, `bash tools/perf/build_perf_release.sh`, `tools/checks/dev_gate.sh quick`, direct MIR metadata probes, active `.inc` raw-scanner grep, seed-off IR no-`slot_load_hi` call grep, exact `kilo_micro_indexof_line` microstat (`C 4 ms / Ny AOT 4 ms`), seed-off `kilo_micro_indexof_line` microstat (`C 4 ms / Ny AOT 10 ms`), and `git diff --check` passed
      - deletion probe: `NYASH_LLVM_SKIP_INDEXOF_LINE_SEED=1` improved from `Ny AOT 497 ms` to `Ny AOT 10 ms`, but bridge deletion remains rejected until generic path reaches keeper speed
      - post-fix 10 ms owner: seed-off generic IR still calls `nyash.box.from_i8_string_const("line")` inside the 400k loop, then calls `nyash.array.string_indexof_hih`; perf top is `nyash.box.from_i8_string_const` / HashMap hashing / `strlen` plus `memchr`, while exact seed emits pointer-array membership and no runtime search
      - fourth slice result: `array_text_state_residence_route` is now stored as its own `FunctionMetadata` field and MIR JSON no longer aliases it from `indexof_search_micro_seed_route`; the generic residence route still derives its payload from the exact proof/action route until H15.5
      - fourth slice verification: route trace emits `indexof_line_text_state_residence`; seed-off `kilo_micro_indexof_line = C 5 ms / Ny AOT 4 ms`; exact `kilo_micro_indexof_line = C 4 ms / Ny AOT 3 ms`; targeted tests, `bash tools/perf/build_perf_release.sh`, `tools/checks/dev_gate.sh quick`, and `git diff --check` passed
      - fifth slice result: `array_text_state_residence_route` top-level now holds only generic contract fields; exact `variant/proof/backend_action/result_use/candidate_outcomes` are quarantined under `temporary_indexof_seed_payload`, and `.inc` validates the top-level contract before consuming the temporary payload
      - fifth slice verification: route trace still emits `indexof_line_text_state_residence`; seed-off `kilo_micro_indexof_line = C 4 ms / Ny AOT 4 ms`; exact `kilo_micro_indexof_line = C 5 ms / Ny AOT 3 ms`; targeted tests, `bash tools/perf/build_perf_release.sh`, `tools/checks/dev_gate.sh quick`, and `git diff --check` passed
      - sixth slice result: unused raw `indexOf` window/liveness rediscovery analyzers were removed from active `.inc` compilation; active observer lowering now includes only metadata defer state plus `array_text_observer_routes` consumer lowering
      - sixth slice verification: route trace still emits `indexof_line_text_state_residence`; top-level residence JSON remains generic-only; seed-off `kilo_micro_indexof_line = C 4 ms / Ny AOT 3 ms`; exact `kilo_micro_indexof_line = C 4 ms / Ny AOT 3 ms`; targeted tests, `bash tools/perf/build_perf_release.sh`, `tools/checks/current_state_pointer_guard.sh`, and `git diff --check` passed
      - seventh slice result: exact leaf/line C dispatch wrappers and backend env `NYASH_LLVM_SKIP_INDEXOF_LINE_SEED` are retired; exact and compatibility-skip runs now route through `array_text_state_residence_route`
      - seventh slice verification: route trace emits `indexof_line_text_state_residence` with no `indexof_line_micro`; exact `kilo_micro_indexof_line = C 5 ms / Ny AOT 3 ms`; retired-env probe stays keeper-fast (`C 4 ms / Ny AOT 4 ms`); targeted tests, `bash tools/perf/build_perf_release.sh`, `tools/checks/current_state_pointer_guard.sh`, `tools/checks/dev_gate.sh quick`, and `git diff --check` passed
      - eighth slice result: renamed the remaining backend surface from `hako_llvmc_ffi_string_search_seed.inc` to `hako_llvmc_ffi_indexof_text_state_residence.inc`; the only remaining seed vocabulary is the explicit MIR temporary payload field
      - eighth slice verification: post-rename route trace still emits `indexof_line_text_state_residence`; `kilo_micro_indexof_line = C 5 ms / Ny AOT 4 ms`; `bash tools/perf/build_perf_release.sh`, `cargo test array_text_state_residence --lib`, `tools/checks/current_state_pointer_guard.sh`, `tools/checks/dev_gate.sh quick`, and `git diff --check` passed
      - ninth slice result: `FunctionMetadata.indexof_search_micro_seed_route` and MIR JSON `indexof_search_micro_seed_route` are retired; `array_text_state_residence_route` is the only exported backend route owner for this path
      - ninth slice verification: MIR JSON has no `indexof_search_micro_seed_route`, still has `array_text_state_residence_route.temporary_indexof_seed_payload`; route trace emits `indexof_line_text_state_residence`; `kilo_micro_indexof_line = C 4 ms / Ny AOT 3 ms`; targeted tests, `bash tools/perf/build_perf_release.sh`, `tools/checks/current_state_pointer_guard.sh`, `tools/checks/dev_gate.sh quick`, and `git diff --check` passed
      - H15 closeout: `temporary_indexof_seed_payload` remains explicit, fixture-backed, and quarantined until a generic residence emitter replaces it
      - next blocker token: `137x-H owner-first optimization return`
      - acceptance checks: `cargo test indexof_search_micro_seed --lib`, `cargo test array_text_observer --lib`, `cargo test array_text_state_residence --lib`, `bash tools/perf/build_perf_release.sh`, route trace showing `indexof_line_text_state_residence`, exact/retired-flag `kilo_micro_indexof_line` keeper microstats, and `tools/checks/current_state_pointer_guard.sh`
      - keeper evidence remains direct-only; exact/middle/whole gates must be recorded before accepting each implementation slice
      - next step: rerun owner-first perf evidence for the active kilo front before source edits
    - `137x-H16` exact array-string store text-shift seam is closed
      - perf-first baseline:
        - `kilo_micro_array_string_store = C 10 ms / Ny AOT 7 ms`
        - `ny_aot_instr=11671010`, `ny_aot_cycles=20593774`
        - `ny_main` owns 97.81% of AOT cycles; annotate points at the emitted `slot + 2 -> text` 16-byte copy
      - decision: MIR route metadata must expose the follow-up substring window used to update loop-carried text; `.inc` may use that metadata to emit text-state update mechanics, but must not rediscover the route from raw blocks
      - guard: this remains the temporary exact array-store bridge; do not widen route legality, public ABI, or runtime ownership
      - acceptance checks: `cargo test array_string_store_micro_seed --lib`, `bash tools/perf/build_perf_release.sh`, exact route trace, `phase137x_direct_emit_array_store_string_contract.sh`, exact `kilo_micro_array_string_store` microstat, and `tools/checks/current_state_pointer_guard.sh`
      - implementation: `array_string_store_micro_seed_route` exports `next_text_window_start=2` and `next_text_window_len=16`; the exact emitter updates loop-carried text from that MIR-owned window via vector shuffle instead of reading `slot + 2`
      - result:
        - `kilo_micro_array_string_store = C 10 ms / Ny AOT 5 ms`
        - `ny_aot_instr=11670690`, `ny_aot_cycles=9512639`
        - post-change asm uses `vpalignr` for the text-state update; the previous `slot + 2 -> text` copy is gone
      - guard held: no route widening, no public ABI, no runtime ownership, and the bridge remains temporary exact metadata
      - next step: rerun owner-first perf evidence before the next exact-bridge shrink
    - `137x-H17` exact text terminator store seam is closed
      - perf-first baseline is the H16 post-change asm:
        - `kilo_micro_array_string_store = C 10 ms / Ny AOT 5 ms`
        - `ny_aot_instr=11670690`, `ny_aot_cycles=9512639`
        - hot loop still spends samples on `movb $0, text+16`
      - decision: remove only the loop-body loop-carried text terminator store; the exact emitter uses fixed 16-byte vector payload and no C-string reads from `text`
      - result:
        - `kilo_micro_array_string_store = C 10 ms / Ny AOT 5 ms`
        - `ny_aot_instr=10870861`, `ny_aot_cycles=9526782`
        - regenerated asm has no loop-body `movb $0, text+16`
      - guard held: slot terminator stores and all route metadata stay unchanged
      - next blocker token: `137x-H owner-first optimization return`
      - next step: rerun owner-first perf evidence before the next exact-bridge shrink
    - `137x-H18` exact loop-carried text SSA seam is closed
      - front: `kilo_micro_array_string_store`
      - current owner:
        - `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_string_store 1 3`
        - `C 9 ms / Ny AOT 5 ms`
        - `ny_aot_instr=10870942`, `ny_aot_cycles=9536178`, `ny_aot_ipc=1.14`
        - asm: `ny_main` 93.62%; local owners are slot vector store, loop-carried text state store, loop increment, suffix stores
      - hot transition: `store <16 x i8> %text.next -> %text.ptr` keeps loop-carried text in memory even though H16 metadata already proves the next window
      - next seam: make the exact emitter carry text as an LLVM SSA vector phi and keep array slot stores unchanged
      - reject seam: do not remove `arr.set` / slot stores in this slice; array-store deadness needs a separate MIR-owned no-escape / consumer proof
      - result:
        - `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`
        - `ny_aot_instr=9270464`, `ny_aot_cycles=2343815`, `ny_aot_ipc=3.96`
        - regenerated asm carries text in `%xmm0`; the stack `text.ptr` loop load/store is gone
      - guard held: array slot stores, suffix stores, route legality, public ABI, and runtime ownership stay unchanged
      - next blocker token: `137x-H owner-first optimization return`
      - next step: rerun owner-first perf evidence before the next exact-bridge shrink
    - `137x-H19` whole indexOf slot-consumer liveness seam is closed
      - previous blocker token: `137x-H19 whole indexOf slot-consumer liveness seam`
      - front: `kilo_kernel_small_hk` whole direct pure-first
      - failure mode: work explosion
      - current owner:
        - `PERF_AOT_DIRECT_ONLY=0 NYASH_LLVM_SKIP_BUILD=1 tools/perf/run_kilo_hk_bench.sh strict 1 3`
        - `kilo_kernel_small_hk = C 82 ms / Ny AOT 6653 ms`
        - perf top: `__memmove_avx512_unaligned_erms` 50.13%, `_int_malloc` 17.28%, `ArrayBox::boxed_from_text` 14.09%
      - hot transition: `TextLane slot -> boxed StringBox object` through an unused `nyash.array.get_hi` before `array.string_indexof_hisi`
      - current MIR fact: `array_text_observer_routes[0].keep_get_live=true` because `current` is also used by `current + "ln"`
      - next seam: teach MIR liveness that same-slot const suffix store is a slot-capable consumer, so the get need not materialize a public object
      - reject seam: do not delete array stores or infer liveness in `.inc`; MIR must own the consumer coverage
      - result:
        - MIR route now exports `array_text_observer_routes[0].keep_get_live=false`
        - lowered IR emits `nyash.array.string_indexof_hisi` directly and no longer emits the row-scan `nyash.array.get_hi` / `nyash.array.slot_load_hi` materialization before it
        - `kilo_kernel_small_hk = C 82 ms / Ny AOT 28 ms`
        - parity stayed `ok`
      - guard held: `.inc` remains a metadata consumer; no array stores were deleted and no runtime legality/provenance inference was added
      - current blocker token: `137x-H next owner proof after H19`
      - next step: rerun owner-first split/front evidence and open the next H-slice only from the measured owner
    - `137x-H20` meso substring concat len fusion seam is closed
      - previous blocker token: `137x-H20 meso substring concat len fusion seam`
      - front: `kilo_meso_substring_concat_len`
      - failure mode: work explosion in runtime helper calls
      - current owner:
        - `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 tools/perf/run_kilo_kernel_split_ladder.sh 1 3`
        - `kilo_meso_substring_concat_len = C 3 ms / Ny AOT 8 ms`
        - `ny_aot_instr=66356109`, `ny_aot_cycles=21046448`
        - `bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_len 'nyash.string.substring_len_hii' 3`
        - top owner: `nyash.string.substring_len_hii` 98.40%
      - hot transition: virtual text view length crosses the runtime handle registry boundary; annotate samples sit on `lock cmpxchg` / `lock xadd`
      - next seam: MIR string-corridor fusion folds `len(left + const + right)` for complementary substring slices back to `source_len + const_len`
      - reject seam: do not add runtime caches, do not revive `.inc` exact seed matching, and do not move legality into helper names
      - result:
        - targeted string-corridor benchmark pack passed
        - lowered IR has no `substring_len_hii`, no `substring_hii`, and `mir_calls=0` for `kilo_meso_substring_concat_len`
        - `kilo_meso_substring_concat_len = C 3 ms / Ny AOT 3 ms`
        - split ladder confirmation: `ny_aot_instr=1190204`, `ny_aot_cycles=909543`
      - guard held: no runtime cache, no `.inc` exact seed revival, and no helper-name legality shift
    - `137x-H21` meso array text loopcarry len/store seam is closed
      - previous blocker token: `137x-H21 meso array text loopcarry len/store seam`
      - front: `kilo_meso_substring_concat_array_set_loopcarry`
      - failure mode: work explosion in runtime array text helper pair
      - current owner:
        - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 8 ms`
        - `ny_aot_instr=72914136`, `ny_aot_cycles=22417148`
        - `nyash.array.string_len_hi` 54.74%
        - `array_string_insert_const_mid_subrange_by_index_store_same_slot_str` 43.77%
      - hot transition: array slot length is read through a runtime helper immediately before same-slot insert-mid subrange store
      - next seam: avoid the separate length helper when same-slot store has a known resulting length or can carry previous slot length as scalar state
      - reject seam: do not delete array stores, do not add semantic cache to `ArrayBox`, and do not infer same-slot legality in `.inc`
      - result:
        - MIR now exports one `array_text_loopcarry_len_store_routes` entry for the loopcarry block
        - route trace hits `array_string_loopcarry_len_store_window` with reason `mir_route_plan`
        - lowered loop body now calls only `nyash.array.string_insert_mid_subrange_len_store_hisi`; standalone `nyash.array.string_len_hi` and `store_hisiiii` are gone from the hot loop
        - targeted perf: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`, `ny_aot_instr=40155587`, `ny_aot_cycles=12429857`
        - split ladder confirmation: `kilo_meso_substring_concat_array_set_loopcarry = C 4 ms / Ny AOT 6 ms`, `ny_aot_instr=40154852`, `ny_aot_cycles=12350248`
        - whole confirmation: `kilo_kernel_small_hk = C 81 ms / Ny AOT 26 ms`, parity `ok`
      - guard held: route legality moved to MIR metadata; `.inc` remains a metadata consumer and array stores remain live
    - `137x-H22` array text len-store helper residency seam is closed
      - previous blocker token: `137x-H22 array text len-store helper residency seam`
      - front: `kilo_meso_substring_concat_array_set_loopcarry`
      - failure mode: remaining runtime helper residence/mutation cost
      - current owner:
        - after H21: `kilo_meso_substring_concat_array_set_loopcarry = C 4 ms / Ny AOT 6 ms`
        - `ny_aot_instr=40154852`, `ny_aot_cycles=12350248`
        - `array_string_insert_const_mid_subrange_len_by_index_store_same_slot_str` closure is 85-96% in `bench_micro_aot_asm`
      - hot transition: one helper still enters the generic array-handle / text-slot mutation path for every iteration
      - next seam: reduce helper residency overhead while keeping semantic legality in MIR and mechanics in runtime
      - reject seam: do not add semantic search/result caches, do not delete array stores, and do not move route legality into runtime
      - rejected probes:
        - small-overlap copy in `try_update_insert_const_mid_subrange_same_len_in_place`
          - result: `C 3 ms / Ny AOT 6 ms`, `ny_aot_instr=41954192`, `ny_aot_cycles=12355443`
          - code reverted; instruction count regressed
        - fast-path return of pre-update `source_len`
          - result: `C 3 ms / Ny AOT 6 ms`, `ny_aot_instr=40154996`, `ny_aot_cycles=12377624`
          - code reverted; no owner move
        - single-entry `slot_update_text_raw` helper path
          - result: `C 3 ms / Ny AOT 6 ms`, `ny_aot_instr=50413920`, `ny_aot_cycles=13248445`
          - code reverted; resident-first split is necessary
      - current verdict:
        - remaining owner is not local string-copy surgery
        - owner is runtime-private array text residence mutation / uncontended write-lock substrate
        - next keeper needs a structural residence/session design or this seam should be deferred to a later allocator/residence pilot
      - closeout:
        - local helper surgery is rejected
        - do not reopen H22 unless fresh perf evidence points at a new intra-helper block
    - `137x-H23` / `137x-H24` are closed
      - H23a proved fallback/promotion is not owner: `update_text_resident_hit=179999`
      - H23b helper-local resident/fallback compaction regressed and was reverted
      - H24 proved the active owner is write-lock acquire/release guard mechanics
      - full evidence stays in `docs/development/current/main/phases/phase-137x/README.md`
    - `137x-H25d` region executor inner mutation owner is closed
      - closed blocker token: `137x-H25d region executor inner mutation owner`
      - active entry: `docs/development/current/main/phases/phase-137x/137x-current.md`
      - ownership map: `docs/development/current/main/phases/phase-137x/137x-array-text-contract-map.md`
      - H25a landed metadata-only `array_text_residence_sessions`; `.inc` and runtime behavior are unchanged
      - H25b landed MIR-owned begin/update/end placement metadata:
        - `begin_placement=before_preheader_jump`
        - `update_placement=route_instruction`
        - `end_placement=exit_block_entry`
        - `skip_instruction_indices`
      - H25c.1 landed `.inc` residence-session metadata reader consumption
        without behavior change; active `.inc` array/text readers now use
        `*_route_metadata` naming
      - H25c.2 is split:
        - H25c.2a runtime-private session substrate landed
          (`ArrayTextSlotSession` + kernel-private `ArrayTextWriteTxn`)
        - H25c.2b single-call executor design gate closed as clean non-keeper
        - H25c.2c-1 MIR nested `executor_contract` metadata landed in code/tests
          (`single_region_executor`, `loop_backedge_single_body`,
          `publication_boundary=none`, `ArrayLane(Text)/Cell`, `store.cell`,
          `LengthOnly`, `text_resident_or_stringlike_slot`)
        - H25c.2c-2 `.inc` reader validation landed; active lowering accepts
          the residence session only when the nested contract matches and still
          maps to the existing per-iteration helper
        - H25c.2c-3 MIR `region_mapping` landed under `executor_contract`:
          loop index PHI/init/next/bound, accumulator PHI/init/next/exit use,
          and row modulus are now MIR-owned metadata and validated by `.inc`
        - H25c.2c-4 landed backend region replacement:
          `.inc` emits one begin-site
          `nyash.array.string_insert_mid_subrange_len_store_region_hiisi`
          call from MIR-owned metadata, skips the covered header/body without
          redefining SSA PHIs, and validates loop/accumulator initial constants
          as MIR facts
        - H25c.3 keeper probe passed as a partial keeper:
          `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 5 ms`,
          `ny_aot_instr=28630426`, `ny_aot_cycles=7033574`
        - target transition: emitted hot loop no longer calls the per-iteration
          fused helper; owner moved into `slot_text_region_update_sum_raw`
          (`79.54%`) with `__memmove_avx512_unaligned_erms` (`9.74%`)
      - H25d perf-first inner mutation work:
        - H25d.1 landed: keep the MIR contract unchanged and remove
          per-iteration `ArrayTextSlotSession` dispatch from the text-resident
          region executor path. Compatible fallback for boxed/stringlike arrays
          remains unchanged.
        - H25d.1 probe: `ny_aot_instr=24851120`, `ny_aot_cycles=6700078`,
          `Ny AOT 5 ms`; next owner moved to the inlined
          `array_string_insert_const_mid_subrange_len_region_store_len`
          mutation closure.
        - H25d.2 landed: split the fixed len-store update into a hot in-place
          path and cold semantic fallback. Do not add new MIR facts; keep UTF-8
          boundary checks in the hot path.
        - H25d.2 final probe: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 3 ms`,
          `ny_aot_instr=16570267`, `ny_aot_cycles=3471656`; next owner is the
          hot mutation closure plus
          `__memmove_avx512_unaligned_erms`.
        - H25d.3 rejected: manual byte moves regressed to
          `ny_aot_instr=22511003`, `ny_aot_cycles=4765539`, `Ny AOT 4 ms`;
          keep the existing `ptr::copy` path.
        - H25d.4 rejected: hoisting `observe::enabled()` out of the
          per-iteration region closure regressed instruction/cycle count to
          `ny_aot_instr=22510404`, `ny_aot_cycles=4773551`; reverted.
        - H25d.5 verdict: close H25d. Residual `memmove` / mutation surgery is
          not a keeper without a new MIR proof; H25d.3/H25d.4 both regressed,
          so accepted code remains H25d.1 + H25d.2.
      - `137x-H25e` post-parity owner refresh is closed
        - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`,
          `ny_aot_instr=9265624`, `ny_aot_cycles=2385663`
        - middle `kilo_meso_substring_concat_array_set_loopcarry`:
          `C 3 ms / Ny AOT 4 ms`,
          `ny_aot_instr=16570861`, `ny_aot_cycles=3387096`
        - whole `kilo_kernel_small`: `C 81 ms / Ny AOT 20 ms`,
          `ny_aot_instr=232160997`, `ny_aot_cycles=83942461`
        - verdict: next code owner is whole-front inner scan
          observer/conditional-store, not H25d residual `memmove`
      - `137x-H26` array text observer-store region contract is landed
        - current blocker token:
          `137x-H27 array text len-half insert-mid edit contract`
        - closed blocker token:
          `137x-H26 array text observer-store region contract`
        - target shape: `array.get(j).indexOf(const) >= 0` followed by
          same-array, same-index const-suffix store in the taken branch
        - MIR evidence already present: `array_text_observer_routes` records
          `array_get_receiver_indexof`, `consumer_shape=found_predicate`,
          `publication_boundary=none`, const needle `"line"`
        - implementation order:
          - H26.1 landed: add a nested observer-store region contract under
            existing observer metadata; do not add a benchmark-named sibling
            plan
            - whole-front MIR JSON emits one `single_region_executor` contract
              with `effects=[observe.indexof, store.cell]`, const needle
              `"line"`, and suffix `"ln"`
            - code split keeps `src/mir/array_text_observer_plan.rs` under
              1000 lines by isolating nested proof logic in
              `src/mir/array_text_observer_region_contract.rs`
          - H26.2 make `.inc` validate metadata and emit one runtime call, with
            no raw CFG rediscovery
            - landed: `.inc` reads the nested observer-store contract before
              block emission, so covered blocks are skipped even when MIR block
              order places the loop header before the begin block
            - add `begin_block` / `begin_to_header_block` to MIR-owned
              `executor_contract.region_mapping` so backend placement does not
              infer loop entry from raw CFG
            - accept only `execution_mode=single_region_executor`,
              `proof_region=loop_backedge_single_body`,
              `publication_boundary=none`,
              `effects=[observe.indexof, store.cell]`, and
              `consumer_capabilities=[compare_only, sink_store]`
          - H26.3 add runtime one-call executor that holds guard/residence
            mechanics inside the call and performs search + suffix mutation
            - landed: `nyash.array.string_indexof_suffix_store_region_hisisi`
              keeps guard/residence inside one ABI call and executes
              compare-only indexOf + same-slot suffix store
            - runtime helper is a generic observer-store region executor, not a
              benchmark-named whole-loop helper
            - runtime returns execution evidence only; it does not decide
              legality, provenance, or publication policy
          - H26.4 keeper probe on `kilo_kernel_small` plus exact/middle
            no-regression
            - landed result:
              - whole `kilo_kernel_small`: `C 82 ms / Ny AOT 10 ms`,
                `ny_aot_instr=149657283`, `ny_aot_cycles=31829608`
              - exact `kilo_micro_array_string_store`:
                `C 10 ms / Ny AOT 3 ms`,
                `ny_aot_instr=9266329`, `ny_aot_cycles=2400782`
              - middle `kilo_meso_substring_concat_array_set_loopcarry`:
                `C 3 ms / Ny AOT 4 ms`,
                `ny_aot_instr=16570773`, `ny_aot_cycles=3435120`
              - asm owner refresh: `<&str as Pattern>::is_contained_in`
                `35.05%`, `__memmove_avx512_unaligned_erms` `23.82%`,
                `nyash.array.string_len_hi` `20.97%`
            - next step: run an owner refresh before opening another code card;
              do not continue H26 by adding source-prefix/source-length/ASCII
              assumptions without MIR proof
        - reject seam: no helper-name shortcut, no runtime-owned legality, no
          `.inc` planner drift, no indexOf result cache, no source-prefix /
          source-length / ASCII assumption without MIR proof
      - `137x-H26e` post-keeper owner refresh is closed
        - commands:
          - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_kernel_small 1 3`
          - `bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`
          - `bash tools/perf/trace_optimization_bundle.sh --input kilo_kernel_small --route direct --callee-substr string_len --out-dir target/perf_state/h26e_owner_refresh`
        - result:
          - whole `kilo_kernel_small`: `C 82 ms / Ny AOT 10 ms`,
            `ny_aot_instr=149657100`, `ny_aot_cycles=31814977`
          - asm top includes `nyash.array.string_len_hi` at `20.76%`
          - emitted outer edit path still calls
            `nyash.array.string_len_hi`, computes `split = len / 2`, then
            calls `nyash.array.string_insert_mid_store_hisii`
        - verdict:
          - H26 stays closed
          - H27 opens as MIR-owned
            `array.get -> length -> source_len_div_const(2) -> same-slot
            insert-mid const` edit contract
          - `.inc` must consume H27 metadata and must not rediscover H27
            legality from raw JSON
          - runtime may compute current cell length and execute the mutation
            only; no legality/provenance/publication ownership
      - `137x-H27` array text len-half insert-mid edit contract is closed
        - current blocker token:
          `137x-H28 array text observer-store search/copy owner split`
        - closed blocker token:
          `137x-H27 array text len-half insert-mid edit contract`
        - implementation:
          - MIR owns the new `array_text_edit_routes` contract:
            `edit_kind=insert_mid_const`,
            `split_policy=source_len_div_const(2)`,
            `proof=array_get_lenhalf_insert_mid_same_slot`,
            `publication_boundary=none`
          - `.inc` consumes the metadata at the `array.get(row)` site and
            emits one `nyash.array.string_insert_mid_lenhalf_store_hisi`
            call; it skips only covered route instructions
          - runtime executes same-slot mutation and computes
            `split = current_text.len() / 2` as the MIR-selected policy
            only
        - route proof:
          - MIR JSON emits one `array_text_edit_routes` entry for
            `bench_kilo_kernel_small.hako`
          - route trace hits
            `stage=array_text_edit_lenhalf result=hit reason=mir_route_metadata`
          - lowered outer edit block no longer calls
            `nyash.array.string_len_hi`
        - result:
          - whole `kilo_kernel_small`: `C 83 ms / Ny AOT 10 ms`,
            `ny_aot_instr=144977171`, `ny_aot_cycles=30931233`
          - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`
          - middle `kilo_meso_substring_concat_array_set_loopcarry`:
            `C 4 ms / Ny AOT 4 ms`
          - quick gate: `tools/checks/dev_gate.sh quick` PASS
        - verdict:
          - small keeper / contract cleanup: instructions and cycles improved
            slightly, wall time stayed in the `10 ms` band
          - next owner is H28 observer-store search/copy mechanics, not more
            len-half edit surgery
      - `137x-H28` array text observer-store search/copy owner split is active
        - owner evidence after H27:
          - `<&str as core::str::pattern::Pattern>::is_contained_in`: `34.68%`
          - `__memmove_avx512_unaligned_erms`: `24.83%`
          - `with_array_text_write_txn` closure: `15.16%`
          - observer-store region closure: `11.02%`
          - `nyash.array.string_insert_mid_lenhalf_store_hisi`: about `1%`
        - first step:
          - inspect the H26 observer-store runtime helper and decide whether
            the next keeper is fixed-literal search mechanics,
            suffix mutation/copy mechanics, or a closeout that needs more MIR
            proof
        - guard:
          - no source-prefix assumption such as every row contains `"line"`
          - no search-result cache
          - no benchmark-named whole-loop helper
          - no runtime-owned legality/provenance/publication
          - no C-side raw shape fallback
        - H28.1 decision:
          - current MIR metadata is sufficient for the first slice; do not add
            a new plan family or C-side planner
          - replace only the runtime executor's literal search mechanics inside
            the existing MIR-proven observer-store helper
          - keep suffix mutation/copy as the next measured owner if search is
            no longer dominant
        - H28.1 result:
          - whole `kilo_kernel_small`: `C 84 ms / Ny AOT 9 ms`,
            `ny_aot_instr=60662079`, `ny_aot_cycles=20100504`
          - asm top moved from `Pattern::is_contained_in` to
            `__memmove_avx512_unaligned_erms` / `__memcmp_evex_movbe` and the
            write-frame closure
          - exact guard `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`
          - middle guard `kilo_meso_substring_concat_array_set_loopcarry`:
            `C 3 ms / Ny AOT 3 ms`
          - annotate correction: `__memcmp_evex_movbe` is the H28.1
            `starts_with` prefix check lowering to libc `bcmp`, not suffix copy
        - H28.2 result:
          - runtime-private short-literal prefix compare now uses a local byte
            loop instead of `starts_with` / libc `bcmp`
          - whole `kilo_kernel_small`: `C 83 ms / Ny AOT 7 ms`,
            `ny_aot_instr=64501392`, `ny_aot_cycles=18956185`
          - exact guard `kilo_micro_array_string_store`: `C 11 ms / Ny AOT 4 ms`
          - middle guard `kilo_meso_substring_concat_array_set_loopcarry`:
            `C 3 ms / Ny AOT 4 ms`
          - asm top after H28.2: `__memmove_avx512_unaligned_erms`,
            `with_array_text_write_txn` closure, and observer-store region
            closure; `__memcmp_evex_movbe` is no longer a top owner
        - H28.3 active:
          - annotate shows the next `__memmove` owner is the short
            `value.push_str(suffix)` append in the observer-store runtime
            executor
          - implement only a runtime-private short-suffix append leaf if it
            removes that copy owner
          - no MIR metadata change, no `.inc` shape logic, no source-prefix
            assumption, no search-result cache
        - H28.3 result:
          - short suffixes now append through a runtime-private byte leaf
            instead of `String::push_str`
          - whole `kilo_kernel_small`: `C 82 ms / Ny AOT 7 ms`,
            `ny_aot_instr=60615291`, `ny_aot_cycles=17586950`
          - exact guard `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`
          - middle guard `kilo_meso_substring_concat_array_set_loopcarry`:
            `C 3 ms / Ny AOT 4 ms`
          - residual owner: `__memmove` remains top, now attributed to capacity
            growth / old-content copy or adjacent write-frame mechanics rather
            than the short suffix byte copy
          - next seam: H28.4 capacity growth / write-frame owner decision
        - H28.4 result:
          - card type: new owner-first slice under H28, not a continuation of
            H25 guard-mechanics surgery
          - target owner: resident `String` capacity miss leading to realloc /
            old-content copy in the observer-store suffix append path
          - trial: Rust-only short append headroom policy in
            `append_short_text_suffix`; no MIR, `.inc`, or public ABI change
          - result: whole stayed noisy but worsened on instruction/cycle/wall
            (`ny_aot_instr=61363741`, `ny_aot_cycles=17616053`, then rerun
            `ny_aot_instr=61364376`, `ny_aot_cycles=17951505`, `Ny AOT 8 ms`)
          - asm: `__memmove` share dropped to `34.76%`, but
            `with_array_text_write_txn` / observer-store closure share rose;
            target transition did not become a keeper
          - verdict: reject and revert code; capacity headroom alone is not
            the next keeper
        - H28.5 result:
          - after reverting H28.4 and rebuilding plain release, whole
            `kilo_kernel_small`: `C 84 ms / Ny AOT 7 ms`,
            `ny_aot_instr=60616017`, `ny_aot_cycles=17782048`
          - asm top: `__memmove_avx512_unaligned_erms` `37.20%`,
            observer-store closure `28.98%`, `with_array_text_write_txn`
            `26.22%`, `nyash.array.string_insert_mid_lenhalf_store_hisi`
            `3.26%`
          - callgraph: `__memmove` samples primarily come from
            `array_string_insert_const_mid_lenhalf_by_index_store_same_slot_str`
            closure (`27.91%`); append/realloc growth is only about `0.93%`
          - verdict: H28 observer-store search/copy split is closed; do not
            chase append capacity further
          - next seam: H29 len-half edit copy owner decision under the H27
            MIR-owned edit contract
        - H29 result:
          - trial: runtime-private bypass of `String::insert_str` for the
            len-half helper, using explicit reserve + suffix shift + middle
            copy; no MIR, `.inc`, or public ABI change
          - behavior checks:
            `cargo test -q -p nyash_kernel insert_mid_lenhalf_store_by_index_returns_result_len`,
            `cargo test -q -p nyash_kernel insert_mid_store_by_index`,
            `cargo test -q detects_lenhalf_insert_mid_same_slot_edit_route --lib`,
            and `cargo fmt --check`
          - perf after trial: whole `kilo_kernel_small`
            `C 83 ms / Ny AOT 7 ms`, `ny_aot_instr=60494965`,
            `ny_aot_cycles=17790198`
          - asm after trial: `__memmove_avx512_unaligned_erms` rose to
            `40.84%`; `with_array_text_write_txn` `30.00%`; observer-store
            closure `20.99%`; `nyash.array.string_insert_mid_lenhalf_store_hisi`
            `3.21%`
          - verdict: reject and revert code; contiguous `String` mid-insert
            still requires the suffix copy, so local byte-copy surgery is not
            the next keeper
          - next seam: H30 array text edit residence representation decision
            under the existing MIR-owned H27 edit contract
        - H30.1 inventory:
          - `ArrayStorage::Text(Vec<String>)` is still matched directly by
            storage promotion, raw text read/write/update helpers, region
            executors, visible `get`, clone/format/equality/debug, and
            generic array ops
          - direct gap/piece replacement would leak representation details and
            make rollback large
          - next clean slice is BoxShape-only: introduce a flat
            `ArrayTextCell` boundary first, with no MIR, `.inc`, public ABI, or
            behavior change; non-flat edit residence can only open behind that
            boundary
        - H30.1 code result:
          - landed flat `ArrayTextCell` boundary for `ArrayStorage::Text`
          - `ArrayStorage::Text` now stores `Vec<ArrayTextCell>` while public
            Array/String behavior remains unchanged
          - verification: `cargo fmt --check`, `git diff --check`,
            `cargo check -q`, `cargo test -q array::tests --lib`,
            `cargo test -q text_contains_literal --lib`,
            `cargo test -q slot_store_text_births_text_lane --lib`,
            `cargo test -q -p nyash_kernel insert_mid_store_by_index --lib`,
            and `tools/checks/dev_gate.sh quick`
        - H30.2 code result:
          - first close the H27 edit operation boundary: the len-half edit
            helper must call a runtime-private `ArrayTextCell` edit operation,
            not expose `&mut String` as the long-term representation truth
          - landed: H27 len-half helper now calls
            `ArrayBox::slot_insert_const_mid_lenhalf_raw`, which dispatches to
            `ArrayTextCell::insert_const_mid_lenhalf` for text-resident slots
          - verification: `cargo fmt --check`, `git diff --check`,
            `cargo check -q`, `cargo test -q slot_insert_const_mid_lenhalf_raw --lib`,
            `cargo test -q -p nyash_kernel insert_mid_store_by_index --lib`,
            `cargo test -q array::tests --lib`, and
            `tools/checks/dev_gate.sh quick`
        - H30.3 closed without keeper:
          - tried a narrow piece-cell/deferred-edit prototype behind
            `ArrayTextCell`; code was reverted
          - release hygiene note: the first perf read was taken before
            `tools/perf/build_perf_release.sh`, so do not use those stale
            numbers as keeper evidence
          - verdict: do not continue local gap/piece representation surgery
            until a fresh valid-release owner proof selects it again
        - H31 result:
          - source inspection selected H32 transaction façade thinning
          - fixed measurement hygiene for this lane: rebuild release with
            `tools/perf/build_perf_release.sh` before judging runtime changes
        - H32 code result:
          - flattened kernel-private `with_array_text_slot_update*` so it
            calls `with_array_box -> slot_update_text_*` directly
          - removed the extra `with_array_text_write_txn` closure surface
          - valid-release whole perf: `kilo_kernel_small = C 84 ms / Ny AOT 7 ms`,
            `ny_aot_instr=60315390`, `ny_aot_cycles=17714067`
          - asm: `with_array_text_write_txn` is gone from the top list;
            `__memmove` remains `40.82%`, len-half closure `25.39%`,
            observer-store closure `24.05%`
          - verdict: keep as structural cleanup / owner-shift, not a wall-time
            keeper
        - H33 result:
          - valid-release direct runner shows no hot `string_len_hi`
          - direct top: `__memmove` `35.52%`, len-half closure `31.17%`,
            observer-store closure `27.45%`
          - verdict: do not reopen len-half byte-copy surgery from `memmove`
            share alone; H29 already rejected that seam
        - H34 active:
          - narrow runtime-private observer-store short-byte leaf thinning
          - touch only `src/boxes/array/ops/text.rs`
          - optimize short literal prefix check and short suffix byte write as
            mechanics only; no MIR, `.inc`, public ABI, or semantic cache
        - H34 result:
          - kept as runtime-private mechanics keeper
          - `kilo_kernel_small = C 83 ms / Ny AOT 7 ms`,
            `ny_aot_instr=50229601`, `ny_aot_cycles=16375916`
          - observer-store closure shrank to `14.03%`; post-H32 was `27.45%`
          - no-regression guards: meso loopcarry `Ny AOT 4 ms`, exact array
            string store `Ny AOT 4 ms`
        - H35 active:
          - decide the next valid card for residual `memmove` / len-half
            closure
          - do not repeat H29 byte-copy surgery without a new representation
            proof
        - H35 result:
          - post-H34 callgraph top: `__memmove` `48.59%`, len-half closure
            `26.13%`, observer-store closure `16.08%`
          - verdict: next step is a representation design gate, not another
            flat `String::insert_str` bypass
        - H36 active:
          - decide whether `ArrayTextCell` opens non-flat / gap / piece
            residence for repeated len-half inserts
          - docs/design first; no MIR or `.inc` route change before the
            runtime residence contract is clear
        - H36 result:
          - SSOT:
            `docs/development/current/main/phases/phase-137x/137x-97-h36-array-text-cell-residence-design-gate.md`
          - do not add a non-flat variant yet
          - first land H36.1 flat-only `ArrayTextCell` operation API split
        - H36.1 result:
          - landed flat-only `ArrayTextCell` operation API split
          - hot-path contains/append now go through `ArrayTextCell`
            methods / string leaf wrappers
          - no MIR, `.inc`, public ABI, or perf keeper claim
          - verification: `cargo fmt --check`, `git diff --check`,
            `cargo test -q array::tests --lib`,
            `cargo test -q -p nyash_kernel insert_mid_store_by_index --lib`,
            and `tools/checks/current_state_pointer_guard.sh`
        - H36.2 result:
          - rebuilt release artifacts before measuring
          - whole `kilo_kernel_small = C 82 ms / Ny AOT 7 ms`,
            `ny_aot_instr=50229407`, `ny_aot_cycles=16401030`
          - asm top: `__memmove` `38.15%`, len-half edit closure `33.22%`,
            observer-store closure `20.21%`
          - verdict: non-flat text residence remains justified, but first
            close visible materialization APIs so representation does not leak
        - H36.3 result:
          - landed BoxShape-only visible materialization split
          - Array visible get/boxing/format/equality/membership/sort now use
            `ArrayTextCell` helpers instead of raw `as_str()` / derived order
          - no `Piece` / `Gap`, no MIR, `.inc`, public ABI, or perf keeper claim
          - verification: `cargo fmt --check`, `git diff --check`,
            `cargo test -q array::tests --lib`,
            `cargo test -q -p nyash_kernel insert_mid_store_by_index --lib`,
            and `tools/checks/current_state_pointer_guard.sh`
        - H36.4 result:
          - rejected narrow `ArrayTextCell::Pieces` pilot
          - behavior gates were green, but whole perf regressed to
            `kilo_kernel_small = C 85 ms / Ny AOT 114 ms`,
            `ny_aot_instr=2084599541`, `ny_aot_cycles=521801542`
          - verdict: naive piece vectors cause work explosion; code reverted
        - H37 result:
          - rebuilt release artifacts from reverted H36.3 code
          - whole `kilo_kernel_small = C 82 ms / Ny AOT 7 ms`,
            `ny_aot_instr=50229360`, `ny_aot_cycles=16404095`
          - asm top: `__memmove` `49.02%`, len-half edit closure `22.74%`,
            observer-store closure `18.88%`
          - verdict: owner returned to flat len-half movement; naive pieces are
            rejected, allocator is not dominant
        - H38 result:
          - bounded mid-gap design fixed for `ArrayTextCell`
          - logical text is `left + right[right_start..]`
          - len-half insert moves the right boundary by offset, not by
            draining the active right tail
          - visible materialization, contains, append, rollback, and
            compaction rules are documented
        - H38.1 result:
          - runtime-private bounded mid-gap pilot landed in `ArrayTextCell`
          - whole `kilo_kernel_small = C 83 ms / Ny AOT 6 ms`,
            `ny_aot_instr=60923714`, `ny_aot_cycles=12531473`
          - asm: `__memmove` fell to `0.23%`; top owners are now len-half
            closure `49.27%` and observer-store closure `41.58%`
          - exact `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
          - middle `kilo_meso_substring_concat_array_set_loopcarry =
            C 3 ms / Ny AOT 4 ms`
          - verdict: owner-moving keeper with instruction-count watch
        - H39 result:
          - focused annotation pinned two different owners
          - len-half edit closure is mostly write-lock acquire
            (`lock cmpxchg` local `62.33%`)
          - observer-store closure is not lock-dominant; it is now
            cell-loop / short-literal / MidGap segment checks
          - verdict: do not reopen representation work immediately
        - H39.1 result:
          - landed runtime-only MidGap generic prefix fast path
          - whole `kilo_kernel_small = C 83 ms / Ny AOT 6 ms`,
            `ny_aot_instr=60443810`, `ny_aot_cycles=11322220`
          - exact `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`
          - middle `kilo_meso_substring_concat_array_set_loopcarry =
            C 3 ms / Ny AOT 3 ms`
          - verdict: small keeper; observer-store mechanics improved but
            outer edit lock-boundary remains
        - H39.2 result:
          - closed as design / stop-line
          - edit-only session is not enough because the outer loop interleaves
            a periodic observer-store region
        - H39.3 result:
          - landed metadata-only combined edit-observer region proof
          - MIR JSON now carries one `array_text_combined_regions` entry for
            `kilo_kernel_small`
        - H39.4 result:
          - landed one-call runtime-private combined edit-observer executor
          - whole `kilo_kernel_small = C 82 ms / Ny AOT 5 ms`,
            `ny_aot_instr=49691801`, `ny_aot_cycles=9882715`
          - exact `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`
          - middle `kilo_meso_substring_concat_array_set_loopcarry =
            C 4 ms / Ny AOT 3 ms`
          - emitted `ny_main` no longer calls per-iteration
            `nyash.array.string_insert_mid_lenhalf_store_hisi`
        - H39.5 result:
          - annotate pins the current owner inside combined executor mechanics
          - first follow-up is runtime-only pow2 index/period arithmetic
        - H39.5.1 result:
          - runtime-only pow2 index/period bitmask cleanup landed
          - whole `kilo_kernel_small = C 83 ms / Ny AOT 6 ms`,
            `ny_aot_instr=49271666`, `ny_aot_cycles=9282981`
          - exact `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`,
            `ny_aot_instr=9265976`, `ny_aot_cycles=2404527`
          - middle `kilo_meso_substring_concat_array_set_loopcarry =
            C 3 ms / Ny AOT 4 ms`, `ny_aot_instr=17651126`,
            `ny_aot_cycles=4237981`
          - verdict: cycles/memmove cleanup only; not a wall-time keeper
        - H39.5.2 result:
          - MidGap right slice/range access now uses debug-asserted unchecked
            helpers in the runtime text-cell leaf
          - whole `kilo_kernel_small = C 84 ms / Ny AOT 5 ms`,
            `ny_aot_instr=42303268`, `ny_aot_cycles=8732285`
          - exact `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`,
            `ny_aot_instr=9265804`, `ny_aot_cycles=2352051`
          - middle `kilo_meso_substring_concat_array_set_loopcarry =
            C 3 ms / Ny AOT 4 ms`, `ny_aot_instr=17651020`,
            `ny_aot_cycles=4233835`
          - result: keeper; `str::Range::get` falls out of the direct AOT top
        - H39.5.3 result:
          - runtime-only 4-byte literal observer leaf; no MIR, `.inc`, or
            public ABI changes
          - whole `kilo_kernel_small = C 85 ms / Ny AOT 5 ms`,
            `ny_aot_instr=35428450`, `ny_aot_cycles=6679916`
          - exact `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`,
            `ny_aot_instr=9266200`, `ny_aot_cycles=2437087`
          - middle `kilo_meso_substring_concat_array_set_loopcarry =
            C 3 ms / Ny AOT 4 ms`, `ny_aot_instr=17650994`,
            `ny_aot_cycles=4214918`
          - result: keeper; next owner refresh must split residual combined
            executor work from `memmove` / allocator mechanics
        - H39.5.4 result:
          - no-code owner refresh after the 4-byte literal observer cleanup
          - preserved-AOT top is combined executor closure `75.26%`,
            `__memmove_avx512_unaligned_erms` `10.03%`, `_int_malloc`
            `2.05%`
          - remaining sampled MidGap edit branch is a byte-boundary legality
            seam; do not skip it in Rust without MIR-owned proof
        - H40 closed:
          - MIR owns byte-boundary / ASCII-preserved proof for covered
            text-cell edit regions before any runtime fast leaf skips boundary
            checks
          - `.inc` consumes metadata only; runtime keeps the checked path when
            proof is absent
        - H40.1 result:
          - MIR emits optional `byte_boundary_proof=ascii_preserved_text_cell`
            for the covered ASCII seed/edit/observer region
          - JSON also carries `text_encoding=ascii_preserved` and
            `split_boundary_policy=byte_index_safe`
          - `.inc` mirrors the proof bit without raw shape rediscovery
          - AOT smoke remains `kilo_kernel_small = C 82 ms / Ny AOT 5 ms`,
            `ny_aot_instr=35428267`, `ny_aot_cycles=6731377`
          - runtime behavior is unchanged; next slice consumes the proof in a
            checked-fallback fast leaf
        - H40.2 result:
          - `.inc` selects the proof-specific helper only from MIR metadata
          - runtime adds a const-specialized byte-boundary-safe edit leaf and
            preserves the checked no-proof path
          - whole `kilo_kernel_small = C 82 ms / Ny AOT 6 ms`,
            `ny_aot_instr=34108663`, `ny_aot_cycles=6613012`
          - exact guard `kilo_micro_array_string_store = C 9 ms / Ny AOT 4 ms`
          - meso guard
            `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 4 ms`
          - 200-run top remains combined executor closure `68.98%`,
            `__memmove_avx512_unaligned_erms` `17.89%`
          - verdict: H40 closes as a narrow keeper; next owner is residual
            MidGap copy/materialization, not byte-boundary legality
        - H41 result:
          - refreshed direct AOT owner with persistent perf data at
            `target/perf_state/h41_kilo_kernel_small.perf.data`
          - top remains combined executor closure `69.87%`,
            `__memmove_avx512_unaligned_erms` `16.26%`,
            `_int_malloc` `2.04%`, `finish_grow` `0.21%`,
            `reserve::do_reserve_and_handle` `0.09%`
          - annotate pins residual local samples in observer scan and the
            existing 2-byte short-suffix write leaf, while the broad copy owner
            stays as external `memmove`
          - verdict: close H41 as owner refresh; next narrow code slice is
            runtime-private prepared suffix append, not new MIR legality
        - H42 rejected:
          - tried runtime-private prepared suffix append plan and reverted it
          - whole `kilo_kernel_small = C 82 ms / Ny AOT 5 ms`,
            `ny_aot_instr=35553658`, `ny_aot_cycles=6944027`
          - exact/meso guards held, but whole instr/cycles regressed from
            H40.2 (`34108663` / `6613012`) and top `memmove` share rose to
            `19.77%`
          - verdict: suffix dispatch/source-load is not the keeper seam
        - H43 closed:
          - H43.1 right-front suffix escape was rejected and reverted
          - whole instructions/cycles regressed from clean H43
            `34108337` / `6544565` to `34826664` / `7281528`
          - `memmove` share rose from `16.93%` to `17.72%`
          - no more local MidGap copy leaves without a fresh sampled block
        - H44 closed:
          - H44.1 runtime-private observer all-hit guard is keeper
          - whole improved from clean H43 `34108337` / `6544565` to
            `24129815` / `5615809`
          - exact/meso guards held
        - H45 closed:
          - refreshed whole perf: `kilo_kernel_small = C 83 ms / Ny AOT 5 ms`,
            `ny_aot_instr=24122891`, `ny_aot_cycles=5842445`
          - saved bundle + dwarf callgraph pin the residual owner to one
            `ArrayTextCell` edit/materialization family inside the combined
            executor (`0x415d90`, `0x415e8f`, `0x416152`)
        - H46 active:
          - blocker token:
            `137x-H46 text-cell residence/materialization design`
          - treat the owner as broad text-cell residence/materialization, not a
            fresh suffix/left-copy micro leaf
          - H46.1 bounded `MidGap + bridge` probe was rejected and reverted:
            `Ny AOT 22 ms`, `ny_aot_instr=142651499`,
            `ny_aot_cycles=90126830`, `__memmove 54.59%`,
            `_int_malloc 21.74%`
          - post-revert whole guard is back at `Ny AOT 5 ms`,
            `ny_aot_instr=24123290`, `ny_aot_cycles=6044833`
  - active phase:
    - `docs/development/current/main/phases/phase-137x/README.md`
  - active current entry:
    - `docs/development/current/main/phases/phase-137x/137x-current.md`
  - method anchor:
    - `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`
  - taskboard:
    - `docs/development/current/main/phases/phase-137x/137x-91-task-board.md`
  - legacy retirement ledger:
    - `docs/development/current/main/phases/phase-137x/README.md#legacy-retirement-ledger`
    - planned deletions stay there as SSOT; do not scatter deletion TODOs through compiler/runtime files
- background lanes:
  - `phase-29bq loop owner seam cleanup landing`
  - `phase-163x primitive-family / user-box fast-path landing`
  - successor planning lane:
  - `phase-289x runtime-wide value/object boundary rollout`
  - status:
    - phase-0 authority/vocabulary lock is docs-only and complete
    - phase-137x keeper `49c356339` is the current string proof
    - demand-backed cutover inventory `289x-96` is closed
    - `137x-B` design cleanout is closed; `137x-C` structure completion gate is closed; `137x-D` exact route-shape keeper is landed
    - `137x-F` is closed and `137x-G` is rejected for now; return lane is `137x-H`
    - array/map remain identity containers; only internal residence may become lane-hosted later
    - `publish` / `promote` stay boundary effects; `freeze.str` stays the only string birth sink
    - all `289x-96` clusters are done; their vocabulary now feeds the constrained `137x-F` implementation bridge
    - carrier responsibility lock is documented:
      - `BorrowedHandleBox` is boundary/cache, not semantic `Ref`
      - `KernelTextSlot` is transport adapter / sink seed, not long-term `TextCell`
      - `StringViewBox` is object-world view, not internal substring carrier
    - latest runtime-private carrier stack is landed:
      - `1e0766779` `nyash_kernel: name TextRef/OwnedText semantic carriers`
      - `ef4eba2bb` `nyash_kernel: migrate value_codec to TextRef/OwnedText read paths`
      - `b0a10794f` `nyash_kernel: adapt exports to TextRef semantic carrier`
      - `403fe5211` `docs: document TextRef/OwnedText carrier step and guarantees`
      - `4e36caf34` `nyash_kernel: keep piecewise reads on TextRef`
      - `5b0bdaa5f` `nyash_kernel: keep concat helpers on TextRef`
    - current structure-first reading:
      - touched export-side read consumers are narrow enough for the active perf return
      - `borrowed_handle.rs` modularization is landed; proof/lifetime keep and boundary box impl are already split behind a thin facade
      - `substring_hii` `ViewSpan` publication cleanup is closed; `StringSpan` now survives until the final handle boundary helper
      - explicit string-only `publish.text` contract gates are closed for `137x-A`; `publish.any` remains deferred/blocked
      - `cache.rs` / `string_materialize.rs` remain deferred modularization candidates, but not prerequisites for the active `137x-E` implementation gate
      - closed implementation gate token is `137x-F Value Lane bridge`; `137x-F1` demand-to-lane executor bridge, `137x-F2` producer outcome manifest split, `137x-E0` MIR/backend seam closeout, and `137x-E1` minimal TextLane are closed
      - `.inc` must consume MIR-owned metadata for legality/provenance and stay backend emit/normalization only
      - pre-E1 cleanup deleted the old `9-block` seed branch and the shared-receiver scanner fallback; active shared-receiver gates are now metadata-only
      - legacy watch item `src/host_providers/llvm_codegen/compat_text_primitive.rs` is retired in `137x-H2`; remaining text object boundary is `src/host_providers/llvm_codegen/mir_json_text_object.rs`
  - parent SSOT:
    - `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
  - phase:
    - `docs/development/current/main/phases/phase-289x/README.md`
  - design brief:
    - `docs/development/current/main/phases/phase-289x/289x-90-runtime-value-object-design-brief.md`
  - taskboard:
    - `docs/development/current/main/phases/phase-289x/289x-91-runtime-value-object-task-board.md`
  - inventory ledger:
    - `docs/development/current/main/phases/phase-289x/289x-92-value-boundary-inventory-ledger.md`
  - demand ledger:
    - `docs/development/current/main/phases/phase-289x/289x-93-demand-vocabulary-ledger.md`
  - container demand table:
    - `docs/development/current/main/phases/phase-289x/289x-94-container-demand-table.md`
  - selected pilot:
    - `docs/development/current/main/phases/phase-289x/289x-95-array-text-residence-pilot.md`
  - cutover inventory gate:
    - `docs/development/current/main/phases/phase-289x/289x-96-demand-backed-cutover-inventory.md`
  - current docs focus:
    - `289x-1g` demand vocabulary ledger is done
    - `289x-2d` Array/Map demand table is done
    - `289x-3a` selected pilot is Array text residence through `KernelTextSlot` store
    - first code cut landed:
      - `crates/nyash_kernel/src/plugin/value_demand.rs`
      - runtime-private demand vocabulary, behavior unchanged
    - Array text-residence leaves now name the demand constants in code
    - `289x-3c` landed:
      - `CodecProfile::demand()` maps every codec profile to runtime-private `DemandSet`
      - `any_arg_to_box_with_profile` and `decode_array_fast_value` now bind demand metadata before old behavior branches
      - behavior unchanged
    - `289x-3d` landed:
      - `BorrowedAliasEncodeCaller::demand()` maps caller names to runtime-private `DemandSet`
      - borrowed-alias encode plans now bind live/cached alias demand vs fallback publish demand before old behavior branches
      - behavior unchanged
    - `289x-3e` landed:
      - `PublishReason::demand()` maps publish reason names to runtime-private `PublishDemand`
      - publish helpers now bind boundary-effect demand before old observation/objectization branches
      - behavior unchanged
    - `289x-3f` landed:
      - array encoded get/load sites now bind `ARRAY_GENERIC_GET_ENCODED`
      - demand names immediate encode, borrowed alias encode, and stable object fallback for generic array reads
      - behavior unchanged
    - `289x-3g` landed:
      - array `store_any` now binds `ARRAY_GENERIC_STORE_ANY`
      - array `append_any` now binds `ARRAY_GENERIC_APPEND_ANY`
      - behavior unchanged
    - `289x-3h` landed:
      - `KernelTextSlotState::demand()` maps slot residence state to runtime-private `DemandSet`
      - `KernelTextSlotBoundary::demand()` maps slot publish/objectize boundary to publish demand
      - behavior unchanged; no ABI change
    - `289x-7a` landed:
      - C shim `ArrayStoreString` route now carries source-preserve plus publish-handle demand metadata
      - stable-object demand remains off; emitted lowering unchanged
      - direct array-store-string smoke still stops before lowering on the existing pure-shape recipe gate
    - `289x-7b` landed:
      - MIR `ThinEntryCandidate` / `ThinEntrySelection` now carry inspection-only demand facts
      - folded `PlacementEffectRoute` now carries demand beside decision/source/state
      - MIR JSON emits the new demand fields; behavior and lowering unchanged
    - `289x-6d` landed:
      - Map key decode now binds explicit i64/any/runtime-data demand metadata
      - Map value store now binds value-residence + alias-invalidation demand metadata
      - behavior unchanged; no typed map lane
    - `289x-6e` landed:
      - Map load now separates materializing-return demand from caller-scoped encode demand
      - behavior unchanged; no public ABI change
      - Rust runtime clusters in `289x-96` are now closed
    - `289x-7c` landed:
      - C shim `get/len/has/push` policy switches now compute explicit demand metadata beside existing routes
      - behavior and emitted lowering unchanged; this is metadata-only preparation for route cutover
    - `289x-7d` landed:
      - main C shim `bname/mname` route classifier now normalizes names into receiver/method surface enums before choosing route bits
      - behavior unchanged; RuntimeData array/map get/has/size/length/push, array-string indexOf, and StringBox length/indexOf route smokes passed
    - `289x-7e` landed:
      - C shim array slot load/store/string-len/string-indexOf concrete emission is now centralized in `hako_llvmc_ffi_array_slot_emit.inc`
      - behavior and helper symbols unchanged; exact kernel slot-store, live-after-get, array set/get, and array-string len/indexOf smokes passed
    - `289x-7f` landed:
      - C shim array-string window matchers now use `hako_llvmc_ffi_array_string_window_policy.inc` for array text-read/read eligibility
      - behavior unchanged; branch/select/cross-block/interleaved/live-after-get/len-live exact window smokes passed
    - `289x-7g` landed:
      - MIR string helper-name vocabulary is centralized in `src/mir/string_corridor_names.rs`
      - behavior unchanged; compat/recovery and recognizer tests plus release build passed
    - `289x-7h` landed:
      - C shim prepass/declaration need classifier now consumes normalized receiver/method surfaces
      - declaration/prepass needs remain exact; no helper declaration widening
      - RuntimeData array/map get/has/size/length/push, array-string indexOf, and array set/get canary smokes passed
    - demand-backed cutover inventory:
      - `289x-96` Rust/C-shim/MIR clusters are closed
      - phase-289x no longer blocks optimization return, and `137x-B` design cleanout is now closed
    - implementation order before the next kilo optimization:
      - `137x-E`: minimal `ArrayStorage::Text` / `TextLane`
      - `137x-F`: runtime-wide Value Lane implementation bridge is closed
      - `137x-G`: allocator / arena pilot is rejected until copy/allocation tax becomes dominant
      - still deferred: string view/value carrier split beyond this gate, Map typed lane, heterogeneous / union slots
    - return-to-optimization gate:
      - phase-289x gate was closed by `289x-7h`
      - `137x-B` container / primitive design cleanout is closed
      - `137x-C` completion gate is closed by `137x-91-task-board.md`
      - `137x-D` exact route-shape keeper is landed
      - optimization resumes as `137x-H` after `137x-F` closeout and `137x-G` reject
- current blocker:
  - `137x-H owner-first optimization return`
  - no broad `phase-289x` cutover blocker; `137x-F` is closed and `137x-G` is deferred
- current cut status:
  - latest implementation candidate:
    - phase-137x branch-target-aware array string get/store seam is now the active keeper candidate
    - exact accepted shape:
      - `array.get -> indexOf("line") -> compare -> branch`
      - branch target uses the fetched string only as `copy -> const suffix -> Add -> same array.set(idx, value)`
    - lowering keeps the observer on `nyash.array.string_indexof_hih` and rewrites the same-slot suffix store to `nyash.array.kernel_slot_concat_his -> nyash.array.kernel_slot_store_hi`
    - contract:
      - no `nyash.array.slot_load_hi` call on that exact same-slot suffix path
      - live-after-get shapes that feed substring/other reuse still keep `slot_load_hi`
    - current status:
      - structure/smoke gates are green
      - perf keeper proof is green:
        - `kilo_micro_array_string_store = C 9 ms / Ny AOT 3 ms`
        - `kilo_kernel_small = C 80 ms / Ny AOT 214 ms`
        - `kilo_kernel_small_hk = C 81 ms / Ny AOT 218 ms` (`repeat=3`, parity ok)
      - this is a landed phase-137x keeper cut; `TextLane` / Value Lane / allocator work now starts only through the separate `137x-E/F/G` gates
      - follow-up structure card in progress:
        - owner family:
          - `array_string_concat_const_suffix_by_index_store_same_slot_str`
          - `array_string_indexof_by_index_str`
          - `append_const_suffix_to_string_box_value`
        - purpose:
          - reduce same-slot exact-route copy/search tax without widening public ABI
        - status:
          - this is still structure-only, not keeper proof
          - the current owner proof remains reject-side after the exact route-shape keeper
        - boundary:
          - start the next perf/asm reread from these concrete helper interiors
          - do not revive this helper-local seam as the next task; `137x-H` now owns the next measured optimization return
      - current source-only get suppression + same-slot string store keeper:
        - compiler seam: `array.get -> length -> substring/substring -> insert-mid set` records the array text source and skips the object-handle get when later uses are proven source-only
        - fused insert-mid store seam:
          - same-slot insert-mid now lowers to runtime-private `nyash.array.string_insert_mid_store_hisii(array_h, idx, middle_ptr, middle_len, split)`
          - the runtime mutates an existing raw `StringBox` residence in place
          - borrowed-handle residences are materialized into an unpublished raw `StringBox` slot without mutating the source stable handle
        - fused suffix store seam:
          - branch same-slot const-suffix store now lowers to runtime-private `nyash.array.string_suffix_store_hisi(array_h, idx, suffix_ptr, suffix_len)`
          - the C lowering no longer allocates a `KernelTextSlot` for this branch path
          - the runtime applies the same residence rule as insert-mid: mutate raw `StringBox`, materialize borrowed alias to unpublished raw `StringBox`
        - fused subrange store seam:
          - same-slot insert-mid subrange now lowers to runtime-private `nyash.array.string_insert_mid_subrange_store_hisiiii(array_h, idx, middle_ptr, middle_len, split, start, end)`
          - the previous `hisiii` helper remains as the pointer/CStr validated compatibility row, but direct lowering uses explicit length
        - fixture/smoke:
          - `apps/tests/mir_shape_guard/array_string_len_insert_mid_source_only_min_v1.mir.json`
          - `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_insert_mid_source_only_min.sh`
          - `apps/tests/mir_shape_guard/array_string_len_piecewise_concat3_source_only_min_v1.mir.json`
          - `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_piecewise_concat3_source_only_min.sh`
        - regression guard:
          - `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_len_live_after_get_min.sh`
          - still requires `slot_load_hi` when later substring values remain live
        - perf/asm proof:
          - exact keeper: `kilo_micro_array_string_store = C 11 ms / Ny AOT 10 ms`, `ny_aot_instr=26922130`
          - exact route proof: `array_string_store_micro result=emit reason=exact_match`
          - meso: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 9 ms`, `ny_aot_instr=127269397`
          - strict whole: `kilo_kernel_small_hk = C 82 ms / Ny AOT 28 ms` (`repeat=3`, parity ok)
          - `ny_main` now emits:
            - edit path: `array.string_len_hi -> array.string_insert_mid_store_hisii`
            - meso subrange path: `array.string_len_hi -> array.kernel_slot_insert_hisi -> string.kernel_slot_substring_hii_in_place -> array.kernel_slot_store_hi`
            - branch suffix path: `array.string_indexof_hih -> array.string_suffix_store_hisi`
          - insert-mid/suffix paths no longer emit `nyash.array.get_hi`, `nyash.array.kernel_slot_insert_hisi`, `nyash.array.kernel_slot_concat_his`, or `nyash.array.kernel_slot_store_hi`
          - `__strlen_evex` and `core::str::converts::from_utf8` are absent from the current whole asm hot report
          - meso subrange path no longer emits `nyash.array.slot_load_hi`, `nyash.string.substring_hii`, `nyash.string.substring_concat3_hhhii`, or `nyash.array.set_his`
        - boundary:
          - this is still a narrow source-only window
          - live-after-get substring reuse keeps object-handle loading
          - `TextLane` / Value Lane / allocator follow through `137x-E/F/G`; MIR legality and broad container lane-hosting remain separate unless those gates need them
        - next owner proof seam:
          - current whole-front asm clusters on `memchr::arch::x86_64::memchr::memchr_raw::find_avx2`, `array_string_concat_const_suffix_by_index_store_same_slot_str`, `__memmove_avx512_unaligned_erms`, `array_string_indexof_by_index_str`, `array_string_insert_const_mid_by_index_store_same_slot_str`, and `array_string_len_by_index`
          - whole-front asm still needs a fresh reread before choosing the next owner
          - old helper-local next-card rule is superseded; the next implementation sequence is `137x-E/F/G`
  - good cut point:
    - the Phase 2.5 read-side alias lane now has array/map proof on all three read outcomes:
      - `live source`
      - `cached handle`
      - `cold fallback`
    - the strict whole stability reread after the read-encode cleanup is now taken
    - fresh proof keeps this lane reject-side for keeper judgment:
      - exact stays closed
      - meso stays open/noisy
      - strict whole stays in the `~800 ms` band
      - asm owner remains read/materialize/copy tax, not a new `TextLane` / MIR legality proof
  - recent cleanup slice:
    - `phase2.5-map-surface-contract-cleanup`
    - status:
      - parked after smallest BoxShape cards
      - `observe` counter registration and sink/test raw-index mirrors are parked
      - legacy map compat surface is quarantine-only; no active lowering/runtime users remain
    - scope:
      - keep raw map string publication at the `value_codec` encode seam, not inside `MapBox`
      - route legacy `nyash.map.get_h` / `nyash.map.get_hh` through the same slot-load substrate contract
      - keep map debug env lookup in a neutral helper instead of `map_substrate -> map_compat` coupling
      - keep raw map observers owned by `map_substrate`, not `map_runtime_facade`
      - keep `map_compat` on shared facade/substrate surfaces only
      - keep runtime-data mixed map lane out of `map_runtime_facade`
      - keep raw map mutation (`clear/delete`) in a dedicated mutation leaf, not in facade wiring
      - keep public `MapBox` clear/delete and raw slot mutation clear/delete on the same narrow `MapBox` helpers
      - retire the empty `map_runtime_facade`; public shells now call shared map leaves directly
      - keep runtime-data get/set on string-key slot leaves instead of reopening `with_map_box`
      - keep raw map materializing loads centralized through a string-key load leaf
      - keep map key decode named as map-key policy, not as an array-fast borrowed-string profile
      - keep `MapBox.size/len/length` lowering on `nyash.map.entry_count_i64`; `entry_count_h` is compat/export residue only
      - keep LL emit map i64-key `get/has` lowering on `nyash.map.slot_load_hi` / `nyash.map.probe_hi`; `get_h` / `has_h` are compat/export residue only
      - keep RuntimeData field fallback on `nyash.map.slot_load_hh` / `nyash.map.slot_store_hhh`; `get_hh` / `set_hh` are compat/export residue only
      - keep C-shim map size emission on `nyash.map.entry_count_i64`; `entry_count_h` is compat/export residue only
      - keep Rust `map_compat` out of the public `map::*` re-export; compat ABI exports/tests, including `entry_count_h`, live inside `map_compat.rs`
      - keep `NewBox(ArrayBox)` construction on the ring1 array provider seam; the deprecated builtin ArrayBox fallback is removed
      - keep `NewBox(MapBox)` construction on the ring1 map provider seam; the deprecated builtin MapBox fallback is removed
      - keep `NewBox(PathBox)` construction on the ring1 path provider seam; the deprecated builtin PathBox fallback is removed
      - keep `NewBox(ConsoleBox)` construction on the ring1 console seam; the selfhost fallback remains but the standalone builtin wrapper is removed
      - keep remaining `builtin_impls` as an explicit fallback quarantine; File/Null/primitive fallbacks are not safe deletion candidates without a separate SSOT
      - keep observe TLS snapshot length in `observe/contract.rs`; backend-local raw snapshot length is removed
  - pending todo:
    - `phase2-deferred-const-suffix-stability`
  - do not open a new ABI / `TextLane` cut until this reread is judged keeper vs reject

## Current Snapshot

- keeper front is still closed:
  - `kilo_micro_substring_concat`
    - `C: 2 ms`
    - `Ny AOT: 3 ms`
  - `kilo_micro_substring_only`
    - `C: 3 ms`
    - `Ny AOT: 3 ms`
- exact `store.array.str` front is now also closed by source-only same-slot string-store bridges:
  - `kilo_micro_array_string_store`
    - `C: 10 ms`
    - `Ny AOT: 3 ms`
  - reading:
    - direct-set + trailing-substring shared reuse is a keeper
    - microasm top is now startup/env dominated, so this exact front is no longer the active owner proof
- current bridge front is now adopted:
  - `kilo_meso_substring_concat_array_set_loopcarry`
    - shape: `substring + concat + array.set + loopcarry`
    - role: natural middle between exact micro and whole kilo on this lane
    - rule: use it to confirm store/publication cuts without the whole-front `indexOf("line")` row-scan noise
- current bridge reread after the source-only concat3 subrange landing:
  - `kilo_meso_substring_concat_array_set_loopcarry`
    - `C: 3 ms`
    - `Ny AOT: 18 ms`
  - reading:
    - now below the prior `56-65 ms` band
    - public `array.get` / `substring_hii` / `substring_concat3_hhhii` / `array.set_his` are gone from the loop body on this bridge shape
- current whole accept gate:
  - `kilo_kernel_small`
    - `C: 80 ms`
    - `Ny AOT: 739 ms` (`repeat=3`)
  - reading:
    - pure-first route/build blocker stays cleared; direct and helper replay compile again
    - loop-body `KernelTextSlot` allocas no longer crash the whole bench; `stacksave/stackrestore` keeps the accept gate open
    - whole remains far from keeper territory, but the next owner is now pinned more precisely
    - emitted LLVM IR now proves both hot store sites are already on the kernel-slot lane:
      - direct-set-only `insert_hsi -> kernel_slot_insert_hsi -> kernel_slot_store_hi`
      - direct-set-only `current + "ln" -> kernel_slot_concat_hs -> kernel_slot_store_hi`
    - whole perf/asm reread now reads the remaining owner as materialization/copy tax, not compiler fallback:
      - hottest named repo family: `array_string_store_kernel_text_slot_at 5.99%` + `objectize_kernel_text_slot_stable_box 1.14%`
      - producer helpers are smaller: `insert_const_mid_into_slot 1.64%`, `nyash.string.kernel_slot_concat_hs 0.60%`
      - whole is still led overall by libc `memmove 19.48%` / `_int_malloc 5.05%`
      - observability split now narrows the live owner further upstream:
        - `const_suffix freeze_fallback = 479728 / 480000`
        - `materialize total = 539728` (`~4.5 GB`)
        - `publish_reason.generic_fallback = 539728`
        - whole-side site counters stay cold:
          - `site.string_concat_hh.* = 0`
          - `site.string_substring_concat_hhii.* = 0`
        - reading:
          - the live whole owner now reads as `const_suffix` freeze fallback above the slot sink
          - the next phase-2 card should keep `const_suffix` unpublished into `KernelTextSlot -> kernel_slot_store_hi`, not only optimize post-freeze objectization
      - latest runtime-private materialize cut is now landed:
        - `kernel_slot_concat_hs` prefers borrowed-text direct materialization under `with_text_read_session_ready(...)`
        - `insert_const_mid_into_slot` now takes the same borrowed-text direct materialization path before owned fallback
      - reading:
        - exact stays closed
        - whole improved versus the prior `781 ms` reread
        - strict whole reread also stays in the same better band:
          - `kilo_kernel_small_hk = C 79 ms / Ny AOT 748 ms` (`repeat=3`, parity ok)
        - this is promising, but still needs more whole-side proof before calling it a keeper
      - latest phase-2 deferred `const_suffix` slot cut is now landed:
        - `kernel_slot_concat_hs` can now leave a deferred `const_suffix` state inside the existing `KernelTextSlot` layout
        - `kernel_slot_store_hi` consumes that state before generic freeze/objectize
        - existing `StringBox` array slots append in place when the deferred source still matches the current slot text
        - regression tests now lock `DeferredConstSuffix -> store -> Empty` for append, existing `StringBox`, and existing borrowed-alias retarget routes
      - latest reread after the deferred-slot landing:
        - `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
        - `kilo_kernel_small = C 79 ms / Ny AOT 726 ms`
        - `kilo_kernel_small_hk = C 81 ms / Ny AOT 808 ms` (`strict`, parity ok)
      - reading:
        - exact stays closed
        - plain whole improved again versus the prior `739 ms` reread
        - strict whole is still noisy enough that the card is not a keeper yet
        - next step is a strict whole stability reread, not a new ABI/`TextLane` cut
      - rejected follow-up probe:
        - replacing BorrowedHandleBox unpublished retarget objectization with an owned-string keep regressed whole:
          - probe band: `kilo_kernel_small = C 81 ms / Ny AOT 980 ms`
          - probe band: `kilo_kernel_small_hk = C 80 ms / Ny AOT 1015 ms`
        - reject reason:
          - store-side `Arc<StringBox>` creation disappeared, but `array.get` / alias encode fallback started allocating a fresh stable object on every read
          - this moved cost from store to `lines.get(...)` and made whole worse
      - restored reread after backing that probe out:
        - `kilo_kernel_small = C 81 ms / Ny AOT 810 ms`
        - `kilo_kernel_small_hk = C 82 ms / Ny AOT 864 ms`
        - next seam was not `owned-string keep`; follow-up had to preserve cheap alias encode on `array.get`
      - follow-up card was read-side alias lane split:
        - `TextReadOnly`
        - `EncodedAlias`
        - `StableObject`
        - stable objectize stays cold and cache-backed, not per-read
      - first phase 2.5 slice is now landed:
        - `BorrowedHandleBox` caches the encoded runtime handle for unpublished keeps
        - `array.get` can reuse the cached stable handle instead of fresh-promoting on every read
        - latest strict reread: `kilo_kernel_small_hk = C 79 ms / Ny AOT 791 ms` (`repeat=3`, parity ok)
      - latest phase 2.5 read-lane follow-on slices are now landed:
        - map string values now preserve borrowed string aliases on store instead of eagerly re-boxing to stable `StringBox`
        - borrowed-alias runtime-handle cache is now shared per alias lineage, so map read clones do not lose the cached encoded handle
        - `perf-observe` now distinguishes array/map read outcomes for:
          - `live source`
          - `cached handle`
          - `cold fallback`
        - end-to-end tests now lock that three-lane contract for both:
          - `array_get_index_encoded_i64`
          - `nyash.runtime_data.get_hh` map reads
      - latest strict reread on the updated phase-2.5 lane:
        - `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
        - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 61 ms`
        - `kilo_kernel_small_hk = C 82 ms / Ny AOT 809 ms`
        - `kilo_kernel_small_hk = C 80 ms / Ny AOT 892 ms`
      - cleanup-parked strict reread:
        - `kilo_kernel_small_hk = C 80 ms / Ny AOT 872 ms` (`repeat=3`, parity ok)
        - `kilo_kernel_small_hk = C 79 ms / Ny AOT 842 ms` (`repeat=3`, parity ok)
      - cleanup-parked asm/top owner proof:
        - command:
          - `PERF_VM_FORCE_NO_FALLBACK=1 PERF_AOT_DIRECT_ONLY=1 bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small_hk 'ny_main' 1`
        - top report:
          - libc copy/alloc remains dominant: `__memmove_avx512_unaligned_erms 21.41%`, `_int_malloc 9.26%`, `malloc 1.51%`
          - hottest named repo read/materialization family:
            - `objectize_kernel_text_slot_stable_box 4.42%`
            - `array_get_index_encoded_i64::{closure} 4.25%`
            - nested `array_get_index_encoded_i64` closure `2.70%`
            - `TextKeepBacking::clone_stable_box_cold_fallback 0.94%`
          - store/producer helpers are lower:
            - `array_string_store_kernel_text_slot_at::{closure} 1.99%`
            - `array_string_indexof_by_index... 1.00%`
            - `string_span_cache_get 0.61%`
            - `nyash.string.kernel_slot_concat_hs 0.40%`
            - `nyash.array.kernel_slot_store_hi 0.30%`
            - `insert_const_mid_into_slot::{closure} 0.22%`
        - reading:
          - this confirms the updated lane should not reopen store-side `owned-string keep`
          - current owner proof points at read-side encode/materialize/objectize around `array.get`, with stable objectization required to stay cached/cold
          - next implementation seam must preserve cheap alias encode and reduce per-read materialization/copy tax before any new `TextLane` / MIR legality card
      - latest read-encode BoxShape cleanup:
        - `array.get` now calls a scalar-checked borrowed-alias encoder after its local int/bool probes, so the generic encoder does not repeat `as_i64_fast` / `as_bool_fast` before the borrowed-alias decision
        - follow-on: borrowed-alias encode planning now snapshots `drop_epoch` once and passes it into the cached-handle check, so live-source and cached-handle decisions share one epoch view
        - contract unchanged:
          - live-source reuse remains first
          - cached stable handle reuse remains second
          - cold stable objectize fallback remains explicit and observable
        - validation:
          - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib array_get_index_reuses_cached_runtime_handle_for_unpublished_alias`
          - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib any_arg_to_box_array_fast_profile_reuses_live_source_handle_for_string`
          - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib --features perf-observe array_get_index_records_cached_handle_hit_for_array_lane`
          - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib --features perf-observe array_get_index_records_live_source_hit_for_array_lane`
          - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib --features perf-observe array_get_index_records_cold_fallback_for_array_lane`
          - `cargo check -q -p nyash_kernel`
          - `cargo test -q -p nyash_kernel --lib` (one earlier `string_concat3_hhh_contract` run was flaky; single rerun and full rerun passed)
          - `tools/checks/dev_gate.sh quick`
        - perf reread after the cleanup is not keeper evidence:
          - exact remains closed: `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
          - meso remains open/noisy: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 65 ms`
          - strict whole is noisy: first `kilo_kernel_small_hk = C 80 ms / Ny AOT 1740 ms`, rerun `C 80 ms / Ny AOT 808 ms`
        - reading:
          - this is structure cleanup on the existing read-side alias lane, not a new keeper optimization
          - next owner remains stable keep creation / first-read handle publication around the existing borrowed-alias store-read chain
      - rejected follow-up probe after the fresh owner proof:
        - attempted unpublished `owned-text keep` for `KernelTextSlot -> existing BorrowedHandleBox` retarget, keeping public ABI and `KernelTextSlot` layout unchanged
        - exact guard stayed closed: `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`
        - meso stayed noisy/open: `kilo_meso_substring_concat_array_set_loopcarry = C 4 ms / Ny AOT 62 ms`
        - strict whole regressed: `kilo_kernel_small_hk = C 84 ms / Ny AOT 902 ms`, rerun `C 82 ms / Ny AOT 892 ms`
        - asm/top removed `objectize_kernel_text_slot_stable_box`, but shifted cost into `__memmove_avx512_unaligned_erms 28.32%`, `_int_malloc 12.47%`, and `array_string_store_kernel_text_slot_at::{closure} 5.89%`
        - reject reason:
          - active whole still calls `array.get_hi`, so delaying stable birth from store to read does not remove object-world demand
          - the seam moved publication/copy tax and increased store/read residence work
          - code was reverted; do not reopen store-side `owned-string keep` or `owned-text keep` without a front that no longer demands an object handle on read
      - rejected follow-up probe: array-slot concat-by-index helper
        - attempted runtime-private `nyash.array.kernel_slot_concat_his(slot, array_h, idx, suffix)` and lowered the hot `array.get_hi -> const_suffix concat -> kernel_slot_store_hi` store to it
        - exact guard stayed closed: `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`
        - meso stayed noisy/open: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 62 ms`
        - strict whole regressed: first `kilo_kernel_small_hk = C 82 ms / Ny AOT 1571 ms`, rerun `C 80 ms / Ny AOT 1033 ms`
        - IR proof:
          - new helper was emitted at the hot concat store
          - earlier `nyash.array.slot_load_hi` still remained before `nyash.array.string_indexof_hih`
        - reject reason:
          - adding a direct concat helper without eliminating the live `array.get_hi` read only adds another executor path
          - code was reverted; do not retry array-slot concat helpers unless the same card also proves the preceding `slot_load_hi` is removed safely
      - reading:
        - phase 2.5 contract is now much tighter on read behavior
        - exact stays closed, but meso / strict whole reopened upward versus the prior `57 ms` / `791 ms` band
        - this reread is reject-side evidence for keeper judgement on the updated lane
        - cleanup queue is now parked after the smallest BoxShape cards
        - next step is whole/meso owner proof before choosing another implementation seam
- accepted task order is now fixed as a phase rollout, not as isolated helper cuts:
  - semantic lock:
    - `String = value`
    - `publish = boundary effect`
    - `freeze.str = only birth sink`
  - `Phase 1`: producer-first unpublished contract with current carriers
    - keep `VerifiedTextSource -> TextPlan -> OwnedBytes -> KernelTextSlot transport`
    - goal: canonical sink continuity before any early handle publish
  - `Phase 2`: isolate publish as a cold effect
    - goal: `objectize` / `issue_fresh_handle` leave producer helpers
  - `Phase 2.5`: split the read-side alias lane
    - goal: `TextReadOnly` / `EncodedAlias` stay common path and `StableObject` stays cold
  - `Phase 3`: `TextLane` storage/residence implementation through `137x-E`
    - goal: specialize array internal text residence without changing public array semantics
  - `Phase 4`: `137x-F` Value Lane bridge closed; `137x-G` allocator pilot deferred before returning to kilo optimization
    - goal: keep runtime-wide residence proven while requiring dominant allocation evidence before allocator work
- current active work is phase 2 / phase 2.5 lane:
  - isolate publish as a cold effect without changing public ABI
  - keep read-side alias continuity cheap and cache-backed
  - keep `KernelTextSlot` as the first canonical sink transport / seed
  - landed slices now include:
    - explicit cold publish adapters around `string_handle_from_owned_*` and `publish_owned_bytes_*`
    - latest-fresh source-capture prework
    - existing `StringBox` slot overwrite in place on `kernel_slot_store_hi`
    - borrowed-text direct materialization on `kernel_slot_concat_hs` / `insert_const_mid_into_slot`
    - map-side borrowed string value store under `CodecProfile::MapValueBorrowString`
    - read-side alias outcome observation on array/map (`live source` / `cached handle` / `cold fallback`)
    - raw map string publication stays at the encode seam, while `MapBox` keeps only read-visibility policy
    - legacy map compat `get_h` / `get_hh` now route through the same slot-load substrate path as raw aliases
    - map debug env lookup is now isolated in a neutral helper instead of depending on compat wiring
    - raw map observers now stay in `map_substrate`; `map_runtime_facade` no longer owns entry-count/cap observer forwarding
    - `map_compat` no longer touches `MapBox` directly and stays on shared facade/substrate surfaces
    - runtime-data mixed map lane now lives in `map_runtime_data`
    - raw map mutation now lives in `map_slot_mutate`
    - `map_runtime_facade` is retired; aliases/compat call `map_probe`, `map_slot_load`, `map_slot_store`, `map_slot_mutate`, and `map_substrate` directly
    - runtime-data map get/set now reuse string-key slot leaves and no longer open `with_map_box` directly
    - raw map materializing loads now share `map_slot_load_str`
    - map key decode now uses `CodecProfile::MapKeyBorrowString`, keeping scalar-prefer/string-alias behavior without depending on the array profile name
  - compiler fallback probe is closed for the whole bench
  - `TextLane` is no longer skipped; it is opened only through the `137x-E` SSOT gate
- current accepted redesign is now locked in narrowed form:
  - keep `public handle ABI`
  - move the first code cut to producer-side unpublished outcome
  - first landing is not a general `TransientText` rollout; it is a narrow runtime-private phase-1 producer contract with:
    - hot sink path: `const_suffix -> KernelTextSlot -> store.array.str`
    - hot shared-receiver reuse: the same producer contract may also feed trailing `substring(...)` without early handle publish
  - rollout/task anchor is now:
    - `docs/development/current/main/design/string-value-model-phased-rollout-ssot.md`
    - `docs/development/current/main/phases/phase-137x/phase137x-text-lane-rollout-checklist.md`
  - compiler/backend consumption is landed for:
    - direct-set-only `const_suffix -> set(...)`
    - narrow shared-receiver exact front: `text + "xy"` reused by `set(...)` + known-length observer + trailing `substring(...)`
  - rule: keep the producer specialized, but widen the internal contract so `set` and trailing `substring` can both consume the deferred `const_suffix` result before any publish boundary
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
  - exact `array/string-store` is now closed; the live next owner family is upstream producer publication on whole
  - hot-corridor carrier design anchor is now:
    - `docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md`
  - cleanup audit after the carrier stack:
    - keep as-is:
      - `text_carrier.rs`
      - `string_classify.rs`
      - `string_store.rs`
      - `string_view.rs`
      - `string_search.rs`
      - current `concat/` submodules
    - next structural candidate:
      - `crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs`
      - reason: proof/lifetime keep, boundary box, and public helper API still share one large file
    - deferred candidates:
      - `crates/nyash_kernel/src/exports/string_helpers/cache.rs`
      - `crates/nyash_kernel/src/plugin/value_codec/string_materialize.rs`
    - debug-only readers may stay implicit where `TextRef` deref is clearer than explicit projection
  - trusted direct MIR no longer duplicates the `text + "xy"` producer across `set(...)` and trailing `substring(...)`
  - runtime gap on exact no longer stays open after the shared-receiver `KernelTextSlot` widening
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
  - new boundary direct-set-only guard is landed for the narrow substrate bridge:
    - fixture: `apps/tests/mir_shape_guard/string_const_suffix_kernel_slot_direct_set_min_v1.mir.json`
    - guard: `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_const_suffix_kernel_slot_store_contract.sh`
  - new shared-receiver guard is landed for the exact-front widening:
    - fixture: `apps/tests/mir_shape_guard/string_const_suffix_kernel_slot_shared_receiver_min_v1.mir.json`
    - guard: `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_const_suffix_kernel_slot_shared_receiver_contract.sh`
  - next code cut is now fixed:
    - keep `const_suffix` producer specialized
    - the first `Pieces3` / `insert_const_mid_fallback` direct-set widening is now landed:
      - runtime seam: `nyash.string.kernel_slot_insert_hsi`
      - compiler seam: deferred `insert_hsi` direct-set consumer lowers to `kernel_slot_insert_hsi -> kernel_slot_store_hi`
      - guard: `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_insert_mid_direct_set_min.sh`
    - the first `Pieces3` / `insert_const_mid_fallback` shared-receiver widening is now landed:
      - runtime seam: `nyash.string.kernel_slot_insert_hsi`
      - compiler seam: deferred `insert_hsi` shared receiver lowers to `kernel_slot_insert_hsi -> kernel_slot_store_hi` while trailing `substring(...)` reuses deferred `piecewise_subrange_hsiii`
      - fixture: `apps/tests/mir_shape_guard/string_insert_mid_kernel_slot_shared_receiver_min_v1.mir.json`
      - guard: `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_insert_mid_shared_receiver_min.sh`
    - the first deferred `Pieces3 substring` direct-set widening is now landed:
      - runtime seam: `nyash.string.kernel_slot_piecewise_subrange_hsiii`
      - compiler seam: deferred `piecewise_subrange_hsiii` direct-set consumer lowers to `kernel_slot_piecewise_subrange_hsiii -> kernel_slot_store_hi`
      - fixture: `apps/tests/mir_shape_guard/string_piecewise_kernel_slot_store_min_v1.mir.json`
      - guard: `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_piecewise_direct_set_min.sh`
    - the first post-store reuse / non-direct-set `Pieces3` widening is now landed structurally:
      - compiler seam:
        - first `substring_hii` over deferred `insert_hsi` can stay unpublished when the result is reused by `set(...)` plus a later `substring(...)`
        - the later `substring(...)` lowers through composed `piecewise_subrange_hsiii` instead of reopening `substring_hii`
      - fixture: `apps/tests/mir_shape_guard/string_piecewise_kernel_slot_post_store_reuse_min_v1.mir.json`
      - guard: `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_piecewise_post_store_reuse_min.sh`
    - latest post-store-reuse reread:
      - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 3 ms`
      - middle `kilo_meso_substring_concat_array_set_loopcarry`: `C 3 ms / Ny AOT 54 ms`
      - whole `kilo_kernel_small`: `C 79 ms / Ny AOT 733 ms` (IPC: C=1.85 / Ny=0.60)
      - reading:
        - exact stays closed
        - middle is flat-to-better (was 56-59 ms baseline, now 54 ms = slight improvement)
        - whole is NOT a regression from this change; pre-existing (has been 700-856 ms for many commits)
        - whole failure mode: stall collapse (IPC 1.85→0.60); instruction count ratio is 0.79 (near-proportional); slow is from memory stalls / publication entry, not extra work
        - whole hot symbols (perf/asm audit 2026-04-18):
          - `libc memmove 18.91%`, `_int_malloc 4.60%` — materialization malloc/memmove is owner
          - `array_string_store_kernel_text_slot_at` closure `7.96%`
          - `array_get_index_encoded_i64` closure `3.44%`
          - `insert_const_mid_into_slot` closure `1.54%`
          - `nyash.string.kernel_slot_concat_hs` `1.21%`
        - `piecewise_subrange_hsiii_composed_window` is NOT in the hot symbols
        - composed-window path is confirmed NOT on the whole hot path; landing is safe
        - whole keeper requires reducing malloc/memmove materialization tax — Phase 2/3 territory, not Phase 1
    - latest phase-2 cold publish slice:
      - code seam:
        - `string_handle_from_owned{,_concat_hh,_substring_concat_hhii,_const_suffix}` now enter explicit cold publish adapters
        - `publish_owned_bytes_*_boundary` and `objectize_kernel_text_slot_stable_box` are outlined cold boundaries
      - docs seam:
        - phase checklist numbering now matches the phased-rollout SSOT (`Phase 2 = cold publish effect`, `Phase 3 = TextLane`, `Phase 4 = MIR contract`)
      - reread:
        - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 3 ms`
        - whole `kilo_kernel_small`: `C 81 ms / Ny AOT 768 ms`
      - reading:
        - exact stays closed
        - whole remains in the same stall-collapse owner family (`ratio_instr=0.79`, `ratio_cycles=0.25`, `ratio_ms=0.11`)
        - this is a valid Phase-2 start, but not yet a whole keeper; next card must reduce publish/source-capture frequency, not just outline the same boundary
    - latest phase-2 source-capture slice:
      - code seam:
        - `with_array_store_str_source(...)` now checks a latest-fresh stable-box cache before falling back to registry slot lookup
        - latest-fresh cache is guarded by `drop_epoch`, so same-thread just-published handles can skip one immediate registry read safely
      - reread:
        - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 3 ms`
        - whole `kilo_kernel_small`: `C 80 ms / Ny AOT 1068 ms`
      - reading:
        - exact stays closed
        - whole remains neutral in the same publication/source-capture owner family (`ratio_instr=0.79`, `ratio_cycles=0.26`, `ratio_ms=0.07`)
        - keep this as valid Phase-2 prework, not a keeper
        - legacy coexistence remains temporary; once the new path proves keeper-grade, delete the legacy dual-routing helpers instead of keeping both
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
  - producer site split is landed, and slot exit boundary is now first-class too:
    - `publish_boundary.slot_publish_handle_total`
    - `publish_boundary.slot_objectize_stable_box_total`
    - `publish_boundary.slot_empty`
    - `publish_boundary.slot_already_published`
  - `objectize_kernel_text_slot_stable_box` now records `publish_reason.need_stable_object`
  - current remaining collapse is no longer `which slot exit happened`; it is the upstream owner before that boundary
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
    - slot publish boundary now exposes:
      - `publish_boundary.slot_publish_handle_total`
      - `publish_boundary.slot_objectize_stable_box_total`
      - `publish_boundary.slot_empty`
      - `publish_boundary.slot_already_published`
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
- latest exact / middle / whole slot-boundary reread closes the slot-exit ambiguity:
  - `publish_boundary.slot_publish_handle_total=0`
  - `publish_boundary.slot_objectize_stable_box_total=0`
  - `publish_boundary.slot_empty=0`
  - `publish_boundary.slot_already_published=0`
  - `publish_reason.need_stable_object=0`
  - read: `KernelTextSlot` exit is now observable and inactive; the live owner remains upstream producer publication, especially `site.const_suffix.*` / `site.freeze_text_plan_pieces3.*` on whole
- next implementation card is now re-ordered accordingly:
  - do not reopen slot-boundary-first probes
  - do not widen generic publish helpers first
  - first substrate card is producer-side `const_suffix` into `KernelTextSlot`
  - after that lands, reopen compiler-side slot consumer wiring separately
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
  - `phase137x-specialized-string-handle-payload` tried and rejected:
    - cut:
      - issue owned-bytes publish through direct `StringBox -> handle`
      - add a string-specialized host-handle payload for fast borrowed `&str`
    - 3-run release reread:
      - `kilo_meso_substring_concat_array_set_loopcarry = 68 ms`
      - `kilo_kernel_small = 950 ms`
    - conclusion:
      - this did not move the active owner off producer publication
      - it regressed both adopted middle and whole
      - code is reverted; do not reopen this seam without new evidence that handle payload shape itself is the owner
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

1. open implementation gates before the next kilo optimization
   - closed gates:
     - `137x-A`: string publication contract closeout
     - `137x-B`: container / primitive design cleanout
     - `137x-C`: structure completion before perf return
     - `137x-D`: exact array store route-shape keeper
     - `137x-E0`: MIR / backend seam closeout
     - `137x-E1`: minimal TextLane / ArrayStorage::Text
   - current blocker: `137x-H owner-first optimization return`
   - order:
     - `137x-E0.1`: remove old `9-block` seed shape
     - `137x-E0.2`: export shared-receiver alias metadata for active guards and remove the `.inc` legacy scanner fallback
     - `137x-E1`: minimal `TextLane` / `ArrayStorage::Text` (landed)
     - `137x-F`: runtime-wide Value Lane implementation bridge (closed)
     - `137x-G`: allocator / arena pilot (rejected / not opened by F closeout)
     - `137x-H`: next kilo optimization return
   - still blocked here:
     - `publish.any`
     - typed map lane
     - heterogeneous / union array slot layout
     - public ABI widening
2. return through `137x-H` after `137x-F/G` closeout
   - `137x-F` consumed phase-289x vocabulary and demand ledgers as implementation input, not as broad runtime rewrite permission
   - `137x-G` is not opened from current evidence: middle `cfree` is 9.45% and whole `__memmove_avx512_unaligned_erms` is 5.39%, while main owners are string len/indexof/slot-write paths
   - next optimization must start owner-first from the measured hot transition, not from an allocator rewrite
3. preserve landed `137x-D` proof as baseline evidence
   - proof card: `137x-D exact array store route-shape proof`
   - front: `kilo_micro_array_string_store`
   - implementation: MIR owns the compact 8-block direct shape as `metadata.array_string_store_micro_seed_route`; `hako_llvmc_match_array_string_store_micro_seed(...)` now only reads that metadata and selects the existing specialized stack-array emitter
   - smoke: `phase137x_direct_emit_array_store_string_contract.sh` requires exact seed emitter selection and no runtime/public helper calls in `ny_main`
   - guard results:
     - exact: `kilo_micro_array_string_store = C 10 ms / Ny AOT 10 ms`, `ny_aot_instr=26922384`
     - middle: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 9 ms`, `ny_aot_instr=129614388`
     - strict whole: `kilo_kernel_small_hk = C 84 ms / Ny AOT 26 ms`, parity ok
5. keep rejected probes as negative evidence
   - unpublished `owned-text keep` removed the `objectize_kernel_text_slot_stable_box` symbol from asm, but strict whole regressed to `902 ms` / `892 ms`
   - reject reason: active whole still demands an object handle at `array.get_hi`, so delayed stable birth only moves the cost
   - `nyash.array.kernel_slot_concat_his` emitted, but the preceding `nyash.array.slot_load_hi` stayed live in IR
   - strict whole regressed to `1571 ms` / `1033 ms`; code reverted
   - next attempt must eliminate the `array.get_hi` demand itself, not only add a second helper after it
6. current proof commands:
   - `PERF_AOT=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_string_store 1 3`
   - `PERF_AOT=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`
   - `PERF_AOT_DIRECT_ONLY=1 bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry 'ny_main' 3`
   - `PERF_VM_FORCE_NO_FALLBACK=1 PERF_REQUIRE_AOT_RESULT_PARITY=1 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small_hk 1 3`
   - `PERF_VM_FORCE_NO_FALLBACK=1 PERF_AOT_DIRECT_ONLY=1 bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small_hk 'ny_main' 1`

## Guardrails

- MIR/lowering still owns legality, `proof_region`, and `publication_boundary`
- keep carrier/publication split physically narrow
- `137x-E/F` are closed; `137x-G` remains deferred until allocation becomes the dominant measured owner
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
