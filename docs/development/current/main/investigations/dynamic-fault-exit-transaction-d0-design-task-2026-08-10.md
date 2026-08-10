# DYNAMIC-FAULT-EXIT-TRANSACTION-D0

Status: accepted with revised boundary; full transaction remains `NoSafeSlice`
Date: 2026-08-10
Depends on: `LOOP-V2-SEMANTIC-PROGRAM-COSEAL-I0` closed
Authority:
`language-result-propagation-and-exit-transaction-ssot.md`,
`dynamic-invocation.md`, and the exact Dynamic V2 semantic program

## Decision

The final direction remains one callable-bounded exit transaction, but it may
not be implemented as one early mega-product.  The existing owners stay
separate until one final consuming co-seal can prove every relation:

```text
Fault authorization
+ complete opaque Dynamic carrier flow
+ optional source-backed Home Flow (only when a stronger contract issues Home)
+ cleanup projection over separate carrier/Home ledgers
+ JoinSig transfer authorization
+ Function Completion coverage
+ canonical physical session
  -> one final exit transaction
```

The Home-capability census closed the unchanged source as `NoSafeSlice`.
However, the language-wide self-contained carrier contract supplies a separate
opaque forward-or-end obligation without claiming Home. Complete carrier flow
and the two-Return physical Completion consumer are still missing, so the full
transaction I0 remains `NoSafeSlice`.

## Corrected Fault census

The earlier question listed only the two CallSlots.  The complete unchanged
`skip_while/4` Recipe has six fault-capable operations:

```text
I1  DynamicLess  Loop condition
I5  DynamicAdd   substring end
I6  CallSlot     substring
I7  CallSlot     indexOf
I9  DynamicLess  inner If condition
I15 DynamicAdd   induction step
```

`DynamicAdd` and `DynamicLess` have their existing fixed
`Normal(value) | Fault(TypeError)` operation contract.  I6/I7 borrow the
existing selector-independent Dynamic invocation envelope.  Neither contract
is a concrete runtime Fault value or primary-outcome product.

## Sole-owner table

| Meaning | Sole owner | Explicit non-owner |
|---|---|---|
| I6/I7 may Fault before normal result publication | `VerifiedDynamicInvocationExecutionEnvelopeV1` plus exact call relation | Recipe class, selector, runtime tag |
| I1/I5/I9/I15 may Fault before normal result publication | verified V2 Dynamic operation contract | provider route, JoinSig |
| actual runtime primary Fault | future canonical Dynamic physical executor and exit transaction | semantic envelope enum |
| V10/ch source-visible Home classification | future source/import capability plus Home Flow | `Dynamic`, runtime tag, local relation |
| opaque carrier forward/end obligation | Dynamic carrier lifecycle/flow | Home relation, runtime tag |
| per-cut-point cleanup obligation | private deterministic projection from carrier flow plus any Home Flow | empty cleanup receipt, Recipe |
| Return/Backedge/PredicateFalse/After transfer | existing JoinSig | cleanup planner, physical layout |
| inner and outer Return source coverage | retained `VerifiedFunctionCompletionV1` | JoinSig, Tail |
| outer operand | Callable Tail | Loop Recipe |
| physical sequencing and poisoned-draft discard | canonical function session | language exit semantics |
| compile-time atomicity | whole unpublished-session discard | runtime Fault transaction |

Fault is never a Recipe value/Exit, JoinSig edge, Completion site, Home, or
physical-session error.  Compiler session discard is not runtime rollback;
Dynamic effects before a Fault remain observable.

## Exact cut-point matrix

Opaque `ch` carrier cleanup exists only after Dynamic carrier flow proves
normal-only V10 publication and a Live obligation. A source-visible Home
cleanup is additional and remains unavailable for the unchanged source.

