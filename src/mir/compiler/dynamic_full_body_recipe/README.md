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
- `dynamic_invocation_contract` remains the complete immutable envelope
  catalog owner. This directory borrows it and never copies targets or
  selector semantics.
- `dynamic_carrier_contract` remains the sole shared lifecycle-vocabulary
  owner. This directory never redefines or infers it.

The semantic source batch owns the exact relation between a catalog callable
and its invocation-local resolver owner. Tests and production integration must
obtain the candidate from that same source authority; equal-looking source
resolved in another session is foreign.

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
- Callable Tail, Completion consumption, or return ABI;
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

The full exit transaction remains `NoSafeSlice` until complete Dynamic carrier
flow, cleanup projection, and the two-Return Completion consumer exist. The
Home-capability census closed separately as `NoSafeSlice`; no Home is inferred
from the logical `Dynamic` class or a runtime tag.

## Invocation-result lifecycle (I0)

The semantic program now moves once into
`VerifiedDynamicInvocationCarrierLifecycleProgramV1`. The private issuer
rechecks the two exact invocation envelopes, derives the two CallSlot results
in Recipe order, binds V10 to the existing Loop-body `ch` relation, and binds
V11 to the exact I9 inner-condition boundary. The two static rows authorize
lifecycle creation only on Normal result publication; Fault creates no runtime
carrier instance.

This is complete only for the Dynamic invocation result family. V9/V17
`DynamicAdd`, callable ingress/rebind/Return, CFG-complete carrier flow,
physical End, Home, cleanup execution, Completion, and production remain
unclaimed.
