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
VerifiedDynamicCallableCompletionProjectionV1
  - inner Recipe Return + outer Callable Tail
  - one logical FunctionExit target
            |
            v  sole consuming issuer
VerifiedDynamicExitTransactionV1
  - private cleanup projection
  - exact Return-site partition
  - borrowed JoinSig authorization
  - primary/suppressed chronology
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
   consume program + carrier flow; derive cleanup and Completion partition

10. DYNAMIC-EXIT-PHYSICAL-SESSION-P0
   session-bound sequencing, fault injection, whole-session discard; caller-zero
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
physical source-ledger progress, retry, or fallback.  The next bounded owner
is the cleanup projection/exit transaction lane.

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

The next bounded owner is `MULTI-RETURN-COMPLETION-CONSUMPTION-D0/I0`.

## Callable Completion projection (D0/I0 closeout)

`MULTI-RETURN-COMPLETION-CONSUMPTION-D0/I0` is closed as a logical
two-route projection. `issue_dynamic_callable_completion_projection_i0`
consumes the complete carrier-cleanup product and retains exactly:

```text
inner Recipe Return -> one function-exit target
outer Callable Tail -> the same function-exit target
```

The existing `VerifiedFunctionCompletionV1` remains the sole owner of exact
return-site coverage, owner/target closure, and common value/unit
classification. The new product consumes that already-sealed evidence through
the carrier chain and does not issue a second Completion contract. It does not
create a result merge, physical Return, ABI representation, final function
seal, DraftSeal, collector, or publication.

Focused closeout gate:

```text
RUSTFLAGS=-Awarnings cargo test -q --lib carrier_completion
```

The next bounded owner is `DYNAMIC-EXIT-TRANSACTION-COSEAL-I0`.

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
