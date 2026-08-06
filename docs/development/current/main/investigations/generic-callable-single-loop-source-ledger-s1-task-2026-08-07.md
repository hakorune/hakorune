# Callable single-loop source ledger S1

Status: `Decision: closed caller-zero implementation slice`

Parent: `GENERIC-CALLABLE-SINGLE-LOOP-SOURCE-RECIPE-D0`

## Change

Expose one immutable, resolver-owned view over the existing verified callable
forest. The view retains typed source-row membership and owner/origin/source
kind identity for declarations, lexical refs, assignment targets, direct
calls, exits, loop-region lookup, and the existing lambda/capture boundary.
It is a borrowed/query view, not a copied ValueId map and not a Loop policy
issuer.

The compiler Loop projector may later borrow this view to derive the S0
condition/body/rebind subset. This row does not consume the callable ledger,
lower a Recipe, allocate CFG/PHI, or change the raw route.

## Implementation receipt

The resolver module now publishes `CallableSemanticSourceLedgerView` and the
forest-owned `VerifiedCallableLoopMembershipV1`. The view keeps typed
declaration, lexical-ref, assignment-target, direct-call, exit, capture, and
Loop queries; Loop membership pairs the resolver-issued source token with its
derived `LoopExecutionFrameKeyV1`. It has no AST, ValueId map, Loop selector,
Recipe producer, or Builder caller.

Four focused resolver tests cover typed rows/identity, exact Loop lookup and
missing-site rejection, the existing lambda capture boundary, and foreign
owner rejection. The next bounded row is the design stop
`GENERIC-CALLABLE-SINGLE-LOOP-SOURCE-RECIPE-MAP-D0`.

## Contract

- Resolver products remain the sole source authority.
- Statement and expression sites remain typed; `SourceNodeSiteV1` is used only
  for path navigation and never as a membership key.
- A Loop root must be issued by `VerifiedResolvedFunctionV1::resolved_loop_source`
  (or the existing loop-region index), including its frame identity. A raw path
  with a matching suffix is rejected.
- The view exposes all source-row families and explicit dispositions; the
  existing lowering-state `variables`/`assignments` maps remain migration-only.
- S0's three loop receipts are a subset claim and cannot close the whole
  callable ledger.
- Production caller remains zero; no fallback, retry, route selection, or
  physical owner changes are permitted.

## Done

- Positive caller-zero receipt proves owner/origin/source-kind and exact typed
  source-row membership for one callable forest.
- Positive loop lookup proves the root site and `LoopExecutionFrameKeyV1`
  originate from the resolver index.
- Foreign-owner and missing-site cases reject before any Builder effect. The
  sealed resolver index has no node-only lookup or family-skip path; duplicate
  and unsupported/opaque dispositions remain explicit acceptance rows for the
  next mapping design rather than claims of this view.
- Focused tests and the current-state pointer guard are green; source and
  current/reference/workstream docs are updated in the same commit. No
  row-specific shared guard exists yet, so the mapping design remains closed
  until its own guard/acceptance is defined.

## Stop

Return to design if the view needs AST/name lookup, a second resolver, copied
ValueIds, implicit row skipping, or a new Loop policy. Do not proceed to the
source-to-Recipe map, physicalizer, production selection, or legacy deletion
until this view is sealed and the next mapping contract names every operation,
carrier, scope, After, and tail row.
