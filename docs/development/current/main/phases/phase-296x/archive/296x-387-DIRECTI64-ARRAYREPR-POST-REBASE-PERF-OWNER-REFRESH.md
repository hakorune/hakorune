---
Status: Landed
Date: 2026-05-30
Scope: refresh the hot-owner classification after the ArrayRepr rebase smoke and decide whether the lane should open the optional next DirectArray family member selection.
Blocker: LEGACY-HELPER-CACHE-OWNER-SELECTION-AFTER-ARRAYREPR-REBASE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-386-DIRECTI64-ARRAYREPR-MATERIALIZATION-SMOKE-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-385-DIRECTI64-ARRAYREPR-LOWERING-CONSUMER-REBASE.md
  - docs/development/current/main/phases/phase-296x/296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD.md
  - tools/allocator/direct_i64_arrayrepr_post_rebase_perf_owner_refresh.py
  - tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_post_rebase_perf_owner_refresh_guard.sh
---

# 296x-387 DirectI64 ArrayRepr Post-Rebase Perf Owner Refresh

## Purpose

Reclassify the hot owner after the rebased `ArrayRepr::DirectI64` smoke.

If the DirectArray path still dominates the perf callgraph, the lane can move
to the optional next DirectArray family member selection rather than reopening
helper micro-optimization. If the legacy helper/cache surface still dominates,
the next row must classify that surface before any new fast path implementation.

## Contract

