# 293x-650 MIMAP-142A Segment Allocation Modeled Local-Free Reuse Ledger Release-Applied Recycle Proof

Status: landed
Date: 2026-05-18

## Decision

`MIMAP-142A` is the allocator behavior row selected by `MIMAP-141A`.

It should prove that a modeled local-free reuse token whose source reuse ledger
row was release-applied can be recorded again as a new live row in the source
reuse ledger, while a still-live duplicate remains rejected.

## Scope

Owner:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako
```

Proof app:

```text
apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-applied-recycle-proof/main.hako
```

Guard:

```text
tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_applied_recycle_guard.sh
```

The row may add a focused proof app, proof manifest entry, local-run guard, and
small owner helper only if the existing owner cannot expose the contract cleanly.

## Acceptance Shape

The proof should show:

```text
first=1,0,<row0>,<token>,<live_count_after_first>
apply=1,0,<row0>,<token>,<live_count_after_apply>
recycle=1,0,<row1>,<token>,<live_count_after_recycle>
live_duplicate=0,<duplicate_reason>,<row1>
reads=-1,<token>,<block_after_recycle>
inactive=0,0,0,0,0,0,0,0,0,0,0
summary=ok
```

Exact field order can be adjusted, but the guard must distinguish first record,
release apply, recycled record, live duplicate rejection, and post-apply /
post-recycle reads.

## Stop Lines

- No real segment allocation/free execution.
- No page-local free-list mutation beyond the already-modeled source reports.
- No direct page array mutation.
- No dependency on the bump-shaped modeled allocation ledger.
- No raw pointer residence.
- No segment-map pointer membership or lookup.
- No arena backing allocation.
- No atomic bitmap execution.
- No page-source or OSVM execution.
- No real thread scheduling or worker spawning.
- No source-level concurrency feature changes.
- No provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No compiler acceptance broadening in this row; split a sidecar if the proof
  exposes a real compiler blocker.

## Required Evidence

```text
bash tools/checks/run_proof_app.sh --only MIMAP-142A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_applied_recycle_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Result

`MIMAP-142A` landed by adding a proof app, proof-app manifest row, route guard,
SSOT, check-script index entry, and memory README owner note.

No new owner behavior was needed: the existing source reuse ledger owner already
keeps lookup live-row aware after release apply. The proof locks that contract:

```text
first live row accepted
release apply marks row0 non-live
same modeled reuse token records as row1 live
live duplicate is rejected against row1
row0 reads fail fast, row1 reads succeed
```

The row selects:

```text
MIMAP-143A
  segment allocation modeled local-free reuse ledger release-applied recycle closeout guard
```