| Cut point | Definitely materialized | ch state | Cleanup | Transfer / Completion |
|---|---|---|---|---|
| I1 Fault | V0-V4; V5 absent | Absent | exact none | Fault terminal; no JoinSig/Completion |
| I5 Fault | V6-V8; V9/V10 absent | Absent | exact none | Fault terminal |
| I6 Fault | V9; V10 absent | Absent | exact none | Fault terminal; no result publication |
| I6 Normal | V10 | carrier Live after later carrier-flow proof | not executed by current row | continue to I7 |
| I7 Fault | V10; V11 absent | V10 may be Live; V11 absent | end V10 exactly once iff Live | Fault terminal |
| I9 Fault | V10-V12; V13 absent | local V10 may be Live; V11 already ended | end V10 iff Live | Fault terminal |
| I12 inner Return | V13=true and V14 | local V10 may be Live | end/forward before transfer | JoinSig Return to FunctionExit; inner site only |
| I15 Fault | V15/V16; V17 absent | local V10 may be Live | discharge every Live carrier | Fault terminal |
| I16 Backedge | V17 and B0 rebound | no Live Loop-body carrier may cross | discharge first | JoinSig Backedge; no Completion |
| PredicateFalse | V4/V5; body not entered | Absent | exact none | JoinSig PredicateFalse to After |
| outer Tail | After B0 only | Absent | function-scope obligations only | Tail to FunctionExit; outer site |

V10/ch remains iteration-local and is never a Recipe carrier, JoinSig payload,
or backedge value.

## Failure precedence

The accepted C-prime chronology is reused without a Dynamic-specific policy:

```text
pending Normal / Return / Break / Continue
  + first cleanup/finalization Fault
  -> cleanup Fault becomes primary

existing body/operator/invocation Fault
  + cleanup/finalization Fault
  -> original Fault stays primary
  -> later Fault is a suppressed diagnostic

after a primary Fault:
  remaining teardown continues best effort
```

A cleanup Fault before Backedge/Return prevents that transfer from being
published.  A compiler preparation/emission failure follows the separate
whole-session discard law and never enters this runtime chronology.

The future typed outcome must distinguish at least `Primary`, `Cleanup`, and
`PrimaryWithSuppressedCleanup`; concatenated strings are not authority.

## Final target architecture

```text
VerifiedDynamicFullLoopSemanticProgramV2
  - exact six Fault authorizations
  - Recipe / JoinSig / After
  - neutral V10/ch relation
  - retained two-site Completion
            +
VerifiedDynamicCarrierFlowV1
  - normal-only publication
  - exact Live/EndAuthorized/Forwarded at every cut point
  - no Live carrier at Backedge/After
            |
            v
VerifiedDynamicExitTransactionCoSealV1
  - transitively owns semantic program, flow, cleanup, and two logical routes
  - one logical FunctionExit target
  - no copied JoinSig/Fault/cleanup authority
            |
            v
PreparedCallableLoopPhysicalizationV1
            |
            v
session-bound physical exit coordinator
```

The final issuer accepts only the complete semantic program, complete carrier
flow, and any independently verified Home Flow. It accepts no caller-supplied
owner, Recipe, JoinSig, Completion, cleanup rows, Fault sites, or physical IDs
and exposes no `into_parts` escape.

## Ordered task ladder

```text
1. DYNAMIC-FAULT-CUTPOINT-CATALOG-I0
   BoxShape only: exact six-site private catalog inside the semantic program

2. DYNAMIC-LOCAL-DESTINATION-HOME-CAPABILITY-D0
   closed NoSafeSlice; no Home implementation opened

3. DYNAMIC-CARRIER-LEXICAL-DISPOSITION-D0
   separate opaque forward-or-end semantics

4. DYNAMIC-INVOCATION-RESULT-LIFECYCLE-I0
   exact I6/V10 local plus I7/V11 temporary obligations

5. DYNAMIC-OPERATOR/CALLABLE-CARRIER-LIFECYCLE-D0/I0
   complete V9/V17, ingress/rebind/Return relations

6. DYNAMIC-CARRIER-FLOW-D0/I0
   per-iteration Absent -> Live -> EndAuthorized/Forwarded and every exit cut

7. DYNAMIC-EXIT-CLEANUP-PLAN-I0
   CLOSED: private carrier-only obligations derived from the complete flow;
   no Home Flow was available or inferred

8. MULTI-RETURN-COMPLETION-CONSUMPTION-D0/I0
   CLOSED: inner Recipe Return + outer Tail -> one logical FunctionExit;
   physical Return/DraftSeal remains later

9. DYNAMIC-EXIT-TRANSACTION-COSEAL-I0
   CLOSED: promoted the existing consuming Completion projection to the final
   bounded co-seal; no standalone wrapper or copied authority was added

10. DYNAMIC-EXIT-PHYSICAL-SESSION-P0
   parked as NoSafeSlice until `PHYSICAL-INPUT-AUTHORITY-BRIDGE-D0` closes the
   missing source-backed physical-input authority
```