```text
output_contract=direct-i64-arrayrepr-post-rebase-perf-owner-refresh-v0
input_contract=direct-i64-arrayrepr-materialization-smoke-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
attribution_source=perf_callgraph
selected_method=HakoAllocPageModel.acquire_usize/1
direct_array_backend_store_pct=...
direct_array_backend_load_pct=...
direct_array_backend_direct_op_pct=...
direct_array_backend_total_pct=...
legacy_field_helper_pct=...
legacy_array_helper_pct=...
arraybox_public_helper_pct=...
legacy_hash_pct=...
legacy_helper_cache_total_pct=...
hako_method_pct=...
direct_array_dominates_legacy_helper_cache=0|1
optional_next_member_open=0|1
selected_boundary=directarray_family_optional_next_member_selection|legacy_helper_cache_owner_selection_after_arrayrepr_rebase
next_diagnostic=directarray_family_optional_next_member_selection|legacy_helper_cache_owner_selection_after_arrayrepr_rebase
selected_next=directarray_family_optional_next_member_selection|legacy_helper_cache_owner_selection_after_arrayrepr_rebase
selected_reason=direct_array_path_still_dominant_after_arrayrepr_rebase_smoke|legacy_helper_cache_still_dominant_after_arrayrepr_rebase_smoke
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The perf report is only used to classify the owner, not to reopen helper
micro-optimization. If the DirectArray path remains dominant after the
ArrayRepr rebase smoke, the next row is the optional next member selection.
If the legacy helper/cache surface remains dominant, the next row is
legacy-helper-cache owner selection.

## Mini Task Board

This row is a measurement/classification row. Keep it docs/report only: do not
change lowering, runtime helpers, or DirectArray storage while completing these
tasks.

### DA-PR-001: Input Evidence Check

Purpose:
Confirm that the row has a usable perf report for the same workload and lane.

Input:
- a `perf report` text file for `representative-object-lifecycle-small-block-v0`
- row386 smoke already landed
- current state points at this row as the active blocker

Output:
- identified perf report path:
  `target/perf_state/row387-post-rebase-real/perf.report`
- workload identity confirmed in notes or report filename
- no code changes

Acceptance:
- row386 remains `Status: Landed`
- row387 remains `Status: Current` while measurement is open, then becomes
  `Status: Landed` once the real report selects the next docs-first row
- report is from the post-ArrayRepr-rebase build
- if no real report exists, use the guard fixture only and stop before closeout

Forbidden:
- no synthetic report for closeout
- no optimization implementation
- no new DirectArray member selection yet

### DA-PR-002: Generate Owner Refresh Report

Purpose:
Run the row tool and produce the key/value owner report.

Input:
- perf report path from `DA-PR-001`
- `tools/allocator/direct_i64_arrayrepr_post_rebase_perf_owner_refresh.py`

Output:
- report with
  `output_contract=direct-i64-arrayrepr-post-rebase-perf-owner-refresh-v0`
- pct fields for DirectArray, legacy field helpers, legacy array helpers,
  public ArrayBox helpers, hash, and Hako method symbols
- generated report path:
  `target/perf_state/row387-post-rebase-real/summary.out`

Acceptance:
- command exits 0
- report contains `summary=ok`
- report contains exactly one `selected_next`
- report keeps `optimization_open=0`

Forbidden:
- no hand-edited pct values
- no perf winner claim
- no helper/cache retirement in this task

### DA-PR-003: Classify Next Boundary

Purpose:
Read the report and choose only the next row boundary.

Input:
- owner refresh report from `DA-PR-002`

Output:
- one selected boundary:
  `legacy_helper_cache_owner_selection_after_arrayrepr_rebase` when the legacy
  helper/cache surface still dominates, otherwise
  `directarray_family_optional_next_member_selection` when DirectArray still
  dominates

Acceptance:
- if `direct_array_dominates_legacy_helper_cache=1`, then
  `optional_next_member_open=1`
- if `direct_array_dominates_legacy_helper_cache=0`, then
  `optional_next_member_open=0`
- the decision is recorded in this card or the closeout note

Forbidden:
- no subjective selection without report counters
- no ArrayRepr redesign in this row
- no broad source reading before the report classification

### DA-PR-004: Guard And Pointer Check

Purpose:
Prove the row contract and current pointers still match.

Input:
- updated row387 card if the report/decision was recorded
- current state file
- check index

Output:
- passing guard output

Acceptance:
- `bash tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_post_rebase_perf_owner_refresh_guard.sh`
  passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

Forbidden:
- no stale current pointer
- no unindexed guard/tool addition
- no hidden env toggle

### DA-PR-005: Closeout Or Stop

Purpose:
Finish the row only when the evidence is real and the next boundary is clear.

Input:
- owner refresh report
- guard results

Output:
- either close row387 and open the next row, or leave row387 current with a
  short stop note explaining the missing evidence

Acceptance:
- closeout only if real post-rebase perf evidence exists
- next row is docs-first
- `CURRENT_STATE.toml` latest-card/current-blocker fields are updated only at
  closeout

## Evidence

The real post-rebase perf report was captured at:

- [perf.report](</home/tomoaki/git/hakorune-selfhost/target/perf_state/row387-post-rebase-real/perf.report>)
- [summary.out](</home/tomoaki/git/hakorune-selfhost/target/perf_state/row387-post-rebase-real/summary.out>)

The actual classification keeps the legacy helper/cache surface dominant:

```text
direct_array_backend_total_pct=0.00
legacy_helper_cache_total_pct=76.34
array_slot_backend_safe_pct=21.30
array_handle_cache_pct=0.91
arraybox_runtime_total_pct=22.21
direct_array_dominates_legacy_helper_cache=0
optional_next_member_open=0
selected_boundary=legacy_helper_cache_owner_selection_after_arrayrepr_rebase
selected_next=legacy_helper_cache_owner_selection_after_arrayrepr_rebase
selected_reason=legacy_helper_cache_still_dominant_after_arrayrepr_rebase_smoke
```

DA-PR-005 therefore resolves as closeout to a docs-first owner-selection row,
not to a DirectArray optional member implementation. The optional next member
selection remains closed until a later refresh produces different evidence.

Read-only worker inventory also found that the current report format undercounts
the residual ArrayBox runtime surface: `array_slot_backend::safe_store_i64`,
its closure, `safe_load_encoded_i64`, and the array handle-cache closure appear
in the perf report but are not included in `legacy_helper_cache_total_pct`.
The next row must split legacy typed-object field helpers from public ArrayBox
runtime/helper cost before selecting an implementation owner.

Forbidden:
- no closeout from guard fixture only
- no direct implementation row without a selected next-row card
- no update to broad mirrors unless lane/blocker/restart order changes

## Acceptance

- direct array backend pct is reported
- legacy helper/cache pct is reported
- ArrayBox/public helper pct is reported
- exactly one next owner is selected
- no optimization implementation is made in this row

## Forbidden

- no new DirectArray member implementation
- no broad source rewrite
- no perf winner claim

## Commands

```bash
python3 tools/allocator/direct_i64_arrayrepr_post_rebase_perf_owner_refresh.py --perf-report <perf-report>
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_post_rebase_perf_owner_refresh_guard.sh
```
