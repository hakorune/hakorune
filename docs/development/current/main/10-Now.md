---
Status: SSOT
Date: 2026-04-24
Scope: current lane / blocker / next pointer only.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/phases/phase-137x/137x-93-container-primitive-design-cleanout.md
  - docs/development/current/main/phases/phase-137x/137x-95-mir-backend-seam-closeout-before-textlane.md
  - docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md
  - docs/development/current/main/design/lifecycle-typed-value-language-ssot.md
  - docs/development/current/main/design/value-corridor-generic-optimization-contract.md
  - docs/development/current/main/phases/phase-289x/README.md
  - docs/development/current/main/phases/phase-289x/289x-90-runtime-value-object-design-brief.md
  - docs/development/current/main/phases/phase-289x/289x-91-runtime-value-object-task-board.md
  - docs/development/current/main/phases/phase-289x/289x-92-value-boundary-inventory-ledger.md
  - docs/development/current/main/phases/phase-289x/289x-93-demand-vocabulary-ledger.md
  - docs/development/current/main/phases/phase-289x/289x-94-container-demand-table.md
  - docs/development/current/main/phases/phase-289x/289x-95-array-text-residence-pilot.md
  - docs/development/current/main/phases/phase-289x/289x-96-demand-backed-cutover-inventory.md
  - docs/development/current/main/design/string-value-model-phased-rollout-ssot.md
  - docs/development/current/main/phases/phase-137x/phase137x-text-lane-rollout-checklist.md
---

# Self Current Task — Now (main)

## Current

