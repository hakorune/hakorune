---
Status: Active
Date: 2026-05-31
Scope: active mimalloc migration and optimization workstream.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-415-MIMALLOC-SOURCE-LEVEL-OWNER-SELECTION.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
---

# Mimalloc Current Workstream

## Goal

Continue mimalloc migration and optimization without reopening Array/helper
fast-path work unless current perf evidence selects it.

Current owner surface:

```text
object_lifecycle_facade
```

## Stop Line

- no new numbered row for inventory-only work
- no row-specific `.sh` guard
- no new Array / RuntimeDataBox / helper fast path without current mimalloc
  perf evidence and positive-net implementation path
- no provider activation
- no allocator replacement
- no hook installation
- no `#[global_allocator]`
- no winner claim

## Checklist

Each task is intended to be small enough for one focused pass. Do not create a
new row for these tasks; update this checklist or use a Ghost Task commit
message.

### Observation

- [x] MIM-001: source-shape inventory for
  `lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako`
  - output: short table of facade methods, field/capsule/page interactions,
    and likely owner candidates
  - no code changes
- [x] MIM-002: smallest owner selection inside `object_lifecycle_facade`
  - output: one selected source-level candidate or explicit no-owner result
  - no fast-path reopen
- [x] MIM-003: perf evidence refresh before source edits
  - output: owner-family evidence and reject/keep reason
  - no source edit before evidence

### Candidate

- [x] MIM-004: implementation boundary selection
  - output: one file/function boundary, or park with reason
  - allowed scope: `.hako` mimalloc source/state shape only
- [x] MIM-005: narrow implementation
  - output: code change only within the selected boundary
  - no Array / RuntimeDataBox / helper fast-path work
- [x] MIM-006: smoke and quick gate
  - output: existing lane guard / dev gate result
  - no new row-specific `.sh`

### Decision

- [x] MIM-007: keeper / nonkeeper decision only if it affects future work
  - output: Workstream Decision Log entry or durable row only when required by
    policy
- [x] MIM-008: SSOT direct edit only if design truth changes
  - output: owning `design/*.md` change with reason in commit message
- [x] MIM-009: cleanup / Ghost Task commit
  - output: commit message records small refactors, guard wording, or pointer
    fixes; no `CURRENT_STATE.toml` progress log
- [x] MIM-010: page selection delegation cleanup
  - output: `objectLifecycleSmallAlloc` delegates selection to
    `queue.selectPage()` and keeps the owner surface inside the existing
    page-queue seam
  - no fast-path reopen
- [x] MIM-011: selected page acquire route cleanup
  - output: `objectLifecycleSmallAlloc` calls `page.acquireFreshSmall(size)`
    after `queue.selectPage()` has selected an available page
  - no new page helper, Array lane, RuntimeDataBox lane, or direct-path reopen
- [x] MIM-012: alloc result reset-attempt capsule cleanup
  - output: `HakoAllocObjectLifecycleAllocResult.resetAttempt()` owns the
    reset-plus-attempt state transition used by `objectLifecycleSmallAlloc`
  - no new helper lane, Array lane, RuntimeDataBox lane, or direct-path reopen
- [x] MIM-013: defer alloc result block publication until acquire success
  - output: `objectLifecycleSmallAlloc` keeps the failure path on the
    `resetAttempt()` sentinel and publishes `last_block_id` only after a
    successful block acquisition
  - no new helper lane, Array lane, RuntimeDataBox lane, or direct-path reopen
- [x] MIM-014: current C gap and perf owner refresh
  - output: compare current hako object-lifecycle small-block EXE with the
    explicit C mimalloc runner at the same 524288 alloc/free count
  - no implementation; use this to choose the next owner before more source
    edits
- [x] MIM-015: direct-front A/B measurement
  - output: compare default/safe measurement with the intended DirectSlot /
    DirectArray exact front before choosing the next implementation owner
  - no implementation; this corrects the active measurement front
- [x] MIM-016: direct exact baseline lock
  - output: treat `direct_slot_exact` + `direct_array_i64_exact` as the
    mimalloc parity optimization baseline; keep default/safe as compatibility
    reference only
  - no implementation
- [x] MIM-017: DirectArray i64 exact store boundary cut
  - output: remove the selected `array_runtime_set_idx_i64` call boundary for
    active direct exact `HakoAllocPageModel` i64 ArraySet store regions
  - do not widen generic ArraySet, public ArrayBox ABI, mixed storage, plugin
    typed ABI, or default/safe behavior
- [x] MIM-018: post-store-boundary perf owner refresh
  - output: reread the direct exact perf top after `nyash.array.set_hii` calls
    are removed from the object-lifecycle small-block EXE
  - choose the next owner from current evidence; do not reopen Array helper
    micro-lanes without positive direct-exact evidence
- [x] MIM-019: inline single-page queue selection
  - output: keep `selectPage()` as the public queue entry while folding the
    `page_count == 1` path into that method
  - no generic queue rewrite, no Array helper lane, no provider/replacement
    activation
- [x] MIM-020: post-single-page-inline owner refresh
  - output: reread the direct exact perf top and choose one next source-level
    owner from current evidence
  - result: `object_lifecycle_facade` remains the source-level owner surface;
    no new fast path reopened
- [x] MIM-021: inline queue selection reset
  - output: fold `beginSelection()` into `selectPage()` so the selected queue
    entry owns its hot reset/write path
  - no generic queue rewrite, no public behavior change
- [x] MIM-022: required inline receiver-leaf parity proof
  - output: restore the `beginSelection()` call in source and use
    `@rune Inline(required)` to reach the MIM-021 manual-inline instruction
    shape through a verified single-object field-set leaf inline
  - no source `Contract(no_alloc)` / `Contract(no_safepoint)` requirement for
    this narrow leaf shape; verifier infers those facts from the body
  - no silent fallback, no source hand-expansion as the final shape
- [x] MIM-023: composite hot-cluster source inventory
  - output: short source-shape note for `objectLifecycleSmallAlloc` and
    `objectLifecycleReleaseBlock`
  - keep the lane source-level only; no new helper, Array, or direct-path
    reopening
- [x] MIM-024: `StateRepr::Direct` candidate inventory
  - output: table of primitive field groups reached by the MIM-023 hot
    clusters
  - classify each group as `all_primitive`, `mixed_handle`, `public_observer`,
    or `escape_unknown`
  - no source syntax, no new `record`/`slots`/`layout` surface
- [x] MIM-025: direct-state feasibility report
  - output: one selected candidate or explicit no-candidate result
  - required report keys: `state_repr=direct_v0`, `field_decl_authority=1`,
    `selected_field_count`, `unsupported_field_count`,
    `materialization_boundary_known`, `positive_net_expected`
  - stop if no positive-net candidate is found
- [x] MIM-026: primitive core source-shape plan
  - output: decide whether a small primitive-only core box is needed, or
    whether an existing field group is enough
  - use only existing `.hako` box fields, `DirectArrayI64`, and
    `@rune Inline(required)`
  - no public facade semantics change
- [x] MIM-027: MIR metadata-only `StateRepr::Direct` producer
  - output: derive direct-state candidate metadata from `field_decls` and
    storage-class facts
  - no backend lowering, no runtime layout change, no helper ABI change
- [x] MIM-028: selected direct-state lowering guard surface
  - output: one method/field group with proven same-object receiver,
    known materialization boundary, and positive helper/call reduction
  - selected-plan silent fallback is a row failure
- [x] MIM-029: narrow direct-state lowering implementation
  - output: direct offset load/store only for the MIM-028 selected group
  - no generic user-box flattening, no public ABI widening, no provider or
    allocator replacement activation
- [x] MIM-030: post-direct-state owner refresh
  - output: compare direct exact front against the previous baseline and
    choose the next owner from current evidence
  - this is measurement / selection, not the fast-path implementation itself;
    direct-state fast-path calculation belongs to MIM-029 after MIM-028
    selects one proven group
  - return to source-level mimalloc work if direct-state does not produce a
    structural keeper
- [x] MIM-031: direct-state special-case residue audit
  - output: list every hardcoded MIM-029 selector, field slot, and storage
    assumption in the same-module body emitter
  - classify the path as `delete`, `keep_as_selected_pilot`, or
    `promote_to_substrate`
  - no implementation and no new fast path
- [x] MIM-032: direct-state same-family candidate inventory
  - output: check whether the same result-capsule family has another
    measured positive-net candidate
  - candidates are limited to all-primitive, typed-field-decl backed methods
    with known materialization boundaries
  - no generic user-box flattening and no source syntax change
- [x] MIM-032a: required-inline user-method consumer
  - output: let verified `@rune Inline(required)` leaf methods inline from
    known user-defined `Callee::Method` calls, not only `Callee::Global`
  - no new source annotation, no Profile, no direct-state emitter branch
  - semantic smoke must keep the representative small-block workload green
- [x] MIM-033: DirectState method-lowering substrate selection
  - output: select whether to extract MIM-029 into a fact-driven
    DirectStateMethodLowering substrate
  - extraction requires either two same-family keepers or fresh MIM-030/MIM-032
    evidence that the selected call boundary remains a current owner
  - if not selected, keep the MIM-029 path temporary and proceed to MIM-034
- [x] MIM-034: selected direct-state pilot closeout
  - output: retire, promote, or explicitly extend the MIM-029 selected pilot
  - retire if MIM-030 shows nonkeeper/no-current-owner evidence
  - promote only through typed `direct_state_plans` / field-decl facts, not by
    adding more by-name callsite branches
  - selected pilot may not remain open-ended after this item
- [x] MIM-035: required-inline fieldget leaf extension and resetAttempt
  - output: extend verified `@rune Inline(required)` leaf inline to
    same-receiver FieldGet/FieldSet increment leaves
  - annotate `HakoAllocObjectLifecycleAllocResult.resetAttempt/0`
  - no Profile, no direct-state emitter branch, no source hand-expansion
- [x] MIM-036: post-resetAttempt owner refresh
  - output: reread direct exact perf owner and select next source/MIR owner
  - current likely candidates are `selectPage/0`, `Main.runOne/2`,
    `objectLifecycleSmallAlloc/1`, `resetToFresh/0`, and
    `objectLifecycleReleaseBlock/2`
  - do not reopen DirectState or Array fast paths without new owner evidence
- [x] MIM-037: release-side structured inline feasibility
  - output: decide whether the hot `objectLifecycleReleaseDirectCachedPage/2`
    boundary should be handled by a structured required-inline extension or
    parked as source-level release-shape work
  - do not hand-expand the helper in `.hako` as the final shape
  - do not reopen DirectState, Array, RuntimeDataBox, or helper micro-lanes
- [x] MIM-038: DirectArray i64 exact get boundary cut
  - output: remove the current direct exact `array_runtime_get_idx` call
    boundary only where current mimalloc perf evidence selects it
  - keep public ArrayBox, mixed storage, plugin typed ABI, and default/safe
    behavior unchanged
  - no generic Array rewrite and no helper micro-lane
- [x] MIM-039: DirectArray i64 exact known-live store boundary cut
  - output: reuse the existing unchecked DirectArray i64 store lane for
    `HakoAllocPageModel.releaseLocalKnownLive/1`
  - reason: caller has already proven the block is live; the method is the
    direct exact known-live release path and does not need public ArrayBox
    bounds/capacity semantics on its two internal i64 stores
  - keep default/safe behavior, public ArrayBox, mixed storage, plugin typed
    ABI, and source syntax unchanged
- [x] MIM-040: known-live release result branch cleanup
  - output: remove the redundant `release_ok != 1` branch after
    `page.releaseLocalKnownLive(block_id)` in the cached-page release path
  - reason: the known-live release method currently returns `1` on all
    non-trap paths; failure remains represented by the cached-page guard before
    the call
  - no new compiler feature, no source hand-expansion, no Array lane widening
- [x] MIM-041: acquireFreshSmall direct-array invariant lowering
  - output: use unchecked DirectArrayI64 load/store lowering for the selected
    `HakoAllocPageModel.acquireFreshSmall/1` exact path
  - reason: the method checks `free_top > 0` before reading `free[free_top-1]`,
    and the returned block id is a page-owned block slot for `block_used`
  - result: direct exact instructions improved from `174380714` to `152851912`;
    `body_elapsed_ns` moved from `6000000` to `5000000`
  - keep default/safe behavior, public ArrayBox, mixed storage, plugin typed
    ABI, and source syntax unchanged
- [x] MIM-042: small-alloc selected page local reuse
  - output: use the `queue.selectPage()` return value and a local
    `selected_page_id` inside `objectLifecycleSmallAlloc/1`
  - reason: avoid rereading selected page/page-id fields already produced by
    the queue selection boundary
  - result: direct exact instructions improved from `152851912` to `151803609`
  - no source hand-expansion, no new compiler feature, no Array lane widening
- [x] MIM-043: DirectArray receiver-fact selection cleanup
  - output: remove the `resetToFresh/0` by-name unchecked DirectArray store
    selector and replace checked DirectArray get/store selection with
    receiver-origin facts
  - reason: `resetToFresh/0` should not be a permanent C-shim special case;
    direct checked Array lowering is valid for proven Array birth/direct-array
    receivers independent of method name
  - result: representative direct exact EXE stayed green and instruction count
    remained stable at `151803725`
  - no source fast-path branch, no public ArrayBox/default-safe behavior change,
    no new row/guard/script
- [x] MIM-044: nested route-result phi receiver acceptance
  - output: preserve user-box route-result box origins through nested phi
    receivers so guard-return source shapes do not collapse downstream
    same-module method routes
  - reason: the `resetToFresh/0` early guard made the selected page flow pass
    through nested phis before `page.acquireFreshSmall(size)`; this is a
    generic origin-flow acceptance issue, not a `resetToFresh` or mimalloc
    by-name special case
  - result: `Main.runOne/2 -> objectLifecycleSmallAlloc/1` returned to
    `direct_function_call`; representative direct exact instructions improved
    from `151803725` to `147028291`
  - no source syntax change, no Profile, no by-name shim selector, no
    public ArrayBox/default-safe behavior change, no new row/guard/script
- [x] MIM-045: same-block copy-forwarding probe
  - output: tested a narrow MIR same-block copy forwarding pass against the
    representative direct exact front
  - structural observation: local copy counts dropped in hot page methods, but
    the pass interfered with same-module lowering metadata / route carriers
    and stopped the EXE build with `unsupported pure shape`
  - decision: nonkeeper for the current pipeline; do not add a global
    copy-delete/forward pass until route metadata is refreshed or rewritten as
    value-attached facts
  - next: return to owner-first perf selection rather than expanding this into
    a metadata-aware rewrite pass inside the mimalloc lane
- [x] MIM-046: small-alloc selected-kind fast branch
  - output: reorder `objectLifecycleSmallAlloc/1` so the current direct exact
    fresh-page kind (`selected_kind == 2`) is the first accepted branch, while
    the reuse path (`selected_kind == 1`) and bad-kind failure remain intact
  - reason: representative direct exact evidence shows the fresh-page path is
    the active small-alloc path; avoid sending it through the reuse-kind check
    first
  - result: direct exact instructions improved from `139295542` to
    `137198567`; `body_elapsed_ns` stayed at `4000000`
  - no source hand-expansion, no compiler feature, no Array lane widening, no
    public ArrayBox/default-safe behavior change
- [x] MIM-047: release result local reuse
  - output: keep `release_result` in a local inside
    `objectLifecycleReleaseBlock/2` and use it for the hot known-page success
    publication
  - reason: the hot release facade already resets and records the same capsule
    at method entry; rereading it through the facade after
    `releaseLocalKnownLive` is unnecessary source traffic
  - result: direct exact instructions improved from `137198567` to
    `134052875`; next owner refresh still selects
    `objectLifecycleReleaseBlock/2` first, followed by `acquireFreshSmall/1`,
    `Main.runOne/2`, and `selectPage/0`
  - no source hand-expansion, no compiler feature, no Array lane widening, no
    public ArrayBox/default-safe behavior change
- [x] MIM-048: result failure owns `last_ok=0`
  - output: make object-lifecycle result `recordFailure()` paths clear
    `last_ok`, then remove the release-path entry `reset()` before
    `recordRequest(page_id, block_id)`
  - reason: success and failure now each own the final ok state; the entry
    reset was writing fields that the hot request/success path immediately
    overwrote
  - result: direct exact instructions improved from `134052875` to
    `133004248`; `body_elapsed_ns` measured `5000000` in the single smoke
  - no source hand-expansion, no compiler feature, no Array lane widening, no
    public ArrayBox/default-safe behavior change
