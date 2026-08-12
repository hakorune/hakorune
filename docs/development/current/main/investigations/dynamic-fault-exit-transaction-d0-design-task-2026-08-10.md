# DYNAMIC-FAULT-EXIT-TRANSACTION-D0

Status: logical/fault/flow/operation-demand foundation and the explicit result
`: i64` Completion relation are landed. The previous checked-Dynamic return
corridor is superseded for this bounded method by accepted target A-prime:
`pos/end: i64 -> exact entry contract -> exact local copy -> mixed typed Recipe
-> I64 carrier -> ImmediateI64 physicalization`. The accepted
`hako.text.scan@1` contract now requires the current I7/I9 exact-I64 semantic
recut. Production remains `NoSafeSlice` until the implementation rows below
close. The full tagged Dynamic corridor is parked rather than partially
implemented.
Date: 2026-08-10
Depends on: `LOOP-V2-SEMANTIC-PROGRAM-COSEAL-I0` closed
Authority:
`language-result-propagation-and-exit-transaction-ssot.md`,
`mirbuilder-final-pipeline-ssot.md`,
`loop-common-physical-demand-and-session-ssot.md`, and the live A-prime
Decision in this card. `dynamic-invocation.md` remains the authority for the
I6/I7 invocation operations; I9 becomes typed CompareI64 in the current recut.
The landed exact-Dynamic V2 program is
an input to Slice B's atomic replacement, not a competing selected program.

## VM lane override (current, non-historical)

Rust VM is not a production backend or a DynamicV2 provider consumer. Its
remaining scope is limited to bootstrap/recovery, compatibility, semantic
reference, and small smoke/regression evidence. No new VM provider, V10/V11
receipt issuer, Dynamic runtime leaf, or feature-only VM implementation may
be added by this card. AOT/LLVM is the only production-capability lane for
the provider/session rows below; Rust VM evidence never satisfies a production
gate and never justifies a second implementation path.

## Landed pre-A-prime foundation (historical baseline)

The sections in this baseline record the already-landed six-Fault and Dynamic
induction program that A-prime now replaces. They remain evidence for why the
old carrier/cleanup/exit owners existed, but they do not select a current task,
Recipe class, Fault count, lifecycle, or backend policy. The live authority
starts at `PHYSICAL-INPUT-AUTHORITY-I0 (A-prime accepted; production
NoSafeSlice)` below.

### Decision

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

### Corrected Fault census

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

### Sole-owner table

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

### Exact cut-point matrix

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

### Failure precedence

The accepted exit chronology is reused without a Dynamic-specific policy:

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

### Final target architecture

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

### Ordered task ladder

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

10. LOOP-JOINSIG-V2-LOGICAL-TRANSFER-VIEW-I0
    CLOSED: JoinSig-owned borrowed logical flow/After view only

11. DYNAMIC-V2-PHYSICAL-INPUT-VIEW-I0
    CLOSED: 17/15 source/control evidence plus final-co-seal HRTB view

12. PHYSICAL-OPERATION-DEMAND-I0
    CLOSED: whole-program Builder-free demand/prepare

13+. Later rows at the time of this historical receipt
    were superseded by the live A-prime dependency DAG below; this baseline no
    longer selects representation, Loop cleanup, session, cutover, parity,
    performance, or selfhost tasks
```

Each implementation row updates its code, focused tests, module README,
landed reference receipt, active card, and guards in the same slice.

### Dynamic carrier-flow slice (D0/I0)

At that historical point this bounded slice was implementation-ready. Its
landed readiness sentence was:

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

### Carrier-flow I0 closeout

`DYNAMIC-CARRIER-FLOW-D0/I0` is closed as the bounded semantic iteration-flow
projection.  `issue_dynamic_carrier_flow_program_v1` consumes exactly one
whole `VerifiedDynamicCarrierRebindTransactionProgramV1` and package-selected
Dynamic lowering now owns the resulting non-splittable flow product.

The closeout proves the existing V9/V10/V11/V17 lifecycle destinations and the
typed I15 normal/fault recurrence.  It does not claim an actual End, Home,
cleanup execution, Return/After forwarding, Completion consumption, CFG/MIR,
physical source-ledger progress, retry, or fallback.  This flow now feeds the
closed cleanup and exit-transaction co-seal chain.

### Carrier cleanup projection (D0/I0 closeout)

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

### Exit-transaction co-seal (D0/I0 closeout)

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

That closeout originally named `DYNAMIC-EXIT-PHYSICAL-SESSION-P0`. The later
audit below inserted the missing Builder-free physical-input authority rows
before any session may open.

### Physical session P0 audit (current downstream boundary)

`DYNAMIC-EXIT-PHYSICAL-SESSION-P0` is now the active implementation row. The
existing `loop_physical_prepare.rs` and callable physical canary remain
`cfg(test)` helpers only; no production issuer yet supplies the complete
physical input for the selected Dynamic package.

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

This audit first identified the parent bridge:

```text
PHYSICAL-INPUT-AUTHORITY-BRIDGE-D0 (accepted parent)
  source authority census
  canonical issuer and co-seal boundary
  physical-input identity / owner / frame / scope checks
  fail-fast and NoSafeSlice matrix
```

The current child order is the JoinSig logical view, Dynamic physical-input
view, and whole-program operation demand specified below. Until those and the
later full callable physical-input row are green, do not remove `cfg(test)`,
promote the static canary, call raw `lower_loop`, or open DraftSeal/Collector.
After a fresh session eventually opens, every failure discards the unpublished
function exactly once; same-session retry and fallback remain forbidden.

Non-claims for this stop:

```text
Home capability
runtime Fault outcome / primary-suppressed chronology
CFG / PHI / MIR emission
DraftSeal / Collector / publication
provider or runtime dispatch
```

### Physical-input authority bridge D0 (accepted parent boundary)

`PHYSICAL-INPUT-AUTHORITY-BRIDGE-D0` was the prior parent design stop. The
logical package and its exit-transaction co-seal are complete enough to be
borrowed by a later physical boundary, but they do not themselves prove that
the callable can be materialized in a fresh MIR function session. The
operation-demand subchain below closes first; the broader physical/session
bridge remains parked.

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

#### Owner census

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

#### Sole issuer and fail-fast contract

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

## PHYSICAL-INPUT-AUTHORITY-I0 (A-prime accepted; production NoSafeSlice)

The selected design is A-prime. It is one forward authority chain, not a
checked conversion from a Dynamic carrier:

```text
pos/end: i64 source contract
  -> exact parser parameter transport
  -> resolved BindingRef relation
  -> mixed parameter contract and HomeDemand projection
  -> exact i = pos relation
  -> mixed typed V2 Recipe
  -> I64 physical input
  -> ImmediateI64 fresh-session receipts
  -> existing two-site Completion
```

The result annotation remains owned by `VerifiedFunctionCompletionV1`; it does
not classify the carrier. The full tagged Dynamic corridor is parked for a
future API that genuinely accepts arbitrary Dynamic values. A-prime failure
returns `NoSafeSlice`; it never auto-opens the tagged corridor, a helper, or a
fallback.

### Caller and API Decision (CLOSED)

Repository census found one canonical declaration and zero canonical
production callers. The only tracked `.skip_while(...)` call targets a
fixture-local shadow declaration. Therefore A-prime is accepted from the
cursor/bound API meaning, not from caller parity:

```text
pos initializes local cursor i
end is the cursor bound
i/end feed comparison and substring indices
i advances by exactly one
both exits return i
```

The canonical source change lands atomically with the Recipe recut:

```hako
skip_while(src, pos: i64, end: i64, pred_chars): i64
```

There is no method-name inference, general parser-cursor typing rule, or claim
that `src`/`pred_chars` are I64.

### Four implementation slices

The implementation uses four responsibilities. Child row names below are
commit/test boundaries, not new public authorities or new task cards.

#### Slice A: A-PRIME-PARAMETER-CONTRACT-I0

Commit A1 — `CALLABLE-PARAMETER-TYPE-TRANSPORT-R0` (CLOSED, BoxShape):

```text
change:
  final callable syntax loan -> resolved callable batch HRTB loan
  retains borrowed declared_type_name: Option<&str>
  and checks syntax/batch declaration identity during the batch loan

contract:
  [None, i64, i64, None] is transported as source spelling only.
  No ABI/Home/Recipe/ValueId/backend meaning is assigned here.
  No AST rescan, name/ordinal repair, or owned-string authority is added.

evidence:
  typed parser/batch positives, top-level non-fabrication, parameter-drift
  rejection, focused parser/batch tests, complete-batch guard, and README
  updates are green.

next:
  A2 must atomically replace the old callable_parameter_demand owner.
```

Commit A2 — `CALLABLE-EXACT-I64-PARAMETER-CONTRACT-I0` (CLOSED):

```text
exact spelling + declaration identity + resolved BindingRef
  -> package-private parameter contract row
     OpaqueHandle
     or ExactTrivial(ExactTrivialParameterAbiV1)
  -> one-way HomeDemand projection
     OpaqueHandle -> Handle
     ExactTrivial -> Trivial
```

Use one small `src/mir/callable_parameter_contract/` owner with README, model,
issuer, and tests. This is an atomic replacement, not a sibling authority:
delete the old `callable_parameter_demand` module/export and its production
callers in the same commit. The existing `ExactTrivialParameterAbiV1::classify`
remains the sole spelling classifier. The existing `ParameterEntryContract`
remains the later runtime checker; it is not constructed before ValueIds exist.
The mixed selected rows must be exactly:

```text
src        OpaqueHandle / Handle
pos        ExactTrivial(I64) / Trivial
end        ExactTrivial(I64) / Trivial
pred_chars OpaqueHandle / Handle
```

This commit deletes the unconditional “all ordinary parameters are Handle”
decision and updates the ownership/result-contract references that still state
that rule. `HomeDemand` is only a one-way derived projection; it is not the
parameter contract authority. The package retains the non-Clone contract; raw
batch slots and arbitrary constructors remain private. No dual producer,
fallback, `.hako` signature change, Recipe/ValueId/backend/session work is
allowed in A2.

Evidence: the old `callable_parameter_demand` module/export/callers are gone;
the new issuer is the sole package production caller; absent ordinary rows
become `OpaqueHandle`, explicit `i64` rows become `ExactTrivial(I64)`, and
unsupported explicit/non-ordinary rows reject. Package-scoped loans retain
the exact contract kind while `HomeDemand` is derived only at the Dynamic
ingress boundary. Contract, package, parser, batch, and complete-batch guard
tests are green; touched Rust remains below the 800-line hard boundary.

Next: Slice B is the only current semantic replacement row.

#### Slice B: A-PRIME-MIXED-RECIPE-SEMANTIC-RECUT-I0 (historical landed base)

Status: CLOSED (I0 landed); its I7/I9 subset is superseded by the current
text-scan exact-I64 recut.

This is one atomic BoxCount/semantic replacement commit. It changes the
canonical source, the sole existing producer, the Fault catalog, and the
selected lifecycle together. No alternate producer or old/new mode is allowed.
The A2 parameter contract is not re-derived here: the sole Recipe producer
consumes a private exact co-seal of the A2 rows and the full-body source
inventory. That relation retains ordinal, BindingRef, source role, and
Recipe input/value class transitively inside the candidate/envelope; no public
bridge, name lookup, ordinal repair, or second parameter authority is allowed.

```text
inputs:
  V0 src Dynamic        V1 pos I64
  V2 end I64            V3 pred_chars Dynamic

binding/carrier:
  B0 induction I64      C0 entry V1:I64

typed replacements:
  I1  CompareI64(Less) -> V5:Bool
  I5  BinaryI64(Add)   -> V9:I64
  I15 BinaryI64(Add)   -> V17:I64

per-call typed classes:
  I6 substring: Dynamic receiver, I64/I64 arguments, Dynamic result
  I7 indexOf:   Dynamic receiver, Dynamic argument, Dynamic result

still Dynamic:
  I6 -> V10 substring result
  I7 -> V11 indexOf result
  I9 DynamicLess(V11, V12) -> V13

return/current:
  V14 / V15 / V17 / After = I64
```

The V2 schema/verifier already supports the mixed program;
`typed_schema_v2.rs` is 757 lines and is a no-add surface.

The Fault catalog is atomically replaced:

```text
old: 6 rows
new: I6 DynamicInvocation, I7 DynamicInvocation, I9 DynamicLess

execution coverage:
  NonFaulting = 12
  FaultBeforeNormalResult = 1
  ExternallyBoundOutcome = 2