- current lane:
  - compiler cleanup lane is primary
  - current-state token: `phase-291x CoreBox surface contract cleanup`
  - active phase: `docs/development/current/main/phases/phase-291x/README.md`
  - phase status SSOT: `docs/development/current/main/phases/phase-291x/README.md`
  - method anchor: `docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md`
  - taskboard: `docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md`
  - current implementation focus: `successor cleanup card selection` (after 291x-120)
  - current phase goal:
    - phase-292x is closed: `.inc` analysis debt is 0 files / 0 lines, with
      1 file / 2 explicit view-owner lines guarded separately
    - keep `.inc` on metadata read / validation / emit / skip / fail-fast only
    - resume CoreBox cleanup one contract row at a time
    - `MapBox.length()` is landed as a read-only alias for the existing Map
      size surface
    - non-empty source-level vm-hako `MapBox.values().size()` is landed and pinned
    - non-empty source-level vm-hako `MapBox.keys().size()` is landed and pinned
    - source-level vm-hako `MapBox.remove(key)` delete-owner alias is landed and pinned
    - source-level vm-hako `MapBox.clear()` state reset is landed and pinned
    - source-level vm-hako `MapBox.set(...)` duplicate-receiver routing is
      landed and pinned
    - `keys()/values()` element publication is landed and pinned by the
      291x-102 acceptance smoke
    - `StringBox.lastIndexOf(needle, start_pos)` is landed and pinned by the
      291x-103 acceptance smoke
    - `MapBox.delete(key)` / `remove(key)` is landed on the catalog-backed
      Unified value path and pinned by the 291x-104 acceptance tests
    - `MapBox.clear()` is landed on the catalog-backed Unified value path and
      pinned by the 291x-105 acceptance tests
    - `ArrayBox.get/pop/remove` element-result publication is landed and pinned
      by the 291x-106 acceptance tests; publish `T` only for known `Array<T>`
      receivers and keep `Unknown` for mixed or untyped receivers
    - MapBox write-return receipt implementation is landed and pinned
    - MapBox bad-key normalization implementation is landed and pinned
    - MapBox get missing-key contract is landed and pinned
    - `291x-110` landed: `MapBox.get(existing-key)` publishes `V` only for
      receiver-local homogeneous Map facts with tracked literal keys; mixed,
      untyped, and missing-key reads stay `Unknown`
    - `291x-111` landed: StringBox `toUpper` / `toLower` now live in the stable
      catalog rows, and `toUpperCase` / `toLowerCase` remain compatibility
      aliases on the same rows
    - `291x-112` landed: `ArrayBox.clear()` is now catalog-backed, uses the
      Unified receiver-only value path, and publishes `Void`
    - `291x-113` landed: `ArrayBox.contains(value)` is now catalog-backed, uses
      the Unified receiver-plus-value path, and publishes `Bool`
    - `291x-114` landed: `ArrayBox.indexOf(value)` is now catalog-backed, uses
      the Unified receiver-plus-value path, and publishes `Integer`
    - `291x-115` landed: `ArrayBox.join(delimiter)` is now catalog-backed, uses
      the Unified receiver-plus-delimiter path, and publishes `String`
    - `291x-116` landed: `ArrayBox.reverse()` is now catalog-backed, uses the
      Unified receiver-only path, and publishes the `String` receipt
    - `291x-117` landed: `ArrayBox.sort()` is now catalog-backed, uses the
      Unified receiver-only path, and publishes the `String` receipt
    - `291x-118` landed: direct source `ArrayBox.slice()` result follow-up
      calls stay on the `ArrayBox` receiver path and do not lower as
      `RuntimeDataBox.length`
    - `291x-119` landed: phase-291x stale status/deferred wording is closed as
      docs-only BoxShape cleanup; no CoreBox behavior changed
    - `291x-120` landed: MapBox taskboard stale follow-up wording is closed as
      docs-only BoxShape cleanup; future-risk rows remain explicitly deferred
  - current app gap read:
    - ArrayBox surface SSOT is landed for `length/size/len/get/set/push/pop/clear/contains/indexOf/join/reverse/sort/slice/remove/insert`
    - `tools/smokes/v2/profiles/integration/apps/phase290x_arraybox_surface_catalog_vm.sh` pins the ArrayBox precedent
    - StringBox surface SSOT is landed for the first stable rows and pinned by `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh`
    - first StringBox stable target is `length/len/size/substr/substring/concat/indexOf/find/replace/trim/lastIndexOf/contains`
    - CoreBox router follow-up has moved `StringBox.length/len/size`, `StringBox.substring/substr`, `StringBox.concat`, `StringBox.trim`, `StringBox.toUpper/toUpperCase`, `StringBox.toLower/toLowerCase`, `StringBox.contains`, one-arg and two-arg `StringBox.lastIndexOf`, `StringBox.replace`, `StringBox.indexOf/find`, `ArrayBox.length/size/len`, `ArrayBox.push`, `ArrayBox.slice`, `ArrayBox.get`, `ArrayBox.pop`, `ArrayBox.set`, `ArrayBox.clear`, `ArrayBox.contains`, `ArrayBox.indexOf`, `ArrayBox.join`, `ArrayBox.reverse`, `ArrayBox.sort`, `ArrayBox.remove`, `ArrayBox.insert`, `MapBox.size`, `MapBox.length`, `MapBox.len`, `MapBox.has`, `MapBox.get`, `MapBox.set`, `MapBox.keys`, `MapBox.values`, `MapBox.delete`, `MapBox.remove`, and `MapBox.clear` to the Unified value path; latest cleanup is `291x-120` MapBox taskboard closeout
    - `291x-107` landed: keep `src/boxes/basic/string_surface_catalog.rs` as
      the String semantic owner, keep `apps/std/string.hako` as public sugar,
      keep `apps/lib/boxes/string_std.hako` internal, delete dead
      `apps/std/string_std.hako`, and pin the public sugar smoke through
      `apps.std.string`
    - `291x-108` landed: keep manifest alias lookup in `hako.toml`, keep
      imported static-box alias binding in the runner text-merge strip path,
      and keep static receiver/type-name lowering scoped to `Alias.method(...)`
      instead of treating imported aliases as namespace roots
    - `291x-109` landed: keep `OpsCalls.map_has(...)` as the only remaining
      selfhost-runtime `pref == "ny"` Map wrapper, and keep
      `crates/nyash_kernel/src/plugin/map_compat.rs` as compat-only legacy ABI
      quarantine instead of a forward route owner
    - MapBox Rust vtable surface is cataloged; legacy `apps/std/map_std.hako` and unused `map_keys_values_bridge.hako` prototype were deleted
    - source-level vm-hako non-empty `MapBox.values()` state-owner shape is pinned by `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_values_vm.sh`
    - source-level vm-hako non-empty `MapBox.keys()` state-owner shape is pinned by `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_keys_vm.sh`
    - source-level vm-hako `MapBox.remove(key)` alias is pinned by `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_remove_vm.sh`
    - source-level vm-hako `MapBox.clear()` state reset is pinned by `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_clear_vm.sh`
    - source-level vm-hako `MapBox.set(...)` duplicate receiver stripping is pinned by `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_set_multiarg_vm.sh`
    - `apps/lib/boxes/map_std.hako` prelude debt is closed; `OpsCalls.map_has(...)` owns the remaining `pref == "ny"` Map-only wrapper
    - static-box `me.*` friction remains a separate semantics/diagnostics topic
    - direct source `slice()` result follow-up calls are pinned by `291x-118`
      to stay on the `ArrayBox` receiver path
    - two-arg `lastIndexOf(needle, start_pos)` is landed and pinned by `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_lastindexof_start_vm.sh`
  - current blocker token: `phase-291x successor cleanup card selection pending`
  - execution mode:
    - `137x-E0 MIR / backend seam closeout` is closed
    - `137x-E1 minimal TextLane / ArrayStorage::Text` is landed before further kilo tuning
    - `137x-F Value Lane bridge` is closed; `137x-F1 demand-to-lane executor bridge` and `137x-F2 producer outcome manifest split` are landed
    - `137x-G` allocator / arena pilot is rejected for now; allocator/copy is secondary, not dominant
  - phase-137x blocker remains recorded as `137x-H46 text-cell residence/materialization design`
  - stop rule:
    - app lane is primary; phase-137x is observe-only unless app work is actually blocked
    - helper-local perf reopen is closed; new perf cards need one-family owner pin plus one-card rollback
  - keeper evidence remains direct-only; exact/middle/whole gates must be recorded before accepting each implementation slice
  - next task order:
    - active entry: `docs/development/current/main/phases/phase-137x/137x-current.md`
    - ownership map: `docs/development/current/main/phases/phase-137x/137x-array-text-contract-map.md`
    - H24 verdict: active owner is write-lock guard mechanics, not fallback/promotion or byte-edit/memmove
    - H25a landed: metadata-only `array_text_residence_sessions`; `.inc` and runtime behavior were unchanged
    - H25b landed: MIR-owned begin/update/end placement metadata and skip indices
    - H25c.1 landed: `.inc` consumes residence-session metadata first, still behavior-preserving
    - H25c.2a landed: runtime-private session substrate
      (`ArrayTextSlotSession` + kernel-private `ArrayTextWriteTxn`)
    - H25c.2b closed: clean one-call update boundary, but non-keeper because
      it still acquires the write lock once per iteration
    - H25c.2c/H25c.3 closed: MIR-owned single-region executor metadata now
      drives one begin-site runtime call; result was `C 3 ms / Ny AOT 5 ms`
    - H25d.1/H25d.2 landed: direct text-resident region loop plus hot/cold
      fixed len-store mutation split
    - H25d result: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 3 ms`,
      `ny_aot_instr=16570267`, `ny_aot_cycles=3471656`
    - H25d.5 closes residual memmove / mutation surgery: H25d.3/H25d.4 both
      regressed, so keep H25d.1/H25d.2 as the accepted code
    - H25e post-parity owner refresh selected the whole-front inner scan:
      `indexOf("line") >= 0` followed by same-slot suffix store
    - H26 landed the MIR-owned observer-store region executor and moved the
      inner `indexOf("line")` + suffix store out of the emitted AOT loop
    - H27 landed the MIR-owned len-half insert-mid edit contract; the outer
      edit path no longer calls `nyash.array.string_len_hi`
    - H28 starts from the remaining observer-store search/copy mechanics:
      fixed const-needle search, suffix mutation/copy, and transaction frame
      cost under the existing MIR-owned H26 region contract
    - H28.1 landed the runtime-private fixed-literal search leaf:
      `kilo_kernel_small = C 84 ms / Ny AOT 9 ms`,
      `ny_aot_instr=60662079`; `Pattern::is_contained_in` is no longer a top
      owner
    - H28.2 landed the runtime-private short-literal prefix compare cleanup:
      `kilo_kernel_small = C 83 ms / Ny AOT 7 ms`,
      `ny_aot_instr=64501392`; `__memcmp_evex_movbe` is no longer a top owner
    - H28.3 landed the runtime-private short-suffix append cleanup:
      `kilo_kernel_small = C 82 ms / Ny AOT 7 ms`,
      `ny_aot_instr=60615291`; short suffix append no longer calls `memcpy`
    - H28.4 rejected the Rust-only text append headroom trial:
      `__memmove` share dropped, but whole instr/cycles/wall did not improve
    - H28.5 callgraph found residual `memmove` is primarily the outer
      len-half edit closure (`27.91%`), not append capacity (`~0.93%`
      realloc/growth)
    - H29 rejected the runtime-private `String::insert_str` bypass:
      whole stayed `Ny AOT 7 ms`, cycles stayed flat, and `__memmove` rose
      to `40.84%`
    - H30 active: decide whether the next clean keeper requires a narrow
      array text edit residence representation, not more local byte-copy
      surgery
    - H30.1 inventory says the next code slice should be BoxShape-only:
      introduce a flat `ArrayTextCell` boundary before any non-contiguous
      text residence prototype
    - H30.1 code slice is landed: `ArrayStorage::Text` now stores
      `Vec<ArrayTextCell>` with a flat-only implementation and no MIR, `.inc`,
      public ABI, or behavior change
    - H30.2 landed: the H27 len-half helper now routes through
      `ArrayBox::slot_insert_const_mid_lenhalf_raw` and the runtime-private
      `ArrayTextCell::insert_const_mid_lenhalf` operation boundary
    - H30.3 closed without keeper: piece-cell code was reverted, and the first
      perf read was stale because release artifacts had not been rebuilt
    - H31 fixed measurement hygiene: runtime perf must follow
      `tools/perf/build_perf_release.sh`
    - H32 landed transaction façade thinning: `with_array_text_write_txn`
      disappeared from valid-release top asm, but wall stayed `Ny AOT 7 ms`
    - H33 closed: no hot `string_len_hi`; next card is observer-store
      short-byte leaf thinning
    - H34 kept: runtime-private short literal prefix check / short suffix byte
      write dropped whole instructions to `50229601` and observer-store
      closure to `14.03%`
    - H35 closed: residual owner is `memmove` / len-half closure, not
      observer-store; do not repeat H29 flat byte-copy surgery
    - H36 design gate closed: do not add non-flat residence yet; first split
      `ArrayTextCell` operation APIs
    - H36.1 landed: flat-only `ArrayTextCell` operation API split, no MIR or
      `.inc` change
    - H36.2 closed: fresh whole stat/asm still points at `memmove` / len-half
      closure, so non-flat residence remains justified
    - H36.3 landed: visible text materialization/comparison is explicit
    - H36.4 rejected: naive piece residence exploded work
      (`Ny AOT 114 ms`, `ny_aot_instr=2084599541`), code reverted
    - H37 closed: reverted-code whole is back to `Ny AOT 7 ms`; top remains
      `memmove` / len-half closure, allocator is not dominant
    - H38 closed: bounded mid-gap design is documented
    - H38.1 landed: bounded mid-gap moved `__memmove` to `0.23%` and whole
      Ny AOT to `6 ms`, but instruction count rose
    - H39 closed: len-half closure is lock-acquire dominated; observer-store
      closure is cell-loop / short-literal / MidGap segment dominated
    - H39.1 landed: MidGap generic prefix fast path improves whole cycles
      to `11.3M`
    - H39.2 closed: edit-only session is not enough because the outer loop
      interleaves a periodic observer-store region
    - H39.3 landed: MIR JSON now carries one combined edit-observer region
      proof for `kilo_kernel_small`
    - H39.4 landed: combined edit-observer proof now lowers to one
      runtime-private executor call; whole `kilo_kernel_small = C 82 ms /
      Ny AOT 5 ms`, `ny_aot_instr=49691801`, `ny_aot_cycles=9882715`
    - H39.5 closed: annotate pins the current owner inside combined executor
      mechanics; first follow-up is runtime-only pow2 index/period arithmetic
    - H39.5.1 landed: runtime-only pow2 index/period bitmask cleanup; whole
      `kilo_kernel_small = C 83 ms / Ny AOT 6 ms`, `ny_aot_instr=49271666`,
      `ny_aot_cycles=9282981`; result is cycles/memmove cleanup only, not a
      wall-time keeper
    - H39.5.2 landed: runtime-only MidGap unchecked slice helper cleanup;
      whole `kilo_kernel_small = C 84 ms / Ny AOT 5 ms`,
      `ny_aot_instr=42303268`, `ny_aot_cycles=8732285`
    - H39.5.3 landed: runtime-only 4-byte literal observer leaf; whole
      `kilo_kernel_small = C 85 ms / Ny AOT 5 ms`,
      `ny_aot_instr=35428450`, `ny_aot_cycles=6679916`
    - H39.5.4 closed: preserved-AOT top is combined executor closure
      `75.26%`, `__memmove_avx512_unaligned_erms` `10.03%`,
      `_int_malloc` `2.05%`; remaining sampled MidGap edit branch is a
      byte-boundary legality seam, not another runtime-only leaf
    - H40 closed: MIR owns `byte_boundary_proof=ascii_preserved_text_cell`;
      `.inc` consumes metadata only; runtime uses a const-specialized
      proof-specific leaf while preserving the checked no-proof path
    - H40.2 result: whole `kilo_kernel_small = C 82 ms / Ny AOT 6 ms`,
      `ny_aot_instr=34108663`, `ny_aot_cycles=6613012`; 200-run top remains
      combined executor closure `68.98%` and `__memmove_avx512_unaligned_erms`
      `17.89%`
    - H41 closed: post-byte-proof annotate kept the combined executor closure
      as top owner and pinned residual local samples in observer scan plus the
      existing short-suffix write leaf; broad copy remains external `memmove`
    - H42 rejected: prepared suffix append worsened whole instructions/cycles
      (`35553658` / `6944027`) and was reverted
    - H43 closed: H43.1 right-front suffix escape was rejected and reverted;
      whole instructions/cycles regressed to `34826664` / `7281528` and
      `memmove` share rose to `17.72%`
    - H44 closed: runtime-private observer all-hit guard is keeper; whole
      improves to `ny_aot_instr=24129815`, `ny_aot_cycles=5615809`
    - H45 closed: rerun whole stat/asm and saved bundle + dwarf callgraph pin
      the residual owner to one `ArrayTextCell` edit/materialization family
      inside the combined executor (`0x415d90`, `0x415e8f`, `0x416152`)
    - H46 active: broaden to text-cell residence/materialization design; do not
      reopen suffix/left-copy micro leaves as standalone probes
    - H46.1 bounded `MidGap + bridge` probe was rejected and reverted:
      `Ny AOT 22 ms`, `ny_aot_instr=142651499`, `ny_aot_cycles=90126830`,
      `__memmove 54.59%`, `_int_malloc 21.74%`; post-revert whole guard is back
      at `Ny AOT 5 ms`, `ny_aot_instr=24123290`, `ny_aot_cycles=6044833`
    - H21 is closed: MIR now owns the loopcarry len/store route; lowered loop body is one `nyash.array.string_insert_mid_subrange_len_store_hisi` call and no standalone `nyash.array.string_len_hi`
    - H20 is closed: pure meso substring concat len now folds to arithmetic, with no loop `substring_len_hii` / `substring_hii`
    - H20 result: `kilo_meso_substring_concat_len = C 3 ms / Ny AOT 3 ms`, `ny_aot_instr=1190204`
    - H19 is closed: whole `array.get -> indexOf` source liveness now treats same-slot const suffix store as a slot-capable consumer; row-scan `array.get_hi` materialization is gone
    - H19 result: `kilo_kernel_small_hk = C 82 ms / Ny AOT 28 ms`
    - H18 is closed: exact `kilo_micro_array_string_store` is `C 10 ms / Ny AOT 4 ms`, `ny_aot_instr=9270464`, `ny_aot_cycles=2343815`, and loop-carried text now stays in an SSA vector
    - keep array slot stores unchanged unless a separate MIR-owned no-escape / consumer proof is opened
    - H17 is closed: exact `kilo_micro_array_string_store` stays `C 10 ms / Ny AOT 5 ms`, `ny_aot_instr=10870861`, `ny_aot_cycles=9526782`, and the loop-body `text+16` terminator store is gone
  - active phase:
    - `docs/development/current/main/phases/phase-291x/README.md`
    - sibling guardrail: `docs/development/current/main/phases/phase-137x/README.md`
  - method anchor:
    - `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`
  - taskboard:
    - `docs/development/current/main/phases/phase-137x/137x-91-task-board.md`
- background compiler lanes:
  - `phase-29bq loop owner seam cleanup landing`
  - `phase-163x primitive-family / user-box fast-path landing`
- successor planning lane:
  - `phase-289x runtime-wide value/object boundary rollout`
  - status:
    - phase-0 authority/vocabulary lock is docs-only and complete
    - phase-137x string lane produced keeper `49c356339`
    - demand-backed cutover inventory `289x-96` is closed
    - `137x-B` design cleanout is closed; `137x-C` structure completion gate is closed
    - `137x-D` exact route-shape keeper is landed
    - `137x-E0`, `137x-E1`, and `137x-F` are closed; `137x-G` is rejected for now before the `137x-H` optimization return
    - array/map remain identity containers; only internal residence may become lane-hosted later
    - `publish` / `promote` stay boundary effects; `freeze.str` stays the only string birth sink
    - all `289x-96` clusters are done; their vocabulary now feeds the constrained `137x-F` implementation bridge
    - carrier responsibility lock is documented:
      - `BorrowedHandleBox` is boundary/cache, not semantic `Ref`
      - `KernelTextSlot` is transport adapter / sink seed, not long-term `TextCell`
      - `StringViewBox` is object-world view, not internal substring carrier
      - `TextCell` remains sink/residence only, not a corridor value
      - next explicit bridge is string-only `publish.text(reason, repr)`; `publish.any` stays deferred
      - borrow/provenance truth stays MIR/lowering-owned under `borrow.text_from_obj`
      - `.inc` emit paths must not rediscover read-side alias legality, provenance, or publication defer conditions
  - parent:
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
      - `137x-E1`: minimal `ArrayStorage::Text` / `TextLane` is landed
      - `137x-F`: runtime-wide Value Lane implementation bridge, constrained by 289x vocabulary
      - `137x-G`: allocator / arena pilot, only after copy/allocation tax remains structural
      - still deferred: string view/value carrier split beyond this gate, Map typed lane, heterogeneous / union slots
    - return-to-optimization gate:
      - phase-289x gate was closed by `289x-7h`
      - `137x-B` container / primitive design cleanout is closed
      - `137x-C` structure completion gate is closed by `137x-91-task-board.md`
      - `137x-D` exact route-shape keeper is landed
      - optimization resumes as `137x-H` after `137x-F` closeout and `137x-G` reject
- blocker:
  - `137x-H owner-first optimization return`; `137x-F1 demand-to-lane executor bridge` and `137x-F2 producer outcome manifest split` are landed, and `137x-G` allocator / arena is rejected for now
  - `137x-H` runtime cleanup: removed dead `ValueLaneAction::PublishBoundary`; array string store now selects `TextCellResidence` or `GenericBoxResidence` once and the executor path only consumes the preselected action
  - `137x-H` backend cleanup: `string_concat_emit_routes` now uses `kernel_plan_read_publication_boundary_window` for publication-boundary checks and no longer replays the corridor fallback in the insert-mid shared-receiver branch
  - `137x-H` backend cleanup: `match_piecewise_slot_hop_substring_consumer` now lives in shared concat emit helpers, so the policy owner no longer owns the slot-hop consumer replay
  - `137x-H` backend cleanup: removed unused `hako_llvmc_string_corridor_read_insert_mid_window_plan_values`; kernel-plan reader is now the single insert-mid window SSOT
  - `137x-H` backend cleanup: removed the standalone corridor triplet reader; `direct_kernel_entry` substring proof now goes through centralized `hako_llvmc_string_kernel_plan_read_concat_triplet_values` (kernel-first, corridor compat fallback)
  - first landed keeper:
    - same-array/same-index piecewise concat3 subrange store originally lowered to `nyash.array.string_insert_mid_subrange_store_hisiii`
    - current direct lowering uses explicit-length `nyash.array.string_insert_mid_subrange_store_hisiiii`
    - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 9 ms`
    - direct-only correctness: `Result: 2880064`, exit code `64`
  - no active `phase-289x` cutover blocker
