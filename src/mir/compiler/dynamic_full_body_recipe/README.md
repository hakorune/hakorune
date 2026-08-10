# Dynamic full-body Recipe boundary

This directory owns the bounded source-to-Recipe path for the unchanged
resolver-backed Dynamic Loop cohort.

```text
complete resolver source inventory
  -> deterministic complete V2 Recipe candidate
  -> atomic source/Recipe/Dynamic-envelope co-seal
```

## Owners

- `mapping.rs` owns the deterministic logical V2 Recipe mapping.
- `claims.rs` owns the private complete role-to-Recipe claim table.
- `coseal/coverage.rs` consumes and validates all six binding roles and all
  twenty-eight source roles.
- `coseal/calls.rs` binds I6 and I7 to exact Dynamic envelopes by resolver
  owner plus exact source call site.
- `coseal/local.rs` validates the already-sealed V10/ch/I7 mapping and lends
  one borrow-scoped neutral view. It owns no Home or cleanup meaning.
- `coseal/semantic_program/` consumes the whole exact envelope, derives one
  non-splittable JoinSig/root-carrier-After closure and the private complete
  six-site Fault authorization catalog. Its next consuming wrapper derives the
  complete two-row invocation-result lifecycle catalog: I6/V10 local and
  I7/V11 temporary, both activated on exact Normal publication and borrowing
  `EndExactlyOnceUnlessForwarded` from the neutral
  `dynamic_carrier_contract` vocabulary through the canonical Dynamic
  envelope. The
  views retain exact local/call/boundary source sites and I7's
  `BorrowedNoEscapeForInvocation` contract. It lends only borrow-scoped views and
  accepts no raw owner, Recipe, JoinSig, After, Continuation, lifecycle row,
  Fault row, or Completion input.
  The following non-splittable wrapper derives the exact two-row operator
  carrier lifecycle: V9 ends after I6's Normal-or-Fault outcome; V17 is
  authorized only for the later exact B0 rebind commit backed by I16 and the
  JoinSig Backedge. It does not perform either effect.
  The ingress wrapper now seals the initial V1/C0/B0 current from plain
  parameter `pos` as `BorrowedIngressNoEnd`, retaining its exact source and
  recipe relation. A consuming rebind wrapper then seals I13
  `ReadBinding(B0)->V15`, I15 `DynamicAdd(V15,V16)->V17`, I16
  `WriteBinding(B0,V17)`, the canonical I15 Fault contract, and the exact
  JoinSig Backedge as one commit-before-end semantic relation. It performs no
  rebind, End, cleanup, Home, CFG, PHI, or physical operation.
- `dynamic_invocation_contract` remains the complete immutable envelope
  catalog owner. This directory borrows it and never copies targets or
  selector semantics.
- `dynamic_carrier_contract` remains the sole shared lifecycle-vocabulary
  owner. This directory never redefines or infers it.

The semantic source batch owns the exact relation between a catalog callable
and its invocation-local resolver owner. Tests and production integration must
obtain the candidate from that same source authority; equal-looking source
resolved in another session is foreign.

## Carrier cleanup projection (D0/I0)

`VerifiedDynamicCarrierCleanupProjectionV1` consumes the whole carrier-flow
product and records only the bounded carrier obligations at six Fault sites,
the exact inner Return, and the Backedge. The eight rows are source/Recipe
cut-point evidence, not physical cleanup instructions:

```text
I1/I5        -> no live local carrier
I6           -> no local carrier + delegate existing V9 publication
I7           -> existing V10 end authorization
I9           -> delegate existing V11 publication + V10 end authorization
I15          -> V10 end authorization, no replacement/backedge
inner Return -> V10 end authorization
Backedge     -> discharge before the exact I16 write/backedge
```

V9 and V11 remain owned by their existing operator/invocation lifecycle
products; the cleanup projection does not issue duplicate End authority. The
Return partition borrows the exact inner/outer source sites already covered by
the retained Completion product and does not consume or extend Completion.
There is no `ResolvedCleanupObligationsV1` extension, Home capability, physical
End, CFG/PHI/MIR, DraftSeal, collector, retry, or fallback in this boundary.

## Exit-transaction co-seal (D0/I0)

`VerifiedDynamicExitTransactionCoSealV1` consumes the complete
carrier-cleanup projection and retains exactly two logical routes to one
function-exit target:

```text
inner Recipe Return -> FunctionExit
outer Callable Tail -> FunctionExit
```

The existing `VerifiedFunctionCompletionV1` remains the sole source owner of
return coverage, owner/target closure, and value/unit classification. This
co-seal only consumes that already-sealed evidence through the carrier
product; it does not create a second Completion contract or a wrapper that
copies the flow/cleanup rows. No runtime chronology, Home capability, result
merge, physical Return, ABI projection, final function seal, collector, or
publication is performed.

## Acceptance rule

The current fixture has seven Dynamic envelope rows. Exactly two are selected
for this Recipe and the other five remain valid unselected catalog rows.
Seven and two are fixture evidence, not language-wide catalog cardinalities.
Additional valid rows, including rows for the same callable owner, do not
invalidate exact I6/I7 lookup.

If an unchanged valid source row exceeds this boundary, widen the compiler or
stop at a named design question. Never rewrite or narrow the source fixture.

## Non-authority

This directory does not own:

- selector-specific type refinement;
- any iteration-local `ch` Home/install/cleanup relation;
- JoinSigV2, continuation, or Dynamic Fault compatibility;
- physical Callable Tail/Completion consumption, or return ABI;
- Builder, MIR, CFG, PHI, provider selection, runtime invocation, retry, or
  fallback.

The envelope-only co-seal product is not itself a semantic program. The next
child boundary consumes it whole; neither product exposes `into_parts`.

## Iteration-local source closure (R0)

The co-seal owns the V10-to-`ch`-to-I7 logical relation. R0 now additionally
requires the exact `ch` declaration owner scope to equal the resolver-sealed
Loop-body scope and closes the complete resolver use inventory as exactly one
I7 argument read, zero binding rebinds, and zero nested captures. The verified
envelope lends that relation only as `DynamicIterationLocalValueRefV2`, a
borrow-scoped view over the retained declaration/read rows plus V10/I6/I7.
There is no standalone `VerifiedCh*` product or copied source authority.

Local Home installation is not available. A self-contained Dynamic carrier
may contain a trivial, owner-bearing, or weak payload; Recipe `Dynamic` and
runtime tags cannot classify it as a Home. The accepted separate carrier
lifecycle requires every normal opaque carrier publication to be forwarded or
ended exactly once without issuing a Home root. Invocation-result lifecycle is
the next bounded child; full carrier flow, exit cleanup, and any stronger Home
classification remain later owners.

## Atomic semantic program (I0)

`VerifiedDynamicFullLoopSemanticProgramV2` now retains the complete envelope,
one private exact Fault cut-point catalog, and one
`VerifiedLoopJoinClosureV2`. The neutral closure derives the root Loop and
exactly one root-owned carrier from the verified Recipe, elaborates the common
JoinSig, and requires After inside the private JoinSig subtree. Raw V2 After
and `LoopJoinSigElaboratorV2` are not production facade entries.

The Fault catalog derives verified `DynamicAdd`/`DynamicLess` rows and exact
I6/I7 invocation relations internally. Its complete Recipe order is:

```text
I1 Less -> V5
I5 Add -> V9
I6 invocation -> V10
I7 invocation -> V11
I9 Less -> V13
I15 Add -> V17
```

These rows authorize only `Fault before normal-result publication`; they do
not create a concrete Fault/Outcome, cleanup, Home, or control edge. The
product also lends exact `L0/B0/Dynamic` After and the existing V10/I6/I7
local relation. It owns no Completion consumption, Dynamic Fault transaction,
Home, physical layout, Builder/MIR/CFG/PHI, publication, retry, or fallback.

The full exit transaction remains `NoSafeSlice` until the logical Completion
projection is co-sealed with the complete carrier flow and a later physical
session. The Home-capability census closed separately as `NoSafeSlice`; no Home
is inferred from the logical `Dynamic` class or a runtime tag.

## Invocation-result lifecycle (I0)

The semantic program now moves once into
`VerifiedDynamicInvocationCarrierLifecycleProgramV1`. The private issuer
rechecks the two exact invocation envelopes, derives the two CallSlot results
in Recipe order, binds V10 to the existing Loop-body `ch` relation, and binds
V11 to the exact I9 inner-condition boundary. The two static rows authorize
lifecycle creation only on Normal result publication; Fault creates no runtime
carrier instance.

This is complete only for the Dynamic invocation result family. V9/V17
`DynamicAdd` is now semantically co-sealed through the ingress/rebind wrapper;
callable Return, CFG-complete carrier flow, physical End, Home, cleanup
execution, Completion, and production remain unclaimed.

## Carrier rebind transaction (I0)

`VerifiedDynamicCarrierRebindTransactionProgramV1` consumes exactly one whole
`VerifiedDynamicCarrierIngressLifecycleProgramV1`. Its first current
disposition is the typed `BorrowedIngressNoEnd` marker; the owned-forwarded
disposition is vocabulary for the later carrier-flow owner and is not issued
here. The product retains no raw Builder state and exposes no split
`install_new`/`take_old` operation. It only proves the chronology needed by a
future physical owner: evaluate the borrowed I13 value, preflight the normal
I15 result, commit I16 as the new B0 current, discharge the displaced current,
then authorize the Backedge. An I15 Fault publishes no V17, I16, displaced
receipt, or Backedge. Cleanup faults, Home, Completion, and actual execution
remain later rows.

## Carrier iteration flow (D0/I0)

`VerifiedDynamicCarrierFlowProgramV1` consumes the whole sealed rebind
transaction and records the semantic recurrence around it. It does not
re-observe the AST/Recipe and does not duplicate the invocation/operator
lifecycle catalogs. The retained projection is:

```text
initial current = BorrowedIngressNoEnd(V1/C0/B0)
I5/V9   -> Live -> EndAuthorized after I6 normal-or-fault outcome
I6/V10  -> Live local -> EndAuthorized at Loop-body exit
I7/V11  -> Live temporary -> EndAuthorized at the exact I9 boundary
I15/V17 -> Live -> Forwarded at I16/B0/Backedge
```

The normal recurrence is typed as `commit -> displaced end authorization -> Backedge`;
an I15 Fault is typed as `preserve current / no replacement / no Backedge`.
`EndAuthorized` and `Forwarded` are logical dispositions only. They are not
physical End operations, cleanup receipts, Home facts, Return/Tail forwarding,
Completion consumption, CFG/PHI/MIR, or runtime/provider routes. Callable
Return and outer Tail remain the later exit/Completion owner.