Each implementation row updates its code, focused tests, module README,
landed reference receipt, active card, and guards in the same slice.

## Dynamic carrier-flow slice (D0/I0)

This bounded slice is now implementation-ready.  Its readiness sentence is:

```text
the whole VerifiedDynamicCarrierRebindTransactionProgramV1
  -> one semantic iteration-flow product
  -> fails before any physical End/Home/cleanup/Completion/CFG operation
```

The sole source authority and issuer are:

```text
VerifiedDynamicCarrierRebindTransactionProgramV1
  -> issue_dynamic_carrier_flow_program_v1(...)
  -> VerifiedDynamicCarrierFlowProgramV1
```

The flow product owns only the opaque carrier-flow rules already issued by
the invocation/operator/rebind products:

```text
initial current: BorrowedIngressNoEnd(V1/C0/B0)
I6/V10: live Loop-body-local obligation
I7/V11: live full-expression-temporary obligation
I5/V9: end after the I6 normal-or-fault outcome
I15/V17: live replacement forwarded at I16/B0/Backedge
```

The state vocabulary is semantic and private:

```text
Absent -> Live -> EndAuthorized | Forwarded
```

`EndAuthorized` and `Forwarded` are disposition rules, not runtime instructions or
cleanup receipts.  The product does not choose an actual end operation, infer
Home, consume Completion, publish a Return/Backedge, build CFG/MIR/PHI, or
execute/retry/fallback.  Callable Return and outer Tail remain the later
Completion owner.

The I0 slice is intentionally bounded to the iteration recurrence and the
four already-issued carrier publication rows (V9/V10/V11/V17).  I15 Fault is
the typed preserve-current/no-replacement/no-Backedge transition.  Return,
PredicateFalse/After forwarding, and callable-tail forwarding remain deferred
to the later exit/Completion owner; this row does not silently issue those
relations.

The package will replace its selected Dynamic rebind field with this whole
flow product.  No raw flow row, current slot, result ValueId, cleanup token,
or standalone ingress is exposed to lowering.

## Carrier-flow I0 closeout

`DYNAMIC-CARRIER-FLOW-D0/I0` is closed as the bounded semantic iteration-flow
projection.  `issue_dynamic_carrier_flow_program_v1` consumes exactly one
whole `VerifiedDynamicCarrierRebindTransactionProgramV1` and package-selected
Dynamic lowering now owns the resulting non-splittable flow product.

The closeout proves the existing V9/V10/V11/V17 lifecycle destinations and the
typed I15 normal/fault recurrence.  It does not claim an actual End, Home,
cleanup execution, Return/After forwarding, Completion consumption, CFG/MIR,
physical source-ledger progress, retry, or fallback.  This flow now feeds the
closed cleanup and exit-transaction co-seal chain.

## Carrier cleanup projection (D0/I0 closeout)

`DYNAMIC-EXIT-CLEANUP-PLAN-I0` is closed as a bounded carrier-only
projection. `issue_dynamic_carrier_cleanup_projection_i0` consumes the whole
verified flow product and atomically retains eight private cut-point rows:

```text
I1/I5        -> NoLiveLocalCarrier
I6           -> NoLiveLocalCarrier + delegated V9 publication
I7           -> EndAuthorized(V10)
I9           -> delegated V11 publication + EndAuthorized(V10)
I15          -> EndAuthorized(V10), no replacement/backedge
inner Return -> EndAuthorized(V10)
Backedge     -> DischargeBeforeBackedge at the exact I16 write
```

V9 and V11 remain owned by the existing operator/invocation lifecycle
products; this projection does not duplicate their End authority. The Return
partition borrows the exact inner/outer source sites from retained Completion
coverage and does not consume or extend `VerifiedFunctionCompletionV1`.
No `ResolvedCleanupObligationsV1` extension, Home capability, physical End,
CFG/PHI/MIR, DraftSeal, collector, retry, or fallback is introduced.

Focused closeout gates:

