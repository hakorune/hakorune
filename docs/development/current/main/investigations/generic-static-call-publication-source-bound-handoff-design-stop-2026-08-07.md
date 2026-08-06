# Generic static-call publication source-bound handoff — design stop

Status: `design stop 2026-08-07; no production caller exists for I1/R0`

## Evidence

The live loop path is:

```text
RawInvocationChildPortV1::lower_loop
  -> PreparedLocatedRawLoopChildEntryV1::lower_with_existing_route_v1
  -> route_loop
  -> route_generic_loop_v1
  -> RecipeComposer::compose_generic_loop_v1_recipe
  -> PlanLowerer::lower
```

The raw child entry verifies condition/body source receipts and then drops
them.  The production Generic route therefore has no
`VerifiedCallableResultActivationPlanV1`, caller ledger, or exact source-site
claim.  It still lowers through the ordinary `PlanLowerer` port, and strict
`None`/lower errors remain `PostEffectRetryDebt` scheduler outcomes.

The only existing receipt-capable bridge is the located test path:

```text
compose_located_generic_loop_v1
  -> VerifiedLocatedCoreLoopPlanV1
  -> CorePlanEffectEmissionPortV1::Claimed
  -> CompletedUnifiedValueCallEmissionV1
  -> static result publication
```

No non-test caller reaches that path.  The current-owner terminal cannot be
selected by a method name or by `StringHelpers` text.

## Decision

Do not wire the located composer directly and do not add a route-local
snapshot.  The next design product must co-seal one source-bound handoff:

```text
whole-source activation owner
  -> exact caller/site claim batch
  -> located GenericLoop plan
  -> physical receipt/publication
  -> outer module candidate rollback
```

The handoff issuer owns the source identity and activation-plan lifetime.  The
located plan may consume the branded claim but may not rediscover source
sites, targets, or types.  The outer module candidate remains the sole
rollback owner.  Any effect-after failure is terminal `Freeze`; it must not
advance the legacy retry schedule.

## Required design questions

1. Which existing whole-source compiler product can install the activation
   plan before `RawInvocationChildPortV1::lower_loop` begins?
2. How is the exact caller/site claim batch borrowed into one loop lowering
   without making route selection or the Builder a second source authority?
3. Which existing function/module completion boundary consumes the handoff
   and preserves candidate rollback on publication or Generic failure?
4. What focused fixture proves the natural
   `StringHelpers.int_to_str/1` initializer without a by-name selector?

The design must answer these with existing typed products or a single new
source-bound owner.  GenericLoop backfill, route-local snapshots, name-based
dispatch, retry/fallback, and broad type inference remain rejected.

## Implementation gate after design close

Only after the handoff is sealed may I1/R0 resume.  The implementation commit
must include the production caller switch, receipt/publication tests, a fresh
strict VM probe crossing `MissingTransientType`, current/reference/workstream
mirrors, and the exact reference-document update.  Until then production
Generic support remains `0`.