- latest active keeper:
  - phase-137x branch-target-aware same-slot suffix store cut is green
  - exact shape:
    - `array.get -> indexOf("line") -> compare -> branch`
    - branch target uses the fetched string only as `copy -> const suffix -> Add -> same array.set(idx, value)`
  - lowered shape:
    - observer: `nyash.array.string_indexof_hih`
    - store: `nyash.array.kernel_slot_concat_his -> nyash.array.kernel_slot_store_hi`
    - no `nyash.array.slot_load_hi` call on that exact same-slot suffix path
  - perf proof:
    - `kilo_micro_array_string_store = C 9 ms / Ny AOT 3 ms`
    - `kilo_kernel_small = C 80 ms / Ny AOT 214 ms`
    - `kilo_kernel_small_hk = C 81 ms / Ny AOT 218 ms` (`repeat=3`, parity ok)
  - boundary:
    - this is a narrow string read/store keeper
    - `TextLane` and Value Lane are closed; allocator is rejected for now, so the next step is `137x-H`
- active follow-up structure card:
  - owner family:
    - `array_string_concat_const_suffix_by_index_store_same_slot_str`
    - `array_string_indexof_by_index_str`
    - `append_const_suffix_to_string_box_value`
  - purpose:
    - reduce same-slot exact-route copy/search tax without widening public ABI
  - current proof:
    - fresh owner proof is still reject-side after the exact route-shape keeper
    - whole-front asm still clusters around the same-slot exact-route helper family
  - boundary:
    - this is structure-only, not keeper proof
    - this helper-local follow-up is no longer the next task
    - `137x-E/F` are closed and `137x-G` is rejected for now; `137x-H` owns the next perf return