```text
RUSTFLAGS=-Awarnings cargo test -q --lib carrier_cleanup
RUSTFLAGS=-Awarnings cargo test -q --lib normal_callable_semantic_package
RUSTFLAGS=-Awarnings cargo test -q --lib semantic_program
RUSTFLAGS=-Awarnings cargo check -q --lib
```

The cleanup projection now feeds the closed exit-transaction co-seal.

## Exit-transaction co-seal (D0/I0 closeout)

`MULTI-RETURN-COMPLETION-CONSUMPTION-D0/I0` and
`DYNAMIC-EXIT-TRANSACTION-COSEAL-I0` are closed as one consuming logical
two-route co-seal. `issue_dynamic_exit_transaction_coseal_i0`
consumes the complete carrier-cleanup product and retains exactly:

```text
inner Recipe Return -> one function-exit target
outer Callable Tail -> the same function-exit target
```

The existing `VerifiedFunctionCompletionV1` remains the sole owner of exact
return-site coverage, owner/target closure, and common value/unit
classification. The promoted co-seal consumes that already-sealed evidence
through the carrier chain and does not issue a second Completion contract or
copy the cleanup/JoinSig/Fault rows. It does not create a runtime chronology,
Home capability, result merge, physical Return, ABI representation, final
function seal, DraftSeal, collector, or publication.

Focused closeout gate:

```text
RUSTFLAGS=-Awarnings cargo test -q --lib exit_transaction
```

The next bounded owner is `DYNAMIC-EXIT-PHYSICAL-SESSION-P0`.

## Physical session P0 audit (2026-08-10)

`DYNAMIC-EXIT-PHYSICAL-SESSION-P0` is a design stop, not an implementation
permission. The existing `loop_physical_prepare.rs` and callable physical
canary are `cfg(test)` helpers only; no production issuer currently supplies
the complete physical input for the selected Dynamic package.

The package already supplies the exact logical source-backed input and the
non-splittable `VerifiedDynamicExitTransactionCoSealV1`, but the physical
boundary still lacks one source-backed co-seal for:

```text
Loop physical demand
Prelude / entry materialization
Callable Tail
exact physical ABI / result representation
physical Completion relation
```

The existing owners remain authoritative:

```text
CanonicalFunctionLoweringSessionV1
  -> fresh unpublished function state and whole-session discard
CanonicalSsaFunctionSessionV2
  -> CFG / SSA / PHI and typed function finish
OpenFunctionDraftSealV1 / PreparedFunctionDraftSealV1
  -> DraftSeal prepare / commit
ModuleDraftCollectorV1
  -> later draft collection / publication
```

The smallest next design slice is therefore:

```text
PHYSICAL-INPUT-AUTHORITY-BRIDGE-D0
  source authority census
  canonical issuer and co-seal boundary
  physical-input identity / owner / frame / scope checks
  fail-fast and NoSafeSlice matrix
```

Until that Decision is accepted, do not remove `cfg(test)`, promote the static
physical canary, call the raw `lower_loop` route from the package, open
DraftSeal/Collector, or add a guessed `Verified*`/`Prepared*` receipt. After a
fresh physical session opens, every failure must discard the unpublished
function exactly once; same-session retry and fallback remain forbidden.

Non-claims for this stop:

```text
Home capability
runtime Fault outcome / primary-suppressed chronology
CFG / PHI / MIR emission
DraftSeal / Collector / publication
provider or runtime dispatch
```

## Physical-input authority bridge D0 (2026-08-10)

`PHYSICAL-INPUT-AUTHORITY-BRIDGE-D0` is the current design stop.  The
logical package and its exit-transaction co-seal are complete enough to be
borrowed by a later physical boundary, but they do not themselves prove that
the callable can be materialized in a fresh MIR function session.

The accepted boundary is therefore two consecutive, non-overlapping stages:

```text
installed source-backed semantic package
  + exact selected scoped lowering input
  + existing source/resolver physical capability issuers
    -> one future physical-input co-seal
       (demand / Prelude-entry / Tail / ABI-result / Completion relation)
    -> zero-effect physical preflight
    -> fresh unpublished function session
    -> common recursive physicalizer
    -> finish_for_draft_seal
    -> DraftSeal prepare/commit
```

