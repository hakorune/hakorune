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

10. LOOP-JOINSIG-V2-LOGICAL-TRANSFER-VIEW-I0
    NEXT: JoinSig-owned borrowed logical flow/After view only

11. DYNAMIC-V2-PHYSICAL-INPUT-VIEW-I0
    pending: 17/15 source/control evidence plus final-co-seal HRTB view

12. PHYSICAL-OPERATION-DEMAND-I0
    pending: whole-program Builder-free demand/prepare

13. PHYSICAL-INPUT-AUTHORITY-I0
    parked: later Prelude/Tail/ABI/physical Completion co-seal

14. DYNAMIC-EXIT-PHYSICAL-SESSION-P0
    parked until rows 10-13 are green
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

That closeout originally named `DYNAMIC-EXIT-PHYSICAL-SESSION-P0`. The later
audit below inserted the missing Builder-free physical-input authority rows
before any session may open.

## Physical session P0 audit (parked parent)

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

## Physical-input authority bridge D0 (accepted parent boundary)

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

### Remaining task ladder after this Decision

```text
LOOP-JOINSIG-V2-LOGICAL-TRANSFER-VIEW-I0
  -> DYNAMIC-V2-PHYSICAL-INPUT-VIEW-I0
  -> PHYSICAL-OPERATION-DEMAND-I0
  -> PHYSICAL-INPUT-AUTHORITY-I0
  -> zero-effect complete callable physical-input co-seal
  -> DYNAMIC-EXIT-PHYSICAL-SESSION-P0
  -> fresh session, common physicalizer, finish/DraftSeal canary
```

The three operation-demand rows add no Builder effect. The broader

```text
PHYSICAL-INPUT-AUTHORITY-I0
  -> one bounded source-backed callable input co-seal
     (Prelude / Tail / ABI / physical Completion)
DYNAMIC-EXIT-PHYSICAL-SESSION-P0
  -> fresh session and unpublished function canary
```

may not begin until the three rows are green. It must not promote the
existing test canary or introduce Home, runtime Fault, retry, or fallback.

## Parked full physical-input frontier (2026-08-10)

The parent owner census is accepted, but the full callable physical-input
boundary remains parked because the existing products cannot yet form the
required input without crossing later Prelude/Tail/ABI/Completion authority.
This is not the current executable row.

After PHYSICAL-OPERATION-DEMAND-I0, one installed-package scoped issuer must
co-seal Prelude/entry, Tail/result, physical ABI, and exact Completion
handoff. It may not reissue Recipe/JoinSig/After, re-verify Completion in
Lower, or adapt V2 through the old V1 demand. Until then, `cfg(test)`
promotion, session open, DraftSeal, Collector, raw `lower_loop`, retry, and
fallback remain forbidden.

### PHYSICAL-OPERATION-DEMAND-AUTHORITY-D0 (revised accepted)

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
control rows                 = I10 If, I12 Exit
CallSlot rows                = I6, I7
Fault rows                   = I1, I5, I6, I7, I9, I15

source effects:
  BindingRead          = 5
  BindingWrite         = 1
  ExternalCall         = 2
  ExpressionEvaluation = 7

execution classes:
  NonFaulting             = 9
  FaultBeforeNormalResult = 4
  ExternallyBoundOutcome  = 2
~~~

ExpressionEvaluation is a source-effect relation, not a Pure claim.
execution_class_v2 remains the exhaustive operation execution owner. The Fault
catalog remains the sole six-row fault authorization owner.

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

Change:
- added one borrowed V2 logical transfer view under the JoinSig subtree;
- lends loop boundary rows, branch rows, and the already co-sealed After.

Contract:
- JoinSig owns flow only;
- no Recipe block/placement/Exit-kind interpretation;
- Return summary is integrity-only; no synthetic ItemKey.

Done:
- `VerifiedLoopJoinClosureV2::logical_transfer_view()` is the sole downstream
  entry and keeps raw `VerifiedLoopJoinSigV2` out of the Dynamic semantic test
  surface;
- exact four boundary rows (`Enter`, `PredicateTrue`, `PredicateFalse`,
  `Backedge`), one I10 branch, one I12 Return, and one co-sealed After pass
  `semantic_program` and `join_sig` focused tests;
- the Loop Return summary is checked against the branch Return's role, target,
  payload, and `Body` origin, then exposed only as summary evidence;
- direct unbranched exits remain rejected by this bounded view.

Stop:
- no Dynamic physical control co-seal, physical demand, Builder, or session.