- current source-only get suppression + same-slot string store keeper:
  - compiler seam:
    - `array.get -> length -> substring/substring -> insert-mid set`
    - when later uses are proven source-only, the get result is kept as array text residence metadata and no object-handle get is emitted
  - fused insert-mid store seam:
    - same-slot insert-mid lowers to runtime-private `nyash.array.string_insert_mid_store_hisii(array_h, idx, middle_ptr, middle_len, split)`
    - raw `StringBox` residence is mutated in place
    - borrowed-handle residence is converted to an unpublished raw `StringBox` slot; the source stable handle is not mutated
  - fused suffix store seam:
    - branch same-slot const-suffix store lowers to runtime-private `nyash.array.string_suffix_store_hisi(array_h, idx, suffix_ptr, suffix_len)`
    - the branch path no longer allocates a `KernelTextSlot`
    - residence rule matches insert-mid: raw `StringBox` is mutated; borrowed alias is materialized into an unpublished raw `StringBox`
  - fused subrange store seam:
    - same-slot insert-mid subrange direct lowering now uses `nyash.array.string_insert_mid_subrange_store_hisiiii(array_h, idx, middle_ptr, middle_len, split, start, end)`
    - the older `hisiii` row remains as the pointer/CStr validated compatibility path
  - validation:
    - source-only fixture smoke now requires MIR metadata and no slot-load fallback:
      `phase137x_boundary_array_string_len_insert_mid_source_only_min.sh`
    - piecewise source-only fixture smoke now requires MIR metadata and no slot-load fallback:
      `phase137x_boundary_array_string_len_piecewise_concat3_source_only_min.sh`
    - live-after-get regression: `phase29ck_boundary_pure_array_string_len_live_after_get_min.sh`
  - perf/asm proof:
    - exact keeper: `kilo_micro_array_string_store = C 11 ms / Ny AOT 10 ms`, `ny_aot_instr=26922130`
    - function-level route proof: `exact_seed_backend_route result=hit reason=mir_route_metadata`
    - exact route proof: `array_string_store_micro result=emit reason=exact_match`
    - concat exact route proof: `concat_const_suffix_micro result=emit reason=exact_match`
    - substring views exact route proof: `substring_views_only_micro result=emit reason=exact_match`
    - substring concat exact route proof: `substring_concat_loop_ascii result=emit reason=exact_match`
    - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 9 ms`, `ny_aot_instr=127269397`
    - `kilo_kernel_small_hk = C 82 ms / Ny AOT 28 ms` (`repeat=3`, parity ok)
    - `ny_main` hot edit path is `array.string_len_hi -> array.string_insert_mid_store_hisii`
    - `ny_main` meso subrange path is `array.string_len_hi -> array.kernel_slot_insert_hisi -> string.kernel_slot_substring_hii_in_place -> array.kernel_slot_store_hi`
    - `ny_main` branch suffix path is `array.string_indexof_hih -> array.string_suffix_store_hisi`
    - insert-mid/suffix paths no longer emit `nyash.array.get_hi`, `nyash.array.kernel_slot_insert_hisi`, `nyash.array.kernel_slot_concat_his`, or `nyash.array.kernel_slot_store_hi`
    - `__strlen_evex` and `core::str::converts::from_utf8` are absent from the current whole asm hot report
    - meso subrange path no longer emits `nyash.array.slot_load_hi`, `nyash.string.substring_hii`, `nyash.string.substring_concat3_hhhii`, or `nyash.array.set_his`
  - boundary:
    - this landed cut was a narrow source-only window; `TextLane` / Value Lane / allocator now move only through `137x-E/F/G`
  - next owner proof seam:
    - whole-front asm clusters on `memchr::arch::x86_64::memchr::memchr_raw::find_avx2`, `array_string_concat_const_suffix_by_index_store_same_slot_str`, `__memmove_avx512_unaligned_erms`, `array_string_indexof_by_index_str`, `array_string_insert_const_mid_by_index_store_same_slot_str`, and `array_string_len_by_index`
    - whole-front asm still needs a fresh reread before choosing the next owner

## Snapshot

- keeper front stays closed:
  - `kilo_micro_substring_concat = C 2 ms / Ny AOT 3 ms`
  - `kilo_micro_substring_only = C 3 ms / Ny AOT 3 ms`
- exact `store.array.str` front is closed again:
  - latest direct reread: `kilo_micro_array_string_store = C 11 ms / Ny AOT 10 ms`, `ny_aot_instr=26922130`
  - reading:
    - the previous `C 10 ms / Ny AOT 144 ms` result was the generic-route fallback after the exact seed matcher missed the compact 8-block MIR shape
    - the exact seed emitter now matches again and emits stack-array IR for this micro
- current bridge front:
  - `kilo_meso_substring_concat_array_set_loopcarry`
  - shape: `substring + concat + array.set + loopcarry`
  - role: adopted middle between exact micro and whole kilo
  - rule: use it to validate store/publication cuts without the whole-front `indexOf("line")` row-scan noise
- current bridge reread after the explicit-length helper cut:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 9 ms`, `ny_aot_instr=127268967`
  - reading:
    - now below the prior `56-65 ms` band
    - public `array.get` / `substring_hii` / `substring_concat3_hhhii` / `array.set_his` are gone from the loop body on this bridge shape
