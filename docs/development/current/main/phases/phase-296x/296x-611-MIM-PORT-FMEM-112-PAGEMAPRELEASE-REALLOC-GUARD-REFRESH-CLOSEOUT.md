---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-112.
Related:
  - docs/development/current/main/phases/phase-296x/296x-610-MIM-PORT-FMEM-111-PAGEMAPRELEASE-SAME-REMOTE-PUBLISH-BODY-CONNECTION.md
  - lang/src/hako_alloc/memory/page_map_release_box.hako
  - apps/mimalloc-pre-realloc-release-invariant-proof/main.hako
  - apps/mimalloc-realloc-same-class-proof/main.hako
  - apps/mimalloc-realloc-alloc-copy-release-proof/main.hako
  - apps/mimalloc-realloc-failure-contract-proof/main.hako
---

# 296x-611 MIM-PORT-FMEM-112 PageMapRelease Realloc Guard Refresh Closeout

## Purpose

Close out the 296x-610 PageMapRelease same/remote publish body connection by
refreshing the realloc/release proof guard pack that consumes the same release
seam.

The release seam now routes through the page-meta same/remote publish helper, so
the M173-M176 proof apps must stay readable and fail close without relying on
giant `&&` summary predicates that hide the first failing invariant.

## Chosen Mode

```text
BoxShape
```

## Required Boundary

```text
do not change page-map ownership rules
do not change realloc same-class or alloc-copy-release behavior
do not open product activation, hooks, global allocator claim, or winner behavior
do not add a new MemOp family
do not restore giant && proof summary conditions
```

## Implementation Shape

```text
Split M173-M176 proof app summary checks into labelled single-condition
ProofCheck.expect(...) rows.

Add guard checks that reject `&&` in those proof apps, so future regressions
fail before the VM executes the app.
```

## Acceptance

```text
M173 pre-realloc release invariant guard stays green
M174 realloc same-class guard stays green
M175 realloc alloc-copy-release guard stays green
M176 realloc failure-contract guard stays green
proof apps expose the first failing label instead of a giant summary condition
PageMapRelease guard remains green after 296x-610
CURRENT_STATE points at the next implementation token after closeout
```

## Non-Goals

```text
changing release/realloc semantics
changing PageMapRelease ownership or unregister ordering
changing page-local state bridge ownership
new product/runtime activation claims
new benchmark keeper claims
```

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_pre_realloc_release_invariant_guard.sh
bash tools/checks/k2_wide_mimalloc_realloc_same_class_guard.sh
bash tools/checks/k2_wide_mimalloc_realloc_alloc_copy_release_guard.sh
bash tools/checks/k2_wide_mimalloc_realloc_failure_contract_guard.sh
bash tools/checks/impl/k2_wide_mimalloc_page_map_release_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed

```text
M173-M176 proof apps now use labelled single-condition expectations for release
and realloc invariants. Their guards reject `&&` in the proof app before
execution, preventing a hidden summary predicate from reintroducing the VM hang
shape found while closing out PageMapRelease same/remote publish body routing.
```

## Closeout

```text
next: 296x-612 hako_alloc next implementation slice selection after release
and realloc guard refresh
```