The physical-input co-seal is a relation product, not a second Recipe, a
second callable package, or a new semantic owner.  It must consume already
verified products and publish only their same-owner/frame/scope/target
compatibility.  Until its canonical issuer exists, no `Verified*` or
`Prepared*` physical receipt is added merely to connect existing fields.

### Owner census

| physical concern | current owner / evidence | bridge status |
| --- | --- | --- |
| exact source/function input | installed package's scoped `ResolvedFunctionLoweringInputV1` view | available as a read-only source view; not a physical receipt |
| logical Loop/Recipe/JoinSig/After | source-backed logical issuers and the selected Dynamic exit co-seal | available; never re-infer transfers here |
| Loop physical demand | `VerifiedLoopOperationPhysicalDemandV1` in `loop_physical_prepare.rs` | caller-zero and `cfg(test)`; cannot be promoted without source-backed issuer |
| Prelude / entry materialization | `VerifiedCallablePreludeV1` plus test-only argument/preparation helpers | semantic prelude exists; physical entry relation is not co-sealed |
| Callable Tail | `VerifiedCallableTailV1` | source relation exists; physical tail/return materialization is not co-sealed |
| ABI / result representation | `ExactTrivialReturnAbiV1` and existing result contracts | classification exists; exact physical result relation is not a production input |
| function Completion | `VerifiedFunctionCompletionV1` and `CanonicalSsaFunctionSessionV2` consumption | semantic completion exists; physical two-site completion relation is not one source-backed input |
| fresh function/session | `CanonicalFunctionLoweringSessionV1` and `CanonicalSsaFunctionSessionV2` | downstream sole owners; session remains unopened at this stop |
| DraftSeal / collection | `OpenFunctionDraftSealV1` / `PreparedFunctionDraftSealV1` / `ModuleDraftCollectorV1` | downstream only; not part of the bridge issuer |

The current `loop_physical_prepare.rs` (795 lines, `#![cfg(test)]`) and
`callable_loop_physical_canary.rs` are evidence and contract fixtures, not
production authorities.  They must not be enlarged to absorb the bridge;
the eventual bridge should live in a new, narrowly owned module or in the
existing package-to-physical boundary after its source issuer is identified.

### Sole issuer and fail-fast contract

The future bridge issuer must accept exactly one installed-package scoped
input and the existing source-backed physical capability products.  It must
reject before a session opens when any of these is missing, duplicated,
foreign, or mismatched:

```text
source/catalog/session brand
FunctionOwner / callable header
Loop owner, frame, Scope/Region, and exact source site
Recipe item/operation coverage and JoinSig transfer relation
Prelude receiver/arguments and entry binding
Tail statement/value site and function-exit target
semantic result class and physical ABI/result projection
Completion owner, return-site coverage, and terminal target
```

Once a fresh session opens, the sole failure policy remains whole unpublished
function discard exactly once.  Same-session repair, retry, compatibility
fallback, raw `lower_loop` entry, and AST/MIR re-matching are forbidden.

### Smallest task ladder after this Decision

```text
PHYSICAL-INPUT-AUTHORITY-BRIDGE-D0   current design stop
  -> source-owner/issuer census and accepted co-seal contract
PHYSICAL-INPUT-AUTHORITY-I0
  -> one bounded source-backed input co-seal, still before session emission
DYNAMIC-EXIT-PHYSICAL-SESSION-P0
  -> fresh session, common physicalizer, finish/DraftSeal canary
```

`PHYSICAL-INPUT-AUTHORITY-I0` may not begin until this D0 is accepted and
must not remove `cfg(test)` from the existing canary as a shortcut.  The I0
must also carry a source-to-Facts-to-Recipe readiness sentence and a focused
negative matrix; it must not introduce Home, runtime Fault, CFG/PHI,
DraftSeal, Collector, provider dispatch, or a new fallback route.

## Physical-input authority I0 design frontier (2026-08-10)

The D0 owner census is accepted.  The next frontier is still a design stop,
`PHYSICAL-INPUT-AUTHORITY-I0-D0`, because the existing products cannot yet
form the required physical input without crossing an authority boundary.

The concrete gaps are:

