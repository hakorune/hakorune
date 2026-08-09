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
+ source-backed Home destination capability / Home Flow
+ cleanup projection
+ JoinSig transfer authorization
+ Function Completion coverage
+ canonical physical session
  -> one final exit transaction
```

The current repository does not yet have the Dynamic local Home capability,
CFG-complete Home Flow, or the two-Return physical Completion consumer needed
for that final co-seal.  Therefore the full transaction I0 is `NoSafeSlice`.
The first safe implementation row is the private, Home-free complete Fault
cut-point catalog.

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
| V10/ch is owner-bearing or not | future neutral destination capability plus CFG-complete Home Flow | `Dynamic`, runtime tag, local relation |
| per-cut-point cleanup obligation | private deterministic projection from verified Home Flow | empty cleanup receipt, Recipe |
| Return/Backedge/PredicateFalse/After transfer | existing JoinSig | cleanup planner, physical layout |
| inner and outer Return source coverage | retained `VerifiedFunctionCompletionV1` | JoinSig, Tail |
| outer operand | Callable Tail | Loop Recipe |
| physical sequencing and poisoned-draft discard | canonical function session | language exit semantics |
| compile-time atomicity | whole unpublished-session discard | runtime Fault transaction |

Fault is never a Recipe value/Exit, JoinSig edge, Completion site, Home, or
physical-session error.  Compiler session discard is not runtime rollback;
Dynamic effects before a Fault remain observable.

## Exact cut-point matrix

`ch` cleanup exists only after a future Home Flow proves normal-only V10
installation as an owner-bearing destination.

| Cut point | Definitely materialized | ch state | Cleanup | Transfer / Completion |
|---|---|---|---|---|
| I1 Fault | V0-V4; V5 absent | Absent | exact none | Fault terminal; no JoinSig/Completion |
| I5 Fault | V6-V8; V9/V10 absent | Absent | exact none | Fault terminal |
| I6 Fault | V9; V10 absent | Absent | exact none | Fault terminal; no result publication |
| I6 Normal | V10 | Absent until a later classifier/flow proves install | not executed by current row | continue to I7 |
| I7 Fault | V10; V11 absent | maybe Available only after future proof | exactly once iff Available | Fault terminal |
| I9 Fault | V10-V12; V13 absent | same | exactly once iff Available | Fault terminal |
| I12 inner Return | V13=true and V14 | same | before transfer iff Available | JoinSig Return to FunctionExit; inner site only |
| I15 Fault | V15/V16; V17 absent | same | exactly once iff Available | Fault terminal |
| I16 Backedge | V17 and B0 rebound | must be Absent before transfer | discharge first iff Available | JoinSig Backedge; no Completion |
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
VerifiedDynamicLoopLocalHomeFlowV1
  - normal-only install
  - exact Available/Absent at every cut point
  - no Available at Backedge/After
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

The final issuer accepts only the complete semantic program and complete Home
Flow.  It accepts no caller-supplied owner, Recipe, JoinSig, Completion,
cleanup rows, Fault sites, or physical IDs and exposes no `into_parts` escape.

## Ordered task ladder

```text
1. DYNAMIC-FAULT-CUTPOINT-CATALOG-I0
   BoxShape only: exact six-site private catalog inside the semantic program

2. DYNAMIC-LOCAL-DESTINATION-HOME-CAPABILITY-D0/I0
   one exact ch destination; no Dynamic/runtime-tag inference

3. DYNAMIC-LOOP-LOCAL-HOME-FLOW-D0/I0
   per-iteration Absent -> Available -> Absent and every exit cut

4. DYNAMIC-EXIT-CLEANUP-PLAN-I0
   private exact zero/one obligations derived only from Home Flow

5. MULTI-RETURN-COMPLETION-CONSUMPTION-D0/I0
   inner Recipe Return + outer Tail -> one FunctionExit/DraftSeal Return

6. DYNAMIC-EXIT-TRANSACTION-COSEAL-I0
   consume program + Home Flow; derive cleanup and Completion partition inside

7. DYNAMIC-EXIT-PHYSICAL-SESSION-P0
   session-bound sequencing, fault injection, whole-session discard; caller-zero
```

Each implementation row updates its code, focused tests, module README,
landed reference receipt, active card, and guards in the same slice.

## First implementation boundary

`DYNAMIC-FAULT-CUTPOINT-CATALOG-I0` may claim only:

```text
the exact semantic program contains exactly six fault-authorized operation
sites with the expected operation family and normal-result publication key;
all six occur before result publication on Fault.
```

It may not issue or simulate a Fault, Home, cleanup, Completion claim, CFG
edge, or physical instruction.

## Hard stops

```text
no Dynamic implies Home
no runtime tag implies Home
no empty cleanup as proof of Home absence
no Fault Recipe value/Exit or JoinSig edge
no Completion consumption before the multi-return owner lands
no cleanup/Return/Backedge publication without complete Home Flow
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