```

The I64 induction is `ExactI64TrivialNoEnd`. Preserve the existing invocation
lifecycle for V10/V11, then replace only the selected Dynamic induction and
operator chain in the same commit:

```text
operator_carrier_lifecycle/**
carrier_rebind.rs and tests
carrier_flow.rs
ingress.rs / BorrowedIngressNoEnd
old six-row profile assumptions
```

Replace the deleted chain with one invocation cleanup projection (target at
most 300 lines), named `invocation_cleanup.rs`. It consumes the existing
`VerifiedDynamicInvocationCarrierLifecycleProgramV1` and is consumed by the
existing exit transaction. It owns only V10/V11 cleanup; the I64 induction
receives no End/Home/owned/borrowed lifecycle. The exact bounded matrix is:

```text
I6 fault                 -> no V10 cleanup
I7 fault                 -> end V10
I9 normal/fault          -> end V11; I9 fault also ends V10
inner Return/backedge    -> end V10
outer Tail               -> no invocation temporary cleanup
```

Primary touched families are `dynamic_full_body_recipe/{mapping,claims,coseal,
physical_demand}`, `normal_callable_semantic_package`, the canonical `.hako`
source, module README, and the MIR/language references. Required negatives
cover class drift, old Dynamic operations, old+new Fault coexistence, missing
I6/I7/I9, I64 End, missing temporary cleanup, and JoinSig payload drift.

Update the existing Dynamic authority guards rather than adding one guard per
child. Production callers of deleted lifecycle issuers and fixed six-row
constants become zero. `dynamic_full_body_source.rs` (671) and
`physical_evidence.rs` (649) receive no new responsibility; split first if
growth would enter the 760 stop band. All Rust source remains below 800 lines.

#### A-PRIME-EXACT-I64-PHYSICAL-CAPABILITY-D0 (live design child; no new card)

Status: ACCEPTED FOR IMPLEMENTATION (bounded A-prime only). The mixed Recipe
has closed the semantic shape; this Decision closes only the exact physical
capability required before a fresh session. It does not open a
Dynamic-to-I64 conversion corridor. Runtime/production behavior remains
`NoSafeSlice` until the implementation row and its gates are green.

The sole accepted A-prime chain is:

```text
pos/end: i64 source contract
  -> package CallableParameterContractKindV1::ExactTrivial / BindingRef rows
  -> exact local copy (i = pos)
  -> mixed typed Recipe (I64 induction/carrier)
  -> I64 JoinSig/After and exact two Completion operands
  -> Builder-free physical demand
  -> fresh session ImmediateI64 realization
```

The Builder-free issuer is the existing final package physical-input spine:
`VerifiedDynamicExitTransactionCoSealV1` lends one HRTB-bounded demand view.
It owns no `ValueId`, `BasicBlockId`, MIR instruction, helper call, or backend
implementation. The fresh-session owner later materializes those facts; it
does not reclassify source meaning.

The target-aware capability matrix is deliberately narrow:

```text
parameter entry:
  exact i64 contract + exact selected call edge -> Direct
  wrong/ambiguous/unsupported representation   -> RejectBeforeEffect

local copy / induction PHI / backedge:
  every incoming exact I64 -> Direct
  any missing or mixed incoming -> RejectBeforeEffect

I6/I7 call arguments:
  mixed Recipe classes and exact I64 pos/end edge -> Direct selected edge
  wrapper, missing metadata, or repair-only edge -> RejectBeforeEffect

inner/outer return:
  exact Completion-site operands are I64 -> Direct
  anything else -> RejectBeforeEffect
```

The AOT/LLVM production lanes consume the same demand; neither may be replaced
by `resolve_i64`, missing-value zero, ptr/int repair, runtime tag inference, or
a backend-name allowlist. Rust VM does not consume the selected production
demand; its caller-zero classifier is non-gating retirement debt.
This corridor needs no Dynamic-to-I64 helper, IntegerBox handle, or tagged
carrier. `Dynamic` remains only the I6/I7 temporary result family and is never
returned by this Recipe.

Required rejection cases are missing/foreign/duplicate/ambiguous parameter,
local-copy, carrier/PHI, call-edge, or Completion evidence; wrong value class;
unsupported backend; and any attempt to consume a raw `ValueId` before the
fresh session. `VerifiedFunctionCompletionV1` remains the sole result/site
owner, GenericLoop and TypeOp remain unchanged, and fallback/retry remain zero.

The issuer boundary and AOT/LLVM Direct-or-RejectBeforeEffect production
contract are now documented and accepted. Focused positive/negative tests are
implementation gates, not a reason to reopen this Decision. Rust VM behavior
is reference/smoke evidence only. Until the production implementation row is
green, current behavior remains `RejectBeforeEffect` and production remains
`NoSafeSlice`.

### Implementation authority close

The semantic A-prime chain is closed, but the physical capability issuer is
not yet implemented.  The sole issuer is a new private module at
`src/mir/compiler/a_prime_i64_physical_capability/issuer.rs`.  It consumes one
borrowed selected-callable lowering input together with the package-owned
`CallableParameterContractKindV1::ExactTrivial`/`BindingRef` rows, the mixed
typed Recipe, `VerifiedDynamicExitTransactionCoSealV1`, and
`VerifiedFunctionCompletionV1`, then emits one Builder-free exact-I64 demand.
It may not contain or publish `ValueId`, `BasicBlockId`, MIR instructions,
helper calls, backend tags, or a second source/Recipe/Completion authority.

The post-MIR `FunctionEntryContract` is not an input authority for this
issuer.  It owns ValueId-bearing runtime metadata after Builder lowering and
may only be consumed by the later physical/session gate.  The semantic issuer
uses the package's source-backed parameter contract kind and BindingRef rows;
it must not recreate them from post-MIR metadata.

The existing `physical_input.rs` remains the Loop placement/operation/Fault
view owner; the test-only prelude issuer and generic backend gates are not
A-prime authorities.  The demand issuer is the only place that co-seals the
callable entry edge, exact `i = pos` relation, I64 carrier/PHI lineage, I6/I7
mixed call-edge facts, and the two Completion-site operands.

The `i = pos` evidence is borrowed from the retained source inventory already
inside the selected final program; it is not re-observed from the AST.  The
private source-facts loan must contain exactly the existing binding rows for
`Pos` and `Induction`, the `PreludeInitializerPos` source row, and the resolver
relation proving that the initializer reads `Pos`.  The issuer may additionally
check the existing `LoopConditionI`/`StepReadI`/`InnerReturnI`/`OuterReturnI`
rows against the same `Induction` binding, but it must not search by name,
ordinal, or a second method inventory.  Missing, foreign, duplicate, or
mismatched rows reject the demand.

The narrow child view is named `DynamicAPrimeI64SourceRelationViewV1<'program>`
and is delegated through the final exit co-seal; it is not a second physical
input view.  This source-facts loan is a narrow child view of the final exit
co-seal.  It is
not a new source observer, a second Recipe producer, or a public source map.
The selected package input supplies the parameter-contract rows; the final
program supplies the retained source/Recipe/Completion relation.  Both are
borrowed under one HRTB callback and are consumed only to issue the exact-I64
demand.

The package's older `VerifiedSourceBackedDynamicCallableV1` sibling remains a
generic source seed and is not the typed A-prime authority; it does not carry
the exact `pos/end` contract.  The A-prime view must therefore delegate to the
retained full-body source inside `VerifiedDynamicExitTransactionCoSealV1`,
not to that legacy seed and not to a new source reissue.

The 671-line `dynamic_full_body_source.rs` is already near the split band and
must not receive the A-prime issuer.  Keep the source-relation view in a new
small sibling module under the A-prime capability/coseal boundary; the existing
observer only lends the already sealed rows.

The minimum borrowed relation is now fixed to these existing facts:

```text
owner / frame / scope-region / exact Loop membership
Pos and End BindingRefs + parameter contract classes
formal Pos -> local Induction declaration -> PreludeInitializerPos site
LoopConditionI / StepReadI / StepTargetI / InnerReturnI / OuterReturnI sites
Recipe binding/carrier entry V1:I64 and inner V14:I64
After/current class I64
the two Completion sites
```

`VerifiedDynamicFullLoopPhysicalInputViewV2` remains the operation/placement/
control/Fault view and is not promoted into a callable-entry authority.  The
legacy `VerifiedDynamicLocalInitializationSourceV1` is evidence for the
source relation, while `PreparedDynamicLocalEntryV1` remains session-local
because it contains physical ValueIds.  Neither legacy product is the A-prime
Builder-free demand or a replacement issuer.

Backend transport is a separate capability adapter.  Rust VM accepts only
exact `VMValue::Integer` in its narrow reference/compatibility lane; that
behavior is not a production capability gate.  AOT/LLVM is Direct only when
the selected mixed signature and every selected call edge carry exact metadata;
missing, ambiguous, wrapper, `resolve_i64`, missing-value zero, ptr/int repair,
retry, and fallback are all `RejectBeforeEffect`.  The LLVM metadata transport
must be fixed before the session/cutover slice; changing a backend-name
allowlist is not an implementation.

No physical session, Completion consumer, or production cutover may start
until this issuer boundary, the AOT/LLVM production capability matrix, and the focused
missing/foreign/duplicate/ambiguous negative tests are landed in the design
record and accepted.  This is a design close, not a new task card.

#### A-PRIME-SOURCE-RELATION-VIEW-I0 (landed behavior-neutral prerequisite)

This is the only implementation slice allowed before the backend transport
Decision closes.  It adds the private HRTB-borrowed
`DynamicAPrimeI64SourceRelationViewV1<'program>` and its focused tests.  The
view is issued from the final exit co-seal using the already retained source,
claims, Recipe, and Completion facts.  It does not lower instructions, open a
session, publish a type, or select a backend.

Acceptance:

```text
exact Pos/End/Induction bindings and declarations
exact PreludeInitializerPos -> Pos resolver relation
exact LoopConditionI / StepReadI / StepTargetI / InnerReturnI / OuterReturnI
exact Recipe binding/carrier/value classes and claim targets
exact two Completion sites
missing/foreign/duplicate/mismatched rows -> typed reject
ValueId / BasicBlockId / MIR / helper / fallback / retry -> absent
```

The implementation must live in a new small sibling module; neither the
671-line source observer nor the common physical-input view grows a new
authority.  This slice landed as `9057ec8b06` with the focused co-seal test.
The pointer now returns to the A-prime backend-transport design stop before
any physical session code.

#### A-prime physical-capability audit addendum (design stop remains open)

The landed source-relation view is intentionally narrow.  It is the source/
Recipe/Completion loan, not the complete physical-demand input.  The next
issuer must not widen that view into a second source or callable authority.
Instead, the private A-prime demand issuer consumes sibling facts under the
same selected-package HRTB boundary and co-seals them once:

```text
selected callable header / exact entry edge
  + package-owned parameter contract rows
  + DynamicAPrimeI64SourceRelationViewV1
  + exact I6/I7 CallSlot argument/result relations
  + existing I64 Recipe carrier / JoinSig / After facts
  + existing two-site VerifiedFunctionCompletionV1 facts
  -> one Builder-free VerifiedAPrimeI64PhysicalDemandV1
```

The source-relation view must not grow `ValueId`, `BasicBlockId`, MIR,
backend tags, call-edge ABI, or session state merely to make this issuer
convenient.  Conversely, the demand issuer may not silently omit the
selected callable identity/entry edge, the exact I6/I7 argument relation, or
the owner-branded function target.  A missing or foreign sibling fact is a
typed `RejectBeforeEffect`, never a name/ordinal/ValueId repair.

The current package loan has one concrete preparation gap: its
`SelectedCallableLoweringInputRefV1` carries the resolved source input,
parameter-contract rows, Dynamic program, and optional method observation,
but not the selected batch row's parser identity/mode/owner relation.  Before
the A-prime issuer is implemented, add one private borrowed
`SelectedCallableSourceIdentityRefV1` projection from that already co-sealed
batch row.  It may carry parser declaration identity, declaration mode,
owner/function-origin, and the optional method-source observation; it must not
expose a canonical key, batch slot, AST, or a new lookup authority.  A
top-level callable must not receive a fabricated `VerifiedCallableHeaderV1`:
its exact owner-branded declaration relation comes from the batch row itself.
This is a transport/view adjustment inside the existing package loan, not a
new semantic product.

The LLVM part is a transport/capability child, not a backend-name allowlist.
Rust MIR JSON remains the single metadata emission owner.  The capability
contract is two-stage, so semantic demand and physical transport are not
confused:

```text
pre-session:
  VerifiedAPrimeI64PhysicalDemandV1
    = backend-independent Direct/Reject requirement

fresh session:
  exact ValueId/BasicBlock/PHI/call/return realization
    -> FunctionMetadata physical receipt
    -> dedicated a_prime_i64_capability JSON emitter
    -> strict LLVM loader/lane
```

The dedicated field encoder must live beside, not inside, the generic
parameter contract encoder (for example
`src/runner/mir_json_emit/a_prime_i64_capability.rs`), while
`mir_json_emit/metadata.rs` remains the sole JSON composition owner and calls
that encoder exactly once.  The Python side gets a matching strict
loader/lane.  It must prove the selected mixed signature and every selected
I6/I7 edge; it must not call `resolve_i64`, fill missing values with zero, or
use ptr/int repair.  The generic parameter-entry metadata and post-MIR
`FunctionEntryContract` remain distinct from the Builder-free demand and must
not be re-used as a semantic source authority.

Therefore implementation is not yet opened.  The remaining design close is
limited to these three contracts:

1. selected callable header/entry-edge ownership and exact function target;
2. exact I6/I7 argument/result edge evidence and its co-seal with the mixed
   Recipe/JoinSig/Completion facts; and
3. one pre-session capability contract plus one post-session
   Rust-MIR-JSON-to-LLVM physical receipt projection with strict
   `Direct | RejectBeforeEffect` behavior.

No new source observer, Dynamic producer, Completion owner, GenericLoop
change, or physical session is allowed while these contracts are open.

#### Slice C: A-PRIME-PHYSICAL-INPUT-I0 (landed Builder-free demand)

Commit C1 — `A-PRIME-PHYSICAL-INPUT-I0` remains Builder-free:

```text
final exit/package HRTB view
  owner/frame/Scope-Region/provenance
  exact pos/end contracts
  exact pos -> induction relation
  mixed Recipe + JoinSig + complete ledger
  exact 3 Fault rows
  ExactI64TrivialNoEnd carrier
  Completion inner site -> I12/V14:I64
  Completion outer site -> After/current:I64
  required target capability
```

It contains no ValueId, BasicBlockId, MIR, helper, PHI token, runtime tag, or
raw Recipe/JoinSig getter.

Status: CLOSED (C1 landed as `5dba33cd0e`). The selected package loan now
retains the opaque callable source identity and issues this demand from the
final Dynamic exit transaction through narrow borrowed source-relation and
physical-input views. Focused package (14 tests), Dynamic body (29 tests),
`cargo check --lib`, complete-batch authority, pointer, and diff gates are
green. This closes the Builder-free demand only; it does not claim a physical
ValueId, backend ABI, session, or production cutover.

The two required production prerequisite branches are now green before the
session; the Rust VM row is reference-only evidence:

```text
branch 1:
  LOOP-UNIFICATION-AFTER-DYNAMIC-D0 selected subset

reference evidence (non-blocking):
  A-PRIME-VM-EXACT-I64-ENTRY-I0

branch 3:
  A-PRIME-LLVM-EXACT-I64-CAPABILITY-I0 (receipt/JSON/strict-preflight transport)
```

The former Rust VM reference checker accepted `VMValue::Integer` under the
exact entry contract, then carried ImmediateI64 directly. That caller-zero
artifact is now retired by `DYNAMIC-V2-RUST-VM-NONCONSUMER-FENCE-R0`; it is
historical evidence only. The Rust VM remains a bootstrap/reference/
compatibility lane and is not a consumer of the selected A-prime capability.

`A-PRIME-LLVM-EXACT-I64-CAPABILITY-I0` is landed as a transport boundary
before the selected cross-backend cutover. It is not a backend-name allowlist
edit:

```text
exact mixed signature
+ every selected call edge proves pos/end ImmediateI64
  -> Direct

missing/ambiguous edge, wrapper, raw integer/handle ambiguity
  -> RejectBeforeEffect
```

The LLVM row owns parameter/call-edge transport only. It eliminates the
selected use of ptr/int repair, missing-argument zero, `resolve_i64`, retry, and
fallback in its strict validator. The fresh session still has to issue the
receipt and consume it exactly once; no production caller is connected yet.
Other backends remain RejectBeforeEffect.

#### Slice D: A-PRIME-SESSION-AND-CUTOVER-I0

This slice follows the Loop and backend branches.

Commit D1 — `FUNCTION-COMPLETION-SITE-KEYED-CLAIMS-R0`:

```text
VerifiedFunctionCompletionV1 explicit sites
  -> private site-keyed physical claim set
expected == claimed
missing/duplicate/foreign/extra/wrong target -> reject
Fault path -> claim zero
```

It replaces the selected singleton bool/one-witness consumer; it does not
create another Completion owner.

Commit D2 — `DRAFT-SEAL-EXIT-PROJECTION-SPLIT-R0` is behavior-neutral. Split
the 688-line `draft_seal.rs` into `draft_seal/exit_projection.rs` (target at
most 350) before multi-return growth. Detached prepare remains the sole Return
writer; commit remains an ownership-only move with fallible work zero.

The next bounded sub-slice is now landed as a Builder-free child of that split:
`draft_seal/multi_site_exit.rs`.  It borrows the completed, site-keyed
Completion claims and issues a non-Clone canonical source-order claim set for
the selected two-site cohort.  It rejects empty, non-two-site, and unit claims;
it does not touch a Builder, CFG, TypeContext, Return instruction, or physical
session.  `into_exact_two()` is the only bounded admission surface, and no
parts API or second Completion owner exists.  This closes only the detached
claim preparation; it does not claim that physical Returns are emitted.

Commit D3 — `A-PRIME-I64-LOOP-PHYSICAL-SESSION-I0`:

```text
formal pos/end -> ImmediateI64 receipts
local i = pos  -> exact-copy receipt
header/backedge PHI -> every incoming ImmediateI64
I1/I5/I15 -> typed direct operations
inner/outer exits -> exact site-keyed block/value claims

DraftSeal prepare:
  Return(i64) at both normal exits
  exact Return count = 2
  synthetic return join / return PHI = 0

any failure:
  discard whole unpublished function once
  publication/retry/fallback = 0
```

Commit D4 is the existing `H2-SELECTED-DYNAMIC-LOOP-CUTOVER-I0`: switch the
named `ParserScanLoopBox.skip_while/4` production lowering, delete the selected
legacy edge in the same commit, and close
`MIRBUILDER-FIRST-PRODUCTION-CUTOVER`.

### Dependency DAG

```text
Slice A parameter contract
  -> Slice B atomic Recipe/semantic recut
  -> Slice C Builder-free input
       +-> Loop authority cleanup + selected If/Exit coverage
       +-> AOT/LLVM production exact-I64 capability
       +-> Rust VM reference/smoke evidence (non-blocking)

all required production branches green
  -> Slice D site claims / DraftSeal split / fresh session / cutover
```

After first cutover, H2/H3/H5 parity and
`MIRBUILDER-HAKO-MIMALLOC-PROMOTION-GATE0` run as required sibling proofs.
Both gate `HAKO-CALLABLE-PARAMETER-RESULT-ISSUER-CUTOVER-I0`, which activates
the Hako producer and retires the Rust selfhost producer in the same commit.
Broader `.hako` migration follows. Fixed topology hard deletion remains a
post-cutover caller-zero cleanup.

Production remains `NoSafeSlice` until D4. A1
`CALLABLE-PARAMETER-TYPE-TRANSPORT-R0` and A2
`CALLABLE-EXACT-I64-PARAMETER-CONTRACT-I0` are closed; the current safe
implementation row is Slice B `A-PRIME-MIXED-RECIPE-SEMANTIC-RECUT-I0`.

#### Production-consumer and finalization audit (2026-08-11)

The A-prime demand is currently a verified Builder-free product with
test-only callers.  This is intentional pre-cutover state, not production
SSOT convergence.  The selected Builder adapter still follows the migration
route

```text
installed package -> source seed -> raw AST descent -> old JoinIR route
```

until Slice D supplies the named consumer.  The selected cutover must instead
consume the package-loaned final program exactly once:

```text
installed package -> A-prime physical demand -> fresh canonical session
```

and must delete the selected old edge in the same commit.  Acceptance is
explicitly:

```text
selected A-prime demand production callers = 1
selected package program consumed         = 1
selected source-seed-only route            = 0
selected raw JoinIR route                  = 0
```

The semantic Completion already carries the exact two-site set, while the
physical consumer is still bounded to one claim and the DraftSeal writer
explicitly rejects multi-site input.  The next session slice must extend the
existing Completion consumption owner to source-site-keyed claims, require
`expected == claimed == 2`, reject missing/duplicate/foreign/extra claims,
and prepare exactly two normal Return instructions without a synthetic return
join or Return PHI.  No second Completion owner is allowed.

The canonical DraftSeal path must also reject missing signature-backed result
types.  The selected A-prime route must have zero callers of name-based
`infer_return_type` and zero callers of the legacy `finalize_function_draft`;
the latter may remain only on explicitly isolated compatibility routes until
their caller-zero retirement gate.  No selected route may repair a type from
method name, ValueId, raw AST, or a missing-value default.

The module drain/finalization owner remains a target SSOT rather than a claim
that every production route already uses it.  A separate caller census is
required before global retirement; it is not a reason to add another finalizer
or to widen the current AOT/LLVM production capability slice.

The 2026-08-11 caller census fixes the retirement boundary more precisely:

```text
name-based infer_return_type:
  type_hint_providers.rs:78 -> legacy finalize_module/finalize_function_draft_with_lookup
  return_type_strategy.rs:204 -> module_lifecycle.rs:418
  selected A-prime caller-zero required before cutover

legacy finalize_function_draft:
  recursive_child_lowering.rs:439,548
    -> port_aware_function_draft_impl.rs:127,132
    -> calls/lowering.rs:162,184
  compatibility production edge remains until selected cutover

finalize_drained_module_once / PreparedInvocationDrainV1:
  production callers = 0 (fixtures/tests only)
  do not claim global drain convergence yet
```

The physical-session slice adds guards for the selected route only; it does
not delete or reroute these compatibility edges prematurely.  The selected
cutover must prove `infer_return_type` caller-zero, legacy finalizer
caller-zero, source-seed-only route zero, and raw-JoinIR route zero in the
same commit that installs the package-backed session.

## Historical checked-Dynamic return design (SUPERSEDED; non-authoritative)

Everything in this section is retained only as the rejected-alternative audit.
It must not select a task, type, helper, backend capability, or current pointer.

The declared-result row is now closed: the selected
`ParserScanLoopBox.skip_while/4` source explicitly declares `: i64`, and the
existing `VerifiedFunctionCompletionV1` owns the declared result classification
and exact two-site Completion set. That is necessary transport and semantic
coverage, but it is not physical operand proof.

The selected Recipe still returns logical `LoopValueClassV2::Dynamic` values:
the inner Return operand is `V14:Dynamic`, and the outer Tail/current carrier
is also Dynamic. `ExactTrivialReturnAbiV1` classifies the source spelling and
the existing `ExistingExactNumericDeferred` relation is intentionally deferred;
neither proves that either logical operand is a physical `MirType::Integer` or
provides a materialized `ValueId`. The installed package's Dynamic physical
input currently owns only logical placement/operation/control/Fault rows, not
Completion, ABI, or return-operand `ValueId` rows.

The existing `ExactNumericRuntimeCheckContract::DynamicIntegerRange` does not
close this gap: its owner and interpreter hook are anchored to numeric
`FieldSet` sites, not callable Return sites, and it has no return-value
publication or exit-transaction relation. Reusing it by changing a site label
would create a second, incomplete return authority; extending it to returns is
a new language/runtime Decision, not an existing proof.

Therefore this row remains a design stop. Do not infer a result or ABI from
loop shape, `return i`, `LoopValueClassV2::Dynamic`, `MirType`,
`FunctionSignature`, TypeContext, runtime tags, selector names, or method names.
GenericLoop remains an exact-MirType verifier and is unchanged. No physical
bridge, session, DraftSeal, Collector, `lower_loop`, retry, or fallback is
opened by this row.

### DYNAMIC-CALLABLE-RESULT-CONTRACT-D0 (revised accepted)

Use the existing `name(args): TYPE_REF` surface; add no Rune or generic result
disposition. The selected fixture becomes
`ParserScanLoopBox.skip_while(src, pos, end, pred_chars): i64`. The source
annotation is the sole declared-result syntax authority;
`VerifiedFunctionCompletionV1` is the sole semantic classification/return-site
aggregate. No loop/body/MIR/runtime inference is allowed.

```text
selected frontend source row (: i64 + declaration identity)
  -> existing ResolvedFunctionLoweringInputV1 source view
  -> verify_function_completion_v1
  -> VerifiedFunctionCompletionV1::ExplicitReturns
       declared result = Annotated("i64")
       exact return sites = inner + outer
       common function target
```

`VerifiedFunctionCompletionV1` and its sealed exit contract remain the sole
semantic result-classification and source return-site aggregate. The selected
frontend row is syntax transport only. Do not add a sibling
`VerifiedDeclaredExactI64CallableResultContractV1` or another annotation
classifier. The physical ABI is a one-way borrowed projection from Completion's
declared result relation; it is not owned by the syntax row.

The bootstrap I0 is implementable now. It must include the canonical production
annotation, the existing Rust final-source/resolved-input identity path,
Completion verification, positive/negative/API guards, and language/module
docs. Hako parity is
explicitly a later nonclaim: after H2/H3/H5, `source_carrier_v1` emits the same
normalized row and one atomic producer cutover retires the Rust frontend from
selfhost production. Both frontend producers are never admitted in one
compilation and there is no retry or fallback between them.

`FuncScannerBox`, compatibility JSON/metadata, body returns,
`MirType`, `FunctionSignature`, ABI, runtime tags, and method names are never
result authority. Missing/non-i64 annotation, foreign provenance/declaration/
owner, duplicate selected producer, or source/Completion mismatch reject before
a function session opens.

#### DYNAMIC-CALLABLE-RESULT-CONTRACT-I0 (CLOSED 2026-08-11)

This is one bounded BoxCount/source-contract slice, not a physical cutover.
The task ID is retained for pointer stability; its responsibility is
"callable declared exact-I64 result through Completion", not construction of a
new Dynamic-specific or sibling result-contract type.

```text
canonical production declaration
  skip_while(src, pos, end, pred_chars): i64
        |
        v
FinalCallableSemanticSyntaxRowRefV1
  existing opaque declaration identity
  existing mode/final slot/method observation
  syntax transport only; no semantic result receipt
        +
VerifiedResolvedCallableSemanticBatchV1 row
  same identity / private batch slot / selected mapping
        |
        v
verify_function_completion_v1(existing resolved input)
        |
        v
VerifiedFunctionCompletionV1::ExplicitReturns
  declared_result = Annotated("i64")
  sites = [inner, outer]
```

The current source view already lends the declaration annotation only to the
Completion verifier. This row must preserve that sole-consumer law; it does not
add another semantic product to the package. A borrowed
`DeclaredExactI64ResultRefV1` may later project from Completion for ABI input,
but it is not independently constructible, Clone authority, or a sibling
receipt.

Implementation scope:

```text
lang/src/compiler/parser/scan/parser_scan_loop_box.hako
  explicit `: i64` on the canonical method

src/mir/resolved_control_flow/function_control.rs
  reuse the existing declared-result + exact-return-site owner; add only the
  narrow borrowed ExactI64 projection if the next ABI consumer requires it

src/mir/normal_callable_semantic_package/dynamic_admission.rs
  retain the existing verified Completion unchanged inside the final program
```

Closeout receipt (2026-08-11):

```text
canonical ParserScanLoopBox.skip_while now declares : i64
Rust final-source/resolved input reaches the existing Completion verifier
Completion declared result = Annotated("i64")
Completion explicit return sites = exactly 2
no sibling result receipt, ABI writer, body/MirType inference, or fallback added
```

Focused evidence:

```text
cargo test -q --lib dynamic_full_body_source       # 6 passed
cargo test -q --lib normal_callable_semantic_package # 11 passed
cargo test -q --lib dynamic_full_body_recipe       # 38 passed
cargo test -q --lib function_control               # 14 passed
cargo test -q --lib source_resolver_handoff        # 3 passed
cargo check -q --lib                               # green (warnings only)
bash tools/checks/current_state_pointer_guard.sh    # ok
bash tools/checks/naming_charter_guard.sh           # ok
```

The next boundary is intentionally a design stop:
`A-PRIME-EXACT-I64-PHYSICAL-CAPABILITY-D0` must close the selected
entry/call-edge provenance and target-aware exact-I64 capability. Until then,
the unsupported behavior is `RejectBeforeEffect`, the slice remains
`NoSafeSlice`, and this row does not claim physical I64 compatibility.

Required tests:

```text
positive:
  selected skip_while row -> Annotated("i64") Completion
  exactly two explicit Completion sites remain ordered and unchanged
  catalog order != batch order still maps by opaque identity
  valid unselected annotated row remains unselected

negative:
  missing / void / non-i64 result
  foreign parser provenance or declaration identity
  foreign selected resolved input or owner
  duplicate selected source row
  Completion declared-result/site mismatch
  body/MirType/signature/runtime/name/ordinal repair attempt
  Rust failure -> Hako/JSON/FuncScanner retry
```

Structural guards fix `verify_function_completion_v1` as the sole selected
semantic classifier, forbid a sibling Verified result receipt and raw AST/text
rescan, add no ABI/Return writer, and keep Hako production producer callers at
zero in this row. Focused parser source, semantic package, resolved Completion,
cargo-check, pointer, formatting, and diff gates must be green in the same
commit. This I0 does not claim that either Dynamic Return operand is physically
I64-compatible.

### Multi-site terminal law (one owner chain)

`VerifiedFunctionCompletionV1::ExplicitReturns` remains the sole logical
source-site owner. No Dynamic-specific Completion, Return, or Tail truth is
introduced.

```text
VerifiedFunctionCompletionV1
  - declared result classification
  - exact ordered Completion return sites
  -> one-way borrowed ABI projection
  + exact BindingRef operand at every return expression
  -> one move-only return-operand set
  -> site-keyed physical completion claims
  -> existing canonical finish terminal
  -> DraftSeal detached prepare projection
       writes one Return in every claimed exit block
       completes all CFG/PHI/type/signature/metadata checks
  -> PreparedFunctionDraftSealV1::commit
       ownership-only move; fallible work = 0
```

Missing, duplicate, foreign, ABI-incompatible, or unconsumed site claims reject
before commit. Profile lowerers write zero Return instructions. DraftSeal
prepare, not commit, is the sole Return writer on the detached projection.
DraftSeal does not invent a canonical return-join or PHI merely to merge exits;
multiple exact Return terminators are canonical unless a separately verified
backend/MIR constraint later proves a join is required.

### Dynamic-to-I64 return operand boundary (PHYSICAL-INPUT gate)

The source declaration `: i64` does not prove that a logical `Dynamic` Recipe
value may be returned as physical `MirType::Integer`. The exact selected cohort
has two such sites:

```text
inner Return:
  Completion site -> Recipe Return operand V14:Dynamic

outer Return:
  Completion site -> Tail/current carrier:Dynamic
```

`PHYSICAL-INPUT-AUTHORITY-I0` must issue one complete Builder-free demand:

```text
DynamicI64ReturnProjectionDemandRefV1
  each Completion site
  -> exact logical operand
  -> required exact-I64 Direct-or-Reject capability
  -> no ValueId / BasicBlockId / MIR

DYNAMIC-EXIT-PHYSICAL-SESSION-P0 later consumes:
  exact demand row
  + producer-issued representation receipt
  + session-local block/value IDs
  -> prepared normal exact-i64 path
  + terminal projection-Fault path
```

The accepted target architecture and current unsupported behavior are distinct:

```text
target architecture:
  A-prime exact-I64 corridor
  = source-level exact-I64 contract
    + typed Recipe/carrier
    + producer-issued exact-I64 physical receipt

current unsupported behavior:
  RejectBeforeEffect
```

No physical/session receipt currently proves the exact-I64 realization of the
selected corridor.  Adding `MirType::Integer` to a Dynamic ValueId because a
declaration says `i64` is forbidden.  A generic Dynamic-to-I64 helper is not
part of this bounded row.  This question belongs to the existing physical-
input row; it does not create a new task card.

The current three-row Fault catalog remains the exact Recipe-operation catalog.
A checked return projection is a callable-terminal sibling keyed by the exact
Completion site; it is never inserted as a Recipe item, JoinSig edge, or a
seventh operation row. The final callable exit transaction must co-seal that
sibling with cleanup and primary/suppressed Fault chronology before session
mutation.

#### PHYSICAL-INPUT-AUTHORITY-I0 decision and landed boundary

The authority split is fixed, but implementation is not yet authorized:

```text
Completion:
  sole declared-result and exact return-site owner

Dynamic Recipe / final exit transaction:
  sole logical return-operand owner (V14 and outer carrier)

fresh callable physical session / terminal:
  sole owner of materialized return ValueIds

Builder-free demand view:
  sole pre-session projection of sites / logical operands / required capability

session-local final-exit realization:
  sole owner allowed to relate demand rows, physical representations, and IDs
```

No fresh physical session currently emits the selected physical exact-I64
receipt. The Builder-free A-prime demand, backend-local VM capability, sealed
receipt model, MIR-JSON transport, and strict LLVM preflight are now landed.
The remaining missing authority is the session-local producer provenance and
the one production consumer that realizes the receipt. Until that consumer
exists, current behavior is `RejectBeforeEffect` and the selected slice remains
`NoSafeSlice`.

The sole next task is therefore the downstream implementation row:

```text
DYNAMIC-EXIT-PHYSICAL-SESSION-P0
  consume the installed-package A-prime demand exactly once
  materialize the fresh-session receipt
  close both site-keyed Completion claims through DraftSeal prepare
```

It must not create a sibling result contract, Static/Dynamic arbitration sum,
standalone initializer bridge, Dynamic source reissuer, GenericLoop change,
Recipe/JoinSig physical type, or public raw `ValueId`/slot API. Loop
unification remains the structural prerequisite inside the downstream session
series; it cannot replace the missing session-local Dynamic-to-i64 conformance
proof.

#### PHYSICAL-INPUT-DYNAMIC-I64-REPRESENTATION-D0 (superseded historical proposal)

The generic checked-Dynamic corridor recorded below is historical design
material only.  It is not the live A-prime contract and must not be revived
for `skip_while`.  In particular, `IntegerBoxHandle`, `TaggedCarrier`, the
strict Dynamic-to-I64 helper, and the generic `Checked` backend cells are
parked for a future genuinely polymorphic Dynamic API.  The live bounded
A-prime corridor is exact source `i64` through a typed Recipe and therefore
has only `Direct` or `RejectBeforeEffect` behavior; see the authoritative
section immediately before `PHYSICAL-OPERATION-DEMAND-AUTHORITY-D0`.

Historical decision (superseded by A-prime): target `CHECKED-DYNAMIC-I64-ABI`
was accepted as the only candidate architecture for the generic Dynamic
corridor; current unsupported behavior remains `RejectBeforeEffect`. Global
all-values-as-handles and a language-wide tagged representation are not opened
by this row. The child remains `NoSafeSlice` until its canonical issuers and
complete selected-corridor table are fixed.

```text
primary stop class: RepresentationDecisionMissing
dependent evidence gap: BackendCapabilityMissing
```

The checked boundary is one authority chain with two different times. They
must not be collapsed into one product.

```text
pre-session / Builder-free
  VerifiedDynamicExitTransactionCoSealV1
    -> HRTB DynamicI64ReturnProjectionDemandRefV1<'program>
       exact Completion-derived I64 ABI and sites
       inner logical operand = I12 / V14
       outer logical operand = L0 / B0 / Dynamic
       required capability = CheckedDynamicCarrierToI64
       no ValueId / BasicBlockId / MIR / helper call

session-local realization
  exact demand row
  + producer-issued DynamicCarrierPhysicalRefV1
      ImmediateI64 { value }
      IntegerBoxHandle { handle }
      TaggedCarrier { tag, payload }
  + source block
    -> PreparedCheckedDynamicI64ReturnProjectionSetV1<'program>
       per site: normal exact-i64 block/value + terminal fault block/reason
```

The conceptual names above describe owner boundaries; they are not authorized
code types until the issuers below are identified. The Completion relation is
borrowed through the final-exit HRTB spine rather than passed in as a sibling.
Inner/outer rows are matched by exact `SourceStmtSiteV1`, never by array
ordinal. A tagged carrier may use multiple physical IDs, so the realization
must not invent one generic `source: ValueId` field.

##### Repository-backed stop evidence (2026-08-11 audit)

The current code confirms that this is a real representation stop rather than
an untracked task name:

```text
src/mir/builder/normal_callable_dynamic_loop_prepare.rs
  PreparedLoopCarrierRepresentationV1::SourceBackedDynamic
  retains only BindingRef source lineage; its private Exact(I64) arm has no
  production issuer.

src/mir/builder/normal_callable_dynamic_loop_rebind.rs
  emits the Dynamic Add BinOp but rejects when the result is unexpectedly
  published as MirType::Integer; it does not publish a representation receipt.

src/mir/builder/normal_callable_dynamic_origin.rs
  tracks ValueId -> BindingRef origin, not ImmediateI64/IntegerBoxHandle/tag data.

lang/src/compiler/parser/scan/parser_scan_loop_box.hako
  annotates only the callable result as `: i64`; `src`, `pos`, `end`, and
  `pred_chars` remain untyped, and `i`/`ch`/`i + 1` are therefore not a source
  exact-I64 representation proof.

src/mir/return_exit_backend_capability.rs
  gates existing exact-numeric cases only; it has no Dynamic checked-capability
  issuer.

src/backend/mir_interpreter/exec/exact_numeric_value_checker.rs
  validates Integer/ExactNumeric values but does not normalize IntegerBox.

src/llvm_py/instructions/exact_numeric_ops.py
  has metadata-driven ptr/int and missing-value compatibility behavior; it is
  not a Dynamic representation proof.

src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/
  current disposition has BorrowedIngressNoEnd and an owned vocabulary, but
  the selected outer projection action is not yet issued/co-sealed.
```

Therefore the next D0 work is not to reopen Completion, Recipe, GenericLoop,
or the existing source inventory. It is to name the representation issuer,
target-aware backend capability issuer, and outer-disposition issuer, then
close their negative matrix before any fresh session or Builder effect.

##### Selected-corridor representation provenance

Every runtime-polymorphic value carries representation provenance from its
producer to its consumer. Return lowering never probes bare bits or repairs a
missing receipt.

| Corridor edge | Required issuer fact | Missing fact |
|---|---|---|
| parameter ingress / prelude local | exact incoming representation and borrowed/owned disposition | `NoSafeSlice` |
| `DynamicAdd` normal result | representation of the new result | `NoSafeSlice` |
| Dynamic invocation normal result | representation of the new result | `NoSafeSlice` |
| local copy / rebind | preserved exact representation relation | `NoSafeSlice` |
| PHI/current carrier | all incoming representations form one verified representation | `NoSafeSlice` |
| inner Return / outer Tail | exact demand row plus retained representation | `NoSafeSlice` |

The selected D0 must decide whether the unchanged `skip_while` corridor is
provably `ImmediateI64` throughout or requires the complete private
`TaggedCarrier { tag, payload }` path. A partial tagged corridor is rejected.
`PreparedLoopCarrierRepresentationV1::SourceBackedDynamic` is source lineage,
not physical representation evidence.

##### Backend capability matrix

Every selected backend/representation cell is exactly `Direct`, `Checked`, or
`RejectBeforeEffect`. Fallback is not a capability class.

| Representation | VM target | LLVM target |
|---|---|---|
| `ImmediateI64` | `Direct` after producer witness | `Direct` after producer witness |
| `IntegerBoxHandle` | `Checked` VM downcast/normalize | `Checked` strict status/out helper |
| private `TaggedCarrier` | `Checked` enum/tag match | `Checked` only after tag/payload survives the whole corridor |
| exact-numeric wrapper | explicit normalization Decision or `RejectBeforeEffect` | `RejectBeforeEffect` in this cohort |
| ambiguous bare `i64` | `RejectBeforeEffect` | `RejectBeforeEffect` |

The backend capability is not semantic-program-owned. The demand records a
required abstract capability; a target-aware physical/session ingress, or a
separately accepted backend-neutral MIR contract, must be the sole capability
issuer. The current compile request does not itself prove the backend.

The strict helper ABI must separate a valid zero from failure, for example:

```c
int32_t hako_integer_try_get_h_v1(
    uint64_t handle,
    int64_t* out_value
);
```

It is called only with an upstream `IntegerBoxHandle` receipt. Success alone
writes `out_value`; invalid handle and non-IntegerBox remain distinct internal
reasons. The helper owns no cleanup, Completion, Fault precedence, or source
meaning. Selected return callers of sentinel-zero `nyash.integer.get_h` remain
zero.

##### Outer disposition and Fault chronology

Projection changes the Dynamic carrier into a primitive result; it does not
silently forward the original carrier. The final design must therefore close
the outer path as well as the inner path:

```text
borrowed ingress current
  -> no owned End obligation

owned current
  -> exact End authorization on projection Normal and projection Fault

projection Normal + cleanup success
  -> publish exact i64 once

projection Fault
  -> no Completion claim and no result publication
  -> projection Fault is primary unless an earlier Fault already exists

cleanup Fault after an earlier Fault
  -> suppressed; remaining teardown is best effort
```

The strict helper never performs End/release. Existing exit-transaction
chronology owns cleanup and primary/suppressed ordering. Until borrowed versus
owned outer disposition is source-backed for every selected path,
`CHECKED-DYNAMIC-I64-ABI` is not accepted for implementation.

##### D0 acceptance and later I0

This child closes only when all of the following are named and source-backed:

```text
complete producer representation table for the selected corridor
sole target-aware backend capability issuer
ImmediateI64 / IntegerBoxHandle / TaggedCarrier closed vocabulary
strict non-sentinel helper ABI
AOT/LLVM production Direct / Checked / RejectBeforeEffect matrix
Rust VM reference/smoke behavior remains non-production evidence

This matrix is retained as superseded capability evidence. Rust VM rows are
reference/compatibility/smoke-only; they are not a production backend,
provider consumer, session prerequisite, or cutover gate. New provider work
belongs only to a named AOT/LLVM production consumer.
exact two-site demand keyed by Completion site identity
outer borrowed-versus-owned cleanup disposition
projection Fault / cleanup Fault / no-result chronology
Completion remains the sole result/site owner
GenericLoop / TypeOp / global value representation remain unchanged
```

Required negative/terminal matrix:

```text
missing / foreign / duplicate / extra site or representation receipt -> reject
wrong logical operand, owner, target, block, or physical representation -> reject
ambiguous bare i64 or partial tagged corridor -> RejectBeforeEffect
IntegerBox(0) -> checked success with value 0
invalid, stale, or non-IntegerBox handle -> terminal projection Fault
projection Fault -> Completion claim 0 / result 0 / physical Return 0
normal paths -> exact two site-keyed i64 claims / physical Returns 2
projection success + cleanup Fault -> cleanup Fault primary / result 0
earlier Fault + later cleanup Fault -> earlier primary / cleanup suppressed
duplicate consumption / retry / fallback -> reject
```

If any row is missing, current behavior is `RejectBeforeEffect` before a fresh
session or Builder effect. Once all rows are accepted,
`PHYSICAL-INPUT-AUTHORITY-I0`
implements only the Builder-free demand view. Session-local materialization,
checked normal/fault blocks, exact site-keyed Completion claims, and detached
DraftSeal prepare remain `DYNAMIC-EXIT-PHYSICAL-SESSION-P0` responsibilities.
Normal paths produce exactly two physical Return terminators; Fault paths
produce zero. No synthetic return join or PHI is introduced. The package-held
final program is used directly, transitional source-seed callers reach zero by
cutover, and retry/fallback remain zero.

#### Live A-prime exact-I64 physical contract

This is the only physical-capability contract currently allowed for the
bounded `ParserScanLoopBox.skip_while/4` cohort:

```text
source pos/end: ExactTrivial(I64)
  -> package CallableParameterContractKindV1 + BindingRef
  -> sealed Pos -> Induction initializer relation
  -> mixed typed Recipe / I64 carrier / I64 JoinSig and After
  -> exact I6/I7 CallSlot argument/result evidence
  -> exact two Completion-site logical operands
  -> VerifiedAPrimeI64PhysicalDemandV1
```

The demand is Builder-free and backend-independent.  It contains the
selected callable identity/header and owner-branded function target, the
entry relation, source/Recipe/JoinSig/After/Completion facts, and a required
exact-I64 capability.  It contains no `ValueId`, `BasicBlockId`, MIR,
backend tag, helper call, or physical block.  The existing post-MIR
`FunctionEntryContract` is not an issuer input; it is a later physical
metadata consumer.

The selected backend matrix is deliberately only:

```text
all exact-I64 source/Recipe/call-edge facts present
  -> Direct
missing / foreign / duplicate / ambiguous / repair-only / unsupported
  -> RejectBeforeEffect
```

The generic Dynamic checked corridor is not a fallback.  A-prime does not
use `resolve_i64`, missing-value zero, `ptrtoint`/`inttoptr`, IntegerBox
downcast, tagged carrier, or backend-name allowlists.

Transport has two non-overlapping stages:

```text
pre-session:
  issue VerifiedAPrimeI64PhysicalDemandV1

fresh session:
  materialize exact ValueId/BasicBlock/PHI/call/return facts
  -> FunctionMetadata physical receipt
  -> metadata.rs (sole JSON composer) calls one dedicated
     a_prime_i64_capability field encoder
  -> strict LLVM A-prime lane
```

The dedicated field encoder and strict Python lane are transport projections,
not semantic authorities.  Missing or foreign physical receipt fails before
LLVM effect; generic resolver repair and the existing fallback path are not
valid for a selected A-prime artifact.

## PHYSICAL-OPERATION-DEMAND-AUTHORITY-D0 (revised accepted)

Decision: accepted for the exact selected Dynamic full-body cohort after
external review and repository-backed owner census. This does not activate a
generic all-V2 physical path.

The physical borrow begins at the final semantic owner already retained by the
installed callable package, never at a raw inner semantic program:

~~~text
VerifiedDynamicExitTransactionCoSealV1
  -> private HRTB borrow spine
  -> VerifiedDynamicFullLoopSemanticProgramV2
     + VerifiedDynamicFullLoopPhysicalEvidenceV2
     + VerifiedLoopJoinClosureV2
  -> DynamicFullLoopPhysicalInputViewV2<'program>
  -> VerifiedDynamicLoopOperationPhysicalDemandV2<'program>
  -> PreparedDynamicLoopOperationProgramV2<'program>
~~~

The final exit co-seal remains non-Clone and non-splittable. Its
with_physical_operation_input callback is the sole future physical ingress.
No getter exposes the raw semantic program, Recipe, JoinSig, After, package
batch slot, or exit/cleanup parts.

#### Authority split

~~~text
JoinSig owner:
  LoopJoinLogicalTransferViewV2
  - loop boundary role / ports / payload
  - branch if_item / condition / arm disposition / exit item / target / payload
  - exact borrowed After
  - no Recipe blocks, placement, Exit kind/value, or physical IDs

Recipe owner:
  verified Loop condition/body/If/Exit structure
  exact item-to-loop/block placement
  - no transfer re-derivation

Dynamic semantic-program owner:
  JoinSig logical view + Recipe control/placement
    -> DynamicLoopPhysicalControlViewV2
  - relation only; no second JoinSig or Recipe

source/Recipe envelope owner:
  existing retained source + claims + Recipe + exact CallSlot rows
    -> VerifiedDynamicFullLoopPhysicalEvidenceV2
  - one private co-seal; no new source observer

final exit co-seal:
  physical control + physical evidence + execution/Fault/context
    -> one HRTB-bounded DynamicFullLoopPhysicalInputViewV2
~~~

The source/effect ledger is issued inside the existing
issue_dynamic_full_loop_source_recipe_envelope_v2 transaction. It relates
already-verified source roles, claims, Recipe placement, and exact source-bound
CallSlot rows. It does not re-observe AST, resolve names, infer targets, or own
execution faultability.

#### Exact bounded evidence

~~~text
retained binding rows        = 6
retained source rows         = 28
Recipe item placements       = 17
operation-source/effect rows = 15
control placement items      = I10 If, I12 Exit
control transfer rows        = 1
CallSlot rows                = I6, I7
Fault rows                   = I6, I7, I9

source effects:
  BindingRead          = 5
  BindingWrite         = 1
  ExternalCall         = 2
  ExpressionEvaluation = 7

execution classes:
  NonFaulting             = 12
  FaultBeforeNormalResult = 1
  ExternallyBoundOutcome  = 2
~~~

ExpressionEvaluation is a source-effect relation, not a Pure claim.
execution_class_v2 remains the exhaustive operation execution owner. The Fault
catalog remains the sole three-row fault authorization owner.

For every operation item, exactly one Expression source claim is the primary
physical anchor. I16 is intentionally special only in evidence cardinality:
StepAssignment remains auxiliary statement coverage while StepTargetI is the
single expression anchor. I6/I7 additionally require that primary expression
site to equal the retained exact CallSlot call site. No numeric item table,
name, inventory ordinal, or catalog order may repair a mismatch.

#### Transfer and Return rule

The actionable bounded transfer set is:

~~~text
Loop boundary:
  Enter
  PredicateTrue
  PredicateFalse
  Backedge

Branch:
  I10 If
    then -> I12 Return(V14) -> FunctionExit
    else -> Fallthrough

After:
  exact L0 / B0 / Dynamic relation
~~~

The existing Loop Return edge is an integrity-only summary of the branch
Return. The JoinSig logical-view issuer verifies matching role, target, and
payload, then excludes that summary from actionable physical rows. Publishing
both the branch Return and the Loop summary as actions is rejected.

Enter and Backedge are identified by loop key plus boundary role and never
receive a synthetic ItemKey. Branch and Exit retain exact I10/I12 item
identity. Direct unbranched Break/Continue/Return cannot retain an exact source
item in the current JoinSig model, so they remain outside this bounded view.
A language-wide all-V2 transfer view is still NoSafeSlice until that origin is
modeled.

#### Complete rejection boundary

Reject before Builder effects on any foreign owner/frame/scope/region/source
provenance, foreign Recipe/JoinSig/After, missing/duplicate/extra placement,
operation, CallSlot, execution, Fault, or transfer row, wrong block/loop/source
anchor/BindingRef/result/target, summary Return action, direct unbranched exit,
or V1 demand/class input.

Structural guards forbid AST/MIR re-observation, raw Recipe or as_sig reads in
physical modules, V2-to-V1 conversion, name/order/ordinal repair, synthetic
ItemKey creation, single-operation extraction, package splitting,
retry/fallback, and physical IDs before prepare.

### Execution order

No new task card or prerequisite is added. The following three rows remain in
this rolling card, and exhaustive positive/negative evidence lands with the
owning implementation commit.

#### LOOP-JOINSIG-V2-LOGICAL-TRANSFER-VIEW-I0

Status: CLOSED (I0 landed)

Landed: one borrowed JoinSig-owned view with four boundary rows, one I10
branch, one I12 Return, and one exact After. It keeps Recipe placement and
Exit meaning outside JoinSig, treats the Loop Return as integrity-only, and
creates no synthetic ItemKey. `semantic_program`, `join_sig`, and cargo-check
gates are green; physical control, demand, Builder, and session remain closed.

Landed next: `DYNAMIC-V2-PHYSICAL-INPUT-VIEW-I0`.

#### DYNAMIC-V2-PHYSICAL-INPUT-VIEW-I0 (CLOSED)

Landed: the envelope owns the 17-placement/15-operation source/effect
co-seal and lends the HRTB view with Recipe control, JoinSig, CallSlot/Fault,
and owner/frame/scope evidence. Exact 17/15/2/6 and 5/1/2/7 tests plus
`exit_transaction`, `dynamic_full_body_recipe`, cargo-check, and pointer gates
are green. Physical schedule, ABI, Completion consumption, and session stay
closed; next was `PHYSICAL-OPERATION-DEMAND-I0`.

#### PHYSICAL-OPERATION-DEMAND-I0

Status: CLOSED (I0 landed)

Landed: `VerifiedDynamicLoopOperationPhysicalDemandV2` consumes the complete
HRTB view, validates 17 placements/15 operations/one control transfer/three Fault rows,
and retains whole Recipe-order arrays with no single-item selector, V1
adapter, or raw lookup. Focused demand, dynamic-body, cargo-check, authority,
pointer, and diff gates are green. Prelude, Tail, ABI, Completion, session,
DraftSeal, publication, provider/runtime, retry, and fallback stay closed.
This is the landed pre-A-prime demand; Slice B atomically recuts the same sole
owner to the mixed-I64 program and exact three-Fault coverage. It is not a
second production mode.

## LOOP-UNIFICATION-AFTER-DYNAMIC-D0 (OPEN — FOURTH IMPLEMENTATION SLICE)
Decision: the bounded mixed-I64/Dynamic cohort can share the existing common
Loop core. This is a post-result/ABI BoxShape lane, not a new source authority.
The A-prime Builder-free demand is now green, so this series is the current
execution frontier. `LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0` landed in
`542b3a794d`: V1 physical layout now consumes a JoinSig-owned logical transfer
view, binds transfer meaning in one private placement binder, and segment
allocation consumes the placement role receipt. The global task names and order remain owned by
`loop-common-physical-demand-and-session-ssot.md`:

```text
LOOP-SEMANTIC-PROGRAM-COSEAL-R0
  -> LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0
  -> LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0
  -> LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0
  -> LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0
```
The common protocol is deliberately small:

```text
verified Recipe placement
  + JoinSig-owned logical transfer view
  -> prepared physical layout

one complete operation/source-effect ledger
  -> one complete physical demand
```
AOT/LLVM production capability is the required sibling branch before the
fresh session. Rust VM reference/smoke evidence may run independently and is
not a session, provider, or cutover gate. The Loop dependency order is fixed
and must not be inverted:

```text
semantic-program co-seal
  -> JoinSig logical transfer authority
  -> complete operation/source-effect ledger
  -> common physicalizer boundary cleanup
  -> selected If/Exit coverage
  -> pre-cutover authority gate
  -> physical session / DraftSeal prepare
```

The series consumes the accepted mixed-I64 program; it never retypes a
Dynamic value or reopens parameter classification. Broader Always/all-family
topology work and fixed-topology deletion remain later, after the selected
production edge has been cut over.

`LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0` is closed. Its private traversal derives
item order and segment boundaries only. JoinSig issues the logical
Predicate/Jump/Backedge/nested-resume transfer evidence; Layout binds that
evidence to placement; Canonical CFG emits it once. Therefore
`physical_layout.rs` and `recursive_after.rs` must not rebuild transfer meaning
from `LoopConditionV1`/Recipe condition data, and `segment_allocator.rs` must
consume the segment placement receipt instead of rescanning Recipe conditions
for Header or Body. `as_recipe()` remains a placement-only traversal in this
row; it may supply block/item order, but it may not supply condition or
transfer meaning. Physical-side name, ordinal, source-order, and current-block
repair are forbidden. No synthetic `ItemKey` or Step block is introduced.
The same BoxShape series includes:

```text
LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0
  V1/V2 consume a complete ordered ledger; repeated Recipe/evidence find scans
  are removed; this is a consumer protocol, never a V2->V1 adapter

LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0
  common stop = ReadyLoopAfterContinuationV1; callable profile-close, Tail,
  ABI, and Completion stay in the callable owner; recursive_after.rs has no
  callable symbols or hard-coded profile counts

LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0
  census fixed-role receipts versus segment receipts; delete old topology only
  after production and test callers reach zero
```

#### LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0 (CLOSED — `28c4bdd5c4`)

The second BoxShape slice is landed. V1 now issues one private
`PreparedLoopOperationLedgerV1` while the Builder-free physical demand is
sealed; both V1 physical dispatchers borrow its complete Recipe-order
operation, ReadBinding, derived-carrier, and WriteBinding arrays. The legacy
projection methods delegate to this ledger and no longer rescan Recipe,
source-evidence, or effect rows. V2 coverage now matches placements by exact
`ItemKey` and rejects duplicate, missing, extra, or mismatched rows instead of
depending on storage-order `zip`. V1 and V2 remain family-native: no adapter,
new source observer, or second Recipe/JoinSig authority was introduced.

Focused physicalizer (27 tests), V1 demand (5 tests), HRTB demand (1 test),
`cargo check --lib`, `git diff --check`, and
`bash tools/checks/loop_physical_transfer_authority_guard.sh` are green. The
next active row is `LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0`.

#### LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0 (CLOSED — `46fbf8d0d7`)

The third BoxShape slice is landed. `recursive_after.rs` now owns only the
neutral `ReadyLoopAfterContinuationV1` and common CFG/SSA/PHI After closure.
`ReadyCallableLoopProfileCloseV1`, the callable `7/4/2/1` coverage check, and
the profile condition witness now live in the existing `#[cfg(test)]`
Callable Tail adapter. Generic G0 continues to consume only the neutral After
receipt. The common physicalizer has no Callable profile symbols, Tail/ABI,
Completion, Return, or DraftSeal authority.

The consolidated transfer guard now enforces this boundary and the focused
physicalizer suite, `cargo check --lib`, and diff checks remain green. Fixed
topology receipts and the segment route were deliberately not deleted; the
next active row is the caller census
`LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0`.

#### LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0 (CLOSED — `1544d128d2`)

The fourth BoxShape slice is a census/guard only. The legacy fixed-role entry
`physicalize_topology_v1` has no non-test caller; the operation-demand variant
has no caller. The segment dispatcher and allocator are referenced only by the
Callable/Generic canaries, and `issue_for_segment` is referenced only by the
segment dispatcher. This proves the old fixed-role route is not currently a
hidden production selector, while the transitive fixed-role types remain in
common operation/emitter code for the historical canaries.

The consolidated transfer guard now rejects unexpected callers and protects
the no-new-caller boundary. No topology type or old issuer is deleted here;
hard deletion remains post-cutover `LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-R0`.
The next active selected-cohort row is `LOOP-PHYSICAL-IF-COVERAGE-I0`.

#### LOOP-PHYSICAL-IF-COVERAGE-I0 (CLOSED — 2026-08-11)

The selected Dynamic I10 branch is now fully co-sealed at the existing
physical-input boundary. `issue_control` consumes the verified placement array,
physical control rows, and JoinSig-owned logical branch view once. It rejects
missing, duplicate, or foreign branch keys; mismatched owner/body/If placement;
wrong condition; and either-arm disagreement. The exact bounded row is I10 in
L0/B1 with condition V13, then B2 containing I12 `Return(V14)` to
`FunctionExit`, and an omitted else represented only as logical/physical
fallthrough. The Loop Return summary remains integrity-only and is never a
second physical action.

The physical-input module has no Recipe/JoinSig rescan, physical ID, CFG,
Completion, ABI, fallback, or retry authority. Positive exact-row assertions
and negative arm/placement/kind tests are green, as are the consolidated
transfer guard, current-pointer guard, and Dynamic full-body test family. The
next selected row is `LOOP-PHYSICAL-EXIT-COVERAGE-I0`; this commit does not
open it.

#### LOOP-PHYSICAL-EXIT-COVERAGE-I0 (CLOSED — 2026-08-11)

The selected Dynamic Return arm is now covered by the same physical-input
co-seal. The exact I12 Exit placement must belong to L0/B2, the logical role
must target `FunctionExit`, and the physical Recipe evidence must retain a
Return operand. The JoinSig summary payload is intentionally not treated as
the Return operand: it is the carrier-transfer summary, while the exact V14
Return operand remains owned by the verified Recipe exit row. A missing Return
operand, non-`FunctionExit` target, foreign item, wrong block, or wrong exit
kind rejects the whole view. The summary Return remains integrity-only and
cannot become a second physical action.

Focused positive/negative tests, the Dynamic full-body family, both authority
guards, and the current-pointer guard are green. The next selected row is
`LOOP-PRECUTOVER-AUTHORITY-H2`; this commit does not open physical session,
Completion, ABI, topology deletion, production cutover, fallback, or retry.

The Dynamic physical-input guard's authority scan is production-scoped: it
stops before the `#[cfg(test)]` fixture module, so synthetic negative fixtures
cannot be mistaken for a production Recipe/JoinSig reconstruction path. The
full file still remains subject to the 800-line boundary.

#### Post-Dynamic cleanup acceptance matrix

The parked series is intentionally concrete about the old V1 surfaces it must
retire. This is still one BoxShape-only series; it does not create another
task family or change the current H2 parser blocker.

```text
physical_layout.rs / recursive_after.rs
  consume JoinSig transfer evidence + verified placement
  never rebuild Predicate/Jump/Backedge/nested resume from Recipe condition
  data; Recipe reads are placement-only

segment_allocator.rs
  consume segment-placement receipt
  never rescan Recipe condition roles for Header/Body

V1/V2 physical consumers
  borrow one complete ordered operation/source-effect ledger
  never repeat Recipe/evidence find scans or zip rows by storage order

common loop physicalizer
  stops at ReadyLoopAfterContinuationV1
  never imports ReadyCallableLoopProfileCloseV1
  never owns Callable Tail/ABI/Completion/Return/DraftSeal
  never hard-codes Pure/Read/Write profile counts

operation_target.rs and topology receipts
  remain in census until fixed-role callers are zero
  segment route becomes the sole production route before deletion
```

The existing cleanup-retirement card remains the owner for unrelated parked
cleanup such as route-neutral Recipe wrapper deduplication, trivial-analyzer
policy-matrix deduplication, and the compact `CURRENT_STATE` migration. Those
rows must not be mixed into this Loop transfer/physicalizer series.

Acceptance for the series requires the corresponding guards and focused tests:
zero Recipe transfer inference in layout/allocator, zero Callable profile
symbols and hard-coded profile cardinalities in common physicalizer code, zero
repeated V1 ledger scans, and a caller census proving the old topology route is
not a hidden second authority. The first R0 guard is
`bash tools/checks/loop_physical_transfer_authority_guard.sh`; it covers the
JoinSig transfer view, placement binder, placement-role allocator, condition
authority exclusion, and the 800-line boundary. A missing JoinSig capability or an unavoidable
row re-pairing is a design stop (`NoSafeSlice`), not a reason to add a lookup,
fallback, or fixture-specific branch.

Open after the A-prime parameter, semantic recut, and physical-input rows
close, and before the I64 physical-session canary. Hako producer parity/cutover
is later. This is one bounded refactor series: no accepted shape,
BoxCount, selector, production switch, legacy deletion, fallback/retry, source
rescan, profile callback, or new public plan may enter it. Guards require zero
Recipe transfer/role inference in layout or allocator, zero callable-profile
counts in the common physicalizer, zero repeated V1 ledger scans, and zero
synthetic placement keys. Any missing JoinSig capability returns to design with
`NoSafeSlice`; the current active row remains the first unfinished A-prime row.

#### Selected H2 pre-cutover BoxCount and gate

The BoxShape series above does not claim a new accepted Loop family. After it
is green, the same rolling card opens only the selected Dynamic cohort's exact
missing control rows:

```text
LOOP-PHYSICAL-IF-COVERAGE-I0
  exact I10 branch/merge transfer; no Layout inference

LOOP-PHYSICAL-EXIT-COVERAGE-I0
  exact I12 Return item/target/value transfer; no route-local Return writer

LOOP-PRECUTOVER-AUTHORITY-H2
  selected Dynamic Recipe/JoinSig/Layout/ledger/If/Exit complete
  competing selected physical authority = 0
```

`LOOP-PHYSICAL-ALWAYS-COVERAGE-I0`, broader all-family parity, and G0-specific
retirement are not prerequisites for this selected method unless the unchanged
source actually requires them. Missing selected If/Exit evidence is
`NoSafeSlice`; it is not repaired in Layout.

`LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0` is pre-cutover census and guard
preparation only. The selected old edge is deleted in
`H2-SELECTED-DYNAMIC-LOOP-CUTOVER-I0`. Hard deletion of fixed-role topology is
post-cutover work:

```text
LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-R0
  remaining production/test callers = 0
  -> delete fixed-role types and old operation_target issuer
```

No topology type is hard-deleted while a required pre-cutover caller remains.

#### LOOP-PRECUTOVER-AUTHORITY-H2 — CLOSED (`d048acea00`)

The selected-cohort census is now machine-checked by
`tools/checks/loop_precutover_authority_guard.sh`. Its deliberately narrow
pre-cutover result is:

```text
legacy AST/JoinIR physical edge     = 1 production caller
new Dynamic physical-demand callers = 0 production callers
```

The single legacy edge is the existing
`PreparedLocatedRawLoopChildEntryV1::lower_with_existing_route_v1` route into
the raw JoinIR loop lowering path. It is an explicit migration allowlist, not
a hidden second new issuer, and must remain until the named
`H2-SELECTED-DYNAMIC-LOOP-CUTOVER-I0` removes it in the same cutover that
installs the package-backed physical route. The new Dynamic physical-demand
issuer is therefore not activated early and has no production caller yet.

This census does **not** claim that the H2 gate is closed or that competing
physical authority is zero. It freezes the exact migration boundary so that
no additional legacy caller, raw physical planner, fallback, or retry can be
introduced while the fresh-session design is audited. The guard also enforces
the selected production file-size limit. Run:

```text
bash tools/checks/loop_precutover_authority_guard.sh
```

before changing the selected production edge. The next execution row is the
AOT/LLVM production capability branch below; `DYNAMIC-EXIT-PHYSICAL-SESSION-P0`
remains downstream until that branch closes.

The session is downstream of the AOT/LLVM production capability branch listed
in the A-prime physical-input contract. It must not open while that branch is
merely named. No Rust VM entry row can open the session, provider, or
production cutover; the former caller-zero row is retired by the fence below.

#### A-PRIME-VM-EXACT-I64-ENTRY-I0 — retired reference artifact

The former backend-local classifier was caller-zero and never a production
capability, provider consumer, session prerequisite, or cutover gate.
`DYNAMIC-V2-RUST-VM-NONCONSUMER-FENCE-R0` deletes that orphan and freezes the
Rust VM DynamicV2/provider/receipt/session caller set at zero. No future VM
adapter is part of the production roadmap; unsupported reference behavior
stays in its existing compatibility lane.

#### A-PRIME-LLVM-EXACT-I64-CAPABILITY-I0 — following mandatory branch

The LLVM branch is required for the selected cross-backend cutover, but it
does not widen the language representation. An exact mixed signature and
every selected call edge may be `Direct` only when the producer witness is
present. A missing/ambiguous edge, wrapper, or raw integer/handle ambiguity
is `RejectBeforeEffect`. Existing ptr/int repair, missing-argument zero,
`resolve_i64`, retry, and fallback are forbidden. Other backends remain
`RejectBeforeEffect` until a separate capability row.

The design consultation is closed with this implementation boundary. The
bounded I0 transport is now landed: a post-session sealed
`APrimeI64PhysicalReceiptV1`, one optional `FunctionMetadata` field, one
dedicated MIR-JSON encoder call, and a strict Python preflight validator.
The receipt carries explicit `ImmediateI64`/`OpaqueHandle` lanes, exact
parameter and `(block, instruction_index)` call-edge rows, and site-keyed
return rows; it is not the Builder-free demand and cannot be fabricated by
tests or metadata spelling. The generic parameter backend gate remains
unchanged: only a canonical fully-covered A-prime projection may exempt its
exact rows; unrelated exact rows still reject on LLVM. No physical session,
production caller, GenericLoop/TypeOp change, or old source-seed/raw-JoinIR
connection is part of this I0. The receipt remains caller-zero until the
named physical session consumes it exactly once.

The formal parameter lane is role-indexed, not merely position-unique:
`src` is ordinal `0`, `pos` is ordinal `1`, `end` is ordinal `2`, and
`pred_chars` is ordinal `3`. The post-session receipt contains only the exact
`pos -> 1` and `end -> 2` rows; swapped or receiver-shifted rows reject before
effect in both the Rust seal and the LLVM-side loader.

#### DYNAMIC-V2-FAMILY-NATIVE-PHYSICAL-EMITTER-D0 — newly exposed prerequisite

The selected fresh-session canary cannot honestly start yet. The landed
`VerifiedDynamicLoopOperationPhysicalDemandV2` is a complete Builder-free
view of 17 placements, 15 operations, one If/control row, one Exit relation,
and the exact Fault rows. It intentionally has no Builder consumer. The
existing `loop_recipe_physicalizer` consumes V1 `LoopOperationV1`/V1 ledger
products; converting the V2 view into those products would create a second
Recipe/source authority and is forbidden.

The next design slice is therefore a family-native V2 physical emitter, kept
inside the selected Dynamic physical boundary. It is not a new semantic
program, Recipe, JoinSig, Completion, or If authority. It may borrow only the
already co-sealed V2 physical-input view and the existing low-level Builder
emission primitives.

Required owner split:

```text
V2 physical demand
  -> one V2-native emission plan
     - exact item/owner/block placement
     - complete operation order
     - source/effect and CallSlot identity checks
     - producer-issued representation receipts
     - exact logical control/disposition rows
  -> canonical unpublished session
     - ValueId/BasicBlockId/PHI/CFG mutation
     - site-keyed Completion claims
     - DraftSeal prepare_exact_two
```

The emitter must remain a consumer, not a planner. It may not read raw Recipe
or source products, search by name/ordinal, reconstruct transfer edges, or
choose a fallback operation family. The normal two-arm `IfCfgSessionV1`
path remains the ordinary If authority; the selected I10 terminal arm uses
the already landed deferred-return token and never emits a Return. The sole
Return writer remains `draft_seal/exit_projection.rs`.

The first bounded family-native output is deliberately narrower than a
language-wide V2 physicalizer:

```text
selected A-prime cohort only
  - ImmediateI64 parameter/local/induction lanes
  - existing DynamicAdd/DynamicLess/CallSlot contracts, with no raw carrier
    reinterpretation
  - exact I10 then=Return / else=Fallthrough disposition
  - exact I12 inner and outer Completion-site claims
  - unsupported operation/representation/backend = RejectBeforeEffect
```

The Builder-free demand must also retain the already verified V2 operation
program as a private, scoped relation.  The A-prime issuer may issue
`PreparedDynamicLoopOperationProgramV2` from the co-sealed physical-input view
exactly once and lend it only to the family-native emitter; the emitter may
not re-issue the demand or reconstruct operation rows from the Recipe.  No raw
operation-program parts, V1 adapter, or cloneable program authority is added.

Two physical capabilities are still explicit gates rather than implied by the
semantic operation contract.  There is no canonical Builder emitter yet for
the normal `DynamicLess` Bool receipt, and the semantic carrier/cleanup
projection has no canonical physical temporary-discharge emitter.  The
family-native emitter therefore accepts those rows only through named
capabilities that issue producer/provenance receipts.  Until those capabilities
exist, the selected canary must reject before its first Builder effect; a
semantic `DynamicLess`/cleanup row is not permission to emit an `Unknown`
value, infer a type, or silently skip cleanup.

Acceptance for this design stop:

```text
[ ] V2 -> V1 adapter count = 0
[ ] one V2-native emitter issuer and one session handoff are named
[ ] all 17 placements and all 15 operations are consumed exactly once
[ ] CallSlot rows use the already co-sealed call relations
[ ] Dynamic operation results carry an explicit producer receipt; no raw-i64
    or handle inference is permitted
[ ] the V2 operation program is co-sealed once and exposed only through a
    scoped loan to this emitter
[ ] the two-stage move-only plan/session API and private V2 ledger are fixed
[ ] the ledger has exactly 17 placements, 15 operations, one If, one Exit,
    three Dynamic Fault rows, and site-keyed two-site Completion claims
[ ] physical schedule segments do not pretend that logical Recipe blocks are
    one-to-one physical BasicBlocks
[ ] DynamicLess has a named normal-Bool physical capability or rejects before
    effect; semantic classification alone is insufficient
[ ] Dynamic temporary cleanup has a named physical discharge capability or
    rejects before effect; no cleanup row is silently dropped
[ ] control rows are consumed as evidence; no physical transfer is rebuilt
    from Recipe
[ ] deferred terminal arm remains un-terminated until DraftSeal projection
[ ] missing/foreign/duplicate/ambiguous relation rejects before Builder effect
[ ] file split keeps each new module below the 700-line refactor band
[ ] the next implementation row is the emitter canary, not production cutover
```

#### DYNAMIC-V2-I6-CALLSLOT-ABI-D0 (HISTORICAL DESIGN — SUPERSEDED BY TEXT-SCAN RECUT)

This earlier design-stop text is retained only as design history. Its
`I7 -> V11 Dynamic OR separately accepted I64` fork is no longer live. The
current contract is the exact-I64 recut recorded below: I7 is
`TextFindNeedle -> V11:I64`, I9 is `CompareI64`, and V11 has no lease or End.
The old section must not be used to authorize an I6-only ABI, a boxed I7
result, or a second provider decision.

Worker audit and the repository/runtime census found no canonical physical
ABI for the selected Dynamic I6 `substring` CallSlot. This is a design stop,
not an invitation to reuse a legacy route. The existing `substring_hii` /
`indexOf_hh` helpers are String-specialized, do not provide a strict
normal-vs-Fault status or cleanup/disposition receipt, and cannot represent
the verified V10/V11 Dynamic result contract. Generic `RuntimeDataBox`,
legacy `BoxCall`, plugin/by-name dispatch, `nyash.integer.get_h`, raw
ptr/int repair, sentinel zero, and fallback/retry are forbidden.

The only admissible next design is one bounded, source-independent physical
capability keyed by the already co-sealed I6 relation:

```text
VerifiedDynamicFullLoopCallRelationsV2
  + exact I6 item/source CallSlot relation
  + receiver V0:Dynamic
  + arguments V6:I64, V9:I64
  + result V10:Dynamic
  + immutable plan/capability brand
      -> one DynamicV2CallSlotAbiPlanV1
      -> backend Direct | Checked | RejectBeforeEffect
      -> one move-only DynamicV2CallSlotResultReceiptV1
```

The CallSlot relation alone is not enough to choose those representation
lanes: `DynamicMember(selector, arity)` intentionally does not own receiver,
result, provider, or ABI facts. A separate route/provider capability must be
co-sealed at the selected physical boundary before the plan may say
`OpaqueHandle`, `ImmediateI64`, or any other lane. The source target remains a
message-identity owner; it must not grow a backend/provider field, and the
physical plan must not become a second provider registry.

The plan must be issued once from the verified relation and must not infer
meaning from selector text, physical `(block, instruction_index)`, or an
independent fingerprint. The canonical source dispatch arities are
`substring/2` and `indexOf/1` (receiver excluded); any receiver-inclusive
physical spelling must be an explicitly named field, not a second arity
authority. Physical location is auxiliary materialization evidence only.
The earlier transport mismatch (`substring/3`/`indexOf/2`) was corrected to
these source arities in `c5e8961c84`; this is transport hardening only and does
not create an I6 producer or a production caller.

The result handoff is split into two linear capabilities:

```text
V10 value-use receipt
  -> consumed exactly once by I7

V10 cleanup/discharge token
  -> consumed by the existing exit-transaction cleanup owner
```

The receipt is non-Clone, session/plan-branded, has no raw `ValueId` getter,
and is consumed through an expected-item/result/brand scoped callback. A
normal numeric zero must be distinguishable from failure; invalid/stale or
unsupported carriers must be terminal Faults with no result publication.
The physical ABI must preserve representation provenance and disposition
(`Forwarded` versus `EndAuthorized`) without moving cleanup into the CallSlot
helper. The selected invocation contract is `MaySuspend`; therefore a
`SynchronousNonSuspending` capability is required for this bounded leaf, or
the call rejects before Builder effect. A suspended result would require a
separate continuation/resume owner and is outside this slice.

The result-class mismatch is explicit and cannot be hidden by a helper:

```text
I6  Dynamic receiver + I64/I64 -> V10 Dynamic opaque handle
I7  Dynamic receiver + V10      -> V11 Dynamic OR a separately accepted I64
                              (never bare i64 inferred as Dynamic)
I9  consumes the accepted V11 class together with V12:I64
```

The current `indexOf` bare-i64 route does not prove the Dynamic V11 contract;
it remains RejectBeforeEffect until the V11 result class is separately sealed.
I7/V11, I9/V13, physical End, and production callers are outside this design
stop, but their result-class decision is a prerequisite for closing the I6
plan. The String-narrowing alternative is parked as a separate future
source-contract design; it must not be smuggled into this Dynamic cohort.

Acceptance for closing this row:

```text
[ ] exactly one I6/V10 plan issuer; no per-call re-resolution
[ ] source/Recipe CallSlot arity remains substring/2 and indexOf/1
[ ] logical ItemKey/source-site or opaque plan brand is co-sealed;
    block/instruction is never the semantic identity
[ ] any physical receipt validator receives the sealed plan/producer brand;
    receiver/argument/result/return ValueIds cannot be arbitrary transport rows
[ ] one named non-test consumer consumes the physical receipt exactly once;
    JSON remains observation-only and `take_a_prime_i64_physical_receipt()`
    has one live consumer before the emitter is called production-ready
[ ] explicit receiver/argument/result lanes and representation provenance
[ ] a separately owned route/provider capability proves each lane; the
    CallSlot selector/arity alone never upgrades Dynamic to String/Integer
[ ] V0/V3 formal Dynamic lanes and every V10/V11 producer/rebind carry a
    HostHandle (or other admitted tag) receipt under the same program/frame/
    scope brand; bare ValueId/Unknown/raw-i64 conversion is rejected
[ ] V10 output carries both an admitted result representation and its
    one-shot lease/discharge capability when the disposition is EndAuthorized
[ ] strict status distinguishes normal zero from Fault
[ ] `MaySuspend` is rejected unless a named synchronous capability exists;
    no implicit continuation or cleanup deferral is introduced
[ ] I7 bare-i64 helper is not accepted as proof of Dynamic V11; V11 result
    class is explicitly decided before I7 implementation
[ ] AOT/LLVM production lanes each classify Direct | Checked |
    RejectBeforeEffect; Rust VM classification is reference/smoke evidence
[ ] V10 value-use and cleanup/discharge capabilities are separate
[ ] receipt is move-only, session-branded, one-shot, and raw-getter-free
[ ] foreign/missing/duplicate/wrong lane/target/arity/status/disposition
    rejects before Builder effect
[ ] swapped arity, foreign plan/site/item, and mutated receiver/argument/result
    rows reject before Builder effect
[ ] no RuntimeDataBox, legacy BoxCall, plugin/by-name, sentinel, raw repair,
    fallback, retry, or I7/V11 claim
```

D0 is accepted as a physical transport design only. It does not accept any
I6/V10 provider or result representation yet: the selected emitter remains
`RejectBeforeEffect` until a separately owned provider descriptor proves the
lane and synchronous capability. No runtime dispatch, I6 implementation,
I7/V11 implementation, physical End, or production caller is opened by this
Decision.

#### DYNAMIC-V2-CALLSLOT-WIRE-SCHEMA-I0-A (NEXT — schema parity only)

This is the first implementation slice after the D0 decision. It defines
transport vocabulary and layout tests only; it has no runtime symbol, registry
lookup, LLVM call, VM dispatch, receipt issuer, or production caller.

The exact fixed-width schema is:

```rust
#[repr(C)]
struct DynamicV2WireValueV1 {
    tag: u32,
    reserved: u32,
    payload: u64,
}

#[repr(C)]
struct DynamicV2CallOutV1 {
    status: u32,
    fault_code: u32,
    result_tag: u32,
    disposition: u32,
    forwarded_input: u32,
    reserved: u32,
    value_payload: u64,
    lease_token: u64,
    continuation_token: u64,
}
```

The wire revision is `1`; all enums are `#[repr(u32)]`, and the C/LLVM
surface uses only `u32/u64` fields. Rust tests, C `_Static_assert` checks,
and the Python/LLVM schema must agree on size, alignment, field offsets,
constants, and validity matrices. Unknown enum values or nonzero reserved bits
reject rather than being normalized.

I0-A fixes these transport-only values:

```text
tag:          INVALID=0, HOST_HANDLE=1, IMMEDIATE_I64=2
status:       NORMAL=0, FAULT=1, SUSPENDED=2 (reserved/rejected in I0-A)
disposition:  NONE=0, FORWARDED=1, END_AUTHORIZED=2
fault_code:   NONE=0, INVALID_RECEIVER=1, INVALID_HANDLE=2, ARITY=3,
              UNSUPPORTED_SLOT=4, TYPE_MISMATCH=5, RANGE=6, RUNTIME=7,
              INVALID_RESULT=8
```

`NORMAL` requires a valid result tag and a valid disposition. `FAULT`
requires a nonzero fault code and zero result/payload/lease/continuation.
`SUSPENDED` is represented for schema parity only and is rejected by the
selected synchronous emitter until a continuation owner exists. I0-A does
not put provider brands, plan identities, selector text, or raw lease tokens
into the compiler semantic product; those remain private capability values
in later slices.

The original I0-A revision was compile-time/out-of-band: the Rust `V1` type,
the Python `WIRE_REVISION`, and the C revision macro must all equal `1`. The
wire structs carry no runtime revision field, so an unknown revision is not a
representable decoded value; a future versioned loader must reject a schema
mismatch before decoding these structs.

I0-A acceptance (historical revision-1 schema; superseded by R0):

```text
[ ] Rust repr(C)/repr(u32) model and C header layout are exact
[ ] size/alignment/field-offset and constant parity tests are green
[ ] compile-time revision parity is `1`; unknown runtime revisions are outside
    this fieldless schema and must be rejected by a future versioned loader
[ ] NORMAL/FAULT/SUSPENDED validity matrix rejects malformed combinations
[ ] normal numeric zero is not a failure sentinel
[ ] no runtime dispatch, registry, LLVM extern call, VM call, or receipt issuer
[ ] no RuntimeDataBox, legacy BoxCall, string helper, selector inference,
    fallback, retry, or production caller
[ ] each new source file remains below the 700-line refactor band
```

#### DYNAMIC-V2-CALLSLOT-WIRE-SCHEMA-I0-A — implementation receipt (2026-08-11)

I0-A is closed as a transport-only slice. Rust `repr(C)`/`repr(u32)` types,
the fixed-width C header, and the Python/LLVM `ctypes` mirror now share the
same revision-1 constants, field offsets, 16-byte value layout, and 48-byte
call-out layout. Rust and Python validators reject unknown enum values,
nonzero reserved fields, malformed Normal/Fault/Suspended combinations, and
fixed-width overflow; numeric zero remains a valid Normal payload. The
forwarded-input sentinel is not treated as a lane when an arity observation is
provided.

Focused receipt:

```text
Rust call-slot wire tests: 8 passed
Python call-slot wire tests: 8 passed
C11 and C++17 header syntax checks: passed
dynamic_v2_physical_input_authority_guard.sh: green
a_prime_i64_parameter_lane_guard.sh: green
current_state_pointer_guard.sh: green
```

The slice adds no runtime symbol, provider registry, VM/LLVM call, receipt
issuer, I6/V10 lane, I7/V11 result decision, physical End, or production
caller. `DynamicV2CallOutV1` remains non-Clone/non-Copy so the later runtime
owner can enforce one-shot lease/discharge consumption. This revision-1
receipt is historical transport evidence; the active wire authority is the
revision-2 row below. No dispatcher-only implementation is opened.

#### DYNAMIC-V2-CALLSLOT-WIRE-AUTHORITY-R0 — implementation receipt (2026-08-12)

The C header remains the sole fixed-width layout owner. The Rust projection
was moved from the MIR emitter canary to `src/abi/dynamic_call_slot_wire.rs`;
the MIR module edge and duplicate file were removed. The Python/LLVM module
remains a checked projection only.

Wire revision 2 preserves the 16-byte value and 48-byte call-out layouts and
all existing tag/status/disposition/fault numeric values. It adds exactly one
valid Normal form for the landed exact-I64 corridor:

```text
Normal + ImmediateI64 + None disposition
  forwarded_input = UINT32_MAX
  lease_token = 0
  continuation_token = 0

Normal + HostHandle
  still requires Forwarded or EndAuthorized lifecycle rules
```

Rust, Python, and C parity guards cover revision/constants, retired MIR
ownership, fixed layouts, and the ImmediateI64 validity/negative matrix.
Focused receipt:

```text
Rust wire tests: 10 passed
Python wire tests: 10 passed
C11 and C++17 header syntax checks: passed
dynamic_v2_callslot_wire_authority_guard.sh: green
dynamic_v2_physical_input_authority_guard.sh: green
current_state_pointer_guard.sh: green
```

No provider, registry, runtime symbol, LLVM/VM caller, receipt issuer,
fallback, retry, or production switch was added.

The next bounded row is the selected AOT executable cell; it remains closed
until its complete provider/admission authority is implemented atomically.

#### DYNAMIC-V2-CALLSLOT-RUNTIME-DISPATCH-I0-B — superseded forecast

The dispatcher-only row is rejected. Runtime code may be added only inside
the later atomic AOT executable cell, together with its canonical provider
authority, production receipt issuer, and selected LLVM consumer.

#### DYNAMIC-V2-CALLSLOT-PROVIDER-PLAN-D0 — accepted design, implementation gated

No product named `ProviderPlan` is accepted. The presealed runtime
architecture and exact `hako.text.scan@1` contract are accepted. I0-B remains
`RejectBeforeEffect` until the exact-I64 semantic recut, VM nonconsumer fence,
and neutral wire revision are closed.

```text
Decision:
  ACCEPT: use complete hako.text.scan@1 plus a presealed canonical Text branch;
  reject an I6-only slot and selector-derived String/provider claims
Source authority + canonical issuer:
  ProviderSlot contract artifact + provider export facts
    -> one consuming ProviderAdmissionSeal
    -> immutable admitted BoxCallableRegistry
    -> RoutePlan -> RuntimeExecutablePlan
Non-authority:
  I6/I7 names, Dynamic value class, raw String/StringBox keys, slot numbers,
  TypeRegistry, LLVM, runtime, PluginLoader compatibility snapshots, Rust VM
Fail-fast boundary:
  incomplete capability, I6/I7/I64 drift, undeclared alias, byte/CP mismatch,
  duplicate/collision, ambiguous branch, foreign plan, or unsafe lease rejects
Smallest next slice:
  DYNAMIC-V2-TEXT-SCAN-I64-SEMANTIC-RECUT-R0 below
Non-claims:
  no provider/admission code, runtime symbol, LLVM hook, I6/I7 emission,
  Physical End, VM consumer, fallback, retry, or production switch
```

The current exact source/Recipe authority closes only these relations:

```text
I6: substring/2, V0:Dynamic, [V6:I64, V9:I64] -> V10:Dynamic
I7: indexOf/1, V3:Dynamic, [V10:Dynamic]       -> V11:Dynamic
```

It does not prove that either receiver is String or belongs to a provider.
Selector spelling, Recipe class, an internal slot number, or runtime type name
must never repair that missing relation.

##### DYNAMIC-V2-CALLSLOT-COMPLETE-SLOT-CONTRACT-D0 — accepted closeout

`hako.text.scan@1` is accepted as a small, independently replaceable
capability. `complete` means that one provider implements every role declared
by this versioned slot; it does not mean every public String method. An I6-only
slot is still forbidden. Expanding this cell to the seven cursor signatures or
the full String surface would add unrelated provider obligations without
proving the selected I6/I7 relation.

The contract owns two semantic roles, not two selector spellings:

| role | exact semantic row | result/lifecycle |
| --- | --- | --- |
| `TextSliceRange` | borrowed canonical Text + two CP offsets -> Text; zero-based half-open range, independently clamped endpoints | Normal is either a forwarded Text input or a fresh immutable Text carrier with one non-reused End lease |
| `TextFindNeedle` | borrowed canonical Text + borrowed canonical Text -> exact I64; first CP index, empty needle `0`, miss `-1` | `ImmediateI64`, trivial no-End, no lease |

Both rows use profile `utf8-codepoint-clamped-v1`, are synchronous and
non-suspending, retain no input, invoke no callback, and do not re-enter Hako.
They execute on the invoking thread. `TextSliceRange` may allocate its result;
`TextFindNeedle` is read-only. Invalid/dead/non-Text handles, wrong wire tags or
arity, and allocation failure are Faults. Clamping and search miss are Normal.
The selected call remains conservatively `OpaqueObservable`; the provider
contract does not reclassify the caller-side invocation envelope as Pure.

The source and relation owners are distinct:

```text
docs/reference/language/strings.md
  -> Text / UTF-8 code-point semantics

neutral hako.text.scan@1 contract artifact
  -> role set, exact row contracts, profile, Home/Fault/effect/sync/lifecycle

selected A-prime profile requirement
  -> required contract id
  -> SubstringCall => TextSliceRange
  -> IndexOfCall   => TextFindNeedle

the consuming ProviderAdmissionSeal
  -> co-seals requirement + artifact + exact I6/I7 relations + provider exports
```

The normalized contract artifact is the sole capability-contract issuer. The
selected A-prime profile owns only the requirement and role map; it cannot
redefine the contract. Selector/name/arity, Recipe `Dynamic`, raw String keys,
`hako.toml`, TypeRegistry, LLVM/runtime, and Rust VM are non-authorities.
Rust/C/Python projections are never independent contract sources.

The selected profile fixes `pos/end` as CP offsets and requires both `src` and
`pred_chars` to satisfy canonical Text at the checked call boundary. I6's Text
result is the exact I7 needle relation. Runtime validates only a presealed
canonical receiver branch; it does not search a registry or derive Text from a
selector.

Raw `String` and `StringBox` exports are cold aliases. Admission may collapse
them to one canonical Text branch only when contract, provider, profile,
target, ABI, and lifecycle all match. Missing/duplicate roles, different
providers/profiles, undeclared aliases, byte-indexed implementations, or more
than one canonical branch reject before Builder effect. Existing env-selected
String operations and parse/default-zero compatibility surfaces cannot be
admitted.

The sole provider spine remains:

```text
contract artifact + provider export facts
  -> BoxCallableRegistryBuilderV1
  -> one consuming ProviderAdmissionSealV1
  -> immutable deterministic BoxCallableRegistry
  -> MethodCallRoutePlan -> RuntimeExecutablePlanV1
  -> private I6/I7 branches -> LLVM early hook -> strict AOT leaf
```

The current mutable/Clone/overwrite registry is only a draft skeleton. The
builder/seal/admitted transition lands with the named AOT consumer; no
caller-zero registry authority is introduced. PluginLoader snapshots,
arity-0 repair, `ffi_bridge`, and `runtime_invoke_boundary` stay compatibility
only.

The audit found one prerequisite semantic inconsistency. Language and the
accepted slot say `indexOf -> I64`, while the current selected Recipe says
`V11:Dynamic`, applies `DynamicLess` at I9, and gives V11 an End obligation.
Boxing the integer or injecting bare I64 into Dynamic is rejected: both create
an unnecessary allocation/lifecycle or an unproved Dynamic representation.

##### DYNAMIC-V2-TEXT-SCAN-I64-SEMANTIC-RECUT-R0 (IMPLEMENTATION RECEIPT)

Change:
  atomically replace I7/V11 with exact I64, I9 with `CompareI64(Less)`, the
  selected Fault catalog from I6/I7/I9 to I6/I7, and V11 cleanup/End with
  trivial no-End. Delete the superseded Dynamic result/lifecycle rows in the
  same commit.

Contract:
  `docs/reference/language/strings.md` and the exact I7 relation are the
  semantic sources. Recipe/call relation, execution classes, Fault catalog,
  invocation lifecycle, cleanup cutpoints, physical evidence/input, and tests
  must move together. This row creates no provider, registry, runtime/LLVM/VM
  consumer, or production switch.

Done:
  V11 is I64 everywhere; I9 is nonfaulting `CompareI64`; selected Fault rows
  are exactly I6/I7; V11 End rows are zero; V10 remains the only invocation
  carrier with `EndExactlyOnceUnlessForwarded`; missing/old/foreign mixed rows
  reject; the existing authority guard freezes the counts.

Implementation receipt (2026-08-12):
  `mapping.rs`, the CallSlot co-seal, semantic Fault/lifecycle/cleanup
  projections, physical evidence/input, A-prime demand, and selected
  preflight now agree on one mixed typed Recipe.  The exact landed counts are
  `Fault=2 (I6/I7)`, `Dynamic lifecycle=1 (I6/V10)`, `cleanup=4`, and
  physical execution classes `NonFaulting=13 / Faulting=0 /
  ExternallyBound=2`.  I7/V11 is `I64` from CallSlot result through I9
  `CompareI64(Less)` and has no lease, End, or Fault row.  Focused
  `dynamic_full_body_recipe` (31/31) and
  `normal_callable_semantic_package` (16/16) tests, the physical-input
  authority guard, and the current-pointer guard are green.  No provider,
  registry, runtime, LLVM, VM, physical session, fallback, retry, or
  production caller was added by this receipt.

Transport projection follow-up (2026-08-12):
  The post-session A-prime Rust/Python validators and the Rust JSON fixture
  were also synchronized with this recut.  The source arities remain
  `substring/2` and `indexOf/1`; substring returns an opaque Text handle,
  while indexOf returns `ImmediateI64`.  The four formal lanes remain
  `src=0,pos=1,end=2,pred_chars=3`, with role-index checks `pos=1` and
  `end=2`.  A stale `substring/3`/`indexOf/2` or I7 opaque result now rejects
  before effect.  Focused Rust receipt/JSON tests (7/2) and Python loader
  tests (9) are green.  This is transport consistency only: no provider,
  runtime, LLVM production hook, VM consumer, or production caller was added.

Stop:
  if any owner still requires V11 Dynamic, an I9 Fault, or V11 End, return to
  design. Do not repair with IntegerBox, fake/no-op lease, Dynamic injection,
  name lookup, fallback, retry, or VM work.

##### Wire and carrier lease decisions

`include/nyrt_dynamic_call_slot_v2.h` remains the sole fixed-width layout,
revision, constant, and validity-vocabulary owner. Rust and Python are checked
projections only. The later wire BoxShape slice moves the Rust projection from
the MIRBuilder subtree to `src/abi/dynamic_call_slot_v2.rs`, deletes the old
definition, and adds C/Rust/Python size/alignment/offset/constant parity. Since
revision 1 rejected every Normal result with disposition `None`, that slice
also bumps the out-of-band revision to 2 and admits exactly one trivial case:
`Normal + ImmediateI64 + None + no forwarded lane + lease 0`. HostHandle
results still require Forwarded or EndAuthorized. It adds no provider, runtime,
LLVM, or VM consumer.

The carrier lease token is never a raw `HostHandle`, `drop_epoch`, legacy
generation-zero `BoxIdentity`, registry generation, or `PlanStamp`. The host
lifecycle controller owns a non-reused opaque `DynamicV2CarrierLeaseIdV1` and
an exact live entry `{ result_handle, lifecycle_class }`:

```text
Normal + EndAuthorized: result handle + fresh live lease are published atomically
Normal + Forwarded:     fresh lease = 0
Fault / Suspended:      result = 0, lease = 0
consume_end:            remove live entry once, verify expected handle, then drop
```

Unknown, duplicate, stale, foreign, mismatched, and counter-overflow tokens
reject without dropping another object. The ABA negative must reuse a freed raw
handle for a new object, retry the stale lease, reject it, and keep the new
object alive.

##### Task DAG after the semantic recut closes

```text
DYNAMIC-V2-RUST-VM-NONCONSUMER-FENCE-R0             BoxShape
  delete caller-zero DynamicV2 VM classifier/module edge; guard every VM
  provider/receipt/adapter/session/production symbol at zero

DYNAMIC-V2-CALLSLOT-WIRE-AUTHORITY-R0               BoxShape
  C header remains owner; move the Rust projection to src/abi, delete the MIR
  copy, revise validity vocabulary to wire revision 2 for trivial I64 Normal,
  freeze C/Rust/Python parity; runtime/provider/VM callers remain zero

DYNAMIC-V2-CALLSLOT-AOT-EXECUTABLE-CELL-I0-B        BoxCount, atomic
  contract artifact + builder/admission/admitted registry transition + exact
  I6/I7 RoutePlan/RuntimeExecutablePlan branches + canonical-session receipts
  + one LLVM early hook + strict AOT leaf + value/lease aggregate
  selected generic fallthrough = 0

I0-B
  -> typed I9 Bool + ordered V10 Physical End all-or-nothing gate
  -> fresh unpublished session + exact-two DraftSeal
  -> selected production cutover and old-edge deletion
  -> H2/H3/H5 parity || Hako-mimalloc promotion gate
  -> Hako frontend issuer cutover
```

The former standalone `DYNAMIC-V2-CALLSLOT-PROVIDER-SPINE-R0` is retired from
the DAG: an admitted registry with no named production consumer would be an
orphan authority. I0-B is one activation commit with small owner modules, not
one large source file; split near 650-700 lines and stop at 800.

```text
I0-B acceptance:
  hako.text.scan@1 roles / admission issuer                         = 2 / 1
  I6-only pseudo slot                                               = 0
  I6/I7 contract, provider, profile identity                        = same
  source-backed static String claim                                 = 0
  String/StringBox canonical Text branch after admission            = 1
  admitted mutable insert / duplicate overwrite                     = 0 / 0
  RoutePlan / RuntimeExecutablePlan / exact image pin owners        = 1 each
  I6/I7 production receipt issuers                                  = 1 each
  I7 ImmediateI64 / I7 lease / V11 End                              = 1 / 0 / 0
  LLVM selected early hook / strict AOT leaf                        = 1 / 1
  runtime registry/name/selector/provider/image lookup or reselection = 0
  selected generic/legacy fallthrough                               = 0
  carrier lease issuer / raw-host-handle lease alias                = 1 / 0
  stale/duplicate/foreign lease acceptance                          = 0
  Rust VM DynamicV2 production consumer                             = 0
  retry / fallback / sentinel-zero repair                           = 0
```

The provider design is closed, but the capability matrix remains
`RejectBeforeEffect` until the semantic recut and every downstream capability
in this DAG are implemented. No provider code is authorized by the D0
closeout alone.

##### Provider-slot audit reconciliation (2026-08-12)

The worker audit considered widening this contract to a `text.cursor` or
`text.index` family because the language-level String API also exposes
`length`, `lastIndexOf`, and overloads. That widening is **not** adopted.
ProviderSlot completeness is scoped to the versioned role set declared by the
slot; it does not mean "every method on the receiver class". The accepted
`hako.text.scan@1` role set is therefore exactly:

```text
TextSliceRange  = substring/2
TextFindNeedle  = indexOf/1
```

Both roles share one canonical Text/UTF-8 code-point profile, one provider,
one ABI/lifecycle contract, and one admission seal. `length`,
`lastIndexOf`, unrelated overloads, and the full String surface are explicit
non-claims, not missing rows to be silently filled by a compatibility
provider. A future cursor/index slot would require a separate Decision and a
separate complete role set; it cannot be mixed into this cell.

This decision does **not** prove that a selected caller produced `end` from
`length`, nor that a current byte-index helper implements the CP profile.
Those are independent caller/capability proofs. The selected AOT cell must
reject before effect when the provider export is byte-indexed, environment
selected, receiver-alias ambiguous, missing either role, or otherwise fails
the exact profile. The language String reference remains the CP semantic
source; this small provider contract is its bounded AOT projection.

The audit also confirms that the next implementation boundary is still one
atomic AOT cell, not a hook-only slice. Live code is still missing the
`ProviderAdmissionSeal`, immutable admitted registry, call-in admission wire,
receiver-identity-bearing `RuntimeExecutablePlan`, canonical-session I6/I7
receipts, and strict LLVM leaf. Until those owners and their named LLVM
consumer land together, the row remains `RejectBeforeEffect`; no Rust VM
provider/adapter or legacy String route may be used to bridge the gap.

##### DYNAMIC-V2-CALLSLOT-AOT-EXECUTABLE-CELL-D0-CLOSEOUT (design stop, 2026-08-12)

The semantic slot decision is closed, but the implementation authority is not
yet sufficiently named to authorize I0-B code. This is a design stop, not a
request for a hook-only canary or a second provider registry.

```text
Decision:
  Keep hako.text.scan@1 as one complete two-role slot
  (TextSliceRange/substring/2 and TextFindNeedle/indexOf/1). Before I0-B,
  close the neutral contract artifact, the admission boundary, the call-in
  wire, and the receiver-identity-bearing executable branch as one owner chain.
Source authority + canonical issuer:
  normalized hako.text.scan@1 contract artifact (CP semantics, role rows,
  Home/Fault/effect/sync/lifecycle/representation) + provider export facts
  -> one consuming ProviderAdmissionSeal -> immutable admitted registry.
  The selected A-prime profile supplies only the required contract id and
  exact I6/I7 role map; it does not redefine the contract.
Non-authority:
  source selector/name/arity, Recipe Dynamic class, raw String/StringBox keys,
  hako.toml, TypeRegistry, PluginLoader snapshots, legacy RoutePlan enum,
  Rust VM, LLVM/Python/Rust wire projections, and runtime registry/name lookup.
Fail-fast boundary:
  missing/foreign/duplicate/ambiguous role, provider, alias, CP profile, ABI,
  lifecycle, registry generation, receiver branch, call-in token, image pin,
  or I6/I7 canonical-session receipt rejects before Builder effect. Runtime
  may inspect and consume only an already sealed branch; it may not search or
  reselect a provider, image, selector, or route.
Smallest next slice:
  docs-only D0 closeout: fix the artifact/requirement/ProviderAdmissionSeal
  ownership, exact call-in admission fields, and RuntimeExecutablePlan branch
  identity below. Then one atomic I0-B activation commit must land the builder
  -> seal -> admitted-registry transition, I6/I7 RoutePlan and executable
  branches, strict LLVM early consumer/leaf, canonical-session receipts, and
  the V10 lease/End consumer together.
Non-claims:
  no full String surface, no I6-only pseudo-slot, no Dynamic-specific registry,
  no Rust VM feature/provider/receipt/session, no runtime lookup, no Physical
  End in isolation, no fallback/retry/sentinel repair, and no production
  switch until the complete atomic cell is green.
```

The D0 closeout fixes the following concrete boundary before implementation:

```text
normalized contract artifact
  source owner to be introduced in the atomic I0-B slice:
    lang/src/runtime/meta/provider_slot_contract_box.hako
  it references canonical CoreMethod row identities and owns contract id,
  version, CP profile, two role rows, input/result lanes, Home/Fault/effect,
  sync/lifecycle and alias rules. Its normalized generated projection belongs
  at lang/src/runtime/meta/generated/provider_slot_contract_manifest.json and
  is not hand-edited. The generator is the only issuer of that normalized
  artifact; Rust/C/Python projections consume it and never restate its rows.

selected A-prime requirement
  owns: required contract id and
        SubstringCall -> TextSliceRange / IndexOfCall -> TextFindNeedle

ProviderAdmissionSeal
  owns: one-time co-seal of requirement + artifact + I6/I7 relations
        + provider export facts + canonical Text aliases + ABI/lifecycle

admitted registry
  owns: immutable deterministic admitted rows, generation/brand and no raw
        mutable insert or duplicate overwrite

call-in admission wire (new ABI surface, not the result-output wire)
  normative owner to be introduced in the atomic I0-B slice:
    include/nyrt_dynamic_call_slot_v2_admission.h
  checked projections:
    src/abi/dynamic_call_slot_v2_admission.rs
    src/llvm_py/builders/dynamic_v2_callslot_admission_wire.py
  owns only: wire revision, registry generation, opaque admission token,
  I6/I7 role, receiver WireValue, argc, and fixed argument WireValue lanes.
  RuntimeExecutablePlan image/entry/PlanStamp/receiver-branch identity is
  referenced by the opaque token and stays in the presealed plan; it is not
  copied into this wire. Selector/name/raw callable key never enters it.

RuntimeExecutablePlan
  owns: presealed receiver branch, provider entry/image pin, PlanStamp, and
        the admitted call-in token. It contains no registry/name lookup.

LLVM selected consumer
  owns: one early hook before generic method lowering and one strict leaf;
  selected malformed/unsupported input is terminal and never falls through.

runtime leaf
  owns: receiver-key check and one consumption of the sealed branch/result;
  it does not construct a plan or choose a provider.

##### I0-B preflight audit receipt (2026-08-12)

The preflight audit confirms that the semantic decision is sufficient but the
implementation cell is not yet present. The focused A-prime transport checks
are green for the exact four-formal lane (`src=0`, `pos=1`, `end=2`,
`pred_chars=3`), including role/index mismatch rejection and I7
`ImmediateI64` result validation. This evidence does not count as an I0-B
production caller or provider implementation.

The only accepted implementation shape remains one atomic activation cell:

```text
contract artifact (canonical Text / CP / two roles)
  + selected A-prime requirement (contract id + I6/I7 role map)
  + provider export facts and canonical aliases
      -> one consuming ProviderAdmissionSeal
      -> immutable admitted registry
      -> receiver-identity-bearing RoutePlan/RuntimeExecutablePlan
      -> call-in admission wire projections
      -> LLVM early selected consumer + strict CP leaf
      -> canonical-session I6/I7 receipts + V10 lease/End
```

Do not land the contract manifest, registry, wire, hook, runtime leaf, or
lease owner as an isolated preparatory authority. They are all named parts of
the same I0-B consumer cell. Existing mutable `BoxCallableRegistry`, plugin
snapshots, legacy String helpers, and Rust VM routes remain compatibility or
reference lanes and are not retargeted.

Required I0-B preflight negatives are now explicit:

```text
missing/foreign/duplicate role or provider       -> RejectBeforeEffect
String/StringBox alias ambiguity                 -> admission reject
byte/env index mode                              -> admission reject
I6/I7 profile or lifecycle mismatch              -> admission reject
I7 Dynamic/lease/End reintroduction              -> admission reject
runtime registry/name/provider/image lookup      -> forbidden
selected generic/legacy fallthrough              -> forbidden
VM DynamicV2 provider/receipt/session consumer    -> zero
```

The active row therefore stays
`DYNAMIC-V2-CALLSLOT-AOT-EXECUTABLE-CELL-I0-B` in `fast` mode. No new
semantic receipt or partial provider module is authorized before the complete
named consumer can be included in the same bounded activation series.
```

The current output wire in `include/nyrt_dynamic_call_slot_v2.h` remains
transport-only. It is not the admission wire and must not be stretched to
carry provider, selector, image, or registry semantics. The Rust and Python
types remain projections of that output wire. A separate neutral call-in
admission schema is required in the I0-B activation slice, with one normative
owner and checked projections; a third hand-written semantic table is
forbidden.

The D0 field/owner decision is now accepted and the active pointer is the
implementation row below.  The implementation is still not landed: adding
only `BoxCallableRegistry`, a provider plan, a runtime symbol, or an LLVM hook
would create an orphan authority.  The whole named chain must remain one
atomic I0-B activation cell.

There is no Rust VM node in this production DAG. Rust VM stays limited to
already-supported bootstrap/recovery/compatibility smoke until caller-zero
retirement; `.hako` is the future semantic-reference owner. A VM provider,
receipt, representation adapter, runtime leaf, or production acceptance gate
is explicitly out of scope.

##### DYNAMIC-V2-RUST-VM-NONCONSUMER-FENCE-R0 (IMPLEMENTATION RECEIPT)

The caller-zero VM A-prime classifier module
`src/backend/mir_interpreter/exec/a_prime_i64_entry.rs` and its module edge
were removed. The Rust interpreter keeps its existing generic numeric
contract behavior; it does not classify selected A-prime arguments, consume a
DynamicV2 demand, issue a provider/receipt, or open a physical session.
`tools/checks/dynamic_v2_vm_nonconsumer_fence_guard.sh` now freezes zero
DynamicV2/A-prime provider, receipt, adapter, session, and production symbols
under the Rust interpreter, Rust VM runner, and VM test/reference roots. This
is a BoxShape fence, not VM feature work, and does not authorize a VM provider
or production gate.

At the ABI boundary, `EndAuthorized` carries one one-shot lease, `Forwarded`
carries no fresh lease, and Fault publishes neither result nor lease.
Suspension remains rejected until a named continuation owner exists.

Implementation boundary to use after this D0 is accepted:

```text
src/mir/builder/resolved_lowering/selected_dynamic_physical_abi.rs
  target size: <= 350 lines; split child modules before 500

preflight (Builder effect = 0)
  input:  VerifiedAPrimeI64PhysicalDemandV1<'program>
         + selected package/source identity
  output: move-only V2-native emission plan
          - exact 17 placement rows
          - exact 15 operation rows
          - exact CallSlot relation rows
          - exact I10 control/disposition requirement
          - exact inner/outer Completion-site keys

session-local realization
  input:  the plan + canonical function session + producer-issued entry values
  output: session-local representation receipts and value map
          - V0..V3 entry values
          - V1/I64 local and induction lane
          - V4..V17 operation results
          - I10 condition and surviving merge evidence
  owner:  canonical SSA/CFG/PHI session; no second map escapes the session

terminal
  input:  verified deferred-return token + site-keyed completion claims
  output: OpenFunctionDraftSealV1::prepare_exact_two(outer_site)
  Return writer: draft_seal/exit_projection.rs only
```

The landed I8 canary uses the following consuming handoff; the older
`plan.emit_into(...)` sketch is superseded and must not be implemented:

```text
issue_selected_dynamic_v2_emission_plan(demand)
  -> PreparedSelectedDynamicV2EmissionPlan
     Builder effect = 0
     no raw Recipe/JoinSig/ValueId escape

DynamicV2PhysicalEmissionSessionV1::begin(
    plan,
    canonical_function_session,
    canonical_ssa_session,
  )
  -> unpublished session owner
     canonical CFG/SSA ownership

session.emit_i8_const()
  -> one session-branded, move-only I8 receipt

session.discard_unpublished()
  -> terminal discard; no production caller
```

The private `DynamicV2NativePreflightLedgerV1` is the only pre-session ledger.
It has
exactly 17 placement rows, 15 operation rows, one If row, one Exit row, the
three selected Dynamic Fault rows (I6/I7/I9), per-item emitted/producer
evidence state, and the two site-keyed Completion claims.  It has no name,
ordinal, zip, or repair lookup and is discarded with the unpublished session
on any failure.  The I8 leaf consumes only its exact evidence row; I7, End,
and the complete session finish remain downstream and cannot expose a second
value map.

The physical plan must not create a one-to-one `LoopBlockKey -> BasicBlock`
map: the verified V2 body places operations on a logical body block around the
I10 branch.  The plan therefore uses private physical schedule segments keyed
by the co-sealed operation/control order (pre-I10 body, I10 terminal arm,
post-I10 continuation/step, backedge, and after).  This is a physical schedule
only; it may not invent a Recipe block, synthetic ItemKey, or new transfer
meaning.  If the exact I10 boundary cannot be proven from the complete V2
ledger, the plan rejects before its first Builder effect.

The segment boundary is derived only from exact operation placement and the
co-sealed I10 control row: operations in the I10 then block are terminal;
operations in the owning body block before the exact I10 item are prelude;
operations after that item are continuation. Source roles remain diagnostic
cross-checks in the ledger and never select a physical segment.

For the selected source shape (`else_block = None`), the emitter uses the
existing `IfCfgSessionV1::open_implicit_false` path and the one-sided deferred
return verifier.  Only the narrow
`crate::mir::builder::resolved_lowering` visibility needed by this sibling
emitter may be widened; the token types remain private and no crate-wide If
API is introduced.  `open_explicit_else` is not a substitute for this shape.

The plan issuer validates the operation family exhaustively. The selected
cohort's first operation cases are:

```text
ReadBinding / ConstI64 / BinaryI64 / CompareI64
  -> existing low-level I64 emission primitives, with V2 rows and V2 value
     keys retained in the family-native ledger

CallSlot
  -> dedicated V2-family emitters that consume their already sealed execution
     class/Fault/CallSlot row and issue an explicit normal-result receipt;
     missing provider or Dynamic temporary-cleanup capability is a pre-effect
     rejection

WriteBinding
  -> canonical session identity/rebind owner, consuming the V2 binding row

If / Exit
  -> control consumer only; no operation emitter and no synthetic ItemKey
```

No operation is emitted through a `LoopOperationV1` conversion. The V2
value ledger is private to this selected session and is discarded with the
unpublished function on any failure. A missing producer receipt, a result
whose class differs from its V2 value row, or a CallSlot without its exact
co-sealed relation rejects before the first Builder effect. The implementation
row must add focused positive/negative tests in a child test module rather
than growing `operation_emitter.rs`, `located_if.rs`, or
`recursive_child_lowering.rs`.

#### DYNAMIC-V2-FAMILY-NATIVE-PHYSICAL-EMITTER-D0 — design closeout

**Accepted with implementation capability gates.**  The owner split and the
two-stage API are now closed. The private V2 operation-program co-seal, the
move-only family-native preflight plan, and its evidence ledger are landed;
they must not claim a fresh-session canary or a production caller. The accepted
text-scan recut replaces the old DynamicLess capability with ordinary typed
CompareI64 and narrows temporary cleanup to V10. This is an implementation
boundary, not a reason to add a semantic authority or a V1 adapter.

The already-landed R0 below records the A-prime issuer's one private
`PreparedDynamicLoopOperationProgramV2` scoped loan and the Builder-free
plan/ledger contract. The subsequent emitter canary, fresh session, production
cutover, and old-route retirement remain separate rows. The old DynamicLess
capability decision is superseded by the exact-I64 recut below.

#### DYNAMIC-V2-PHYSICAL-DEMAND-COSEAL-R0 — landed

The A-prime issuer now consumes the complete V2 physical-input view once,
issues `issue_dynamic_full_loop_operation_physical_demand_v2`, and calls
`prepare_all()` exactly once before constructing the demand.  The demand owns
the resulting non-`Clone` operation program and lends it only through
`with_operation_program`; the previous raw physical-input accessor was
removed.  Physical-demand rejection remains typed and terminal.

The landed focused proof checked 17 placements, 15 operations, and the former
three-Fault catalog. The semantic recut must atomically replace that catalog
with exact I6/I7 coverage. This slice added no Builder effect, fresh session,
cleanup emitter, production caller, fallback, or retry.

The next execution row after the capability decisions is:

```text
DYNAMIC-V2-PHYSICAL-EMITTER-I0
```

It is preflight-first. The first bounded sub-slice is recorded below; the
remaining I0 work must still construct the complete private V2 ledger and
named capability gates before any session effect.

Until this row is accepted and implemented, the selected Dynamic fresh-session
canary remains `NoSafeSlice`. Do not call the old raw JoinIR route a canary and
do not promote the package adapter. This is a BoxShape/authority boundary,
not a reason to add a compatibility fallback.

#### DYNAMIC-V2-PHYSICAL-EMITTER-PREFLIGHT-S0 — landed

The selected package loan now has one Builder-free preflight entry point:

```text
issue_selected_dynamic_v2_emission_plan(A-prime demand)
  -> move-only V2-native schedule plan
```

This slice consumes the A-prime demand once, lends the sole co-sealed
`PreparedDynamicLoopOperationProgramV2` only while validating the plan, and
records the bounded 15-operation schedule across the pre-I10, terminal, and
continuation segments. Its former 17/15/1/3 count is a landed baseline; the
recut replaces the final count with two Fault rows while preserving the
co-sealed CallSlot relation and the exact I10
then-Return/else-Fallthrough shape. The only caller is the focused package
canary; no production caller exists yet.

The plan is intentionally not a session emitter. A private preflight ledger
now co-seals copied placement owner/block/kind rows (17), operation
source/effect/execution/CallSlot identity rows (15), the one If/one Exit
control disposition, the selected Fault rows, and the two Completion site keys. It
does not contain `ValueId`/`BasicBlockId` or emitted receipts; those remain
session-local downstream work. The named CallSlot/temporary-cleanup physical
capabilities also remain open.
No Builder/session/ValueId/BasicBlock effect, production caller, V1 adapter,
raw Recipe/JoinIR route, fallback, or retry is introduced. The selected canary
therefore remains `NoSafeSlice` until the remaining I0 gates are closed.

#### DYNAMIC-V2-PHYSICAL-EMITTER-LEDGER-S1 — landed

`PreparedSelectedDynamicV2EmissionPlanV1` now owns one private
`DynamicV2NativePreflightLedgerV1`. The ledger is only an accounting view over
the existing co-sealed V2 physical input; it is not a second source, Recipe,
JoinSig, Fault, or Completion authority. It is move-only with the plan and is
available only through a scoped loan. The A-prime demand and its sole V2
operation program are consumed once at plan issuance; later session work must
consume the plan ledger and add only session-local producer receipts.

This S1 slice is complete only for Builder-free evidence. It deliberately does
not claim the emitter I0, a fresh session, a CallSlot provider capability, a
temporary-discharge capability, or a production caller.

Before the capability I0 row, the ledger's move-only contract must be made
literal: `DynamicV2NativePreflightLedgerV1` must not implement `Clone` in
production. This is now landed in `0ef252baf7`; tests may compare a separate
borrowed coverage projection, but a physical sibling must not copy the
evidence ledger and create a second emission authority.

#### DYNAMIC-V2-DYNAMICLESS-BOOL-CAPABILITY-D0 — superseded

This design was internally correct for the former `V11:Dynamic` Recipe, but
the accepted `hako.text.scan@1` contract proves that `indexOf/1` returns
exact I64. The following semantic recut supersedes the DynamicLess demand,
its I9 Fault row, and the V11 cleanup obligation. Git history owns the landed
D0/I0 evidence; it must not remain a second live contract in this card.

The recut uses the ordinary verified V2 `CompareI64(Less)` operation family.
It consumes exact I7/V11 and I8/V12 I64 producer rows and produces V13:Bool.
There is no selected DynamicLess provider, execution envelope, physical
capability demand, or Fault handoff after the recut.

#### DYNAMIC-V2-TEMPORARY-CLEANUP-CAPABILITY-D0 — revised target

Only V10, the Text result of I6, remains an invocation carrier. V11 is an
ImmediateI64 trivial result and never receives a lease or End. Until the
provider leaf is activated, V10's existing conservative exit coverage remains:

```text
I6 Fault       -> []
I7 Fault       -> dispose V10 according to its exact lifecycle receipt
inner Return   -> dispose V10 according to its exact lifecycle receipt
Backedge       -> dispose V10 according to its exact lifecycle receipt
```

A Forwarded V10 has no fresh lease; an EndAuthorized V10 has exactly one
non-reused lease. A future earlier release at I7 normal completion is an
optimization and is not part of the recut. Primary/suppressed Fault chronology
remains owned by the exit transaction. Generic Drop/Arc, last-use inference,
raw handles, and no-op End are forbidden.

The Builder-free cleanup demand and its move-only admission aggregate must be
atomically regenerated from these verified rows during
`DYNAMIC-V2-TEXT-SCAN-I64-SEMANTIC-RECUT-R0`. Old six-row cleanup evidence,
any I9 Fault row, and any V11 discharge row are rejected rather than accepted
as compatibility input.

#### DYNAMIC-V2-PHYSICAL-CAPABILITY-ADMISSION-I0 — historical landed base

Commit `0ef252baf7` landed a move-only pre-effect gate for the former
DynamicLess/six-row-cleanup model and proved that the preflight ledger is not
Clone and that source roles do not select physical segments. Those structural
properties remain required. The accepted semantic recut replaces only the
obsolete I9/V11 meaning and cleanup counts; it does not add a session or
production caller.

The already-landed I8 unpublished-session canary remains valid: it emits
`ConstI64(0) -> V12` through the canonical integer emitter and issues one
session-branded move-only I64 receipt. The exact four-formal transport layout
remains `src=0,pos=1,end=2,pred_chars=3`; swapped lanes reject.

#### DYNAMIC-V2-PHYSICAL-EMITTER-I0 — downstream after the recut

After the recut, the remaining selected physical leaves are finite:

```text
I6 TextSliceRange provider receipt -> V10 Text value + lifecycle aggregate
I7 TextFindNeedle provider receipt -> V11 ImmediateI64, no lease
I8 canonical ConstI64 receipt       -> V12 ImmediateI64
I9 canonical CompareI64             -> V13 Bool
V10 ordered physical discharge      -> existing exit transaction
```

I6/I7 must come from the same admitted `hako.text.scan@1` provider/profile.
I7 consumes the exact V10 value receipt; I9 consumes exact V11/V12 receipts.
The physical End leaf consumes only the V10 lifecycle aggregate. No operation
resolves a selector, rescans Recipe/source roles, infers a representation from
a ValueId, or uses the generic RuntimeDataBox/legacy BoxCall path.

The A-prime post-session transport remains move-only and one-shot. It retains
the exact four formal lanes, exact inner/outer Completion sites, and canonical
CallSlot role/receiver/argument/result identities. Rust VM has no provider,
receipt, adapter, or session node in this path.

The complete gate stays RejectBeforeEffect until I6, I7, V10 discharge, and
the strict AOT/LLVM leaves are all present. Only then may the fresh unpublished
session, site-keyed Completion consumption, exact-two DraftSeal, and selected
production cutover open. Fallback, retry, sentinel-zero repair, and selected
legacy finalization remain zero.

#### DYNAMIC-EXIT-PHYSICAL-SESSION-P0 — downstream implementation boundary

The fresh-session row remains downstream of the AOT/LLVM production capability contract. It
may consume the already-landed Builder-free physical input, but it must not
issue new source/Recipe/JoinSig meaning. Its first production adapter must consume
the selected package loan and the A-prime physical demand exactly once.

Before opening a session, the implementation must keep these prerequisites
explicit and local:

```text
backend capability       = AOT/LLVM Direct|Checked or RejectBeforeEffect;
                             Rust VM is reference/smoke only
physical demand           = named package-backed production consumer
Completion                = site-keyed claims for both exact return sites
DraftSeal                 = new submodule for multi-Return prepare; commit moves only
common large files        = no new responsibility in operation_emitter.rs or flat draft_seal.rs
legacy route              = the one census allowlist remains until H2 cutover
failure                   = pre-effect reject or whole unpublished-session discard
fallback/retry            = forbidden
```

`ResolvedFunctionCompletionConsumptionV1` must stop using a single boolean
and one optional witness for this selected two-site cohort. Extend the existing
Completion owner through a site-keyed claim set in its own small module; do not
create a sibling result or return authority. The physical session keeps
`ValueId`/`BasicBlockId` receipts session-local, while the pre-session demand
remains Builder-free. `draft_seal.rs` is already near the file-size boundary,
so multi-site claim/projection logic belongs in focused child modules rather
than in the flat file. The current slice now has a source-order claim set and
an unpublished detached projection that can place Return×2 only on distinct
un-terminated blocks; it does not connect to a live session or production
caller. A missing backend capability, terminated If branch, or missing exact
relation is `NoSafeSlice`, not a fallback route.

The fresh-session implementation has one additional mandatory boundary before
the live two-site owner can open. The selected I10 `then = Return` /
`else = Fallthrough` disposition must be fixed before either branch is closed.
The normal `IfCfgSessionV1::close_then` / `close_else` path remains the
all-fallthrough `Jump -> merge -> PHI` authority and is not widened for this
cohort. A selected Dynamic session may issue only a private, source-keyed
deferred-return disposition token: the terminal arm remains un-terminated and
is excluded from merge predecessors; the surviving fallthrough arm alone
jumps to merge. A tokenless un-terminated block, two-terminal-arm shape,
no-terminal-arm shape, join rows on the terminal arm, foreign/duplicate
disposition, or a terminated terminal block rejects before effect.

`exit_projection.rs` remains the sole Return writer. The If disposition
records CFG evidence only; it never emits a Return and never creates a return
join or PHI. The selected session consumes the package-held Dynamic program,
the A-prime demand, the disposition token, and the site-keyed two-site
Completion claim set exactly once, then passes the same private exit set into
`prepare_exact_two()`. The live owner must preserve `site -> block/value`
identity (no array zip or ordinal repair), require `expected == claimed == 2`,
and produce exactly two normal Return instructions in the unpublished draft.
This keeps DraftSeal as the single Return writer while avoiding post-hoc CFG
repair.

The builder-disconnected disposition token and one-sided predecessor verifier
are now landed in `if_materialization.rs`; they do not emit a Return and have
no production caller. The implementation remains staged inside this one
rolling card:

```text
1. detached claim/projection (closed)
2. live DraftSeal prepare_exact_two using the same projection pipeline
3. bounded deferred-return disposition evidence for I10 (closed)
4. V2 family-native physical emitter design/implementation:
     complete V2 demand -> V2-native emission plan -> session-local receipts
5. selected Dynamic fresh-session canary:
     package loan -> A-prime demand -> ImmediateI64 claims
     -> disposition-aware branch close -> prepare_exact_two
6. production caller switch only after all five gates are green
```

No new Completion, Recipe, JoinSig, If, or physical Return authority is
created by these child rows. The production A-prime caller remains zero until
the canary consumes the entire chain and proves the old source-seed/raw-JoinIR
edge can be deleted in the same cutover.

### Pre-session contract hardening required before the live canary

The selected V2 path must not pass an arbitrary `&SourceStmtSiteV1` as the
outer argument to `prepare_exact_two()`. The outer site must be borrowed from
the already site-keyed A-prime/Completion relation (or a private typed
`DynamicAPrimeOuterCompletionSiteV1` capability issued by that relation). A
caller cannot select the inner site, an unrelated callable site, or repair the
pair by array order/ordinal. The DraftSeal owner remains the only Return
writer; this requirement only narrows the input capability.

The same live-canary slice must use exact signature/Recipe/producer evidence
for type preparation. `infer_return_type(name)` and any function/method-name
compatibility whitelist remain legacy-only and must have zero callers on the
selected V2 route. Missing exact type evidence is `RejectBeforeEffect`, never
name-based repair. These are hardening conditions on the existing family-native
emitter row, not new semantic or physical authorities.

## Hard stops

```text
no result annotation implies I64 carrier
no untyped parameter becomes I64 without the exact parameter-contract chain
no old Dynamic induction lifecycle survives the mixed-Recipe recut
no End/Home obligation is attached to the I64 induction
no Dynamic invocation temporary cleanup is deleted with the induction lifecycle
no Fault becomes a Recipe value/Exit or JoinSig edge
no Layout/allocator re-infers transfer or segment role from Recipe
no AOT/LLVM production Direct classification without the complete selected receipt
no Rust VM provider/session classification; VM remains reference/smoke only
no Completion consumption before the site-keyed multi-return owner lands
no physical CFG/DraftSeal/collector/publication before the fresh session
no retry/fallback, source narrowing, terminal Dynamic-to-I64 helper, or tagged-corridor auto-open
no test-only semantic, ABI, lifecycle, or backend-capability constructor
```

## File-size plan

```text
resolved_control_flow/function_control.rs        current 606
  Completion remains the existing owner; keep additions minimal
  split before growth crosses the 650-700 refactor band

builder/resolved_lowering/completion_consumption.rs  current 191
  site-keyed claim set and focused tests belong here

builder/resolved_lowering/draft_seal.rs          current 545
  keep multi-site claim/projection logic out of the flat file
  exit projection lives in focused children:
    draft_seal/exit_projection.rs                current 162
    draft_seal/multi_site_exit.rs                current 272
  live-session integration remains downstream

builder/recursive_child_lowering.rs              current 785
  explicit no-addition surface

loop_recipe_contract/join_sig/
  transfer_view_v2.rs
  transfer_view_v2_tests.rs

dynamic_full_body_recipe/coseal/
  operation_source.rs
  semantic_program/exit_transaction/
    physical_input.rs
    physical_input_tests.rs
  semantic_program/dynamic_temporary_cleanup.rs  target <= 300

callable_parameter_contract/
  README.md
  model.rs
  issuer.rs
  tests.rs

builder/resolved_lowering/selected_dynamic_physical_abi.rs
  session-local ImmediateI64 receipts only; target <= 350

llvm_py/builders/selected_callable_physical_abi.py
llvm_py/instructions/mir_call/selected_callable_abi.py
llvm_py/instructions/selected_exact_i64_return.py
  selected direct ABI only; generic resolver/repair imports forbidden

llvm_py/builders/function_lower_prepass.py      current about 740
  explicit no-addition surface; delegate to the new preflight module

dynamic_full_body_recipe/physical_demand/
  mod.rs
  model.rs
  issuer.rs
  tests.rs
```

Split at roughly 650-700 lines, stop adding at 760, and keep 800 as the hard
limit. Do not add these relations to `typed_schema_v2.rs` (757),
`join_sig/flow.rs`, the LLVM resolver (about 768), or a standalone public
`VerifiedCh*` product.

##### I0-B implementation-readiness audit (2026-08-12, workers + clean-tree closeout)

The design is implementation-ready only as one bounded AOT/LLVM activation
cell. A partial provider registry, wire, hook, or runtime symbol is not an
accepted intermediate commit because it would create an authority with no
named production consumer. The shared tree was returned clean after this
audit; no orphan provider skeleton is landed.

The accepted capability is exactly one versioned slot, not the full String
surface and not an I6-only pseudo-slot:

```text
hako.text.scan@1
  profile: utf8-codepoint-clamped-v1
  receiver: canonical Text
  aliases: String | StringBox, canonicalized before admission
  TextSliceRange  = substring/2
      inputs  CanonicalText, ImmediateI64, ImmediateI64
      result  CanonicalText, EndAuthorized lease
  TextFindNeedle = indexOf/1
      inputs  CanonicalText, CanonicalText
      result  ImmediateI64, no lease and no End
```

The contract source is the sole semantic source for this slot. The selected
A-prime requirement owns only the required contract id and the two role map;
it does not restate CP semantics, lifecycle, or provider ABI. `hako.toml`,
the lockfile, TypeRegistry, PluginLoader compatibility snapshots, selector
strings, raw `String`/`StringBox` keys, LLVM/Python/Rust projections, and the
Rust VM are non-authorities.

The remaining implementation row is deliberately explicit:

```text
1. contract source -> normalized/generated projection
2. A-prime requirement + exact I6/I7 source/Recipe relations
3. provider export facts + String/StringBox alias co-seal
4. one consuming ProviderAdmissionSeal
5. immutable deterministic admitted registry
6. receiver-identity RoutePlan -> RuntimeExecutablePlan
7. separate call-in wire (header owner; Rust/Python checked projections)
8. strict AOT/LLVM leaf using CodePoint primitives only
9. I6 V10 result + monotonic lease/End; I7 ImmediateI64 with lease=0
10. canonical-session I6/I7 materialization receipts
11. LLVM early selected consumer before generic method lowering
12. focused positive/negative tests and reusable authority guard
```

Steps 1--12 are one activation commit semantically, even if the files are
split into small modules. The commit must not contain a provider plan without
its consumer. The strict leaf may inspect only the admitted token, generation,
receiver lane, and exact argument lanes. It must never perform registry,
selector, provider, image, or compatibility lookup. The AOT path is the only
production path; no Rust VM provider/adapter/receipt/session is to be added.

The first strict kernel expectations are fixed here so the representation
boundary cannot drift during implementation:

```text
I6 substring/2:
  canonical StringBox text only
  CodePoint substring with clamped [start,end)
  HostHandle result + one non-reused lease token

I7 indexOf/1:
  canonical StringBox text pair only
  CodePoint index, not-found = -1, zero is normal
  ImmediateI64 result, lease token = 0, End = 0

Fault:
  invalid/foreign/dead handle, lane, generation, token, or role
  -> Fault with no result and no lease; selected route never falls through
```

The strict leaf must publish an I6 result from a retained object identity,
not use the raw host handle as a lease token. End removes a one-shot lease,
checks retained identity before dropping the handle, and rejects stale,
foreign, duplicate, or ABA-reused tokens. Existing compatibility exports
such as `StringBox::invoke_surface`, environment-selected byte mode, and
sentinel-zero conversions are forbidden on this selected path.

Required mechanical acceptance for the activation cell:

```text
contract source / normalized generator                 = 1 / 1
ProviderAdmissionSeal issuer                          = 1
admitted registry mutable insert / Clone              = 0 / 0
complete TextScan role coverage                       = 2
I6/I7 same contract/provider/profile                   = 1
String/StringBox canonical branch after alias seal    = 1
receiver identity + generation + image/entry stamp    = present
call-in wire normative owner                           = 1
LLVM selected early consumer / strict leaf             = 1 / 1
canonical I6/I7 session receipt                        = 1 / 1
I6 lease issuer / End consumer                         = 1 / 1
I7 lease / End                                         = 0 / 0
selected generic/legacy fallthrough                    = 0
runtime selector/provider/registry/image lookup        = 0
Rust VM DynamicV2 provider/receipt/session consumer    = 0
fallback / retry / sentinel-zero repair               = 0
```

Required negative cases include missing, foreign, duplicate, and ambiguous
roles/provider/aliases; byte or environment index mode; I6/I7 result or
lifecycle drift; wrong receiver/argument lane; wrong formal index; foreign
generation/token/image; malformed Normal/Fault; stale or duplicate lease;
and selected generic-method fallback. Any missing row is `RejectBeforeEffect`
and keeps `DYNAMIC-V2-CALLSLOT-AOT-EXECUTABLE-CELL-I0-B` open.

##### AGENTS.md audit receipt (2026-08-12)

The local `AGENTS.md` entry contract was read in full. It agrees with the
tracked current pointer, the active card, the AOT-only backend choice, the
one-authority/no-orphan rule, and the 760/800-line budget. It is gitignored
local guidance; no durable rule belongs there, so no `AGENTS.md` edit is
needed. The tracked active card remains the durable task source.
