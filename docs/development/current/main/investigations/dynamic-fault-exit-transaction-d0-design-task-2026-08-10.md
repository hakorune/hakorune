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
  - exact Live/Ended/Forwarded at every cut point
  - no Live carrier at Backedge/After
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
   per-iteration Absent -> Live -> Ended/Forwarded and every exit cut

7. DYNAMIC-EXIT-CLEANUP-PLAN-I0
   private obligations derived from carrier flow and any separate Home Flow

8. MULTI-RETURN-COMPLETION-CONSUMPTION-D0/I0
   inner Recipe Return + outer Tail -> one FunctionExit/DraftSeal Return

9. DYNAMIC-EXIT-TRANSACTION-COSEAL-I0
   consume program + carrier flow; derive cleanup and Completion partition

10. DYNAMIC-EXIT-PHYSICAL-SESSION-P0
   session-bound sequencing, fault injection, whole-session discard; caller-zero
```

Each implementation row updates its code, focused tests, module README,
landed reference receipt, active card, and guards in the same slice.

## Current implementation boundary

`DYNAMIC-FAULT-CUTPOINT-CATALOG-I0` is closed. The selected next row,
`DYNAMIC-INVOCATION-RESULT-LIFECYCLE-I0`, may claim only:

```text
the exact semantic program internally derives both verified invocation-result
obligations: I6/V10 local and I7/V11 temporary; Fault publishes neither.
```

It may not claim V9/V17, complete function flow, issue or simulate a Fault,
Home, physical cleanup, Completion claim, CFG edge, or MIR instruction.

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
  tests.rs

future exit_transaction/
  mod.rs
  completion_partition.rs
  cleanup_projection.rs
  tests/{golden,negative,api_guard}.rs
```

Split at roughly 650-700 lines, stop adding at 760, and keep 800 as the hard
limit.  Do not create a standalone public `VerifiedCh*` product.
