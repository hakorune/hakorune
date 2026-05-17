# 293x-644 MIMAP-138A Segment Allocation Modeled Local-Free Reuse Ledger Release Apply Route

Status: selected current
Date: 2026-05-18

## Decision

`MIMAP-138A` is the allocator behavior row selected by `MIMAP-137A`.

It should consume a successful `MIMAP-134A` local-free reuse ledger release
report and apply that release to the source local-free reuse ledger:

```text
MIMAP-130A live local-free reuse ledger row
  -> MIMAP-134A release facts row
  -> mark the matching source reuse ledger row non-live
```

This row closes only the scalar ledger liveness contract. It must not become
real segment free, page mutation, or reusable token recycle in the same row.

## Scope

Owner:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako
```

Proof app:

```text
apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-proof/main.hako
```

Guard:

```text
tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_guard.sh
```

The row may add a small apply report type and one source-ledger-owned method
that consumes a successful release report, verifies token/row shape, flips the
matching `live_flags` row from `1` to `0`, decrements `live_count`, and rejects
duplicate, missing, upstream-rejected, and unsupported requests.

## Acceptance Shape

The proof output should make the apply behavior explicit:

```text
apply=1,0,<row_index>,<token>,<live_count_after>
duplicate=0,<duplicate_reason>,<existing_or_row>
missing=0,<missing_reason>,-1
unsupported=0,<unsupported_reason>
reads=-1,-1
inactive=0,0,0,0,0,0,0,0,0,0,0
summary=ok
```

Exact field order can be adjusted in the implementation, but it must be stable
in the guard and distinguish successful apply, duplicate apply, missing or
upstream-rejected release reports, unsupported stop-line request, and inactive
stop-line families.

## Stop Lines

- No real segment allocation/free execution.
- No page-local free-list mutation.
- No direct page array mutation.
- No dependency on `segment_allocation_modeled_ledger_box.hako`,
  `releaseModeledToken`, or `recordModeledConsume`.
- No released-token recycle in this row.
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
bash tools/checks/run_proof_app.sh --only MIMAP-138A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
