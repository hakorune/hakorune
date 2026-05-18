# 293x-657 HAKO-ALLOC-ID-BRAND-002 Allocator Scalar ID Brand First Pilot

Status: landed
Date: 2026-05-18

## Decision

Use the newly accepted direct MIR brand constructor seam for one allocator
source pilot.

This row should apply `SegmentId`, `PageId`, and `BlockId` only where the
current compiler can actually verify the call boundary: a same-box helper with
brand-typed parameters, fed by explicit `BrandName(value)` constructors.

## Scope

- Add top-level allocator ID brand declarations to the focused source file or a
  focused proof source where the pilot lives.
- Brand only the `makeReuseToken(segment_id, page_id, reused_block_id)` call
  boundary in the local-free reuse ledger owner.
- Keep fields, returns, arrays, token storage, and cross-module calls scalar for
  now.
- Preserve existing proof output and allocator behavior.

## Stop Lines

- No field/return/typed-local/cross-module brand inference.
- No broad allocator source rewrite.
- No token brand vocabulary expansion.
- No allocator behavior change.
- No real segment allocation/free execution.
- No page-source or OSVM execution.
- No thread scheduling or worker spawning.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No backend `.inc` matcher.
- No silent fallback.

## Required Evidence

```text
cargo test -q mir_brand_constructor
bash tools/checks/run_proof_app.sh --only MIMAP-142A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Result

`HAKO-ALLOC-ID-BRAND-002` landed the first allocator scalar ID brand pilot in
`segment_allocation_modeled_local_free_reuse_ledger_box.hako`.

The row added `SegmentId`, `PageId`, and `BlockId` declarations and applied
them only to the same-box `makeReuseToken(...)` helper boundary. Storage,
reports, token values, arrays, and behavior remain scalar `i64`.

Evidence:

```text
cargo test -q mir_brand_constructor
bash tools/checks/run_proof_app.sh --only MIMAP-142A
git diff --check
```

Selected next row: `HAKO-ALLOC-ID-BRAND-003` allocator scalar ID brand pilot
closeout guard.
