# Generic static-call publication source-bound handoff — design stop

Status: `design closed 2026-08-07; I1/R0 implementation may open at the bounded issuer slice`

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

## Accepted source-bound handoff

The smallest production issuer is the candidate-install window in
`ModuleBuilderInvocationSessionV1::complete_normal_default_program_root_catalog_lifecycle`:

```text
CatalogSeal
  -> CatalogInstall
  -> borrowed target/result proof projection
  -> owned VerifiedStaticCallResultPublicationHandoffV1
  -> callable lowering
```

CatalogInstall here is still a candidate-only pre-effect operation.  The
handoff borrows the just-installed catalog from the candidate, normalizes the
proofs, and is dropped before candidate commit; it never self-references the
catalog and needs no `Arc` or clone.

`VerifiedNormalCallableSemanticSourceV1` remains a selected-callable semantic
loan provider; it does not become a call-result activation owner.  The new
handoff is AST-free, Builder-free, `ValueId`-free, non-`Clone`, and single-use.
It owns only the opaque catalog brand, canonical caller/site/target identity,
and the sealed required-i64 argument ordinals.  The borrowed declaration,
target, result, and import views are dropped after normalization; no catalog
clone, `Arc`, or second Builder catalog is allowed.

The handoff is lent from the lifecycle stack into one callable lowering scope:

```text
source-bound handoff
  -> RawInvocationChildPortV1
  -> raw static-call terminal
  -> physical receipt/publication
  -> existing GenericLoop verifier
```

The raw port receives only the branded handoff/claim loan.  It must not store a
plan in `MirBuilder`, rediscover a caller/site by name or AST walk, or create a
route-local snapshot.  The first admitted row is exactly
`StringHelpers.int_to_str/1` `Body(0).Initializer(0)` targeting
`StringHelpers.to_i64/1`; all other callers/sites are typed `Unselected` or
`Rejected` until a separate source row is designed.

The admitted source row is before the loop (`Body(0).Initializer(0)`), so this
I1 does not open the disconnected located-loop claim family at all.  The
existing `VerifiedCallableResultActivationPlanV1` and its schedule/ledger
remain test-only evidence.  A later loop-body result row would require its own
source-bound design; it may not be smuggled in through this raw terminal hook.

The outer `ModuleBuilderInvocationSessionV1` / `CanonicalModuleLoweringSessionV1`
remains the only rollback and publication owner.  A physical effect or
publication failure after handoff selection is terminal `Freeze` and drops
the whole candidate; it never returns `PostEffectRetryDebt` or advances the
legacy route schedule.  Successful callable completion must finish the
source-bound owner before the candidate is admitted, and successful module
commit publishes the module and the handoff-owned activation state together.

## Design questions — resolved

1. The lifecycle candidate window installs the sealed catalog first, then
   issues the handoff from borrowed proof views; the outer candidate owns its
   lifetime.
2. A stack-borrowed source-bound owner is consulted by the raw static-call
   terminal before the loop; the Builder remains uninvolved in source
   authority and GenericLoop stays verifier-only.
3. `ModuleBuilderInvocationSessionV1` / `CanonicalModuleLoweringSessionV1`
   consume the handoff at the same atomic candidate boundary.
4. The natural fixture is the exact
   `StringHelpers.int_to_str/1` `Body(0).Initializer(0)` ->
   `StringHelpers.to_i64/1` row; selection is proof-driven, never by name.
5. Located loop claim consumers remain disconnected; no second I1-only loop
   claim family or generic activation-source seam is introduced.

The design answers these with existing typed products plus one narrow
source-bound owner.  GenericLoop backfill, route-local snapshots, name-based
dispatch, retry/fallback, and broad type inference remain rejected.

## Implementation gate after design close

I1/R0 may now resume with the bounded issuer slice.  The implementation commit
must include the production caller switch, receipt/publication tests, a fresh
strict VM probe crossing `MissingTransientType`, current/reference/workstream
mirrors, and the exact reference-document update.  Until then production
Generic support remains `0`.

The bounded issuer/terminal slice now has a normal-lifecycle consumer after
`CatalogInstall`, with focused owner, lifecycle, and raw-terminal tests green.
This does not count as GenericLoop selection or broad Generic production
support: the fresh strict VM receipt and the named Generic caller switch are
still required before that claim opens.