- current whole accept gate:
  - `kilo_kernel_small`
  - current strict reread result: `kilo_kernel_small_hk = C 82 ms / Ny AOT 28 ms` (`repeat=3`, parity ok)
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
  - `Phase 3`: `TextLane` storage/residence implementation through `137x-E`
  - `Phase 4`: `137x-F` Value Lane bridge closed; `137x-G` allocator pilot deferred before `137x-H`
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
  - next seam had to preserve cheap alias encode on read; `owned-string keep` was not the keeper
  - follow-up card was read-side alias lane split:
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
      - the active whole/meso tax now points at read-side encode/materialize/objectize around `array.get`
      - stable objectization must stay cached/cold; do not reopen the rejected store-side `owned-string keep`
      - next implementation seam must preserve cheap alias encode before any new `TextLane` / MIR legality card
  - latest read-encode BoxShape cleanup:
    - `array.get` uses a scalar-checked borrowed-alias encoder after its local int/bool probes
    - this removes duplicate `as_i64_fast` / `as_bool_fast` probes before the borrowed-alias decision without changing the live-source / cached-handle / cold-fallback order
    - follow-on: borrowed-alias encode planning snapshots `drop_epoch` once and reuses it for cached-handle validation, keeping source and cache decisions on the same epoch view
    - validation passed:
      - targeted array/map borrowed-alias tests
      - `cargo check -q -p nyash_kernel`
      - `cargo test -q -p nyash_kernel --lib`
      - `tools/checks/dev_gate.sh quick`
    - perf reread is not keeper evidence:
      - exact remains closed: `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
      - meso remains open/noisy: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 65 ms`
      - strict whole is noisy: `kilo_kernel_small_hk = C 80 ms / Ny AOT 1740 ms` then rerun `C 80 ms / Ny AOT 808 ms`
    - next owner remains stable keep creation / first-read handle publication around the current borrowed-alias store-read chain
  - fresh owner proof after the read-encode cleanup:
    - exact remains closed: `kilo_micro_array_string_store = C 9 ms / Ny AOT 4 ms`
    - meso remains open/noisy: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 61 ms`
    - strict whole remains in the same band: `kilo_kernel_small_hk = C 80 ms / Ny AOT 812 ms` (`repeat=3`, parity ok)
    - asm/top:
      - libc copy/alloc dominates: `__memmove_avx512_unaligned_erms 25.02%`, `_int_malloc 9.58%`, `malloc 0.96%`
      - named repo owners still cluster around read/materialize/slot tax:
        - `array_get_index_encoded_i64::{closure} 4.39%`
        - `objectize_kernel_text_slot_stable_box 3.62%`
        - nested `array_get_index_encoded_i64` closure `2.09%`
        - `array_string_store_kernel_text_slot_at::{closure} 2.01%`
        - `TextKeepBacking::clone_stable_box_cold_fallback 0.49%`
    - reading:
      - this confirms the cleanup is not keeper evidence
      - do not reopen store-side `owned-string keep`
      - this proof alone did not open successor work; `TextLane`, Value Lane, and allocator now open through `137x-E/F/G`
  - latest runtime-private carrier vocabulary step landed:
    - `crates/nyash_kernel/src/plugin/value_codec/text_carrier.rs` now names `TextRef` and `OwnedText`
    - borrowed-alias read closures and slot text readers consume `TextRef` without changing the public ABI
    - selected export read helpers (`string_plan`, `string_search`, `charCodeAt`) now keep `TextRef` longer before the final `&str` projection
    - `KernelTextSlot` stays transport-only; slot `OwnedBytes` state naming remains physical transport detail
    - `TextCell` stays future-only; this card is structure-only, not a `TextLane` rollout
  - rejected follow-up probe after the fresh owner proof:
    - attempted unpublished `owned-text keep` for `KernelTextSlot -> existing BorrowedHandleBox` retarget without changing public ABI or `KernelTextSlot` layout
    - exact guard stayed closed: `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`
    - meso stayed noisy/open: `kilo_meso_substring_concat_array_set_loopcarry = C 4 ms / Ny AOT 62 ms`
    - strict whole regressed: `kilo_kernel_small_hk = C 84 ms / Ny AOT 902 ms`, rerun `C 82 ms / Ny AOT 892 ms`
    - asm/top removed `objectize_kernel_text_slot_stable_box`, but shifted cost into `__memmove_avx512_unaligned_erms 28.32%`, `_int_malloc 12.47%`, and `array_string_store_kernel_text_slot_at::{closure} 5.89%`
    - reading:
      - active whole still demands an object handle at `array.get_hi`
      - delaying stable birth from store to read only moved publication/copy tax
      - code was reverted; do not reopen store-side `owned-string keep` or `owned-text keep` without a front that no longer demands object handles on read
  - rejected follow-up probe: array-slot concat-by-index helper
    - attempted runtime-private `nyash.array.kernel_slot_concat_his(slot, array_h, idx, suffix)` for the hot `array.get_hi -> const_suffix concat -> kernel_slot_store_hi` store
    - exact guard stayed closed: `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`
    - meso stayed noisy/open: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 62 ms`
    - strict whole regressed: first `kilo_kernel_small_hk = C 82 ms / Ny AOT 1571 ms`, rerun `C 80 ms / Ny AOT 1033 ms`
    - IR proof:
      - `nyash.array.kernel_slot_concat_his` was emitted at the hot concat store
      - the preceding `nyash.array.slot_load_hi` still remained before `nyash.array.string_indexof_hih`
    - reading:
      - a direct concat helper is not enough while the producer `array.get_hi` remains live
      - code was reverted; do not retry this helper-local seam before the `137x-E` storage gate
  - reading:
    - phase 2.5 runtime contract is now fixed more tightly than the first `array.get`-only slice
    - exact stays closed, but meso / strict whole reopened upward versus the prior keeper-candidate band
    - cleanup queue is parked; the strict reread remains reject-side rather than a keeper
    - next step is the `137x-E` storage gate before returning to any new kilo owner proof
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

