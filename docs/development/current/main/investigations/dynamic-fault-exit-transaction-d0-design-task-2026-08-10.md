# DYNAMIC-FAULT-EXIT-TRANSACTION-D0

Status: logical/fault/flow/operation-demand foundation and the explicit result
`: i64` Completion relation are landed. The previous checked-Dynamic return
corridor is superseded for this bounded method by accepted target A-prime:
`pos/end: i64 -> exact entry contract -> exact local copy -> mixed typed Recipe
-> I64 carrier -> ImmediateI64 physicalization`. Production remains
`NoSafeSlice` until the implementation rows below close. The full tagged
Dynamic corridor is parked rather than partially implemented.
Date: 2026-08-10
Depends on: `LOOP-V2-SEMANTIC-PROGRAM-COSEAL-I0` closed
Authority:
`language-result-propagation-and-exit-transaction-ssot.md`,
`mirbuilder-final-pipeline-ssot.md`,
`loop-common-physical-demand-and-session-ssot.md`, and the live A-prime
Decision in this card. `dynamic-invocation.md` remains the authority for the
I6/I7/I9 Dynamic temporary operations. The landed exact-Dynamic V2 program is
an input to Slice B's atomic replacement, not a competing selected program.

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

### Physical session P0 audit (parked parent)

`DYNAMIC-EXIT-PHYSICAL-SESSION-P0` remains parked, not an implementation
permission. The existing `loop_physical_prepare.rs` and callable physical
canary are `cfg(test)` helpers only; no production issuer yet supplies the
complete physical input for the selected Dynamic package.

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

#### Slice B: A-PRIME-MIXED-RECIPE-SEMANTIC-RECUT-I0

Status: CLOSED (I0 landed)

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

#### Slice C: A-PRIME-PHYSICAL-CAPABILITY-I0

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

After C1, three branches must be green before the session:

```text
branch 1:
  LOOP-UNIFICATION-AFTER-DYNAMIC-D0 selected subset

branch 2:
  A-PRIME-VM-EXACT-I64-ENTRY-I0

branch 3:
  LLVM-SELECTED-DYNAMIC-EXACT-I64-DIRECT-I0
```

VM accepts `VMValue::Integer` under the exact entry contract, then carries
ImmediateI64 directly. Bool/String/BoxRef/foreign values reject before body
effect. `ExactNumeric` also rejects in this bounded row until an explicit
normalization Decision exists; the present checker does not normalize it to an
Integer carrier.

`LLVM-SELECTED-DYNAMIC-EXACT-I64-DIRECT-I0` is mandatory before the selected
cross-backend cutover. It is not a backend-name allowlist edit:

```text
exact mixed signature
+ every selected call edge proves pos/end ImmediateI64
  -> Direct

missing/ambiguous edge, wrapper, raw integer/handle ambiguity
  -> RejectBeforeEffect
```

The LLVM row owns parameter/call-edge capability only. It must eliminate the
selected use of ptr/int repair, missing-argument zero, `resolve_i64`, retry, and
fallback. Primary surfaces are the existing backend-capability owner, MIR/JSON
call-edge metadata, `llvm_py/builders/function_lower.py`, a split child of the
approximately 740-line `function_lower_prepass.py`, and
`instructions/mir_call/global_call.py`. Other backends remain
RejectBeforeEffect.

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
       +-> VM exact I64 capability
       +-> LLVM exact I64 capability

all three branches green
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

The next boundary is intentionally a design stop: `PHYSICAL-INPUT-AUTHORITY-I0`
must close the producer provenance, target-aware backend capability issuer,
strict helper ABI, and outer disposition required by the selected
`CHECKED-DYNAMIC-I64-ABI` architecture. Until then, the unsupported behavior is
`RejectBeforeEffect`, the slice remains `NoSafeSlice`, and this row does not
claim physical I64 compatibility.

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
  -> required checked-I64 capability
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
  CHECKED-DYNAMIC-I64-ABI
  = boundary-local checked projection
    + producer-issued representation provenance

current unsupported behavior:
  RejectBeforeEffect
