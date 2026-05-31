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
- [ ] MIM-029: narrow direct-state lowering implementation
  - output: direct offset load/store only for the MIM-028 selected group
  - no generic user-box flattening, no public ABI widening, no provider or
    allocator replacement activation
- [ ] MIM-030: post-direct-state owner refresh
  - output: compare direct exact front against the previous baseline and
    choose the next owner from current evidence
  - this is measurement / selection, not the fast-path implementation itself;
    direct-state fast-path calculation belongs to MIM-029 after MIM-028
    selects one proven group
  - return to source-level mimalloc work if direct-state does not produce a
    structural keeper

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

Not opened in this pass.

The current source evidence is enough to narrow the candidate family, but not to
land a broader direct-offset implementation for the full facade.

### MIM-030: post-direct-state owner refresh

Pending.

This is not the direct-state fast path itself. MIM-030 runs only after MIM-029
has installed a selected direct-state fast path, then rereads the direct exact
front and picks the next owner from measured evidence.

Until MIM-028 selects one proven positive-net field group, `object_lifecycle_facade`
remains the active owner surface and the lane stays on source-level mimalloc work.

## Decision Log

- 2026-05-31: `@rune` parser surface is now default-on. Historical
  `NYASH_FEATURES=rune` usage remains compatible, but mimalloc proof apps and
  direct-state diagnostics should no longer require it in their commands.
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

## Parking Lot

- Array lane extension backlog remains in
  `docs/development/current/main/design/array-lane-extension-roadmap-ssot.md`.
- RuntimeDataBox route policy archaeology stays historical unless a current
  mimalloc perf pass selects it again.
- DirectArray optional member work stays closed until selected by current
  mimalloc evidence.