- [x] MIM-049: release request write delayed to failure path
  - output: remove hot-path `release_result.recordRequest(page_id, block_id)`
    from `objectLifecycleReleaseBlock/2`; failure paths now use
    `recordFailureRequest(page_id, block_id, reason)` to preserve observer
    state
  - reason: hot success already writes the same page/block through
    `recordSuccess(page_id, block_id)`, so entry request writes were redundant
    on the measured direct exact path
  - result: direct exact instructions improved from `133004248` to
    `131955824`; `body_elapsed_ns` measured `4000000` in the single smoke
  - no source hand-expansion, no compiler feature, no Array lane widening, no
    public ArrayBox/default-safe behavior change
- [x] MIM-050: alloc attempt write delayed to failure shape
  - output: replace hot-path `alloc_result.resetAttempt()` with
    `alloc_result.recordAttempt()` in `objectLifecycleSmallAlloc/1`; early
    failures now publish sentinel state through `recordFailureNoSelection()`,
    and selected-page failures use `recordFailureAfterSelectedPage()`
  - reason: hot success overwrites page/block/reason/ok observer state, so the
    entry reset was redundant on the measured direct exact path; failure
    methods preserve the previous sentinel/selected-page observer semantics
  - result: direct exact instructions improved from `131955824` to
    `129858558`; `body_elapsed_ns` measured `5000000` in the single smoke
  - no source hand-expansion, no compiler feature, no Array lane widening, no
    public ArrayBox/default-safe behavior change
- [x] MIM-051: page selection miss state delayed to failure shape
  - output: remove hot-path `beginSelection()` from `selectPage/0`; failed
    selection paths now publish sentinel state and increment `miss_count`
    through `recordSelectionMiss()`
  - reason: the representative direct exact path always selects the single
    active page and overwrites the selected fields, so entry reset writes were
    redundant on the hot path
  - result: direct exact instructions improved from `129858558` to
    `127761175`; `body_elapsed_ns` measured `4000000` in the single smoke
  - no source hand-expansion, no compiler feature, no Array lane widening, no
    public ArrayBox/default-safe behavior change
- [x] MIM-052: release known-page handle write removed from hot success
  - output: remove `release_known_page = cached_page` from the first cached
    known-page release success path; the fallback path still owns
    `release_known_page` when it needs a temporary page carrier
  - reason: the first cached-page success returns immediately after
    `releaseLocalKnownLive()` and result publication, and `release_known_page`
    has no public observer
  - result: direct exact instructions improved from `127761175` to
    `127237309`; `body_elapsed_ns` measured `4000000` in the single smoke
  - no source hand-expansion, no compiler feature, no Array lane widening, no
    public ArrayBox/default-safe behavior change
- [x] MIM-053: selected-page handle mirror removed from selection success
  - output: keep selected `index`, `page_id`, and `kind` publication, but stop
    writing the internal `last_selected_page` handle mirror on `selectPage/0`
    success paths
  - reason: `selectPage()` already returns the selected page object, and the
    public observer surface exposes selected kind/page-id/index rather than the
    handle mirror
  - result: direct exact instructions improved from `127237309` to
    `126713126`; `body_elapsed_ns` measured `4000000` in the single smoke
  - no source hand-expansion, no compiler feature, no Array lane widening, no
    public ArrayBox/default-safe behavior change
- [x] MIM-054: cached release page-id recheck removed from hot success
  - output: trust the `last_alloc_page_id` + `last_alloc_page` identity cache
    on the first cached known-page release success path
  - reason: `recordLastAllocPage()` records the page id together with the page
    handle after allocation, and `HakoAllocPageModel.page_id` is fixed by
    `birth()`
  - result: direct exact instructions improved from `126712645` to
    `124615619`; `body_elapsed_ns` measured `4000000` in the representative
    smoke
  - no source hand-expansion, no compiler feature, no Array lane widening, no
    public ArrayBox/default-safe behavior change; fallback release still
    revalidates the cached page id
- [x] MIM-055: post-direct-memory owner refresh
  - output: reread the direct exact perf owner after the direct-memory substrate
    wave (`DirectArray` / `Span` facts and required FastPath diagnostics)
  - start from current measured evidence before source edits
  - no new DirectArray / Span / `direct {}` / helper lane unless perf selects
    it and the expected net is positive
  - decide whether the next owner is mimalloc source shape, direct-state,
    DirectArray/Span diagnostic use, or no-current-owner
- [x] MIM-056: single-active-page small-alloc selection route
  - output: add a narrow queue-side `trySelectSingleActivePage()` route used by
    `objectLifecycleSmallAlloc/1` before falling back to public `selectPage()`
  - reason: post-MIM-055 perf selected the single-page active success branch
    inside `selectPage/0`; the hot representative path does not need the full
    reusable/multipage selection body
  - preserve public `selectPage()` semantics and failure/miss accounting by
    falling back to `selectPage()` when the narrow active-page route does not
    match
  - no new compiler feature, no `direct {}` syntax, no Array/Span/helper lane
    widening, no provider/replacement activation
- [x] MIM-057: post-single-active-page owner refresh
  - output: reread the direct exact perf top after MIM-056 before source edits
  - result: repeated direct exact perf selected `Main.runOne/2` as harness
    surface, then `objectLifecycleReleaseBlock/2`, `acquireFreshSmall/1`,
    `objectLifecycleSmallAlloc/1`, `releaseLocalKnownLive/1`, and
    `trySelectSingleActivePage/0`
  - decision: ignore the benchmark harness owner for source cleanup and keep
    the next implementation on the top hako_alloc owner surface
  - no new compiler feature, no Array/Span/helper lane widening
- [x] MIM-058: cached release page-index guard removal
  - output: remove the hot cached release success check on
    `last_alloc_page_index >= 0` in `objectLifecycleReleaseBlock/2`
  - reason: `recordLastAllocPage(index, page_id, page)` records the page id,
    index, and handle together after allocation; the hot cached success path
    only needs the fixed page id and non-null page handle, while the fallback
    path still owns index validation when it is needed
  - result: representative direct exact EXE stayed green, `mimalloc_lite_exe`
    and `allocator_stress_exe` stayed green, quick gate stayed green, and the
    one-sample direct exact perf stat measured `115056193` instructions with
    `body_elapsed_ns=3000000`
  - no source hand-expansion, no compiler feature, no Array lane widening, no
    public ArrayBox/default-safe behavior change
- [x] MIM-059: PageQueue receiver-local selection publication cleanup
  - output: extract repeated active/reuse selection publication writes inside
    `HakoAllocObjectLifecyclePageQueue` into receiver-local
    `@rune Inline(required)` helpers
  - reason: the previous helper-extraction probe crashed when the helper call
    remained as a same-module EXE call, while mixed-base required inline was
    rejected; the accepted shape keeps the helper body receiver-local and
    verified-inline only
  - result: representative direct exact EXE, `mimalloc_lite_exe`,
    `allocator_stress_exe`, `current_state_pointer_guard.sh`, and
    `dev_gate.sh quick` stayed green
  - no source hand-expansion, no non-inline helper call, no compiler feature,
    no Array lane widening, no public ArrayBox/default-safe behavior change
- [x] MIM-060: direct-exact runtime mode fail-fast contract
  - output: audit and implement a fail-fast guard for direct-exact EXEs that
    are run without the matching runtime backend modes
  - reason: direct-slot/direct-array lowering may emit pointer/direct-buffer
    assumptions that are invalid when runtime `HAKO_TYPED_OBJECT_STORE` or
    `HAKO_ARRAY_SLOT_STORE` falls back to safe/default storage
  - result: direct-exact generated `ny_main` now calls
    `nyash.runtime.require_backend_modes_i(flags)` when compile-time direct
    env selects direct-slot and/or direct-array lowering; missing runtime env
    exits with a `[freeze:contract][direct-exact/runtime-mode]` diagnostic
    instead of reaching the pointer/handle mismatch
  - no silent env forcing, no fallback to safe runtime storage after direct
    lowering was selected
- [x] MIM-061: post-runtime-mode-guard smoke and owner refresh
  - output: rerun the direct exact representative smoke and selected app
    smokes after MIM-060, then choose the next mimalloc owner from current
    evidence
  - result: representative direct exact EXE stayed green with
    `body_elapsed_ns=4000000`; `mimalloc_lite_exe`,
    `allocator_stress_exe`, and the mixed-base helper proof stayed green
  - runtime guard check: representative direct exact build/run used
    `HAKO_TYPED_OBJECT_STORE=direct_slot_exact` and
    `HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact`
  - owner refresh: `perf report` selected
    `HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1` at 24.05%,
    followed by `HakoAllocPageModel.acquireFreshSmall/1` at 20.23%,
    `Main.runOne/2` at 17.06%,
    `HakoAllocPageModel.releaseLocalKnownLive/1` at 13.79%, and
    `HakoAllocObjectLifecycleReleaseBlock/2` at 12.59%
  - residue: lowered IR still has 3 executable
    `nyash.array.slot_load_hi` calls
  - selected next: MIM-062 direct-array load residue inventory; this is an
    inventory of an existing DirectArray route surface, not a new fast-path
    lane, new syntax, or mixed-base inline widening

- [x] MIM-062: direct-array load residue inventory
  - output: classify the remaining executable `nyash.array.slot_load_hi` calls
    in the direct exact representative EXE by method, receiver origin, index
    facts, extent facts, and CFG safety
  - implemented tool:
    `tools/allocator/direct_array_load_residue_inventory.py`
  - target calls observed after MIM-061:
    `HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2`,
    `HakoAllocObjectLifecyclePageQueue.selectPage/0`, and
    `HakoAllocPageModel.reactivate/0`
  - result:
    `slot_load_hi_executable_call_count=3`;
    `objectLifecycleReleaseBlock/2` is missing `RangeIndexFact`;
    `selectPage/0` and `reactivate/0` have `RangeIndexFact` but are missing
    `DirectArrayExtentFact` / `RegionStabilityFact`
  - selected next owner:
    `direct_array_load_fact_gap_split`
  - stop line: inventory only unless positive route evidence is found; no
    source hand-expansion, no generic Array rewrite, no `direct {}` syntax, and
    no mixed-base inline widening

- [x] MIM-063: direct-array load fact gap selection
  - output: choose exactly one of the two MIM-062 buckets for implementation:
    either the `objectLifecycleReleaseBlock/2` range-index producer gap or the
    `selectPage/0` / `reactivate/0` extent-stability fact gap
  - decision: park both buckets for now
  - reason: the remaining `nyash.array.slot_load_hi` calls are concrete IR
    residue, but the current direct exact perf top does not select those
    residual load sites as the next sampled owner; widening RangeIndexFact or
    extent/stability facts here would be fact plumbing without current
    positive owner evidence
  - selected next owner: hot sampled hako_alloc body shape inventory, starting
    from `objectLifecycleSmallAlloc/1`, `acquireFreshSmall/1`, and
    `objectLifecycleReleaseBlock/2`
  - stop line: do not implement DirectArray load fact widening until a later
    perf refresh selects one of the parked buckets as a sampled owner

- [x] MIM-064: hot sampled body shape inventory
  - output: classify the sampled blocks inside the current direct exact top
    hako_alloc methods before source edits
  - target methods:
    `HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1`,
    `HakoAllocPageModel.acquireFreshSmall/1`, and
    `HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2`
  - required output: small table of sampled block shape, source owner, likely
    structural seam, and reject/keep reason
  - no DirectArray/Span/direct-block/mixed-base-inline widening
  - no source edit before the owner table is written
  - inventory:
    - `objectLifecycleSmallAlloc/1`: top annotated block sampled the
      single-active fresh path; the source still joined that known `kind=2`
      route with the fallback `selectPage()` route before checking
      `selected_kind`
    - `acquireFreshSmall/1`: DirectArray access is already direct; samples sit
      on page-local counter / peak-used observer updates, not on a missing
      array helper route
    - `objectLifecycleReleaseBlock/2`: hot cached success path is clean of the
      residual `pages.get(known_index)` helper; the residual helper belongs to
      fallback page lookup and stays parked
    - `releaseLocalKnownLive/1`: sampled work is direct page-local state update
      and last-release retirement branch; no new array fact is selected
    - `trySelectSingleActivePage/0`: sampled work is selection publication and
      counters; useful context for the small-alloc split, not a new helper lane
  - selected next owner:
    `single_active_small_alloc_fresh_path_split`
  - reason: the selected source seam removes a sampled known-kind branch
    without adding syntax, compiler features, DirectArray/Span widening, or
    mixed-base inline support

- [x] MIM-065: single-active small-alloc fresh-path split
  - output: split the `trySelectSingleActivePage()` success route in
    `objectLifecycleSmallAlloc/1` before the fallback `selectPage()` route
  - reason: when the single-active route returns a page, the selected index is
    `0` and selected kind is `2`; sending that path through the generic
    selected-kind fallback branch kept a hot sampled branch in the direct exact
    body
  - result: representative direct exact EXE stayed green with
    `body_elapsed_ns=4000000` in the runner smoke and `body_elapsed_ns=3000000`
    under the follow-up perf-stat run; the direct exact perf-stat run measured
    `instructions=111384596` and `cycles=21025771`
  - smokes: `mimalloc_lite_exe` and `allocator_stress_exe` stayed green
  - no compiler feature, no source hand-expansion of helper bodies, no
    DirectArray/Span/direct-block/mixed-base-inline widening, and no
    public ArrayBox/default-safe behavior change

- [x] MIM-066: post-single-active-fresh-split owner refresh
  - output: reread the direct exact perf owner after MIM-065 and choose the
    next source/MIR owner from current evidence
  - include the remaining `slot_load_hi` residue only if the new perf evidence
    samples it as a current owner
  - result: current evidence kept the owner on sampled `.hako` source shape,
    not on parked DirectArray load residue or a new language surface
  - source-shape inventory selected alloc-result active success capsule fusion
    as the first low-risk probe, with queue publication and known-available
    acquire parked behind it
  - no source edit was kept before the refresh decision

- [x] MIM-067: alloc-result active success capsule fusion probe
  - output: test whether the single-active fresh success path can replace
    `recordSelectedPage()` + `recordBlock()` + `recordSuccess(2)` with one
    alloc-result capsule method
  - attempted shape: `recordActiveSuccess(page_id, block_id)` on
    `HakoAllocObjectLifecycleAllocResult`
  - verifier result: `@rune Inline(required)` failed correctly with
    `InlinePlanViolation tag=body-too-large instruction_count=18 budget=16`
  - non-inline result: representative direct exact smokes stayed green, but
    perf-stat instructions regressed from MIM-065 `111384596` to `114532706`;
    cycles moved from `21025771` to `20914567`, and `body_elapsed_ns` stayed
    at `3000000`
  - decision: nonkeeper; reverted the source change and do not widen the
    inline verifier or keep a non-inline capsule fusion for this path
  - next: return to owner-first selection; likely candidates are a
    known-available `acquireFreshSmall` source route or a receiver-local queue
    publication cleanup, but only after current evidence selects one

- [x] MIM-068: post-active-success-nonkeeper owner refresh
  - output: reread the direct exact owner after reverting MIM-067 and select
    exactly one next source/MIR owner
  - start from the current source-shape candidates:
    known-available acquire route, receiver-local single-active queue
    publication, or no source keeper
  - no DirectArray/Span/direct-block/mixed-base-inline widening unless fresh
    evidence selects it
  - result: reverted direct exact front returned to the MIM-065 instruction
    band with `instructions=111384805`; perf selected
    `objectLifecycleSmallAlloc/1` first, followed by
    `objectLifecycleReleaseBlock/2`, `trySelectSingleActivePage/0`,
    `acquireFreshSmall/1`, and `releaseLocalKnownLive/1`
  - selected probe: known-available acquire route, because
    `trySelectSingleActivePage()` already proves the single-active page has
    `free_top > 0` before the following acquire

- [x] MIM-069: known-available acquire route probe
  - output: test a narrow `acquireFreshKnownAvailableSmall(size)` source route
    for the single-active fresh path
  - result: representative direct exact smoke stayed green, but the non-inline
    same-module method boundary regressed perf-stat instructions from
    `111384805` to `118724495`; cycles measured `21087030` and
    `body_elapsed_ns=4000000`
  - decision: nonkeeper; reverted the source change
  - reason: the source invariant is valid, but extracting the shape into a new
    helper adds a call boundary in the current EXE lowering and loses the net
    win
  - next: do not add another source helper unless it is verified
    `@rune Inline(required)` and stays inside the accepted receiver-local leaf
    budget; return to owner-first selection before the next edit

