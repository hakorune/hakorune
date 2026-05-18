# 293x-771 MIMAP-248A Segment Arena Backing Modeled No-Escape Address Residence Inventory

Status: landed
Date: 2026-05-19

## Decision

Add a scalar/model inventory that records an accepted no-escape address
capability as a modeled residence row without creating real raw pointer
residence or pointer-derived lookup.

## Context

MIMAP-244A inventories the no-escape address capability boundary, MIMAP-245A
adds diagnostics, and MIMAP-246A closes out the family with representative L3
evidence. The next bridge should prove that an accepted capability can become a
modeled residence ledger entry while the address carrier remains a scalar
non-dereferenceable token.

## Scope

- Consume a MIMAP-244A no-escape address capability report.
- Record scalar segment / arena / lifetime / address-carrier facts as a modeled
  residence inventory row.
- Reject missing capability, non-accepted reports, escape blockers, and
  requests for pointer lookup / real arena backing / segment-map / atomic /
  OSVM / worker / provider / backend matcher behavior.
- L2 daily validation; L3 evidence is deferred to a future closeout pack unless
  this row introduces a new backend route shape.

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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_no_escape_address_residence_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-248A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Scope

- Added the modeled no-escape address residence inventory owner and typed
  report.
- Added a proof app that records an accepted no-escape address capability as a
  scalar/model residence row while keeping the address carrier
  non-dereferenceable.
- Added the MIMAP-248A L2 guard, proof manifest entry, check index entry, and
  SSOT.

## Selected Next Row

MIMAP-248A selects:

```text
MIMAP-249A segment arena backing modeled no-escape address residence diagnostics
```

Reason:

```text
the modeled residence inventory is present. Add observer-only diagnostics for
its accepted/reject counters before closing out the residence family.
```
