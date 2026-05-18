# 293x-772 MIMAP-249A Segment Arena Backing Modeled No-Escape Address Residence Diagnostics

Status: landed
Date: 2026-05-19

## Decision

Add observer-only diagnostics for the MIMAP-248A modeled no-escape address
residence inventory before closing out the residence family.

## Context

MIMAP-248A should record accepted no-escape address capability reports as
scalar/model residence rows. The next row should summarize the accepted,
missing, rejected, escape, and closed-substrate counters before closeout.

## Scope

- Observer-only diagnostic report for MIMAP-248A counters and last report facts.
- Summary flags for missing / rejected / invalid / escape rejects.
- Summary flags for real pointer residence, pointer lookup, arena backing,
  segment-map, atomic bitmap, OSVM/page-source, worker/provider, and backend
  matcher requirement rejects.
- L2 validation only; L3 evidence is deferred to a future closeout pack.

## Stop Lines

- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_no_escape_address_residence_diagnostics_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-249A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Scope

- Added the modeled no-escape address residence diagnostics owner and typed
  report.
- Added a proof app that observes MIMAP-248A counters and last residence report
  facts without recording new residence rows from the diagnostic owner.
- Added the MIMAP-249A L2 guard, proof manifest entry, check index entry, and
  diagnostics SSOT.

## Selected Next Row

MIMAP-249A selects:

```text
MIMAP-250A segment arena backing modeled no-escape address residence closeout pack
```

Reason:

```text
the modeled no-escape address residence inventory and observer diagnostics are
now present. Close out the family with representative exact-MIR L3 evidence
before selecting the next allocator bridge.
```