- [x] MIM-070: post-known-available-nonkeeper owner refresh
  - output: reread the direct exact owner after reverting MIM-069 and select
    exactly one next source/MIR owner
  - treat new source helper extraction as suspect unless current evidence plus
    inline eligibility show a positive net path
  - current candidate: page-local observer counter cost, starting with
    `HakoAllocPageModel.requested_bytes` inside `acquireFreshSmall`
  - decision note: do not delete the counter directly; first classify whether
    it is public semantics, proof/evidence payload, or diagnostic-only state
  - result: selected `requested_bytes` observer-counter classification as the
    next smallest candidate, then closed it as nonkeeper because it is public
    semantics plus proof evidence

- [x] MIM-071: observer counter classification
  - output: classify page-local counters reached by the current hot path as
    `public_semantics`, `proof_evidence`, or `diagnostic_only`
  - first field: `HakoAllocPageModel.requested_bytes`
  - required checks:
    - list every public accessor / proof / app readback that depends on the
      field
    - identify whether the current parity workload already computes the value
      outside the hot path
    - no source change except docs/checklist
  - no counter elision yet
  - result: `requested_bytes` is `public_semantics + proof_evidence`, not
    `diagnostic_only`
  - write sites:
    - `lang/src/hako_alloc/memory/page_box.hako`: `acquire`,
      `acquire_usize`, and `acquireFreshSmall` add `requested_size`
    - `lang/src/hako_alloc/memory/page_box.hako`: `resetToFresh` clears it
  - production/public readback:
    - `lang/src/hako_alloc/memory/page_heap_box.hako`: `requestedBytes()`
      sums `small_page.requested_bytes + medium_page.requested_bytes`
    - `lang/src/hako_alloc/memory/osvm_backed_fast_path_heap_box.hako`:
      `requestedBytes()` sums each page's `requested_bytes`
  - proof/evidence readback:
    - `apps/hako-alloc-mimalloc-comparison-representative-small-block-proof/main.hako`
      prints and asserts `page.requested_bytes == 33254`
    - `apps/mimalloc-page-model-proof/main.hako`,
      `apps/mimalloc-local-free-retire-proof/main.hako`, and several
      comparison proofs print/assert the page field directly
  - current object-lifecycle in-process parity workload already computes
    `requested_bytes = 33254 * operation_repeat` outside the hot page field,
    but that does not remove the field's public/proof role elsewhere

- [x] MIM-072: `gate` observer-counter policy selection
  - output: decide which observer counters may move behind build-time `gate`
    code without changing production semantics
  - allowed targets:
    - test/proof-only print and assertion payloads
    - diagnostic-only counters with no public API contract
    - alternate observer modules selected by `gate Build.test` or a declared
      feature predicate
  - forbidden targets:
    - hiding a production public accessor change behind `gate`
    - changing hot-core box layout behind `gate` without a dedicated layout row
    - using `gate` as a fast-path selector
  - if `requested_bytes` remains public semantics, keep the field/update live
    and select a different implementation route
  - decision: do not gate away `HakoAllocPageModel.requested_bytes` in
    production; doing so would break public `requestedBytes()` semantics and
    existing page-model proof evidence
  - allowed follow-up: proof apps may stop reading the page field directly
    when they can compute requested bytes from the workload contract, but that
    is proof cleanup only and is not a hot-path keeper

- [x] MIM-073: selected observer-counter implementation
  - output: implement exactly one selected counter move/elision from MIM-072
  - acceptable shapes:
    - move proof-only requested-byte calculation to the proof/report side
    - split observer counters into a gated observer facade/module
    - leave hot path unchanged and close as nonkeeper if public semantics
      require the update
  - no broad counter cleanup, no provider activation, no DirectArray reopening
  - result: no implementation for `requested_bytes`; close as nonkeeper for
    hot-path elision because the counter is public semantics

- [x] MIM-074: observer-counter measurement
  - output: rerun direct exact measurement after MIM-073, compare against the
    current direct exact baseline, and mark keeper/nonkeeper in this
    workstream
  - keeper requires instruction improvement or a clear structural reduction
    with no public stats/proof regression
  - result: skipped by design; no code changed, so no measurement is meaningful

- [x] MIM-075: post-observer-counter-nonkeeper owner refresh
  - output: reread the current direct exact source/MIR owner after
    `requested_bytes` counter elision is rejected
  - candidate set:
    - `trySelectSingleActivePage` body
    - `acquireFreshSmall` page body
    - `objectLifecycleReleaseBlock` release side
    - MIR call/copy materialization if source shapes continue to nonkeep
  - do not reopen observer-counter gating unless a field is classified
    `diagnostic_only`
  - no implementation before the next owner is selected
  - result: no new perf run required because MIM-071..MIM-074 made no code
    change; the prior direct exact owner evidence remains the current evidence
  - source-shape status:
    - `trySelectSingleActivePage` local cleanup: nonkeeper in current probe
    - `acquireFreshSmall` field-traffic localization: nonkeeper in current
      probe
    - `objectLifecycleReleaseBlock` local alias cleanup: nonkeeper in current
      probe
    - `requested_bytes` elision: rejected before implementation because it is
      public semantics plus proof evidence
  - selected next: `mir_call_copy_materialization_refresh`
  - selected reason: recent source-level probes did not produce a keeper, so
    the next useful owner is the remaining MIR call/copy materialization around
    the same hot methods rather than another `.hako` local rewrite

- [x] MIM-076: MIR call/copy materialization refresh
  - output: attribute remaining copies/calls around the current hot methods
    without changing source semantics
  - target methods:
    - `HakoAllocObjectLifecyclePageQueue.trySelectSingleActivePage/0`
    - `HakoAllocPageModel.acquireFreshSmall/1`
    - `HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2`
  - required output:
    - callsite receiver/arg/result copy counts
    - local-SSA copy counts
    - PHI edge copy counts
    - selected single compiler owner or explicit no-owner result
  - no source cleanup, no counter gating, no DirectArray reopening before the
    attribution selects a positive-net implementation path
  - prerequisite fix: `@rune Gate(...)` sugar had regressed member-level
    `@rune Inline(required)` parsing because identifier matching treated every
    rune name as `Gate`; fixed the parser to restrict only the exact `Gate`
    rune to top-level sugar and added a regression test
  - evidence command:
    `NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 target/release/hakorune --backend mir --emit-mir-json <tmp>/app.mir.json apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako`
    followed by `tools/allocator/mir_callsite_copy_attribution.py` for each
    target method
  - result:
    - `trySelectSingleActivePage/0`: `instruction_count=72`,
      `call_count=0`, `copy_count=18`, `local_ssa_copy_count=18`,
      `phi_edge_copy_count=0`, `dominant_copy_owner=local_ssa_copy_materialization`
    - `acquireFreshSmall/1`: `instruction_count=77`, `call_count=2`,
      `copy_count=26`, `receiver_copy_count=2`, `arg_copy_count=4`,
      `result_copy_count=3`, `local_ssa_copy_count=21`,
      `phi_edge_copy_count=0`, `dominant_copy_owner=local_ssa_copy_materialization`
    - `objectLifecycleReleaseBlock/2`: `instruction_count=244`,
      `call_count=10`, `copy_count=89`, `receiver_copy_count=11`,
      `arg_copy_count=16`, `result_copy_count=3`,
      `local_ssa_copy_count=46`, `phi_edge_copy_count=13`,
      `dominant_copy_owner=local_ssa_copy_materialization`
  - selected owner: `local_ssa_copy_materialization`
  - selected reason: all three candidates are dominated by local-SSA copy
    materialization; callsite receiver/arg/result copies are secondary, and
    helper-family attribution is zero for this refresh
  - next: inspect MIR/local-SSA copy producer around these three methods and
    choose one narrow structural reducer; do not add another `.hako` source
    cleanup before that owner is localized

- [x] MIM-077: block-local copy forwarding probe
  - selected owner: `local_ssa_copy_materialization`
  - attempted reducer: optimizer-level same-block `Copy` alias forwarding,
    followed by existing DCE cleanup
  - structural result before rejection:
    - `trySelectSingleActivePage/0`: `copy_count 18 -> 1`
    - `acquireFreshSmall/1`: `copy_count 26 -> 2`
    - `objectLifecycleReleaseBlock/2`: `copy_count 89 -> 4`
  - backend compatibility finding:
    - forwarding field bases made pure-first fail with
      `unsupported pure shape`, first blocker `Main.runOne/2` field_get
    - keeping field bases but forwarding call operands made pure-first fail
      on a same-module `mir_call`
    - keeping both field bases and call operands allowed EXE build, but routed
      into a much slower shape
  - measurement:
    - representative direct exact EXE smoke still produced `summary=ok`
    - `body_elapsed_ns=369000000`, a large regression from the current
      3-4ms band
  - decision: nonkeeper; reverted the optimizer pass
  - reason: the current pure-first backend treats some local materialization
    copies as route-shape carriers, so generic block-local copy forwarding can
    reduce MIR copy counts while destroying the fast backend route
  - next: do not add generic copy propagation in the optimizer; if copy cleanup
    is reopened, make it route-aware and prove the backend lowering route stays
    identical before measuring

- [x] MIM-078: route-carrier copy classification
  - output: extend the existing local-SSA copy position probe with route-carrier
    role counts; no optimizer pass and no source rewrite
  - implementation: `tools/allocator/mir_local_ssa_copy_position_probe.py`
    now reports copies whose alias chain feeds known backend route-sensitive
    operands:
    - `field_base_route_carrier_copy_count`
    - `call_operand_route_carrier_copy_count`
    - `call_result_route_carrier_copy_count`
    - `compare_operand_route_carrier_copy_count`
    - `field_set_value_route_carrier_copy_count`
    - `backend_route_carrier_copy_count`
    - `route_aware_candidate_copy_count`
  - evidence command:
    `NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 target/release/hakorune --backend mir --emit-mir-json <tmp>/app.mir.json apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako`
    followed by `tools/allocator/mir_local_ssa_copy_position_probe.py`
    for each MIM-076 target method
  - result:
    - `trySelectSingleActivePage/0`: `copy_count=18`,
      `backend_route_carrier_copy_count=13`,
      `route_aware_candidate_copy_count=5`,
      `dominant_route_carrier_role=field_base`
    - `acquireFreshSmall/1`: `copy_count=26`,
      `backend_route_carrier_copy_count=11`,
      `route_aware_candidate_copy_count=15`,
      `field_base_route_carrier_copy_count=6`,
      `call_operand_route_carrier_copy_count=4`,
      `call_result_route_carrier_copy_count=3`
    - `objectLifecycleReleaseBlock/2`: `copy_count=89`,
      `backend_route_carrier_copy_count=50`,
      `route_aware_candidate_copy_count=34`,
      `field_base_route_carrier_copy_count=20`,
      `call_operand_route_carrier_copy_count=27`,
      `call_result_route_carrier_copy_count=3`
  - decision: copy cleanup remains closed for implementation; the current
    evidence says many copies are route-shape carriers, so the next keeper must
    be route-aware and prove unchanged lowering route before deleting or
    forwarding any copy
  - next: select a narrow route-aware copy candidate only after one site family
    has a stable `before_route == after_route` proof; otherwise return to perf
    owner refresh instead of another generic MIR cleanup

- [x] MIM-079: post-route-carrier perf pair refresh
  - output: rerun the representative direct exact Hako EXE and explicit C
    mimalloc body-timing pair after the MIM-077 nonkeeper was reverted and
    MIM-078 classified copy carriers
  - environment:
    - `HAKO_TYPED_OBJECT_STORE=direct_slot_exact`
    - `HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact`
    - `NYASH_FEATURES=rune`
    - `NYASH_DISABLE_PLUGINS=1`
  - evidence command:
    - `tools/allocator/hako_exe_memory_runner.sh --app apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako --workload representative-object-lifecycle-small-block-v0 --runtime-config empty --operation-repeat 1 --out <tmp>/hako.out`
    - `tools/allocator/c_mimalloc_explicit_runner.sh --workload representative-object-lifecycle-small-block-v0 --alloc-count 64 --block-size 512 --operation-repeat 1 --in-process-repeat 8192 --allow-ldconfig-discovery --out <tmp>/c.out`
    - `tools/allocator/hako_mimalloc_object_lifecycle_body_timing_pair_adapter.py --hako-report <tmp>/hako.out --c-report <tmp>/c.out --out <tmp>/pair.out`
  - result:
    - `hako_body_elapsed_ns=4000000`
    - `c_body_elapsed_ns=3938387`
    - `body_elapsed_ratio=1.016`
    - `summary=ok`
  - interpretation: the current direct exact mainline is back near the C
    body-timing band; the 369ms/385ms readings seen around MIM-077 are not a
    keeper baseline and must not drive source edits
  - decision: do not reopen generic copy forwarding; with current body timing
    close to C, the next step should be instruction/perf-owner evidence before
    any further source or MIR optimization

- [x] MIM-080: post-C-near-band owner and source-probe refresh
  - output: refresh direct exact Hako/C evidence and test only the thinnest
    source-contract candidates before reopening compiler work
  - evidence:
    - Hako/C body pair: `hako_body_elapsed_ns=4000000`,
      `c_body_elapsed_ns=3219386`, `body_elapsed_ratio=1.242`
    - perf stat: Hako `instructions=126160783`, `cycles=27694119`; C
      `instructions=65100489`, `cycles=17830340`
    - rough ratios: `instruction_ratio_hako_over_c=1.94`,
      `cycle_ratio_hako_over_c=1.55`
    - perf top with repeated exact EXE runs:
      `objectLifecycleSmallAlloc/1=24.39%`,
      `trySelectSingleActivePage/0=15.76%`,
      `objectLifecycleReleaseBlock/2=15.42%`
  - source probes rejected and reverted:
    - `trySelectSingleActivePage(): HakoAllocPageModel` return annotation
      increased `objectLifecycleSmallAlloc/1` from `instruction_count=186`,
      `copy_count=97` to `instruction_count=188`, `copy_count=99`
    - `recordSinglePageActiveSelection()` receiver-local helper exceeded the
      required-inline verifier budget with `instruction_count=20 budget=16`
    - `objectLifecycleReleaseBlock/2` localizing `last_alloc_page_id` reduced
      one field get but increased MIR instructions/copies/phis, so it stayed
      reverted
    - route-aware same-block Copy reuse for `LocalKind::FieldBase` reduced
      `objectLifecycleSmallAlloc/1` MIR copy count from `97` to `95`, but did
      not improve the representative direct exact EXE perf stat
      (`instructions=126160889` vs prior `126160783`, cycles worsened), so it
      stayed reverted
  - decision: do not keep another `.hako` local-binding/helper-extraction
    probe or tiny copy-forwarding tweak for these sites; the next keeper must
    either be current-owner evidence for a real source semantic reduction or a
    route-aware MIR materialization improvement that proves a machine-code win,
    not only a MIR copy-count reduction

### Next Cleanup TODO

Use these as Ghost Tasks inside this workstream. Do not create numbered rows
unless one of them changes a durable contract or implementation boundary.

1. `lang/c-abi/shims/hako_llvmc_ffi_same_module_generic_method_emit.inc`
   - done: split the generic-method emitter by responsibility, starting with
     the biggest method families
   - done: kept the existing public emission behavior intact
   - evidence: `bash tools/build_hako_llvmc_ffi.sh`
   - no new row-specific guard

2. `src/llvm_py/instructions/field_access_helpers.py`
   - done: split helper-heavy field access support into smaller typed-object /
     direct-slot / fallback helpers
   - done: kept the `field_access.py` entry thin
   - evidence: `python -m py_compile src/llvm_py/instructions/field_access.py src/llvm_py/instructions/field_access_helpers.py src/llvm_py/instructions/field_access_helpers_common.py src/llvm_py/instructions/field_access_helpers_typed.py`
   - no new row-specific guard

3. `src/mir/global_call_route_plan/string_return_profile.rs`
   - done: split profile collection / candidate judging / report emission
   - done: kept the route-plan decision points readable and fact-driven
   - evidence: `cargo check -q`
   - no new row-specific guard

