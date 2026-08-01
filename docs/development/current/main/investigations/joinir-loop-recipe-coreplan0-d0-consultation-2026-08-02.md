# JoinIR Loop Recipe/CorePlan — D0 consultation

Status: external design question
Date: 2026-08-02
Current row: `JOINIR-LOOP-RECIPE-COREPLAN0-D0`
Decision requested: choose one safe all-route Loop ownership boundary, or
close the row as `NoSafeSlice`.

## Goal

Retire the two normalized-shadow mutation routes before a selected Script
`Loop` can use the existing located-source handoff. The target pipeline is:

```text
source Loop
-> facts / Recipe or CorePlan
-> verify
-> one physical lowering owner
-> existing located Loop source handoff
-> lower once / publish once
```

The question is not how to add another normalized-shadow route. It is whether
the existing Loop routes can be represented by one verified, reusable plan
without duplicating lowering or retaining mutation-only compatibility owners.

## Current facts

The explicit JoinIR VM execution bridge was retired in `90e7ea307e`.
Ordinary VM execution remains the only VM owner. Structured JoinIR-to-MIR
conversion remains a neutral transformation in `src/mir/join_ir_to_mir/` and
is not an execution route.

Current normal Loop handling still has two mutation-bearing normalized-shadow
paths:

```text
1. Normalized shadow lowerers
   `LoopTrueBreakOnceBuilderBox` and
   `LoopTrueIfBreakContinueBuilderBox`
   construct a JoinModule plus `JoinFragmentMeta` from StepTree / env state.

2. `NormalizationExecuteBox::merge_normalized_joinir`
   reads the host variable map, creates `JoinInlineBoundary`, changes a
   Normalized phase snapshot to Structured for conversion, merges MIR blocks,
   and reconnects Loop exits through DirectValue semantics.
```

There is also the historical direct mutation helper:

```text
`ExitReconnectorBox::reconnect(exit_values, variable_map)`
```

It updates the host variable map by carrier name. It is not a verified Loop
plan and must not become the replacement authority.

The repository already has a `control_flow/plan/` area and route-selection
facts. The design must decide whether it can own one Loop-specific plan, not
whether names or files can simply be moved.

## Required invariants

The proposed design must preserve all of these.

```text
- one selected production Loop owner; no second Loop resolver/lowerer
- source identity remains located; no raw AST reconstruction or compact-index
  inference
- facts / recipe / verification are mutation-free
- the plan carries exact input order, carrier exit mapping, continuation
  identity, and result/exit disposition; no name-based recovery
- `ValueId`, CFG block creation, MIR mutation, and publication remain in one
  physical lowering terminal
- invalid source shape rejects before mutation; internal plan/coverage drift is
  a hard invariant failure, not a fallback to raw lowering
- lowering is once-only; no retry, reclassification, or old-route fallback
- ordinary VM remains unchanged; do not recreate a JoinIR VM lane
- raw/reference routes remain unchanged unless the same atomic owner switch
  explicitly covers them
- every touched source/check file remains below 800 lines
```

## Candidate directions

### A. Verified Loop Recipe/CorePlan (preferred if complete)

Create one move-only `VerifiedLoopRecipeV1` / `VerifiedLoopCorePlanV1` from
existing canonical facts and located source. It must be able to represent every
Loop route that the selected production caller can currently enter, including
the two normalized-shadow shapes or a typed decline before any mutation.

The recipe is consumed exactly once by a physical lowerer. It may describe
JoinIR construction and inline-boundary requirements, but cannot contain a
mutable `MirBuilder`, a `variable_map`, a VM executor, or fabricated source
paths.

### B. Narrow one normalized-shadow route only

Reject unless it can delete both mutation routes in the same commit. Replacing
only break-once while retaining break/continue as a mutable alternate owner is
not a safe cutover.

### C. Retain normalized-shadow Loop to R4

Accept only if an all-route typed retained operation can be named and the
generic compatibility portal can later reach zero. “Keep it because Loop is
hard” is not a sufficient retention contract.

## Questions to answer

1. Is Candidate A possible at the current seam? If yes, name the smallest
   plan input, verified product, unique producer, physical consumer, and the
   two old mutation edges removed atomically.
2. What is the exact all-route Loop domain? Distinguish accepted route shapes,
   typed source-level decline, and compiler invariant rejection. Do not infer
   completeness from the two existing normalized-shadow builders alone.
3. Can `JoinInlineBoundary` be a downstream projection of the verified plan,
   or does its current reliance on host `variable_map` prove a prerequisite
   source/binding authority is missing?
4. Is a phase-only `Normalized -> Structured` snapshot compatible with the
   final pipeline? If not, name the smallest prerequisite that removes it
   without reviving a second converter or VM execution path.
5. What focused fixtures prove input order, carrier exit mapping, continuation
   mapping, rejection-before-mutation, selected/legacy MIR parity, and fresh
   compiler reuse after failure?
6. If A cannot state all-route coverage and one atomic old-edge deletion,
   give the precise `NoSafeSlice` or R4 retained-operation closeout instead of
   a partial Loop I0.

## Explicit non-claims

```text
- Do not activate broad Script Loop Complete merely because a plan type exists.
- Do not migrate anything to the .hako interpreter.
- Do not reopen the retired JoinIR VM bridge.
- Do not change Loop grammar, result semantics, VM semantics, or source
  diagnostics in this D0.
- Do not add a second JoinIR-to-MIR converter, a source rewrite, or a
  name-based dispatch table.
```

## Desired answer format

```text
Decision: Accept / NoSafeSlice / R4 retention
Ceremony: T1 / T2 / Refactor Series
Named production caller:
Exact all-route input domain:
Verified product and unique issuer:
Physical lowering consumer:
Atomic old-edge deletion:
Failure / retry / publication contract:
Focused tests and guards:
First executable row, or exact stop reason:
```
