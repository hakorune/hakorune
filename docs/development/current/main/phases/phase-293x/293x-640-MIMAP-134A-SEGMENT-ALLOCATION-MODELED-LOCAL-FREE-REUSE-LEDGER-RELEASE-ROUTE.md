# 293x-640 MIMAP-134A Segment Allocation Modeled Local-Free Reuse Ledger Release Route

Status: selected current
Date: 2026-05-18

## Decision

`MIMAP-134A` is the allocator behavior row selected by `MIMAP-133A`.

It should add a release route for the dedicated local-free reuse ledger from
`MIMAP-130A`:

```text
successful MIMAP-126A local-free reuse report
  -> MIMAP-130A live reuse ledger row
  -> release the reuse ledger token
  -> live reuse row becomes inactive
```

This row stays scalar and reuse-ledger-local. It must not widen or call the
bump-shaped modeled ledger release route.

## Scope

Owner:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako
```

Proof app:

```text
apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-proof/main.hako
```

Guard:

```text
tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_guard.sh
```

The row may add a release report box and methods on the existing reuse ledger
owner. It may expose scalar observer facts for token, row index, duplicate or
missing release reasons, live before/after, and ledger counts.

## Acceptance Shape

The proof output should make the release behavior explicit:

```text
release=1,0,<row_index>,<token>,<reused_block>,1,0,<count>,<live_after>
duplicate=0,<duplicate_reason>,<row_index>,0
missing=0,<missing_reason>,-1
unsupported=0,<unsupported_reason>
inactive=0,0,0,0,0,0,0,0,0,0,0
summary=ok
```

Exact field order can be adjusted in the implementation, but it must be stable
in the guard and distinguish successful release, duplicate release, missing
token, unsupported stop-line request, and inactive stop-line families.

## Stop Lines

- No real segment allocation/free execution.
- No page-local free-list mutation.
- No direct page array mutation.
- No dependency on `segment_allocation_modeled_ledger_box.hako`,
  `releaseModeledToken`, or `recordModeledConsume`.
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
bash tools/checks/run_proof_app.sh --only MIMAP-134A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