Evidence:
- `RUSTFLAGS=-Awarnings cargo test -q --lib semantic_program` (17 passed)
- `RUSTFLAGS=-Awarnings cargo test -q --lib join_sig` (31 passed)
- `cargo check --lib` (pass)

Landed next: `DYNAMIC-V2-PHYSICAL-INPUT-VIEW-I0`.

#### DYNAMIC-V2-PHYSICAL-INPUT-VIEW-I0 (CLOSED)

Landed:
- the existing envelope now owns one private 17-placement/15-operation
  source/effect co-seal; I16 uses `StepTargetI` as primary and retains the
  statement claim only as auxiliary coverage;
- `VerifiedDynamicExitTransactionCoSealV1` lends one HRTB physical-input
  view combining the JoinSig logical view, verified Recipe control/placement,
  exact CallSlot/Fault rows, and owner/frame/scope/provenance;
- the bounded cohort exposes four actionable Loop boundaries and one
  branch-owned Return; the Loop Return summary is integrity-only.

Evidence:
- exact 17/15/2/6 and 5/1/2/7 counts pass in envelope tests;
- `RUSTFLAGS=-Awarnings cargo test -q --lib exit_transaction` (3 passed);
- `RUSTFLAGS=-Awarnings cargo test -q --lib dynamic_full_body_recipe` (8 passed);
- `cargo check --lib`, current-state guard, and `git diff --check` pass.

Stop:
- no physical schedule, block, CFG, PHI, ABI, Completion consumption, or
  session. Next: `PHYSICAL-OPERATION-DEMAND-I0`.

#### PHYSICAL-OPERATION-DEMAND-I0

Status: CLOSED (I0 landed)

Landed:
- `VerifiedDynamicLoopOperationPhysicalDemandV2` consumes only the complete
  final-exit HRTB view and validates all 17 placements, 15 operations, one
  control row, and six Fault rows before issuing a move-only demand;
- `PreparedDynamicLoopOperationProgramV2::prepare_all()` retains the complete
  Recipe-order operation array and exposes no single-item selector, V1 adapter,
  raw Recipe/JoinSig/source lookup, or physical identity;
- the implementation is a separate Dynamic V2 owner, not an extension of the
  existing V1 operation demand.

Evidence:
- `RUSTFLAGS=-Awarnings cargo test -q --lib exit_transaction` passes the
  complete HRTB demand test;
- `RUSTFLAGS=-Awarnings cargo test -q --lib dynamic_full_body_recipe` passes;
- `cargo check --lib`, physical-input authority guard, current-state guard,
  and `git diff --check` pass.

Stop:
- no Prelude, Tail, ABI, physical Completion, CFG/PHI, function session,
  DraftSeal, Collector, publication, provider/runtime route, retry, or fallback.
  Next: `PHYSICAL-INPUT-AUTHORITY-I0`.

### LOOP-UNIFICATION-AFTER-DYNAMIC-D0 (PARKED)

After `PHYSICAL-INPUT-AUTHORITY-I0`, audit one shared loop core for recursive
Recipe/JoinSig, physical-input, and whole-program demand boundaries. Keep
source observers, Prelude/entry, Tail/result, Home, ABI, Completion, provider,
and runtime separate. No V2-to-V1 adapter, raw re-scan, name/order repair, new
accepted shape, production switch, or legacy deletion. This is BoxShape-only;
worker review is required if shared ownership is open. Accept one common
boundary or explicit `NoSafeSlice`; keep files below 800 lines and leave the
current executable row unchanged.

### Landed prerequisite

PHYSICAL-CALLSLOT-TARGET-HANDOFF-I0 is closed. The source/Recipe envelope
consumes exactly two VerifiedSourceBoundDynamicMemberCallV1 rows and retains
their selector/arity, source sites, owner/frame/scope relation, and Recipe
CallSlot operands/result privately. It issues no executable target, provider
handle, runtime route, or public target catalog.

The next executable row is
LOOP-JOINSIG-V2-LOGICAL-TRANSFER-VIEW-I0.

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
loop_recipe_contract/join_sig/
  transfer_view_v2.rs
  transfer_view_v2_tests.rs

dynamic_full_body_recipe/coseal/
  operation_source.rs
  semantic_program/exit_transaction/
    physical_input.rs
    physical_input_tests.rs

dynamic_full_body_recipe/physical_demand/
  mod.rs
  model.rs
  issuer.rs
  tests.rs
```

Split at roughly 650-700 lines, stop adding at 760, and keep 800 as the hard
limit. Do not add these relations to `typed_schema_v2.rs`, `join_sig/flow.rs`,
or a standalone public `VerifiedCh*` product.
