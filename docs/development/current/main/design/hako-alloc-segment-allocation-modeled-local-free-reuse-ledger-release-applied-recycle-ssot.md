---
Status: SSOT
Decision: accepted
Date: 2026-05-18
Scope: MIMAP-142A segment allocation modeled local-free reuse ledger release-applied recycle proof.
Related:
  - docs/development/current/main/phases/phase-293x/293x-650-MIMAP-142A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLIED-RECYCLE-PROOF.md
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-closeout-ssot.md
---

# Hako Alloc Segment Allocation Modeled Local-Free Reuse Ledger Release-Applied Recycle SSOT

## Decision

`MIMAP-142A` proves the source local-free reuse ledger can recycle a modeled
reuse token only after the previous matching source row has been release-applied
and marked non-live.

The accepted seam is:

```text
record local-free reuse source row
  -> record release facts for that row
  -> apply release facts to the source row
  -> source row becomes non-live
  -> same modeled reuse token can be recorded again as a new live row
  -> still-live duplicate remains rejected
```

This is still a scalar modeled ledger contract. It is not real segment free,
page mutation, raw pointer residence, segment-map lookup, OSVM release, provider
activation, or backend lowering.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako
```

The owner already keeps `findIndex(...)` live-row aware and `tokenAt(...)` /
`reusedBlockAt(...)` fail-fast for non-live rows. `MIMAP-142A` fixes that as a
guarded contract.

## Proof

```text
apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-applied-recycle-proof/
tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_applied_recycle_guard.sh
tools/checks/run_proof_app.sh --only MIMAP-142A
```

## Stop Lines

- no real segment allocation/free execution
- no page-local free-list mutation beyond existing modeled reports
- no direct page array mutation
- no dependency on the bump-shaped modeled allocation ledger
- no raw pointer residence
- no segment-map pointer membership or lookup
- no arena backing allocation
- no atomic bitmap execution
- no page-source or OSVM execution
- no real thread scheduling or worker spawning
- no source-level concurrency feature changes
- no provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`
- no backend `.inc` app/name matcher

## Next

```text
MIMAP-143A segment allocation modeled local-free reuse ledger release-applied recycle closeout guard
```