```text
Dynamic semantic program = V2 Recipe / V2 JoinSig
existing common physical demand = V1 Recipe/operation contract
  -> no V2-to-V1 adapter is permitted

VerifiedFunctionCompletionV1
  -> consumed by source admission and reduced to logical summary
  -> no exact physical Completion handoff exists

ExactTrivialReturnAbiV1
  -> source-spelling classification only
  -> not physical FunctionSignature or entry ABI authority

VerifiedCallablePreludeArgumentListV1 / physical canary
  -> cfg(test) evidence only
  -> not a production entry materialization issuer
```

Therefore the next design must choose a single package-to-physical boundary,
not add a wrapper around the logical Dynamic exit co-seal:

```text
installed package
  -> selected scoped lowering loan
  -> one private physical-input issuer
       V2-aware Loop demand
       Prelude/entry relation
       Tail/result relation
       exact physical ABI/result projection
       exact Completion handoff
  -> one non-splittable physical input
```

The logical package remains the owner of logical Recipe/JoinSig/After/Fault
relations.  The physical issuer may consume a scoped package loan, but it may
not expose the semantic program internals, reissue Recipe/JoinSig/After, or
re-verify Completion later in the lowerer.  Completion must either be handed
off from the source issuer at this boundary or the issuer must return
`NoSafeSlice`; an empty or summary-only replacement is not evidence.

The I0-D0 acceptance sentence is:

```text
This selected package loan maps once to one physical-input product,
and every missing/foreign/mismatched demand, entry, Tail, ABI, or Completion
fails before a function session opens.
```

Required design evidence before implementation:

```text
one owner table for all five physical subcontracts
one source-backed issuer per missing subcontract, or an explicit NoSafeSlice
one package-to-physical co-seal boundary
one V2-capable demand path (no V2 -> V1 coercion)
one exact Completion handoff (no lower re-verification)
one negative matrix for foreign owner/frame/scope/site/ABI/completion
```

No code, `cfg(test)` promotion, physical receipt constructor, session open,
DraftSeal, Collector, raw `lower_loop`, retry, or fallback is allowed at this
frontier.  Once this design is accepted, the implementation slice is
`PHYSICAL-INPUT-AUTHORITY-I0`; the next physical-session row remains parked
until that slice is green.

### First missing axis: operation-demand issuer D0

The first sub-question is deliberately narrower:
`PHYSICAL-OPERATION-DEMAND-ISSUER-D0`.

The current Dynamic chain retains the verified V2 source/Recipe envelope and
the V2 JoinSig/After relation internally, but the final logical exit co-seal
does not expose an operation/effect product or a physical demand.  The
existing `VerifiedLoopOperationPhysicalDemandV1` cannot be reused by casting
or reconstructing the V2 Recipe: it requires a V1 operation/effect product,
V1 context, and V1 continuation, and it is not a Dynamic V2 source issuer.

The design target is a one-way, private projection at the logical-to-physical
boundary:

```text
exact Dynamic V2 source/Recipe/JoinSig program
  -> V2-aware operation/effect/demand projection
     (Recipe order, complete item coverage, exact source/effect relations)
  -> later physical-input co-seal
```

This projection must not:

```text
V2 -> V1 cast or shape adapter
re-run Dynamic admission or AST observation
rebuild Recipe keys from MIR/name/order
expose the internal envelope/JoinSig/After
become a second Recipe or a public selector
open a Builder/session or allocate physical IDs
```

The D0 acceptance evidence is an owner table for the V2 operation/effect
source, a single issuer location, the exact consume boundary relative to the
logical exit co-seal, and a negative matrix for foreign owner/frame/scope,
missing/duplicate item coverage, wrong JoinSig transfer, and V1-family
coercion.  If a source-backed V2 operation/effect issuer cannot be identified,
this sub-question remains `NoSafeSlice`; no partial demand receipt is added.

### CallSlot target handoff D0 (2026-08-10)

The operation-demand audit found one narrower prerequisite which must be
named before a physical-demand I0 can claim complete CallSlot coverage.  The
current `VerifiedDynamicFullLoopCallRelationsV2` retains item/role relation,
but the exact source-bound target object is not retained by the final Dynamic
semantic program.  A future demand issuer must not repair that loss with
method name, Box name, arity, catalog order, or runtime lookup.

The accepted boundary is:

```text
VerifiedSourceBoundDynamicMemberCallV1
  -> one source-backed target handoff
  -> private V2 CallSlot relation
  -> operation/effect ledger
  -> later physical demand projection
```