4. PageQueue helper-extraction crash investigation
   - done: MIM-059 accepted a receiver-local `@rune Inline(required)` shape and
     avoided the non-inline same-module helper call that previously crashed
   - done: added `apps/pagequeue-mixed-base-helper-proof` as the small
     mixed-base same-module no-coredump fixture; it runs VM, MIR route proof,
     pure-first EXE build, and EXE execution without requesting inline
   - done: reference docs now point the future reopen path through
     `EffectSummary` before any narrow publication plan
   - rejected shape: helper body that reads `page.page_id` and writes `me.*`
     stays rejected for now because it is a mixed-base required-inline body
   - remaining follow-up: only reopen mixed-base helper extraction if a small
     VM/EXE parity fixture selects it; mimalloc source cleanup no longer
     depends on that shape
   - design rule: do not widen the generic `Inline(required)` verifier to
     "multiple bases are probably fine"; mixed-base extraction must come back
     only as a narrow recipe or as a same-module call lowering fix
   - reopen order:
     1. done: add a no-coredump fixture for the non-inline same-module mixed-base
        helper call; unsupported lowering must become a compile-time diagnostic,
        not an EXE crash
     2. add helper `EffectSummary` vocabulary for receiver reads/writes,
        foreign reads/writes, handle publications, calls, allocations, and
        safepoints
     3. if still selected, add a narrow
        `ReceiverSnapshotPublicationPlanV0` recipe; start with scalar foreign
        read publication before permitting foreign handle publication
   - v0 rejected surfaces: multiple foreign bases, foreign writes, nested calls,
     branch/loop bodies, allocation, dynamic field access, and handle
     publication that needs a runtime barrier

5. Direct-exact runtime environment dependency audit
   - status: done by MIM-060; correctness hygiene, not a perf keeper
   - observation: manually built direct-exact EXEs must be run with the same
     `HAKO_TYPED_OBJECT_STORE=direct_slot_exact` and
     `HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact` environment used for
     lowering; running the EXE without those runtime env values can crash
   - implemented contract: direct-exact lowering emits a `ny_main` prologue
     check for the selected backend modes and exits with a clear fail-fast
     diagnostic when runtime env is missing
   - do not mix future env hygiene with MIM source-shape optimizations unless
     current evidence selects it

### Parked Direct Memory View Roadmap

This is a parking lot, not an active implementation lane. Keep mimalloc
optimization on the current `DirectState` / `DirectArrayI64` / `RangeIndexFact`
surface unless fresh perf evidence selects a representation fast path.

Design decision:

```text
do_not_add_raw_pointer_surface=1
native_ptr_direct_deref=0
native_ptr_index_operator=0
native_ptr_pointer_arithmetic=0
source_reference_docs_changed=0
reason=the_source_language_has_no_accepted_DirectMemory_or_Span_syntax_yet
```

Long-term layering:

```text
MemoryRegion:
  the owned or borrowed storage region

MemoryView:
  the typed way to see that region

MemoryAccessPlan:
  the selected load/store route for one site

Proof:
  bounds / alignment / alias / lifetime / stability / initialization facts
```

Terminology rules:

```text
direct:
  fast-route contract; generic fallback is not allowed when requested

unsafe_memory:
  permission to create a view over external/native memory

unchecked:
  proof result that removes bounds checks
```

Do not merge those three meanings.

Recommended order when this becomes active:

1. Keep current source syntax unchanged.
   - continue using `me.field`, `direct_array[i]`, and ordinary loops
   - do not add `RawPtr<T>`, `&`, `*`, `->`, or pointer arithmetic

2. Strengthen existing DirectArray planning first.
   - add `proof_ids` to `DirectArrayAccessPlanV0`
   - make the plan carry `element_type` before adding more DirectArray
     storage kinds
   - add region/view vocabulary only as metadata, not source syntax
   - preserve `direct != unchecked`

3. Normalize proof facts.
   - `RangeIndexFact`
   - `DirectArrayExtentFact`
   - `RegionStabilityFact`
   - later: alias/lifetime/alignment/initialization facts

4. Add `SpanI64` / `SpanMutI64` only after DirectArray proof facts are stable.
   - span is a no-escape borrowed view
   - span cannot be returned, stored in fields, captured, published, or cross
     provider boundaries
   - mutable span requires unique access to its region

5. Add `direct {}` only as a fast-route requirement.
   - it does not make memory unsafe
   - it does not imply unchecked
   - it fails if a required `FastPathPlan` is missing

6. Add `unsafe memory` / `Bytes` later.
   - `NativePtr` remains opaque
   - `Bytes` owns byte-offset `load_*_at` / `store_*_at` methods
   - byte access must carry alignment and bounds policy

7. Add `LayoutSpan` and bulk memory patterns only after Span/Bytes are proven.
   - layout field access stays separate from existing enum record payload
     terminology
   - bulk `fill` / `zero` / `iota` / `copy` should first be recognized from
     loop idioms, not added as source APIs

This parking lot intentionally does not update `docs/reference/**`: no source
syntax, public ABI, or language-level unsafe memory contract is accepted here.

DirectArray family order:

```text
source_visible_v0:
  Array
  DirectArrayI64

active_now:
  DirectArrayI64

before_new_storage_kinds:
  DirectArrayAccessPlanV0.element_type

next_if_mimalloc_or_current_perf_selects:
  DirectArrayBool
  DirectArrayUsize_or_U64

later:
  TextLane
  DirectArrayHandle
  ValueLane

deferred:
  record_or_union_inline_layout
```

Rules:

- User-facing classification is two-tier: `Array` and the `DirectArray`
  family.
- `DirectArray` is not a standalone untyped source type in v0.
- `DirectArrayI64` remains i64-only.
- Direct storage element type must be explicit in the v0 source type.
- Do not infer an untyped `DirectArray` into i64 storage from observed writes.
- `DirectArrayI64` is not a subtype of `Array`.
- Do not add implicit `Array` <-> `DirectArrayI64` conversion; materialization
  or copy must be explicit if needed.
- Do not turn DirectArray into a public mixed ArrayBox replacement.
- Do not add a new DirectArray kind without current perf evidence or an active
  array/value-lane task.
- Sentinel-bearing ids/indexes stay signed; non-negative counts/sizes may move
  to `usize`/`u64` only through an explicit storage-kind task.

## MIM-023..MIM-027 Source-Shape And Metadata Notes

### MIM-023: composite hot-cluster source inventory

`objectLifecycleSmallAlloc` and `objectLifecycleReleaseBlock` are still the
two composite hot clusters in the current owner surface.

`objectLifecycleSmallAlloc` shape:

```text
resetAttempt -> queue.selectPage -> selected_index / selected_kind checks
-> optional page.reuse() -> page.acquireFreshSmall(size)
-> alloc_result.recordBlock(block_id)
-> recordLastAllocPage(index, page_id, page)
-> alloc_result.recordSuccess(selected_kind)
```

`objectLifecycleReleaseBlock` shape:

```text
resetReleaseResult -> release_result.recordRequest(page_id, block_id)
-> page/block validation
-> objectLifecycleReleaseDirectCachedPage
-> objectLifecycleReleaseKnownPageIndex
-> pages.get(known_index) fallback
-> page.releaseLocal(block_id)
-> release_result.recordSuccess / recordFailure
```

The hot path still mixes primitive counters, cached page handles, page lookup,
and observer/result capsules. That means the cluster is source-shaped but not
yet a full direct-state box.

### MIM-024: primitive group inventory

Primitive-only groups reached by the hot clusters:

| Group | Classification | Evidence |
| --- | --- | --- |
| `alloc_result` scalar fields | `all_primitive` | `last_page_id`, `last_block_id`, `last_reason`, `last_ok`, and count fields are scalar state only |
| `release_result` scalar fields | `all_primitive` | `last_page_id`, `last_block_id`, `last_reason`, `last_ok`, and count fields are scalar state only |
| `alignment_result` scalar fields | `all_primitive` | `last_requested`, `last_normalized`, `last_reason`, `last_supported` are scalar state only |
| `realloc_result` scalar fields | `all_primitive` | request / new-page / new-block / status / requested-size are scalar state only |
| `last_alloc_page_index`, `last_alloc_page_id`, `release_known_page_fast_path_count`, `release_known_page_fallback_count` | `all_primitive` | facade-level primitive counters and cache indices |
| `last_alloc_page`, `release_known_page`, `object_lifecycle_queue.pages`, `first_page` | `mixed_handle` | cached page objects and collection storage stay in object/handle world |
| `stats_surface` / observer readback methods | `public_observer` | read-only surface over the result capsules |
| page lookup fallback / `page.releaseLocal*` / `page.acquireFreshSmall` | `escape_unknown` | direct page-object behavior is outside a direct-state box boundary |

### MIM-025: direct-state feasibility report

The candidate family is **not** the whole facade. The direct-state candidate is
the existing scalar capsule family plus the primitive cache counters, not a new
`.hako` surface:

```text
StateRepr::Direct candidate family:
  alloc_result
  release_result
  alignment_result
  realloc_result
  selected primitive facade counters
```

Feasibility summary:

```text
state_repr=direct_v0
field_decl_authority=1
selected_field_count=positive
unsupported_field_count=nonzero on the composite facade
materialization_boundary_known=1
positive_net_expected=only for the scalar capsule family
```

The current facade as a whole is still mixed-handle / public-observer heavy, so
it is not a direct-state owner by itself.

### MIM-026: primitive core source-shape plan

No new primitive-only core box is needed yet.

The existing scalar capsules already provide the narrow primitive core, and the
facade counters stay as ordinary box fields. Keep the source surface as-is:

- ordinary typed box fields
- `DirectArrayI64`
- `@rune Inline(required)`

Do not add `record`, `slots`, or `layout` syntax for this lane.

### MIM-027: MIR metadata-only producer

The producer is implemented as a metadata-only module plan:

```text
metadata_owner=src/mir/direct_state_plan.rs
module_metadata_field=direct_state_plans
json_export=direct_state_plans
state_repr=direct_v0
field_decl_authority=1
```

Inputs are `field_decls` and declared storage-class facts. Unsupported fields
are counted, not silently accepted. No new syntax, runtime layout, helper ABI,
backend lowering, provider activation, or allocator replacement is opened.

### MIM-028: selected direct-state guard surface

Selected.

MIR JSON proof:

```text
emit_route=direct
env.NYASH_FEATURES=not_required
env.HAKO_TYPED_OBJECT_STORE=direct_slot_exact
env.HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact
direct_state_plan_count=9
summary=ok
```

Selected guard:

```text
selected_box=HakoAllocObjectLifecycleAllocResult
selected_method=HakoAllocObjectLifecycleAllocResult.recordSuccess/1
state_repr=direct_v0
selected_field_count=9
unsupported_field_count=0
materialization_boundary_known=1
positive_net_expected=1
post_cut_perf_symbol_pct=10.58
same_object_receiver_required=1
silent_fallback_allowed=0
```

Selected fields:

```text
last_page_id
last_block_id
last_reason
last_ok
attempt_count
success_count
failure_count
reusable_success_count
active_success_count
```

Rejected for first lowering:

```text
HakoAllocObjectLifecycleFacade:
  selected_field_count=4
  unsupported_field_count=8
  reason=mixed handle/object facade

HakoAllocObjectLifecyclePageQueue:
  selected_field_count=16
  unsupported_field_count=3
  reason=mixed page object / ArrayBox surface

HakoAllocObjectLifecycleFacadeStatsSnapshot:
  reason=observer snapshot, not active hot write owner

alignment/realloc result capsules:
  reason=not the representative small-block hot owner
```

This guard opens MIM-029 only for the selected alloc-result scalar capsule
method/field group. It does not open generic user-box flattening or whole-facade
direct-state lowering.

### MIM-029: narrow direct-state lowering implementation

Landed for the MIM-028 selected callsite only.

Implementation boundary:

```text
implemented_owner=ny_llvmc_same_module_method_call_emit
implemented_owner_file=lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc
selected_callsite=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1:b588.i13
selected_method=HakoAllocObjectLifecycleAllocResult.recordSuccess/1
selected_backend=direct_slot_exact
source_surface_changed=0
public_abi_changed=0
generic_user_box_flattening=0
```

Structural proof:

```text
record_success_direct_call_count=0
direct_state_alloc_label_count=36
direct_state_fields=last_reason,last_ok,success_count,reusable_success_count,active_success_count
semantic_smoke_summary=ok
result_code=0
allocation_count=524288
free_count=524288
```

The lowering consumes the existing typed-object plan and DirectSlot V0 layout.
It verifies the expected field slots/storage before emitting direct stores/RMWs:

```text
last_reason: i64 slot 2
last_ok: i64 slot 3
success_count: usize/u64 slot 5
reusable_success_count: usize/u64 slot 7
active_success_count: usize/u64 slot 8
```

The direct path is enabled only for `HAKO_TYPED_OBJECT_STORE=direct_slot_exact`.
Default/safe and generic user-box method calls keep the existing route.

Temporary residue rule:

```text
temporary_special_case_owner=MIM-029-selected-direct-state-lowering
temporary_special_case_kind=selected_pilot
allowed_until=MIM-034
retire_condition=MIM-030_nonkeeper_or_no_current_owner
promote_condition=fact_driven_DirectStateMethodLowering_selected_by_MIM-033
forbidden_extension=additional_by_name_callsite_branches
```

The MIM-029 path is intentionally narrow, but it is not allowed to become a
permanent hardcoded emitter branch. MIM-031..MIM-034 track the audit,
same-family inventory, substrate selection, and final closeout.

### MIM-030: post-direct-state owner refresh

Completed.

This is not the direct-state fast path itself. MIM-030 runs only after MIM-029
has installed a selected direct-state fast path, then rereads the direct exact
front and picks the next owner from measured evidence.

Measured direct exact body samples after MIM-029:

```text
front=direct_exact
env.HAKO_TYPED_OBJECT_STORE=direct_slot_exact
env.HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact
sample_count=3
body_elapsed_ns=8000000,9000000,8000000
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
summary=ok
```

Single direct exact `perf stat` reread:

```text
hako_direct_instructions=236247273
hako_direct_cycles=45160170
body_elapsed_ns=8000000
```

Low-sample perf owner reread:

```text
HakoAllocPageModel.releaseLocalKnownLive/1=40.82%
HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1=16.28%
HakoAllocObjectLifecycleReleaseResult.recordRequest/2=14.43%
HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2=14.01%
HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2=13.64%
```

Interpretation:

```text
direct_state_alloc_success_keeper=structural
previous_direct_body_elapsed_ns=13000000
current_direct_body_elapsed_ns=8000000,9000000,8000000
previous_direct_instructions=369629325
current_direct_instructions=236247273
record_success_removed_from_top_owner=1
next_owner_family=release_side_hotpath
new_direct_state_hardcode_allowed=0
next_task=MIM-031 direct-state special-case residue audit
```

### MIM-031..MIM-034: direct-state pilot residue closeout

MIM-031..MIM-034 completed.

These items exist to prevent the MIM-029 selected callsite lowering from
becoming permanent hardcode. MIM-031 audits the exact selectors and slot
assumptions. MIM-032 checks whether the same family has enough evidence for a
second direct-state keeper. MIM-033 decides whether a generic substrate is
earned. MIM-034 closes the pilot by deleting it, promoting it to fact-driven
lowering, or documenting a bounded extension.

MIM-031 audit result:

```text
audited_file=lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc
temporary_special_case=HakoAllocObjectLifecycleAllocResult.recordSuccess/1
selector_box=HakoAllocObjectLifecycleAllocResult
selector_method=recordSuccess
receiver_backend_required=direct_slot_exact
arg_count_required=1
arg0_kind=plain_i64
hardcoded_slot_0=last_reason:i64:slot2:store_const_0
hardcoded_slot_1=last_ok:i64:slot3:store_const_1
hardcoded_slot_2=success_count:u64_or_usize:slot5:rmw_add1
hardcoded_slot_3=reusable_success_count:u64_or_usize:slot7:rmw_add1_if_selected_kind_eq_1
hardcoded_slot_4=active_success_count:u64_or_usize:slot8:rmw_add1_if_selected_kind_eq_2
classification=keep_as_selected_pilot
promotion_ready=0
delete_now=0
reason=keeper_but_single_method_only_and_recordSuccess_is_no_longer_top_owner
next_task=MIM-032 direct-state same-family candidate inventory
```

The selected pilot remains bounded by `allowed_until=MIM-034`. MIM-032 may
inspect `HakoAllocObjectLifecycleReleaseResult` because the post-MIM-029 owner
reread moved heat to the release side, but it must not add another by-name
emitter branch. Any second keeper has to feed MIM-033's substrate decision.

MIM-032 inventory result:

```text
candidate_family=HakoAllocObjectLifecycleReleaseResult
direct_state_plan_available=1
field_decl_authority=1
selected_field_count=6
unsupported_field_count=0
materialization_boundary_known=1
positive_net_expected=1
hot_method_candidate=recordRequest/2
hot_method_source_annotation=@rune Inline(required)
hardcode_extension_allowed=0
selected_implementation=MIM-032a-required-inline-user-method-consumer
reason=recordRequest_already_has_required_inline_contract;consume_existing_contract_before_adding_direct_state_method_lowering
```

