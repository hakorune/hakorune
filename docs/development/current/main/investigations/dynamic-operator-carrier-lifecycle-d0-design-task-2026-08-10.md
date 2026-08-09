# DYNAMIC-OPERATOR-CARRIER-LIFECYCLE-D0

Status: design consultation required; implementation 0
Date: 2026-08-10
Depends on: `DYNAMIC-INVOCATION-RESULT-LIFECYCLE-I0` closed

## Goal

Select the representation-neutral lifecycle contract for the unchanged V2
Dynamic operators before issuing any full carrier-flow product.

The exact operation census is:

```text
I1  DynamicLess -> V5  Bool
I5  DynamicAdd  -> V9  Dynamic
I9  DynamicLess -> V13 Bool
I15 DynamicAdd  -> V17 Dynamic
```

The Bool results are exact trivial Recipe values. The unresolved question is
whether the language-wide `DynamicAdd` normal result is canonically one
self-contained carrier with `EndExactlyOnceUnlessForwarded`, and how its
operands/results are moved, borrowed, forwarded, or rebound.

## Required census

Inspect the semantic operation contract, resolver source relations, current
runtime/reference implementations, and physical projection for:

```text
DynamicAdd normal/fault outcome
operand borrow/move/no-escape rule
string concatenation and numeric-add result carriage
V9 temporary argument destination
V17 induction binding rebind/backedge destination
old carrier release/drop behavior
operation versus provider/runtime authority
```

## Decision questions

1. Does `DynamicAdd` itself guarantee a self-contained normal carrier, or is a
   separate source/operator capability missing?
2. Are both operands borrowed for the operation, moved, or family-dependent?
3. Is V9 forwarded into the I6 invocation argument or ended before/after I6?
4. Does V17 replace the existing induction carrier, and which owner ends the
   displaced carrier before Backedge?
5. How does Fault before result publication preserve existing operand
   obligations?
6. Can one profile-neutral operator lifecycle product cover both rows without
   selector/runtime-tag inference?

## Required output

```text
owner/non-owner table
normal/fault and operand/result lifecycle law
exact V9 and V17 destination mapping
forward/end/rebind chronology
Candidate/Declined/Unresolved/Rejected matrix
first implementation row or explicit NoSafeSlice
negative tests, guards, README/reference updates
```

## Hard stops

```text
no Dynamic Recipe class alone -> lifecycle contract
no runtime tag/provider/selector -> semantic ownership
no string-only or numeric-only source narrowing
no V9/V17 guessed last-use cleanup
no hidden clone/share
no Home classification
no CFG/MIR/physical cleanup, Completion, retry, or fallback
```

All proposed Rust sources split near 650-700 lines, stop additions at 760, and
remain below the 800-line hard limit.