```

No existing source/semantic authority currently proves either logical Dynamic
operand is physical `i64`, and the checked path is not executable until every
selected producer and backend cell has a canonical issuer. Adding
`MirType::Integer` to a Dynamic ValueId because the declaration says `i64` is
forbidden. This question belongs to the existing physical-input row; it does
not create a new task card.

The current three-row Fault catalog remains the exact Recipe-operation catalog.
A checked return projection is a callable-terminal sibling keyed by the exact
Completion site; it is never inserted as a Recipe item, JoinSig edge, or a
seventh operation row. The final callable exit transaction must co-seal that
sibling with cleanup and primary/suppressed Fault chronology before session
mutation.

#### PHYSICAL-INPUT-AUTHORITY-I0 decision and next task

The design decision is fixed, but implementation is not yet authorized:

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

No existing verified product proves either logical Dynamic operand is physical
`i64`. `CHECKED-DYNAMIC-I64-ABI` is the selected target, but
its producer provenance, backend capability issuer, and outer disposition are
still unresolved. Until those canonical issuers exist, current behavior is
`RejectBeforeEffect` and the slice remains `NoSafeSlice`.

The sole next task is therefore the remainder of this existing row:

```text
PHYSICAL-INPUT-AUTHORITY-I0
  child design: PHYSICAL-INPUT-DYNAMIC-I64-REPRESENTATION-D0
  design only until the complete CHECKED-DYNAMIC-I64-ABI capability is accepted
  then issue the Builder-free two-site demand; realization remains session-local
```

It must not create a sibling result contract, Static/Dynamic arbitration sum,
standalone initializer bridge, Dynamic source reissuer, GenericLoop change,
Recipe/JoinSig physical type, or public raw `ValueId`/slot API. Loop
unification remains the next structural lane after this row; it cannot replace
the missing Dynamic-to-i64 conformance proof.

#### PHYSICAL-INPUT-DYNAMIC-I64-REPRESENTATION-D0 (design child; no new card)

Decision: target `CHECKED-DYNAMIC-I64-ABI` is accepted as the only candidate
architecture; current unsupported behavior remains `RejectBeforeEffect`. Global
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
VM and LLVM Direct / Checked / RejectBeforeEffect matrix
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

## LOOP-UNIFICATION-AFTER-DYNAMIC-D0 (PARKED)
Decision: the bounded Dynamic cohort can share the existing common Loop core,
but this is a post-result/ABI BoxShape lane, not a new source authority or
current execution row. The global task names and order remain owned by
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
This parked BoxShape series opens after `A-PRIME-PHYSICAL-INPUT-I0` is green.
VM and LLVM capability are sibling branches and must also be green before the
fresh session. The Loop dependency order is fixed and must not be inverted:

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

`LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0` is the first implementation row. Its
private traversal may derive item order and segment boundaries only. JoinSig
issues the logical Predicate/Jump/Backedge/nested-resume transfer evidence;
Layout binds that evidence to placement; Canonical CFG emits it once. Therefore
`physical_layout.rs` and `recursive_after.rs` must not rebuild transfer meaning
from `LoopConditionV1`/`as_recipe()`, and `segment_allocator.rs` must consume the
segment placement receipt instead of rescanning Recipe conditions for Header or
Body. Physical-side name, ordinal, source-order, and current-block repair are
forbidden. No synthetic `ItemKey` or Step block is introduced.
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

#### Post-Dynamic cleanup acceptance matrix

The parked series is intentionally concrete about the old V1 surfaces it must
retire. This is still one BoxShape-only series; it does not create another
task family or change the current H2 parser blocker.

```text
physical_layout.rs / recursive_after.rs
  consume JoinSig transfer evidence + verified placement
  never rebuild Predicate/Jump/Backedge/nested resume from Recipe

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
not a hidden second authority. A missing JoinSig capability or an unavoidable
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
## Hard stops

```text
no result annotation implies I64 carrier
no untyped parameter becomes I64 without the exact parameter-contract chain
no old Dynamic induction lifecycle survives the mixed-Recipe recut
no End/Home obligation is attached to the I64 induction
no Dynamic invocation temporary cleanup is deleted with the induction lifecycle
no Fault becomes a Recipe value/Exit or JoinSig edge
no Layout/allocator re-infers transfer or segment role from Recipe
no VM/LLVM Direct classification without the complete selected receipt
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

builder/resolved_lowering/draft_seal.rs          current 688
  no new multi-site logic in the flat file
  first move exit projection behavior-neutrally into:
    draft_seal/exit_projection.rs                target <= 350
  then extend the detached projection in the physical-session series

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
