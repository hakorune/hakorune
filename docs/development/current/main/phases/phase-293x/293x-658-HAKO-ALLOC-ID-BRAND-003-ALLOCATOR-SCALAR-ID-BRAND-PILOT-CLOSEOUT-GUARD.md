# 293x-658 HAKO-ALLOC-ID-BRAND-003 Allocator Scalar ID Brand Pilot Closeout Guard

Status: selected current
Date: 2026-05-18

## Decision

Add a focused closeout guard for the first allocator scalar ID brand pilot.

The guard should protect the small boundary introduced by
`HAKO-ALLOC-ID-BRAND-002` without promoting brands to fields, returns, arrays,
cross-module inference, or token storage.

## Scope

- Add a local-run/index-listed guard for the `HAKO-ALLOC-ID-BRAND-002` pilot.
- Require `SegmentId`, `PageId`, and `BlockId` declarations in the reuse ledger
  owner.
- Require the same-box `makeReuseToken(...)` helper to use brand-typed
  parameters.
- Require the call site to pass explicit `SegmentId(...)`, `PageId(...)`, and
  `BlockId(...)` constructors.
- Re-run the MIMAP-142A proof app to preserve behavior.

## Stop Lines

- No new allocator behavior.
- No field/return/typed-local/cross-module brand inference.
- No broad allocator source rewrite.
- No token brand vocabulary expansion.
- No real segment allocation/free execution.
- No page-source or OSVM execution.
- No thread scheduling or worker spawning.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No backend `.inc` matcher.
- No silent fallback.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_id_brand_first_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
