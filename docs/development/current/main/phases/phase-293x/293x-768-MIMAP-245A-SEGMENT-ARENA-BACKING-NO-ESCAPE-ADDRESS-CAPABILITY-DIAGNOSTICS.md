# 293x-768 MIMAP-245A Segment Arena Backing No-Escape Address Capability Diagnostics

Status: landed
Date: 2026-05-19

## Decision

Add observer-only diagnostics for the MIMAP-244A no-escape address capability
inventory before closing out the family.

## Context

MIMAP-244A should inventory owner/lifetime/address-carrier facts and reject
escape or closed-substrate requirements without creating real pointer
residence. The next row should summarize those counters and last report facts
before closeout.

## Scope

- Observer-only diagnostic report for MIMAP-244A counters and last report facts.
- Summary flags for matrix/lifetime/address/escape rejects.
- Summary flags for real pointer residence, pointer lookup, arena backing,
  segment-map, atomic bitmap, OSVM/page-source, worker/provider, and backend
  matcher requirement rejects.
- L2 validation only; L3 evidence is deferred to a future closeout pack.

## Stop Lines

- No real raw pointer residence.
- No pointer-derived lookup.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_no_escape_address_capability_diagnostics_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-245A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Scope

- Added the no-escape address capability diagnostics owner and typed report.
- Added a proof app that observes MIMAP-244A counters and last capability
  report facts without recording new capability rows from the diagnostic owner.
- Added the MIMAP-245A L2 guard, proof manifest entry, check index entry, and
  diagnostics SSOT.

## Selected Next Row

MIMAP-245A selects:

```text
MIMAP-246A segment arena backing no-escape address capability closeout pack
```

Reason:

```text
the no-escape address capability inventory and observer diagnostics are now
present. Close out the family with representative exact-MIR L3 evidence before
selecting the next allocator bridge.
```
