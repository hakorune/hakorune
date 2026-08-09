# DYNAMIC-LOCAL-DESTINATION-HOME-CAPABILITY-D0

Status: design consultation required; implementation 0
Date: 2026-08-10
Depends on: `DYNAMIC-FAULT-CUTPOINT-CATALOG-I0` closed
Parent Decision:
`dynamic-fault-exit-transaction-d0-design-task-2026-08-10.md`

## Goal

Select the one neutral source-backed capability that may classify the exact
V10-to-`ch` destination relation before CFG-complete Home Flow.  The result
must distinguish a self-contained trivial, owner-bearing, or weak Dynamic
carrier without inferring ownership from Recipe `Dynamic`, a runtime tag, a
selector, or a physical representation.

## Existing input boundary

```text
VerifiedDynamicFullLoopSemanticProgramV2
  -> borrow exact V10 producer I6
  -> borrow exact ch declaration / BindingRef / Loop-body scope
  -> borrow exact I7 read
  -> retain complete Dynamic invocation envelope
```

This is a neutral lifetime/source relation only.  It proves no Home install,
availability, release, cleanup, terminality, or field ownership.

## Required census

Before selecting a type or issuer, inspect all existing owners for:

```text
semantic value capability / nominal type classification
Dynamic carrier representation-independent result relation
local destination declaration and initializer/assignment relation
Home ABI result relation
Home root/destination branding
weak/self-contained carrier semantics
CFG Home Flow input and disposition
```

For every candidate, record whether it is source-backed, reusable outside this
Loop profile, and able to represent all three Dynamic carrier categories
without runtime inspection.

## Design questions

1. Which source/semantic owner can distinguish owner-bearing from trivial/weak
   before physical lowering?
2. Is the capability attached to the produced value, the local destination,
   or an atomic value-to-destination relation?
3. How is the exact I6 normal-only publication distinguished from I6 Fault?
4. What does the capability lend to Home Flow without itself claiming
   `Available`?
5. How are foreign owner/frame/scope/declaration/value relations rejected?
6. Is the first unchanged-source cohort actually classifiable, or must this
   remain `NoSafeSlice` until a more general callable result contract lands?

## Required Decision output

```text
owner/non-owner table
one canonical issuer and exact inputs
typed carrier-category vocabulary
source/value/destination co-seal rule
Candidate/Declined/Unresolved/Rejected matrix
normal-only publication relation
borrow/move API and lifetime boundary
first implementation slice or explicit NoSafeSlice
negative tests and guards
reference/README/task order
```

`NoSafeSlice` is the correct result when the repository lacks a source-backed
classifier.  It must not be converted into an unconditional owner-bearing
test receipt.

## Nonclaims

```text
Home Available/Consumed state
CFG-complete Home Flow
cleanup obligation / release / fini / DropPlan
Fault execution or primary outcome
JoinSig transfer changes
Completion consumption
Builder/MIR/CFG/PHI/physical IDs
provider/runtime tag classification
production activation / retry / fallback
```

## Hard stops

```text
no Dynamic implies Home
no SelfContainedDynamicCarrier implies one Home
no runtime Box/tag/pointer inspection
no method-name or selector classification
no Recipe key as source identity
no standalone forged VerifiedCh/Home product
no test-only arbitrary capability constructor
no source fixture narrowing
```

All proposed Rust files must split near 650-700 lines, stop additions at 760,
and remain below the 800-line hard limit.
