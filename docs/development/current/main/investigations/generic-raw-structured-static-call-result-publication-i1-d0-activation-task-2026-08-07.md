# Generic raw structured static-call result publication I1 D0

Status: `design stop after caller-zero I0/R0; production activation remains 0`

Parent evidence:

- `generic-raw-structured-static-call-result-publication-d0-task-2026-08-07.md`
- `generic-raw-structured-static-call-result-publication-i0-r0-implementation-task-2026-08-07.md`
- `docs/reference/mir/generic-loop-stage-matrix.md`

## Goal

Design one whole-source activation path for the natural
`StringHelpers.int_to_str/1` static-call result publication boundary.  The
activation must connect the already landed source demand and the
`CompletedUnifiedValueCallEmissionV1` receipt to the current-owner terminal,
then let the existing local materializer transport the published type to the
GenericLoop verifier.  This row is a design product only; it does not open a
production caller until every owner and rollback boundary is sealed.

## Required authority split

```text
whole-source exact caller/site proof
  -> activation demand
current-owner terminal physical Call
  -> CompletedUnifiedValueCallEmissionV1
publication consumer
  -> type_ctx result type
local materializer
  -> Copy + metadata propagation only
GenericLoop
  -> verifier only
```

The source product remains a locator and cannot carry `ValueId`, Builder
state, or inferred type.  The physical receipt remains the only destination
authority.  No method-name, owner-name, route-string, final metadata, or
runtime-value selector is allowed.

## Mandatory route-transaction audit

Before activation, the candidate composition/attempt transaction must name one
rollback owner for all mutable state it can touch:

- `variable_map` / Binding SSA state;
- `type_ctx` and publication receipts;
- physical instruction stream / block state;
- pending cleanup or error state, if the route reaches it.

Snapshotting only `variable_map` is not sufficient.  A failed attempt must
leave no Call, type fact, or local publication behind.  The audit may choose a
smaller staged transaction, but it must not add a retry or alternate route.

## Activation boundary

Admit exactly one canonical source site first.  Success requires a fresh
strict VM probe to pass the previous `MissingTransientType` boundary and to
show the same destination/type relation in the post-call local path.  Foreign
site, missing catalog, alternate route, failed physical Call, duplicate
publication, and rollback residue remain typed rejects.

Out of scope:

- broad unannotated-call inference;
- nested required-argument result publication;
- GenericLoop backfill or route-specific type guessing;
- retry/fallback or compatibility-route promotion;
- legacy deletion, backend widening, or generic monomorphization.

## Completion evidence

- one sealed activation plan with explicit source/terminal/rollback owners;
- success and rollback fixtures plus focused tests;
- fresh canonical strict VM receipt;
- current-state and Generic reference mirrors updated in the same
  implementation commit;
- no production caller is counted until the receipt and guard prove the
  complete path.