MIM-032a implementation:

```text
implemented_file=src/mir/passes/inline_soft_leaf.rs
previous_inline_call_consumer=Callee::Global_only
new_inline_call_consumer=Callee::Global_or_known_user_defined_Callee::Method
method_target_symbol=box_name.method/source_arg_count
method_inline_args=receiver_plus_explicit_args
new_source_surface=0
new_profile_surface=0
new_direct_state_emitter_branch=0
unit_test=cargo test -q inline_soft_leaf --lib
semantic_smoke=representative-object-lifecycle-small-block-v0 direct exact EXE
release_record_request_method_call_count=0
summary=ok
```

This is not a DirectState substrate promotion. It is a generic consumer fix for
the existing required-inline contract. MIM-033 still decides whether the
MIM-029 direct-state selected pilot has earned extraction into a fact-driven
method-lowering substrate.

MIM-033 post-MIM-032a reread:

```text
front=direct_exact
sample_count=3
body_elapsed_ns=8000000,8000000,8000000
release_record_request_method_call_count=0
alloc_record_success_method_call_count=2
hako_direct_instructions=231529109
hako_direct_cycles=42064140
summary=ok
```

Low-sample perf owner reread:

```text
HakoAllocPageModel.releaseLocalKnownLive/1=21.66%
HakoAllocObjectLifecycleAllocResult.resetAttempt/0=21.16%
Main.runOne/2=20.35%
HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1=19.59%
HakoAllocObjectLifecyclePageQueue.selectPage/0=15.74%
```

MIM-033 decision:

```text
selected_owner=do_not_promote_direct_state_method_lowering_yet
promotion_ready=0
same_family_keeper_count=1
same_family_candidate_recordRequest_handled_by_required_inline=1
record_success_still_current_top_owner=0
next_owner_family=release_page_and_alloc_reset_attempt
reason=single_selected_direct_state_keeper_plus_required_inline_consumed_release_candidate
next_task=MIM-034 selected direct-state pilot closeout
```

DirectState method lowering remains a future substrate candidate, but not the
next implementation. The current evidence favors returning to ordinary
perf-owner selection around `releaseLocalKnownLive/1`, `resetAttempt/0`, and
the remaining small-alloc/selectPage shape.

MIM-034 closeout:

```text
selected_pilot=MIM-029_alloc_result_recordSuccess_direct_state
closeout_decision=keep_bounded_selected_pilot
delete_now=0
promote_now=0
bounded_keep_reason=structural_keeper_with_non_top_residual_call_boundary
allowed_extension=0
additional_by_name_branches_allowed=0
reopen_promotion_only_if=future_perf_owner_selects_direct_state_method_family_with_two_or_more_keepers
next_owner_selection=ordinary_perf_owner_first
next_candidate_family=releaseLocalKnownLive_or_resetAttempt_or_smallAlloc_selectPage
summary=ok
```

The MIM-029 path remains in place because it is a measured structural keeper,
but it is closed as a bounded selected pilot. Future work must not extend it by
adding another by-name branch. If DirectState method lowering reopens, it must
start from a fresh owner selection and a fact-driven substrate plan.

### MIM-035: required-inline fieldget leaf extension and resetAttempt

Completed.

`AllocResult.resetAttempt/0` became the next visible result-capsule owner after
MIM-032a. It was already a single-block primitive receiver leaf, but it performs
one counter increment:

```text
attempt_count = attempt_count + 1
```

The required-inline verifier therefore needed one narrow vocabulary extension:
same-receiver `FieldGet` / scalar op / `FieldSet`, with the same single-base
guard already used for receiver field-set leaves.

Implementation:

```text
source_annotation_added=HakoAllocObjectLifecycleAllocResult.resetAttempt/0:@rune Inline(required)
compiler_change=required_inline_same_base_fieldget_fieldset_leaf
method_call_count_after.HakoAllocObjectLifecycleAllocResult.resetAttempt/0=0
method_call_count_after.HakoAllocObjectLifecycleReleaseResult.recordRequest/2=0
new_profile_surface=0
new_direct_state_emitter_branch=0
source_hand_expansion=0
```

Validation:

```text
cargo_test_inline_required=ok
cargo_test_inline_soft_leaf=ok
cargo_build_hakorune=ok
cargo_build_release_hakorune=ok
semantic_smoke=representative-object-lifecycle-small-block-v0 direct exact EXE
body_elapsed_ns=7000000
allocation_count=524288
free_count=524288
summary=ok
```

Perf reread:

```text
hako_direct_instructions=228907410
hako_direct_cycles=41469062
body_elapsed_ns=7000000
```

Low-sample owner reread:

```text
HakoAllocObjectLifecyclePageQueue.selectPage/0=27.81%
Main.runOne/2=15.74%
HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1=15.28%
HakoAllocPageModel.resetToFresh/0=13.60%
HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2=13.27%
```

Interpretation:

```text
resetAttempt_call_boundary_cut=keeper
direct_state_reopen=0
next_task=MIM-036 post-resetAttempt owner refresh
```

## MIM-036 Post-resetAttempt Owner Refresh

Source/MIR reread:

```text
front=direct_exact
target=representative-object-lifecycle-small-block-v0
selected_slice=HakoAllocPageModel read-only page-state accessors used by HakoAllocObjectLifecyclePageQueue.selectPage/0
source_annotations_added=
  HakoAllocPageModel.isRetired/0:@rune Inline(required)
  HakoAllocPageModel.isDecommitted/0:@rune Inline(required)
  HakoAllocPageModel.freeCount/0:@rune Inline(required)
profile_added=0
new_syntax=0
source_hand_expansion=0
```

Structural result:

```text
selectPage_single_page_path_method_calls_erased=3
remaining_HakoAllocPageModel_accessor_calls=multi_page_fallback_loop_only
direct_state_reopen=0
array_fast_path_reopen=0
```

Validation:

```text
cargo_test_inline_required=ok
cargo_test_inline_soft_leaf=ok
cargo_build_hakorune=ok
cargo_build_release_hakorune=ok
semantic_smoke=representative-object-lifecycle-small-block-v0 direct exact EXE
allocation_count=524288
free_count=524288
summary=ok
```

Perf reread:

```text
previous_hako_direct_instructions=228907410
candidate_hako_direct_instructions=221043533
instruction_delta=-7863877
instruction_delta_pct=-3.43
candidate_body_elapsed_ns=11000000
candidate_cycles=56648756
```

Low-sample owner reread:

```text
HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1=30.21%
HakoAllocPageModel.releaseLocalKnownLive/1=19.06%
HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2=16.41%
HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2=9.92%
HakoAllocPageModel.acquireFreshSmall/1=9.26%
HakoAllocObjectLifecyclePageQueue.selectPage/0=8.24%
HakoAllocPageModel.resetToFresh/0=6.54%
```

Interpretation:

```text
keeper=structural_instruction_keeper
selectPage_owner_reduced=1
next_owner=release_side_structured_inline_feasibility
next_task=MIM-037
```

## MIM-037 Release-side Structured Inline Feasibility

Selected slice:

```text
front=direct_exact
selected_owner=release_result_capsule_hot_success_path
source_shape=
  objectLifecycleReleaseBlock -> release_result.reset()
  objectLifecycleReleaseDirectCachedPage -> release_result.recordSuccess(page_id, block_id)
```

Implementation:

```text
source_annotations_added=
  HakoAllocObjectLifecycleReleaseResult.reset/0:@rune Inline(required)
  HakoAllocObjectLifecycleReleaseResult.recordSuccess/2:@rune Inline(required)
wrapper_bypass_hot_path=1
structured_branch_inline_added=0
source_hand_expansion=0
profile_added=0
```

Structural result:

```text
objectLifecycleReleaseBlock_mir_calls_before=11
objectLifecycleReleaseBlock_mir_calls_after=10
objectLifecycleReleaseDirectCachedPage_recordSuccess_call_erased=1
remaining_hot_boundaries=
  objectLifecycleReleaseDirectCachedPage/2
  HakoAllocPageModel.releaseLocalKnownLive/1
  array_runtime_get_idx
```

Validation:

```text
cargo_test_inline_required=ok
cargo_test_inline_soft_leaf=ok
cargo_build_hakorune=ok
cargo_build_release_hakorune=ok
semantic_smoke=representative-object-lifecycle-small-block-v0 direct exact EXE
allocation_count=524288
free_count=524288
summary=ok
```

Perf reread:

```text
previous_hako_direct_instructions=221043533
candidate_hako_direct_instructions=212654532
instruction_delta=-8389001
instruction_delta_pct=-3.80
candidate_body_elapsed_ns=7000000
candidate_cycles=36640249
```

Low-sample owner reread:

```text
HakoAllocObjectLifecyclePageQueue.selectPage/0=26.80%
HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2=26.05%
nyash_kernel::plugin::array_runtime_facade::array_runtime_get_idx=22.18%
HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1=15.91%
HakoAllocPageModel.releaseLocalKnownLive/1=7.28%
```

Interpretation:

```text
keeper=structural_instruction_keeper
structured_branch_inline_required=not_yet
current_mimalloc_perf_selects_array_get_boundary=1
next_task=MIM-038
```

## Decision Log

- 2026-06-01: MIM-054 removed the duplicate `cached_page.page_id == page_id`
  check from the first cached known-page release success path. The hot path has
  already matched `page_id == me.last_alloc_page_id`, and the cached handle and
  id are recorded together by `recordLastAllocPage()` after allocation;
  `HakoAllocPageModel.page_id` is fixed by `birth()`. Fallback release keeps
  the defensive id check. Representative direct exact smoke and the known-live
  release smoke stayed green, and instruction count moved from 126.7M to
  124.6M.
- 2026-06-01: Direct-memory substrate cleanup returned control to mimalloc.
  Next task is MIM-055: reread direct exact perf ownership before any source
  edit or new fast-path reopening.
- 2026-05-31: MIM-044 added generic nested-phi receiver origin acceptance for
  user-box route results. The `resetToFresh/0` fast guard introduces an extra
  guard-return control shape, which made the selected page flow through nested
  phis before `page.acquireFreshSmall(size)`. The fix preserves the
  `HakoAllocPageModel` origin through that phi chain instead of adding a
  reset-specific or mimalloc-specific route. The representative direct exact
  EXE stayed green, `objectLifecycleSmallAlloc/1` returned to
  `direct_function_call`, and instruction count moved from 151.8M to 147.0M.
- 2026-05-31: MIM-043 removed `HakoAllocPageModel.resetToFresh/0` from the
  unchecked DirectArray store selector and replaced the checked DirectArray
  get/store method-name allowlists with receiver-origin fact checks in both
  pure generic lowering and same-module body emission. This keeps
  `resetToFresh/0` on the normal checked DirectArray path without adding a new
  source fast-path branch or another reset-specific C-shim route. The
  representative direct exact EXE stayed green and instruction count stayed
  stable at 151.8M.
- 2026-05-31: MIM-039 extended the existing unchecked DirectArray i64 store
  lane from `resetToFresh/0` to the proven known-live release method
  `HakoAllocPageModel.releaseLocalKnownLive/1`. This removes public ArrayBox
  bounds/capacity branches from the two internal store sites while preserving
  default/safe behavior. The representative direct exact EXE stayed green,
  disasm shows the checked store branches removed, and instruction count moved
  from 184.9M to 178.6M.
- 2026-05-31: MIM-040 cleaned the cached-page release source shape by removing
  the redundant failure branch after `releaseLocalKnownLive/1`. That method is
  the proven known-live release path and returns `1` on all non-trap paths, so
  the cached-page guard remains the failure boundary. The representative direct
  exact EXE stayed green, the `recordReleaseFailure` call disappeared from
  `objectLifecycleReleaseDirectCachedPage/2`, and instruction count moved from
  178.6M to 174.4M.
- 2026-05-31: MIM-038 cut the selected DirectArray i64 exact get boundary in
  the C shim direct exact path. The hot `HakoAllocPageModel.acquireFreshSmall/1`
  site now lowers `ArrayBox.get` over a proven DirectArrayI64 handle to direct
  buffer load instead of calling `nyash.array.get_hi` /
  `nyash.array.slot_load_hi`. The representative direct exact EXE stayed green,
  disasm shows the helper call removed, and instruction count moved from
  212.7M to 184.9M. Public ArrayBox/default-safe behavior, mixed storage,
  plugin typed ABI, and source syntax are unchanged.
- 2026-05-31: MIM-037 narrowed the release-side owner to the result-capsule
  success path instead of opening a branch inliner. `ReleaseResult.reset/0` and
  `ReleaseResult.recordSuccess/2` now use explicit `@rune Inline(required)`,
  and hot release source calls the capsule directly rather than through facade
  wrappers. The representative direct exact EXE stayed green and instructions
  moved from 221.0M to 212.7M. The next owner is selected by current evidence:
  `array_runtime_get_idx` is now a 22% direct exact hot symbol.
- 2026-05-31: MIM-036 annotated the three read-only `HakoAllocPageModel`
  accessors used by the `selectPage/0` single-page hot path with
  `@rune Inline(required)`. This removed three hot-path method-call boundaries
  without Profile syntax or source hand-expansion. The representative direct
  exact EXE stayed green and instruction count moved from 228.9M to 221.0M.
  The next owner shifted to release-side structured helper boundaries:
  `releaseLocalKnownLive/1`, `objectLifecycleReleaseBlock/2`, and
  `objectLifecycleReleaseDirectCachedPage/2`.
- 2026-05-31: MIM-035 extended required-inline leaf verification to the narrow
  same-receiver FieldGet/FieldSet increment shape and annotated
  `AllocResult.resetAttempt/0`. This removed that method-call boundary without
  adding Profile syntax, direct-state branches, or source hand-expansion. The
  representative direct exact EXE stayed green, body timing reached 7ms, and
  perf ownership moved toward `selectPage/0`, `Main.runOne/2`,
  `objectLifecycleSmallAlloc/1`, `resetToFresh/0`, and release block.
- 2026-05-31: MIM-034 closed the selected direct-state pilot. The MIM-029
  `AllocResult.recordSuccess/1` branch remains as a bounded selected pilot
  because it is a structural keeper, but it is not promoted and cannot be
  extended with more by-name branches. The lane returns to ordinary perf-owner
  selection for `releaseLocalKnownLive/1`, `AllocResult.resetAttempt/0`,
  `objectLifecycleSmallAlloc/1`, or `selectPage/0`.
- 2026-05-31: MIM-033 rejected immediate DirectState method-lowering substrate
  promotion. MIM-032a consumed the release-side same-family candidate through
  the generic required-inline path, leaving only one direct-state selected
  keeper. The post-MIM-032a direct exact reread stayed at 8ms with 231M
  instructions, and the top owners moved to `releaseLocalKnownLive/1`,
  `AllocResult.resetAttempt/0`, `Main.runOne/2`, `objectLifecycleSmallAlloc/1`,
  and `selectPage/0`. MIM-034 should close the MIM-029 pilot as bounded keep or
  delete, not promote it yet.
- 2026-05-31: MIM-032 found a same-family release-side candidate, but the
  cleanest first fix was not another direct-state branch. `recordRequest/2`
  already carried `@rune Inline(required)`, while the soft-leaf pass only
  consumed `Callee::Global`. MIM-032a now consumes known user-defined
  `Callee::Method` calls too, mapping receiver plus explicit args into the leaf
  body. The representative direct exact EXE stayed green and the
  `ReleaseResult.recordRequest/2` MIR method-call count is now 0.
- 2026-05-31: MIM-030 accepted MIM-029 as a structural keeper on the direct
  exact front: body samples moved from the prior 13ms band to 8/9/8ms and a
  single perf stat reread moved from 369M to 236M instructions. The next heat
  moved to release-side methods, so MIM-031 audited the existing direct-state
  special case and classified it as `keep_as_selected_pilot`, not a generic
  substrate. MIM-032 opens same-family inventory without permitting another
  by-name emitter branch.
- 2026-05-31: MIM-029's direct-state emitter branch is a selected pilot, not a
  permanent hardcode. MIM-031..MIM-034 now track the required residue audit,
  same-family inventory, substrate selection, and closeout. Adding more by-name
  direct-state callsite branches is forbidden unless MIM-033 first promotes the
  path to fact-driven DirectState method lowering.
- 2026-05-31: `@rune` parser surface is now default-on. Historical
  `NYASH_FEATURES=rune` usage remains compatible, but mimalloc proof apps and
  direct-state diagnostics should no longer require it in their commands.