This is a relation handoff, not a new target catalog and not a public
`CallSlot`/function-pointer API.  The handoff must preserve the exact target,
caller/receiver/argument/result source sites, resolver/source brand, owner,
frame, scope/region, and the Recipe item identity.  It may be retained
transitively inside the private V2 semantic program or co-sealed by the
future physical-input bridge, but only one issuer may decide that relation.

Required negative evidence:

```text
missing/duplicate CallSlot target
foreign source-bound target or resolver brand
same name/arity with a different target
same item/role with a different source site
catalog-order or batch-slot repair
target lookup after the logical co-seal
raw function pointer or runtime dispatch handle
```

Until this handoff is sealed, `PHYSICAL-OPERATION-DEMAND-I0` remains parked.
The smallest next design row is:

```text
PHYSICAL-CALLSLOT-TARGET-HANDOFF-D0
  -> one private source-bound target retention/co-seal decision
  -> then PHYSICAL-OPERATION-DEMAND-I0
```

No code, V2-to-V1 adapter, Recipe rebuild, public target catalog, physical
session, DraftSeal, Collector, retry, or fallback is authorized in this row.

### CallSlot target handoff D0 closeout (2026-08-10)

The independent source/target audit closes this design stop with one precise
interpretation: `VerifiedSourceBoundDynamicMemberCallV1` is an exact
source-bound Dynamic message relation, not an executable/provider target.
It owns the resolver owner, call/receiver/result sites, receiver BindingRef and
origin, ordered argument sites, and selector/arity dispatch identity. It must
not be upgraded into a runtime target or a third callable catalog.

The accepted I0 boundary is:

```text
ResolvedFunctionLoweringInputV1
  -> owned source-bound call rows (HRTB ends here)
  -> consume Box<[VerifiedSourceBoundDynamicMemberCallV1]>
  -> exact Dynamic source/Recipe envelope
       private rows: { Recipe item, source role, source-bound call row }
  -> semantic program retains the rows transitively
  -> future physical-demand issuer borrows one scoped view
```

The handoff is non-`Clone`, non-splittable, and has no `into_parts`, raw target
getter, CallSlot getter, function pointer, provider handle, or runtime route.
The retained source already owns the same owner/frame/scope-region relation;
the handoff co-seals against it. A separate ResolverCatalogBrand is not
invented: this row is a source-bound message relation, not a reusable
declaration/executable target. If a later physical issuer requires a brand or
an executable target, that is a separate `NoSafeSlice` axis.

I0 must add exact evidence for:

```text
two expected rows are consumed exactly once
expected selector/arity matches each source role
all supplied rows are consumed; extra rows reject
owner/frame/scope-region/source-site equality
call/receiver/result/ordered-argument equality
Recipe CallSlot item/receiver/args/result/value-class equality
missing, duplicate, foreign, reused, or mismatched target rejection
```

After this closeout, the live order is:

```text
PHYSICAL-CALLSLOT-TARGET-HANDOFF-I0
  -> PHYSICAL-OPERATION-DEMAND-I0
  -> PHYSICAL-INPUT-AUTHORITY-I0
```

The I0 remains pre-session and source-only. It may not open a Builder or
physical session, issue ABI/Completion/DraftSeal/Collector products, or add a
fallback/retry path.

## Hard stops

```text
no Dynamic implies Home
no runtime tag implies Home
no empty cleanup as proof of Home absence
no Fault Recipe value/Exit or JoinSig edge
no Completion consumption before the multi-return owner lands
no cleanup/Return/Backedge publication without complete carrier/Home flow
no physical cleanup/CFG/DraftSeal/collector/publication
no retry/fallback or source narrowing
no test-only semantic/Home constructor
```

## File-size plan

```text
dynamic_full_body_recipe/coseal/semantic_program/
  mod.rs
  fault_cut_points.rs
  carrier_flow.rs
  carrier_cleanup.rs
  tests.rs

future exit_transaction/
  mod.rs
  completion_partition.rs
  tests/{golden,negative,api_guard}.rs
```

Split at roughly 650-700 lines, stop adding at 760, and keep 800 as the hard
limit.  Do not create a standalone public `VerifiedCh*` product.