1. open implementation gates before the next kilo optimization
   - current blocker: `137x-H owner-first optimization return`
   - `137x-E0`: MIR / backend seam closeout is closed
   - `137x-E1`: minimal `TextLane` / `ArrayStorage::Text` is landed
   - `137x-F`: runtime-wide Value Lane implementation bridge is closed
   - `137x-G`: allocator / arena pilot is rejected / not opened by F closeout
   - `137x-H`: next kilo optimization return
2. keep closed gates immutable
   - `137x-A`: string publication contract closeout is closed
   - `137x-B`: container / primitive design cleanout is closed
   - `137x-C`: structure completion before perf return is closed
   - `137x-D`: exact array store route-shape keeper is landed
3. keep real blockers explicit
   - `publish.any` remains blocked
   - typed map lane remains blocked
   - heterogeneous / union array slot layout remains blocked
   - public ABI widening remains blocked
4. implement `137x-F` from the gate SSOT
   - SSOT: `docs/development/current/main/phases/phase-137x/137x-94-textlane-value-allocator-implementation-gate.md`
   - start from landed array string storage/residence evidence
   - keep `String = value` and public Array / String ABI unchanged
5. preserve `137x-D` evidence as baseline
   - closed card: `137x-D exact array store route-shape proof`
   - exact: `kilo_micro_array_string_store = C 10 ms / Ny AOT 10 ms`, `ny_aot_instr=26922384`
   - middle: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 9 ms`, `ny_aot_instr=129614388`
   - strict whole: `kilo_kernel_small_hk = C 84 ms / Ny AOT 26 ms`, parity ok

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-137x/README.md`
3. `docs/development/current/main/phases/phase-137x/137x-94-textlane-value-allocator-implementation-gate.md`
4. `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
5. `docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md`
6. `docs/development/current/main/design/string-value-model-phased-rollout-ssot.md`
6. `docs/development/current/main/phases/phase-137x/phase137x-text-lane-rollout-checklist.md`
7. `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
8. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
9. `docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md`
10. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
11. `docs/development/current/main/design/string-birth-sink-ssot.md`
12. `docs/development/current/main/15-Workstream-Map.md`
13. `docs/development/current/main/phases/phase-289x/289x-90-runtime-value-object-design-brief.md`

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
cargo test -p nyash_kernel --lib --tests --no-run
```