- 2026-05-31: MIM-029 landed the first narrow direct-state lowering. The
  selected `alloc_result.recordSuccess(selected_kind)` callsite in
  `objectLifecycleSmallAlloc` now emits DirectSlot offset stores/RMWs directly
  in the caller when the direct exact front is active. This removes the selected
  same-module `recordSuccess/1` call boundary without changing source, public
  ABI, default/safe behavior, provider activation, allocator replacement, hooks,
  or global allocator state. MIM-030 is the next measurement/owner refresh.
- 2026-05-31: MIM-023..MIM-026 closed as the source-shape direct-state audit.
  MIM-027 adds a real metadata-only `StateRepr::Direct` producer
  (`direct_state_plans`) derived from typed `field_decls`. MIM-028 selects the
  first lowering guard as `HakoAllocObjectLifecycleAllocResult.recordSuccess/1`
  because it has a positive all-primitive direct-state plan and remains hot in
  the post-DirectArray perf top. MIM-029..MIM-030 stay open.
- 2026-05-31: Rows 388-413 are historical DirectArray / RuntimeDataBox /
  helper-cache closeout evidence. Row414 returned the lane to mimalloc
  source-level work. Row415 keeps `object_lifecycle_facade` as the active owner
  surface. Continue inside this workstream instead of opening inventory-only
  rows.
- 2026-05-31: MIM-001 source-shape inventory completed. The next owner should
  be selected from `objectLifecycleSmallAlloc`, cached release, or realloc-grow
  source shape. Observer/result readback methods stay out of the first source
  optimization candidate.
- 2026-05-31: MIM-003..MIM-007 completed as a narrow source-shape cleanup, not
  a perf keeper. `objectLifecycleSmallAlloc` now binds `alloc_result` before
  reset and calls `alloc_result.reset()` directly. This removes the remaining
  facade result helper call from the small-alloc helper-family probe, but does
  not materially improve the exact-EXE timing or MIR copy owner. Do not open a
  durable row from this cleanup.
- 2026-05-31: MIM-009 closed as a Ghost Task commit. Keep the source-shape
  cleanup recorded here and avoid spawning a new row or dedicated guard for it.
- 2026-05-31: MIM-010 closed as a page-queue delegation cleanup. The small-alloc
  entry now hands page selection back to `queue.selectPage()` instead of
  branching on page count in the facade. Helper-call count dropped, and the
  remaining helper family is still page-hotpath owned; do not treat this as a
  perf keeper.
- 2026-05-31: MIM-011 closed as a narrow source-shape keeper. After the page
  queue has selected an available page, `objectLifecycleSmallAlloc` now uses
  `page.acquireFreshSmall(size)` instead of the more generic
  `page.acquire_usize(size)`. This keeps the page-model owner boundary intact
  while avoiding the extra generic acquire fallback shape in the selected hot
  region.
- 2026-05-31: MIM-012 closed as a source-shape keeper inside the result
  capsule boundary. `resetAttempt()` combines the hot reset-plus-attempt
  transition without inlining result fields into the facade. This keeps capsule
  ownership intact while removing one public method call from the selected
  small-alloc region.
- 2026-05-31: MIM-013 closed as a source-shape keeper. The acquire failure path
  now relies on the existing `resetAttempt()` `last_block_id=-1` sentinel, and
  `recordBlock(block_id)` runs only after `block_id >= 0`. This removes the
  redundant failed-acquire block publication without changing public failure
  observation.
- 2026-05-31: MIM-014 refreshed the gap against C. Current hako remains about
  107x higher in instructions and 162x higher in cycles than the explicit C
  mimalloc runner for the same 524288 alloc/free count. perf owner is no longer
  a narrow source-shape cleanup by itself: legacy typed-object field helpers
  dominate, with Array safe store/load as the secondary owner.
- 2026-05-31: MIM-015 showed that MIM-014 used the default/safe front, not the
  intended DirectSlot / DirectArray exact front. With
  `HAKO_TYPED_OBJECT_STORE=direct_slot_exact` and
  `HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact`, legacy `field_*` and
  `exact_slot_*` symbols disappear from the EXE, body time is 13ms for the same
  524288 alloc/free count, and the remaining gap to C is about 5.68x
  instructions / 4.26x cycles. Treat the direct exact front as the next owner
  baseline.
- 2026-05-31: MIM-016 locks the measurement split. `direct exact` is the .hako
  mimalloc parity optimization baseline. `default/safe` remains the public /
  fallback compatibility reference and is excluded from parity owner selection
  unless a later explicit public-front lane selects it.
- 2026-05-31: MIM-017 selected and landed a narrow DirectArray i64 exact store
  boundary cut for proven `HakoAllocPageModel` direct exact store sites.
  `nyash.array.set_hii` callsites disappear from the object-lifecycle
  small-block EXE. The direct exact measurement moved from 369.63M
  instructions / 78.49M cycles / 13ms body to 269.35M instructions / 58.17M
  cycles / 9ms body. C mimalloc at the same 524288 alloc/free count is 65.10M
  instructions / 18.09M cycles / 3.46ms body, leaving about 4.14x
  instructions / 3.22x cycles / 2.60x body. Source shape, default/safe
  ArrayBox, generic ArraySet, and public ABI remain unchanged.
- 2026-05-31: MIM-018 refreshed the post-store-boundary owner. With ArraySet
  calls gone, the hottest direct exact surface moved to the page queue /
  facade source shape: `selectPage/0`, `releaseLocalKnownLive/1`,
  `selectSinglePageFastPath/0`, and facade small/release methods.
- 2026-05-31: MIM-019 landed as a source-shape keeper. The single-page branch
  is now handled directly in `selectPage()` instead of calling
  `selectSinglePageFastPath()` and then `acceptSelectedPage()`. The exact EXE
  still reports `summary=ok`; instructions moved from about 269.35M to
  256.24M and cycles from about 58.4M to 53.0M. Public queue semantics and the
  generic multi-page path remain unchanged.
- 2026-05-31: MIM-021 landed as a small structural keeper. `selectPage()` now
  performs the hot selection reset directly instead of calling
  `beginSelection()`. Exact EXE remains `summary=ok`; instructions moved from
  about 256.24M to 254.15M. Keep the public `beginSelection()` method for
  non-hot callers and source readability.
- 2026-05-31: MIM-022 is selected as a language-optimization cleanup before
  adding more source hand-expansions. The target source shape is
  `@rune Inline(required) beginSelection()` plus a normal
  `me.beginSelection()` call inside `selectPage()`. For this narrow receiver
  reset helper, `Inline(required)` is enough: the verifier must accept a
  receiver-local `FieldSet` leaf on one stable base and infer `no_alloc` /
  `no_safepoint` from the body shape. `Profile(...)` is parked for v0 and
  should not be introduced unless explicit inline/contract annotations become
  repeated user-facing noise.
- 2026-05-31: MIM-022 landed as a verified source-shape proof. The leaf
  verifier now accepts a single-object field-set body shape, and the mimalloc
  source keeps `beginSelection()` as the readable public entry while the hot
  path stays inline-expanded through the MIR proof lane.
- 2026-05-31: MIM-020 refresh reran the post-selectPage keeper measurement
  (median 550ms across three samples) and rechecked the facade source shape.
  `object_lifecycle_facade` remains the source-level owner surface; the
  immediate leaf candidates are already source-inline or still reject as
  composite/call-heavy, so no new fast path is justified from this refresh.
- 2026-05-31: MIM-020 source refresh narrowed the remaining hot source shapes
  to the composite clusters around `objectLifecycleSmallAlloc` and
  `objectLifecycleReleaseBlock`. Leaf capsules such as `recordSelectedPage`
  and `recordBlock` are already source-inline; the next diagnostic boundary is
  source inventory of those composite hot clusters, not a new fast path.
- 2026-05-31: Direct state work stays internal for v0. The source surface is
  ordinary typed box fields, `DirectArrayI64`, and `@rune Inline(required)`.
  The design/report name is `StateRepr::Direct` / `state_repr=direct_v0`,
  derived from `field_decls` plus storage-class and boundary facts. Do not add
  `record`, `slots`, or `layout` syntax for this lane.

## MIM-001 Source-Shape Inventory

| Surface | Method(s) | Shape | Candidate read |
| --- | --- | --- | --- |
| small allocation | `objectLifecycleSmallAlloc` | resets and updates `alloc_result`, selects queue page, calls `page.acquireFreshSmall`, records last page cache | primary owner candidate; most direct allocation path |
| last-page cache write | `recordLastAllocPage` | writes `last_alloc_page_index`, `last_alloc_page_id`, `last_alloc_page` after successful small alloc | candidate only as part of small allocation, not standalone |
| cached release | `objectLifecycleReleaseDirectCachedPage`, `objectLifecycleReleaseBlock` | checks last allocated page fields, calls `page.releaseLocalKnownLive`, falls back to known-page lookup | secondary owner candidate; already source-shaped for fast path |
| known-page lookup | `objectLifecycleKnownPageIndexById`, `objectLifecycleReleaseKnownPageIndex` | scans `object_lifecycle_queue.pages` when cache misses | fallback surface; optimize only with current perf evidence |
| aligned alloc | `objectLifecycleSmallAllocAligned` | normalizes alignment, then delegates to small alloc | not first owner; mostly wrapper around small allocation |
| realloc shrink | `objectLifecycleReallocShrink`, `validateReallocShrinkPage` | validates page/block state and records success/failure | not first owner unless realloc workload is active |
| realloc grow | `objectLifecycleReallocGrow`, `objectLifecycleReallocGrowFromPage` | validates old block, calls small alloc, then release, records move | candidate only if grow workload is the active perf owner |
| observers/stats | `objectLifecycle*Count`, result getters, `objectLifecycleStatsSnapshot` | readback over queue/result fields | not implementation owner; keep as public observer surface |

Likely owner candidates for MIM-002:

```text
candidate_0=objectLifecycleSmallAlloc
candidate_1=objectLifecycleReleaseDirectCachedPage
candidate_2=objectLifecycleReallocGrowFromPage
fallback_surface=objectLifecycleKnownPageIndexById
observer_surface=objectLifecycle* getters / stats snapshot
```

## MIM-002 Owner Selection

Selected owner:

```text
selected_owner=objectLifecycleSmallAlloc
selected_reason=representative_small_block_workload_enters_facade_through_small_alloc_and_uses_release_realloc_as_secondary_paths
implementation_open=0
fast_path_reopen=0
```

Rejected for first source edit:

| Candidate | Reason |
| --- | --- |
| `objectLifecycleReleaseDirectCachedPage` | secondary release path; current smoke already proves cached release correctness, but it is not the first allocation entry |
| `objectLifecycleReallocGrowFromPage` | composite path built from small alloc plus release; optimize only when realloc workload is active |
| `objectLifecycleKnownPageIndexById` | fallback scan; no source edit without perf evidence that cache misses dominate |
| observers / stats | public readback surface; not a hot source-level owner candidate |

MIM-003 must gather current perf evidence before source edits. The selected
source owner does not reopen Array, RuntimeDataBox, helper, provider,
replacement, hook, or global allocator work.

## MIM-003 Perf Evidence Refresh

Command shape:

```text
hako_exe_memory_runner:
  app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
  workload=representative-object-lifecycle-small-block-v0
  runtime_config=empty
  operation_repeat=1

mir tools:
  tools/allocator/mir_callsite_copy_attribution.py
  tools/allocator/hako_mimalloc_small_alloc_helper_copy_family_probe.py
```

Before source edit:

```text
body_elapsed_ns=550000000
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
instruction_count=153
call_count=12
copy_count=61
phi_count=18
helper_call_count=6
helper_copy_count=22
dominant_callee_family=page_hotpath_helpers
dominant_copy_owner=local_ssa_copy_materialization
callsite_0_callee=acquire_usize
callsite_0_attributed_copy_count=8
helper_family_call_count=5
facade_result_helpers_call_count=1
page_hotpath_helpers_call_count=4
summary=ok
```

Interpretation:

```text
selected_source_boundary=objectLifecycleSmallAlloc.alloc_result_reset_binding
selected_reason=source_has_one_remaining_facade_result_wrapper_call_before_alloc_result_local_binding
fast_path_reopen=0
implementation_open=1
```

## MIM-004 / MIM-005 Narrow Source Cleanup

Changed only:

```text
file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
change=bind alloc_result local before reset and call alloc_result.reset directly
```

After source edit:

```text
body_elapsed_ns=562000000
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
instruction_count=154
call_count=12
copy_count=62
phi_count=18
helper_call_count=5
helper_copy_count=21
dominant_callee_family=page_hotpath_helpers
dominant_copy_owner=local_ssa_copy_materialization
callsite_0_callee=acquire_usize
callsite_0_attributed_copy_count=8
helper_family_call_count=4
facade_result_helpers_call_count=0
page_hotpath_helpers_call_count=4
summary=ok
```

3-sample timing smoke after source edit:

