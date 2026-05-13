# 293x-252 C194b Verifier-Owned Page Lifecycle Invariants

Status: Complete

## Purpose

C194b moves the selected page lifecycle invariants frozen by M207 from proof-only
MIR JSON checks into MIR verifier-owned contracts. This keeps future lifecycle
rows from silently drifting away from the frozen report/function surface that
M208+ will consume.

## Decision

Decision: accepted.

Add a narrow verifier owner:

```text
mir::verification::hako_alloc_page_lifecycle
```

The owner checks only the current M207 lifecycle report/function surface. It
does not inspect allocator algorithms, execute proof apps, or broaden backend
lowering/materialization behavior.

## Row Contract

C194b verifies:

```text
if the M207 lifecycle surface is present:
  required lifecycle functions exist:
    HakoAllocPageLifecycleInvariantObserver.report/17
    HakoAllocPageLifecycleInvariantObserver.observeHeapPage/3
    HakoAllocPurgeStateAwareDecommitGuard.attemptHeapPage/2
    HakoAllocRecommitHeapIntegration.attemptHeapPage/3
    HakoAllocPageModel.acquire/1
    HakoAllocPageModel.releaseLocal/1

  HakoAllocPageLifecycleInvariantReport typed object plan exists

  selected report fields remain strong i64 lanes:
    state
    active
    retired
    decommitted
    recommitted
    acquire_allowed
    decommit_candidate
    recommit_required
    duplicate_decommit_blocked
    marked_generations
    recommitted_generations
```

## Stop Lines

- Do not change allocator behavior.
- Do not broaden the verifier into allocator algorithm proof.
- Do not reopen visible record materialization.
- Do not add packed record backend lowering.
- Do not touch provider activation, hooks, process allocator replacement, or
  `.inc` allocator/provider matching.

## Acceptance

- `MirVerifier::verify_module(...)` runs the C194b lifecycle invariant checker.
- Valid M207 lifecycle owner surfaces pass unchanged.
- Missing lifecycle functions, missing report plan, and malformed lifecycle i64
  fields produce stable verifier errors.
- The existing M207 proof/guard remains green.
- C194b guard stays local-run / index-listed and is not added to quick/dev
  gates.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_page_lifecycle_verifier_invariants_guard.sh
cargo test -q hako_alloc_page_lifecycle
```