```text
sample_count=3
sample_0_hako_external_elapsed_ms=560
sample_1_hako_external_elapsed_ms=570
sample_2_hako_external_elapsed_ms=550
after_hako_elapsed_median_ms=560
after_hako_elapsed_min_ms=550
after_hako_elapsed_max_ms=570
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Decision:

```text
cleanup_effect=accepted_as_source_shape_cleanup
perf_keeper_claim=0
remaining_owner=local_ssa_copy_materialization_and_page_hotpath_helpers
next_task=MIM-008_or_MIM-009_cleanup_then_resume_owner_selection
```

## Evidence

- Active handoff guard:
  `bash tools/checks/k2_wide_phase296x_mimalloc_source_level_owner_refresh_guard.sh`
- Direct-path closeout guard:
  `bash tools/checks/k2_wide_phase296x_post_directarray_remaining_direct_path_surface_check_guard.sh`
- Current pointer guard:
  `bash tools/checks/current_state_pointer_guard.sh`

### MIM-011 Evidence

Current baseline before the source edit:

```text
sample_count=3
body_elapsed_ns=555000000,555000000,557000000
external_elapsed_ms=560,550,560
summary=ok
```

After switching the selected page acquire route to `acquireFreshSmall`:

```text
sample_count=3
body_elapsed_ns=544000000,544000000,546000000
external_elapsed_ms=550,540,550
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
summary=ok
```

MIR shape after the source edit:

```text
instruction_count=139
call_count=10
copy_count=63
helper_call_count=3
page_hotpath_helpers_call_count=3
dominant_callee_family=page_hotpath_helpers
dominant_copy_owner=local_ssa_copy_materialization
top_callsite_callee=acquireFreshSmall
top_callsite_attributed_copy_count=8
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
```

### MIM-012 Evidence

Baseline before the source edit is the MIM-011 selected-page acquire route:

```text
sample_count=3
body_elapsed_ns=544000000,544000000,546000000
external_elapsed_ms=550,540,550
summary=ok
```

After adding `HakoAllocObjectLifecycleAllocResult.resetAttempt()` and using it
from `objectLifecycleSmallAlloc`:

```text
sample_count=3
body_elapsed_ns=540000000,538000000,538000000
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
summary=ok
```

MIR shape after the source edit:

```text
instruction_count=133
call_count=10
copy_count=61
phi_count=14
helper_call_count=3
helper_copy_count=13
dominant_callee_family=page_hotpath_helpers
dominant_copy_owner=local_ssa_copy_materialization
top_callsite_callee=acquireFreshSmall
top_callsite_attributed_copy_count=8
facade_result_helpers_call_count=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
```

### MIM-013 Evidence

Baseline before the source edit is the MIM-012 reset-attempt capsule cleanup:

```text
sample_count=3
body_elapsed_ns=540000000,538000000,538000000
summary=ok
```

After deferring `recordBlock(block_id)` until after `block_id >= 0`:

```text
sample_count=3
body_elapsed_ns=543000000,536000000,536000000
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
summary=ok
```

MIR shape after the source edit:

```text
instruction_count=132
call_count=10
copy_count=60
phi_count=14
helper_call_count=3
helper_copy_count=11
dominant_callee_family=page_hotpath_helpers
dominant_copy_owner=local_ssa_copy_materialization
top_callsite_callee=acquireFreshSmall
top_callsite_attributed_copy_count=6
facade_result_helpers_call_count=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
```

### MIM-014 Evidence

Current exact-EXE timing after MIM-013:

```text
sample_count=3
hako_body_elapsed_ns=539000000,536000000,540000000
hako_allocation_count=524288
hako_free_count=524288
hako_select_page_single_fast_path_count=524288
hako_release_known_page_fast_path_count=524288
summary=ok
```

Explicit C mimalloc runner, using `--in-process-repeat 8192` to match the same
524288 alloc/free count:

```text
sample_count=3
c_body_elapsed_ns=3946035,3245566,3830324
c_allocation_count=524288
c_free_count=524288
summary=ok
```

One-sample `perf stat` instruction/cycle comparison:

```text
c_instructions=65099825
c_cycles=18404351
c_body_elapsed_ns=3589438
hako_instructions=6947442686
hako_cycles=2973686467
hako_body_elapsed_ns=539000000
instruction_ratio_hako_over_c=106.72
cycle_ratio_hako_over_c=161.58
body_elapsed_ratio_hako_over_c=150.16
```

Current hako `perf report --no-children --sort=symbol,dso` top symbols:

```text
nyash.object.field_set_hii=25.46%
nyash.object.field_get_u64_hii=21.51%
nyash.object.field_get_hii=19.68%
nyash_kernel::plugin::array_slot_backend::safe_store_i64=12.95%
nyash.object.field_set_u64_hiu=12.91%
nyash_kernel::plugin::array_slot_backend::safe_store_i64::closure=4.64%
nyash_kernel::plugin::array_slot_backend::safe_load_encoded_i64=1.63%
array_handle_cache_get_index_encoded_i64_closure=0.93%
```

Interpretation:

```text
primary_owner=legacy_typed_object_field_helper_surface
secondary_owner=public_arraybox_safe_store_load_surface
source_shape_cleanup_remaining=not_primary_without_new_owner_evidence
new_fast_path_open=0
next_task=classify_why_legacy_field_helpers_remain_hot_before_more_source_edits
```

### MIM-015 Evidence

Default/safe front from MIM-014:

```text
hako_default_body_elapsed_ns=539000000,536000000,540000000
hako_default_instructions=6947442686
hako_default_cycles=2973686467
primary_owner=legacy_typed_object_field_helper_surface
secondary_owner=public_arraybox_safe_store_load_surface
```

Direct exact front:

```text
env.HAKO_TYPED_OBJECT_STORE=direct_slot_exact
env.HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact
sample_count=3
hako_direct_body_elapsed_ns=13000000,13000000,13000000
hako_direct_allocation_count=524288
hako_direct_free_count=524288
hako_direct_select_page_single_fast_path_count=524288
hako_direct_release_known_page_fast_path_count=524288
summary=ok
```

Direct exact `perf stat`:

```text
hako_direct_instructions=369629325
hako_direct_cycles=78489843
hako_direct_body_elapsed_ns=13000000
c_instructions=65099825
c_cycles=18404351
c_body_elapsed_ns=3589438
instruction_ratio_hako_direct_over_c=5.68
cycle_ratio_hako_direct_over_c=4.26
body_elapsed_ratio_hako_direct_over_c=3.62
```

Direct exact `perf report --no-children --sort=symbol,dso` top symbols:

```text
HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1=22.06%
nyash_kernel::plugin::array_runtime_facade::array_runtime_set_idx_i64=19.96%
HakoAllocObjectLifecyclePageQueue.selectPage/0=17.16%
HakoAllocPageModel.acquireFreshSmall/1=8.72%
HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0=6.01%
Main.runOne/2=5.00%
HakoAllocPageModel.releaseLocalKnownLive/1=4.31%
HakoAllocPageModel.isRetired/0=2.85%
HakoAllocObjectLifecycleReleaseResult.recordSuccess/2=2.82%
HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3=2.16%
```

Symbol check on the direct exact EXE:

```text
legacy_field_symbol_count=0
exact_slot_symbol_count=0
```

Interpretation:

```text
measurement_front_correction=required
default_safe_front_owner=legacy_helpers
direct_exact_front_owner=hako_source_shape_and_array_runtime_set_idx_i64
next_task=consult_design_on_direct_front_baseline_and_next_owner
new_row_required=0
```

## MIM-017 DirectArray I64 Exact Store Boundary Cut

Scope:

```text
front=direct_exact
typed_object_store=direct_slot_exact
array_slot_store=direct_array_i64_exact
selected_owner=array_runtime_set_idx_i64_call_boundary
public_arraybox_abi_changed=0
generic_array_set_changed=0
default_safe_behavior_changed=0
```

Implementation:

```text
direct_array_birth_symbol=nyash.array.direct_i64.birth_h
same_module_direct_store_sites=HakoAllocPageModel.*
legacy_array_set_symbol_call_count_after=0
```

Verification:

```text
python_collection_method_tests=ok
direct_exact_exe_summary=ok
allocation_count=524288
free_count=524288
hako_body_elapsed_ns=9000000
hako_instructions=269353327
hako_cycles=58165812
c_body_elapsed_ns=3460932
c_instructions=65099033
c_cycles=18088685
instruction_ratio_hako_over_c=4.14
cycle_ratio_hako_over_c=3.22
body_elapsed_ratio_hako_over_c=2.60
```

Post-cut perf top:

```text
HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2=14.35%
HakoAllocObjectLifecycleAllocResult.recordSuccess/1=10.58%
HakoAllocPageModel.releaseLocalKnownLive/1=10.09%
HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1=7.64%
HakoAllocObjectLifecyclePageQueue.selectPage/0=7.63%
HakoAllocPageModel.freeCount/0=6.57%
nyash_kernel::plugin::array_runtime_facade::array_runtime_get_idx=3.80%
```

Interpretation:

```text
array_set_boundary_cut=keeper
array_set_micro_lane_reopen=0
next_task=MIM-018 post-store-boundary perf owner refresh
```

## MIM-018 DirectArray FastPathPlan Phase Split

Decision:

```text
source_surface_changed=0
direct_block_syntax_open=0
profile_syntax_open=0
fastpath_owner=mir_metadata
c_shim_method_name_owner=temporary_bridge_only
```

Phase plan:

```text
phase_1=DirectArrayAccessPlanV0 checked metadata
phase_2=lowerer consumes checked DirectArrayAccessPlanV0
phase_3=proved_unchecked range_index and stack_top_pop proofs
phase_4=caller_precondition proof for releaseLocalKnownLive
phase_5=remove C shim by-name unchecked allowlist
```

Phase 1 implementation:

```text
metadata=direct_array_access_plans
derived_from=generic_method_routes
op=load|store
array_kind=DirectArrayI64
element_type=i64
bounds_policy=checked
proof_kind=exact_front_contract
fallback_policy=allow_checked
cfg_shape=checked_branching
store_semantics=not_store|append_or_overwrite
checked_plan_cfg_safety=successor_phi_safe
checked_plan_successor_phi_site_excluded=1
lowering_changed=0
unchecked_changed=0
```

Phase 2 implementation:

```text
consumer=llvm_c_shim_checked_direct_array_get_set
selection_source=fn_metadata.direct_array_access_plans
receiver_origin_rescan_removed_for_checked_path=1
consumer_requires_result_value_match=1
unchecked_by_name_allowlist_changed=0
lowering_behavior_expected_same=1
```

Phase 3 guardrail:

```text
proved_unchecked_cfg_shape=branchless
range_index_store_semantics=append_or_overwrite
range_index_store_requires_sequential_zero_based_index=1
capacity_only_store_proof_allowed=0
append_or_overwrite_unchecked_requires_len_max_update=1
```

Phase 3a implementation:

```text
proof_kind=range_index
selection_source=fn_metadata.direct_array_access_plans
cfg_shape=branchless
store_semantics=append_or_overwrite
required_range_index_facts=lower_const_zero,index_value_matches_route_key,step_one,end_exclusive,index_body_read_only
required_extent_fact_v0=constant_upper_bound_within_direct_array_i64_default_capacity
dynamic_extent_bounds=checked_until_direct_array_extent_fact
lowering=len_preserving_branchless_store
legacy_by_name_allowlist_changed=0
```

Phase 3a follow-up:

```text
range_index_fact_view=implemented_from_loop_range_fact
counting_loop_fact_producer=implemented_for_strict_tail_increment_shape
direct_array_extent_fact_surface=implemented_consumer_and_json_emit
direct_array_extent_fact_producer=implemented_for_same_receiver_array_field_capacity_range
resetToFresh_source_shape=while_style_counting_loop
resetToFresh_requires=direct_array_access_plan_reread_against_generated_metadata
resetToFresh_metadata_reread=ok
resetToFresh_range_index_fact_count=1
resetToFresh_direct_array_extent_fact_count=3
resetToFresh_proved_unchecked_direct_array_store_count=3
```

Phase 2 verification:

```text
rust_unit_direct_array_access_plan=ok
rust_unit_direct_array_access_plans_json=ok
cargo_check=ok
c_shim_build=ok
representative_direct_exact_exe_smoke=ok
direct_array_access_plan_function_count=8
direct_array_access_plan_site_count=13
dev_gate_quick=ok
excluded_reason=checked_direct_array_lowering_splits_blocks_and_successor_phi_requires_proved_unchecked_or_cfg_rewrite
next_phase=proved_unchecked_range_index_and_stack_top_pop_proofs
```

Smoke repair follow-up:

```text
exact_numeric_runtime_check_json=implemented
exact_numeric_runtime_check_ny_llvmc_exe=nyrt_assert_helper_no_cfg_split
direct_array_i64_push_transport=implemented_for_seedFreeBlocks_push_i64
representative_direct_exact_exe_smoke=ok
summary_fields=33254,33792,0,64,0,0
```

Python LLVM metadata-consumer cleanup:

```text
selection_source=fn_metadata.direct_array_access_plans
python_lowerer_method_name_allowlist_removed=1
proved_unchecked_range_index_branchless_store_consumer=implemented
representative_direct_exact_exe_smoke=ok
c_shim_by_name_unchecked_bridge_still_remaining=1
remaining_c_shim_targets=acquireFreshSmall_get_store,releaseLocalKnownLive_store
next_phase=stack_top_pop_and_caller_precondition_proofs_before_c_shim_bridge_removal
```

C shim stack_top_pop proof migration:

```text
proof_kind=stack_top_pop
producer=direct_array_access_plan
covered_methods=HakoAllocPageModel.acquireFreshSmall/1,HakoAllocPageModel.acquire_usize/1
metadata_reread=ok
load_plans_proved_unchecked_stack_top_pop=2
store_plans_proved_unchecked_stack_top_pop=2
c_shim_unchecked_get_by_name_removed=1
c_shim_acquire_unchecked_store_by_name_removed=1
temporary_release_by_name_bridge_remaining=1
remaining_c_shim_target=HakoAllocPageModel.releaseLocalKnownLive/1
rust_unit_direct_array_access_plan=ok
cargo_check=ok
representative_direct_exact_exe_smoke=ok
next_phase=caller_precondition_proof_for_releaseLocalKnownLive
```

C shim caller_precondition proof migration:

```text
proof_kind=caller_precondition
producer=direct_array_access_plan
covered_method=HakoAllocPageModel.releaseLocalKnownLive/1
covered_surface=block_used[block_id]=0,local_free[local_free_top]=block_id
metadata_reread=ok
store_plans_proved_unchecked_caller_precondition=2
c_shim_by_name_unchecked_bridge_removed=1
c_shim_unchecked_target_selection_source=fn_metadata.direct_array_access_plans
remaining_c_shim_by_name_unchecked_bridge=0
rust_unit_direct_array_access_plan=ok
cargo_check=ok
representative_direct_exact_exe_smoke=ok
next_phase=perf_reread_and_next_owner_selection
```

Post fastpath perf reread:

```text
front=direct_exact
hako_body_elapsed_ns=5000000
c_body_elapsed_ns=3583434
body_elapsed_ratio_hako_over_c=1.395
hako_instructions=147159262
hako_cycles=26847820
c_instructions=65099839
c_cycles=18507393
instruction_ratio_hako_over_c=2.26
cycle_ratio_hako_over_c=1.45
legacy_field_helper_symbols_hot=0
array_runtime_set_idx_i64_hot=0
selected_next_owner=objectLifecycleReleaseBlock_source_shape
selected_reason=direct_array_and_direct_state_fastpath_surfaces_are_no_longer_top_owner
```

Release direct cached source-shape cleanup:

```text
selected_change=collapse_single_use_objectLifecycleReleaseDirectCachedPage_into_objectLifecycleReleaseBlock
reason=single_use_hot_helper_boundary;direct_cached_release_is_the_common_release_path
source_public_api_changed=0
compiler_fastpath_changed=0
representative_direct_exact_exe_smoke=ok
hako_body_elapsed_ns=5000000
hako_instructions=139819482
hako_cycles=25869327
instruction_delta_vs_post_fastpath=-7339780
cycle_delta_vs_post_fastpath=-978493
instruction_keeper=accepted
cycle_keeper=accepted
post_cleanup_top_symbols=objectLifecycleReleaseBlock,objectLifecycleSmallAlloc,acquireFreshSmall,releaseLocalKnownLive,selectPage
next_phase=post_source_shape_reread_and_next_owner_selection
```

Page queue residue cleanup:

```text
selected_change=collapse_single_use_acceptSelectedPage_into_selectPage
selected_residue=delete_dead_selectSinglePageFastPath
reason=thin_queue_wrapper_and_dead_fastpath_residue
source_public_api_changed=0
compiler_fastpath_changed=0
representative_direct_exact_exe_smoke=ok
hako_body_elapsed_ns=4000000
hako_instructions=139294874
hako_cycles=25535067
instruction_delta_vs_post_release_cleanup=-524508
cycle_delta_vs_post_release_cleanup=-329735
instruction_keeper=accepted
cycle_keeper=accepted
post_cleanup_top_symbols=objectLifecycleSmallAlloc,objectLifecycleReleaseBlock,acquireFreshSmall,releaseLocalKnownLive,selectPage
next_owner_candidate=objectLifecycleSmallAlloc_source_shape
next_reason=release and queue thin wrappers are gone; remaining hot owner is the small alloc body itself
```

Inline required reread for small-alloc tail:

```text
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
candidate_tail=recordLastAllocPage/3
source_manual_inline=0
reason=recordLastAllocPage already carries @rune Inline(required)
optimized_mir_recordLastAllocPage_call_present=0
representative_direct_exact_exe_smoke=ok
hako_body_elapsed_ns=4000000
hako_instructions=139296109
hako_cycles=25636139
perf_top_symbol_0=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
perf_top_symbol_0_pct=47.43
decision=no_source_manual_inline
next_owner_candidate=objectLifecycleSmallAlloc_body_or_page_method_boundary
summary=ok
```

SelectPage return type recovery:

```text
selected_change=annotate_HakoAllocObjectLifecyclePageQueue.selectPage_return_type
source_change=selectPage(): HakoAllocPageModel
reason=objectLifecycleSmallAlloc_page_receiver_was_RuntimeDataBox_union
small_alloc_page_reuse_callee_before=RuntimeDataBox.reuse
small_alloc_page_acquire_callee_before=RuntimeDataBox.acquireFreshSmall
small_alloc_page_reuse_callee_after=HakoAllocPageModel.reuse
small_alloc_page_acquire_callee_after=HakoAllocPageModel.acquireFreshSmall
manual_inline=0
representative_direct_exact_exe_smoke=ok
hako_body_elapsed_ns=5000000
hako_instructions=139295829
hako_cycles=25492114
structural_keeper=accepted
performance_keeper=neutral
next_owner_candidate=page_method_boundary_or_page_body_directness
summary=ok
```

Page method return type cleanup:

```text
selected_change=annotate_hot_page_method_return_types
methods=acquireFreshSmall:i64,reuse:i64,releaseLocalKnownLive:i64
reason=make_page_method_contract_explicit_after_selectPage_return_type_recovery
mir_call_result_dst_type_changed=0
representative_direct_exact_exe_smoke=ok
hako_body_elapsed_ns=4000000
hako_instructions=139295615
hako_cycles=25246959
performance_keeper=neutral
source_contract_keeper=accepted
next_owner_candidate=page_body_direct_state_or_copy_cleanup
summary=ok
```

MIM-055 post-direct-memory owner refresh / DirectArray plan consumer:

```text
front=direct_exact
typed_object_store=direct_slot_exact
array_slot_store=direct_array_i64_exact
baseline_before_hako_body_elapsed_ns=7000000
baseline_before_c_body_elapsed_ns=3375038
baseline_before_body_elapsed_ratio=2.074
perf_owner_before=array_runtime_set_idx_i64
perf_owner_before_pct=22.74

root_cause=DirectArrayAccessPlan existed for resetToFresh loop stores but same-module/generic consumers still fell through to nyash.array.slot_store_hii
selected_fix=consume proved_unchecked range_index DirectArrayAccessPlan as site authority
method_name_special_case_added=0
source_public_api_changed=0
direct_block_syntax_added=0

structural_check=ok
resetToFresh_loop_store_sites=3
resetToFresh_slot_store_hii_before=3
resetToFresh_slot_store_hii_after=0
resetToFresh_direct_array_i64_range_index_after=3
remaining_slot_store_hii_calls=1

representative_direct_exact_exe_smoke=ok
hako_body_elapsed_ns=4000000
c_body_elapsed_ns=3317189
body_elapsed_ratio=1.206
performance_keeper=accepted

post_fix_perf_top_0=HakoAllocObjectLifecyclePageQueue.selectPage/0
post_fix_perf_top_0_pct=35.62
post_fix_perf_top_1=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
post_fix_perf_top_1_pct=23.36
post_fix_perf_top_2=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
post_fix_perf_top_2_pct=19.16
next_owner_candidate=selectPage_body_directness_or_queue_state_shape
summary=ok
```

MIM-056 single-active-page small-alloc selection route:

```text
front=direct_exact
selected_owner=HakoAllocObjectLifecyclePageQueue.selectPage/0
selected_reason=post_MIM055_perf_top_selected_single_page_active_success_branch
selected_change=objectLifecycleSmallAlloc_tries_queue.trySelectSingleActivePage_before_public_selectPage_fallback
public_selectPage_semantics_changed=0
fallback_miss_accounting_changed=0
source_hand_expand_full_selectPage=0
compiler_feature_added=0
direct_block_syntax_added=0

before_perf_event_count=408621989
after_perf_event_count=381674429
before_top_symbol=HakoAllocObjectLifecyclePageQueue.selectPage/0
before_top_pct=35.62
after_top_symbol=HakoAllocObjectLifecyclePageQueue.trySelectSingleActivePage/0
after_top_pct=24.12

representative_direct_exact_exe_smoke=ok
hako_body_elapsed_ns=3000000
c_body_elapsed_ns=3940951
body_elapsed_ratio=0.761
winner_claim=0
performance_keeper=accepted_as_structural_and_single_run_timing_win
next_owner_candidate=trySelectSingleActivePage_body_or_acquireFreshSmall_release_balance
summary=ok
```

MIM-081 direct-exact C gap taxonomy:

```text
front=direct_exact
investigation_scope=worker_plus_local_perf_asm
trusted_latest_hako_body_elapsed_ns=4000000
trusted_latest_c_body_elapsed_ns=4196520
trusted_latest_hako_instructions=111744621
trusted_latest_c_instructions=65100343
trusted_instruction_ratio_hako_over_c=1.72
trusted_cycle_ratio_hako_over_c=1.15

current_hot_symbol_0=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
current_hot_symbol_0_pct=29.95
current_hot_symbol_1=HakoAllocObjectLifecyclePageQueue.trySelectSingleActivePage/0
current_hot_symbol_1_pct=21.24
current_hot_symbol_2=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
current_hot_symbol_2_pct=20.92
current_hot_symbol_3=HakoAllocPageModel.releaseLocalKnownLive/1
current_hot_symbol_3_pct=11.39
current_hot_symbol_4=Main.runOne/2
current_hot_symbol_4_pct=6.22
current_hot_symbol_5=HakoAllocPageModel.acquireFreshSmall/1
current_hot_symbol_5_pct=5.95

c_runner_hot_shape=mi_malloc,memset,pointer_table_store,mi_free,pointer_table_clear,requested_bytes_counter,allocation_count,free_count
c_runner_absent_shape=page_model,queue_selection,result_capsule,last_selected_publication,last_alloc_publication,proof_counters,retirement_checks

legacy_field_helper_symbol_count=0
array_runtime_set_idx_i64_hot=0
remaining_gap_kind=source_semantics_and_publication_shape
remaining_gap_not_selected=direct_array_missing_surface

gap_bucket_0=result_capsule_publication
gap_bucket_0_surface=alloc_result_and_release_result_attempt_selected_block_success_reason_fields
gap_bucket_0_status=public_facade_semantics_or_proof_surface

gap_bucket_1=queue_selection_publication_and_stats
gap_bucket_1_surface=last_selected_index,last_selected_page_id,last_selected_kind,select_count,single_page_fast_path_count
gap_bucket_1_status=proof_observer_surface

gap_bucket_2=page_model_counters_and_retirement_semantics
gap_bucket_2_surface=alloc_count,local_free_count,release_count,requested_bytes,peak_used,retire_checks
gap_bucket_2_status=public_page_model_semantics

gap_bucket_3=same_module_boundaries_and_branch_shape
gap_bucket_3_surface=smallAlloc_to_trySelect/acquireFreshSmall,releaseBlock_to_releaseLocalKnownLive,constant_kind_success_branch
gap_bucket_3_status=optimize_only_with_current_machine_code_owner_evidence

gap_bucket_4=backend_residue
gap_bucket_4_surface=pointer_tag_unmask,overflow_traps,cold_fallback_array_get_symbol_presence
gap_bucket_4_status=secondary_not_current_primary_owner

invalid_worker_measurement_note=one_worker_hako_run_used_wrong_front_or_stale_binary;only_c_structural_findings_from_that_run_are_used
next_candidate_0=observer_light_comparison_measurement_for_result_capsule_and_queue_publication
next_candidate_0_condition=comparison_only_or_gate_protected;do_not_silently_change_public_semantics
next_candidate_1=EffectSummary_or_publication_plan_if_result_publication_remains_current_owner
next_candidate_2=stop_chasing_source_micro_bindings_without_new_perf_owner
summary=ok
```

MIM-082 direct-exact measurement env hygiene:

```text
selected_fix=canonical_direct_exact_env_preset_and_pair_runner
env_preset=tools/allocator/mimalloc_direct_exact_env.sh
pair_runner=tools/allocator/hako_mimalloc_direct_exact_pair.sh
required_typed_object_store=direct_slot_exact
required_array_slot_store=direct_array_i64_exact
required_features=rune
required_plugins_disabled=1
required_toml_env_skip=1
worker_front_mismatch_guard=1
manual_env_typing_discouraged=1
new_row_created=0
new_row_specific_guard_created=0
next_candidate=observer_light_comparison_measurement_for_result_capsule_and_queue_publication
summary=ok
```

MIM-083 observer-light comparison app:

```text
selected_fix=comparison_only_observer_light_object_lifecycle_app
observer_light_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-observer-light/main.hako
production_facade_changed=0
page_model_changed=0
provider_activation=0
allocator_replacement=0
hook_installed=0
global_allocator=0
comparison_scope=facade_queue_result_publication_removed_from_hot_body
page_local_semantics_preserved=1
direct_exact_pair_runner=tools/allocator/hako_mimalloc_direct_exact_pair.sh
worker_front_mismatch_guard=1

standard_hako_body_elapsed_ns=4000000
standard_c_body_elapsed_ns=3463799
standard_body_elapsed_ratio=1.155
observer_light_hako_body_elapsed_ns=2000000
observer_light_c_body_elapsed_ns=4514702
observer_light_body_elapsed_ratio=0.443
timing_resolution_note=env.now_ms_body_timing_is_coarse;use_as_directional_not_winner_claim

observed_result=observer_light_removes_large_hot_publication_cost
remaining_page_model_cost_still_present=1
winner_claim=0
next_candidate=perf_stat_or_asm_on_observer_light_to_quantify_instruction_delta
summary=ok
```

MIM-084 observer-light instruction delta:

```text
measurement_tool=tools/allocator/hako_mimalloc_direct_exact_app_perf_stat.sh
front=direct_exact
runs=3
standard_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
observer_light_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-observer-light/main.hako

standard_hako_instructions_median=111391946
standard_hako_cycles_median=20722680
standard_hako_body_elapsed_ns_median=4000000
observer_light_hako_instructions_median=61048806
observer_light_hako_cycles_median=11572983
observer_light_hako_body_elapsed_ns_median=2000000
instruction_delta_standard_minus_observer_light=50343140
cycle_delta_standard_minus_observer_light=9149697
instruction_reduction_pct=45.2
cycle_reduction_pct=44.2

observed_result=facade_queue_result_publication_accounts_for_most_remaining_direct_exact_instruction_gap
comparison_only=1
production_semantics_changed=0
winner_claim=0
next_candidate=separate_page_model_counter_cost_from_remaining_page_local_semantics
summary=ok
```

MIM-085 active-success result helper probe:

```text
candidate=alloc_result.recordActiveBlockSuccess
intent=replace_hot_recordBlock_plus_recordSuccess_2_with_branchless_same_receiver_leaf
production_semantics_changed=0
direct_exact_pair_smoke=ok
before_hako_instructions_median=111391946
after_hako_instructions_median=111391972
instruction_delta=+26
decision=nonkeeper_reverted
reason=tiny_result_method_shape_does_not_change_current_machine_code_owner
next_candidate=hotcore_boundary_instead_of_result_micro_helper
summary=ok
```

MIM-086 hot-core boundary extraction:

```text
selected_change=extract_observer_light_facade_to_object_lifecycle_hot_core_box
new_module=lang/src/hako_alloc/memory/object_lifecycle_hot_core_box.hako
comparison_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-observer-light/main.hako
module_export_added=lang/src/hako_alloc/hako_module.toml
stage1_embedded_module_snapshot_refreshed=1
production_facade_changed=0
public_result_capsule_changed=0
queue_observer_changed=0
page_model_changed=0
purpose=reusable_page_local_small_alloc_release_core_for_direct_exact_optimization
owner_contract_updated=1
module_index_updated=1
expected_perf_effect=neutral_structural_boundary_only
direct_exact_pair_smoke=ok
observer_light_hako_body_elapsed_ns=2000000
observer_light_c_body_elapsed_ns=3219767
observer_light_body_elapsed_ratio=0.621
observer_light_hako_instructions_median=61048900
observer_light_hako_cycles_median=11647625
previous_observer_light_hako_instructions_median=61048806
instruction_delta=+94
decision=structural_keeper_perf_neutral
next_candidate=compose_public_facade_over_hot_core_or_measure_page_model_counter_owner
summary=ok
```

MIM-087 hot-core post-extraction owner refresh:

```text
input_bundle=/tmp/hotcore_trace_bundle
front=direct_exact
target_function=Main.runOne/2
perf_top_0_pct=57.46
perf_top_0_symbol=Main.runOne/2
perf_top_1_pct=27.79
perf_top_1_symbol=HakoAllocPageModel.releaseLocalKnownLive/1
perf_top_2_pct=13.55
perf_top_2_symbol=HakoAllocPageModel.acquireFreshSmall/1
mir_call_count=4
mir_call_0=HakoAllocPageModel.resetToFresh
mir_call_1=HakoAllocObjectLifecycleHotCore.objectLifecycleSmallAlloc
mir_call_2=HakoAllocObjectLifecycleHotCore.objectLifecycleReleaseBlock
mir_call_3=HakoAllocObjectLifecycleHotCore.objectLifecycleReleaseBlock
selected_next_owner=hotcore_wrapper_call_boundary
candidate=inline_or_plan_HakoAllocObjectLifecycleHotCore_small_alloc_release_wrapper
source_manual_inline_selected=0
summary=ok
```

MIM-088 hot-core wrapper inline probe:

```text
candidate=mark_HakoAllocObjectLifecycleHotCore_small_alloc_release_Inline_required
direct_exact_pair_smoke=fail_fast_expected
failure_tag=required-not-verified
objectLifecycleSmallAlloc_reason=expected_one_block_got_11
objectLifecycleReleaseBlock_reason=expected_one_block_got_9
decision=nonkeeper_reverted
reason=current_Inline_required_is_leaf_only;hotcore_wrapper_needs_compiler_wrapper_plan_not_source_rune
next_candidate=small_checked_wrapper_inline_plan_or_hotcore_operation_plan
summary=ok
```

## Next Task Order

Keep `Inline(required)` narrow. It remains the one-block / receiver-local leaf
contract. The hot-core methods below are multi-block direct-exact call
boundaries, not required-inline leaves:

```text
HakoAllocObjectLifecycleHotCore.objectLifecycleSmallAlloc/1
HakoAllocObjectLifecycleHotCore.objectLifecycleReleaseBlock/2
```

The next implementation work must make the compiler-owned plan visible before
lowering changes. `hako_check` is the user-facing explanation surface, but the
truth stays in MIR metadata emitted by the compiler.

- [x] MIM-089: HotCore method summary metadata contract
  - output: `HotCoreMethodSummaryV0` / equivalent MIR metadata for selected
    hot-core callees
  - include: method name, block count, return kind, allocation/provider/public
    observer/materialization counts, dynamic/generic fallback counts, and
    nested direct-exact call counts
  - acceptance: 9-block and 11-block HotCore methods are reportable without
    widening `Inline(required)`
  - result: observer-light MIR JSON emits two summaries:
    `objectLifecycleReleaseBlock/2` has `block_count=9`, `return_kind=scalar_i64`,
    `summary=ok`, `generic_method_fallback_count=0`, and
    `nested_direct_exact_call_count=1`; `objectLifecycleSmallAlloc/1` has
    `block_count=11` with the same accepted scalar/no-fallback shape
  - no lowering change, no source hand-expansion, no new rune/profile syntax
- [x] MIM-090: DirectExactHotCoreCallPlan report-only producer
  - output: `DirectExactHotCoreCallPlanV0` / equivalent MIR metadata for call
    edges such as `Main.runOne/2 -> HotCore` and `HotCore -> PageModel`
  - include: caller, callee, receiver exactness, same-module/static-exact
    dispatch policy, scalar i64 return, generic dispatch count, dynamic route
    count, boxed fallback count, and failure reason when no plan is produced
  - acceptance: report-only plan exists before any static-call lowering
  - result: observer-light MIR JSON emits five report-only plans:
    `Main.runOne/2 -> objectLifecycleSmallAlloc/1`,
    two `Main.runOne/2 -> objectLifecycleReleaseBlock/2` call edges,
    `objectLifecycleSmallAlloc/1 -> acquireFreshSmall/1`, and
    `objectLifecycleReleaseBlock/2 -> releaseLocalKnownLive/1`; all report
    `dispatch_policy=static_exact`, `summary=ok`, and
    `lowering_consumer_enabled=false`
  - no body inline and no benchmark winner claim
- [x] MIM-091: hako_check fastpath-explain plan visibility extension
  - output: extend the existing read-only `hako_check fastpath-explain` surface
    to display HotCore summaries and DirectExactHotCoreCallPlan call edges
  - rule: `hako_check` must not infer optimization truth; it only renders
    compiler MIR metadata and fails strict mode when expected metadata reports
    a missing/fallback route
  - user-facing goal: explain what was optimized, what stayed generic, and why
  - result: `fastpath_explain.py` now reports HotCore summary counts,
    DirectExactHotCore call-plan counts, static-exact dispatch counts, fallback
    counters, and top call-edge rows from compiler-emitted MIR JSON metadata
  - no source rewrite, no MIR emission in the Python adapter, no keeper
    selection
- [x] MIM-092: strict diagnostic for plan-to-fallback mismatch
  - output: fail-fast metadata/report when a direct-exact plan exists but the
    lowering result uses generic dispatch, dynamic route, boxed fallback, or
    unsupported helper route
  - acceptance: planned-but-fallback is visible as a regression before
    performance measurement
  - result: `hako_check fastpath-explain --require-clean` now treats
    `lowering_consumer_enabled=1` plus generic/dynamic/boxed fallback on a
    direct-exact HotCore call plan as unclean and prints
    `direct_exact_plan_fallback_*` rows
  - no static-call lowering yet
- [ ] MIM-093: static exact call lowering consumer
  - output: consume `DirectExactHotCoreCallPlanV0` to lower selected generic
    method calls to static exact symbol calls
  - acceptance: representative direct-exact semantic smoke stays green;
    `hako_check fastpath-explain` shows lowered static-exact call edges and no
    plan-to-fallback mismatch
  - body inline remains out of scope
- [ ] MIM-094: post-static-call perf reread
  - output: direct-exact perf/stat and perf-top reread after MIM-093
  - decide whether remaining owner is PageModel body shape, DirectArray/Span
    proof residue, late hot inline, or no-current-owner
  - no `LateHotInlinePlan` unless current evidence still selects call overhead

## Parking Lot

- Array lane extension backlog remains in
  `docs/development/current/main/design/array-lane-extension-roadmap-ssot.md`.
- RuntimeDataBox route policy archaeology stays historical unless a current
  mimalloc perf pass selects it again.
- DirectArray optional member work stays closed until selected by current
  mimalloc evidence.
