---
Status: closed — external review reconciled with current language authority
Date: 2026-08-08
Decision: accepted with current-authority corrections
---

# Callable contract external-review reconciliation

## Accepted architecture

The review correctly preserves the one-way boundary:

```text
ordered source declaration
  -> resolver declaration capability
  -> declared callable contract
  -> reusable target
  -> exact source-bound call relation
  -> Recipe CallSlot
  -> Verify / Lower

method body
  -> semantic conformance verifier
  -> publication gate
```

Declaration meaning and body conformance remain separate products. No target,
CallSlot, Builder route, provider route, or physical ABI may be inferred from a
method name, a `HashMap`, MIR metadata, or a body-only observation.

## Current-authority corrections

The external proposal used `CallableContract(exact_trivial_i64)` and allowed
receiver reads under `Pure`. Both are intentionally rejected by the landed
language Decision:

```text
accepted source: @rune CallableContract(query)

signature owns:
  arity and semantic parameter/result types

query owns:
  exact receiver reads and the no-write/no-escape/no-effect obligations

Pure owns:
  no receiver/heap/global read

physical ABI owns:
  scalar representation and MirType/FunctionSignature validation
```

This prevents one source profile per implementation cohort and keeps semantic
effects distinct from physical representation. The bounded first fixture may
remain `length(): i64`, but neither `length` nor `i64` names the contract.

Use the existing `Handle` vocabulary. Do not add `HandleOnly` as a second Home
capability spelling.

## Ordered inventory correction

The frontend correction is accepted and already started:

```text
FRONTEND-ORDERED-BOX-METHOD-INVENTORY-D0 closed
FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R0 landed at 9233fc27b3
```

The inventory owns source/selected order and a private name index. Selected
build-gate methods remain `ExplicitSource` and retain an outer-to-inner gate
path. Generated and CompatibilityOnly rows cannot back the first resolver
contract issuer. Legacy JSON/name-order views are compatibility projections,
not source authority.

## Disposition

```text
issuer absent in the repository -> NoSafeSlice development state
source lacks CallableContract(query) -> Declined
contract exists but exact type/site is unavailable -> Unresolved
identity or declaration contradiction -> Rejected
exact source-backed aggregate -> Candidate
body violates declared contract -> conformance Rejected
```

`NoSafeSlice` is never a fifth source disposition.

## Task order

```text
R1 AST field/compatibility consumer cutover
R2 ordinary/interface/static ExplicitSource issuance
R3 selected-gate and generated producer transactions
R4 ordered JSON v2 + legacy JSON v1 CompatibilityOnly
R5 Builder compatibility projection migration and old helper retirement
Hako Box declaration carrier D0 and typed parser cells
Rust/.hako normalized inventory parity
resolver declared query instance contract
resolver target
source-bound call relation and CallSlot
body contract conformance
production activation only after conformance
```

Every implementation row must update its landed owner README and relevant
`docs/reference/**` receipt in the same commit. Future reference text must not
claim an issuer, target, conformance proof, or production route before it
exists.

The finite executable ordering, legacy retirement conditions, test matrix,
and implementation-coupled reference updates are owned by:

```text
callable-contract-and-instance-call-implementation-task-map-2026-08-08.md
```

This reconciliation owns the external-review disposition; the task map owns
execution order. Neither is a second language reference.

## Stop lines

```text
no exact_trivial_i64 source profile
no receiver-read Pure widening
no source-order reconstruction from HashMap/name sort
no generated/compat row promoted to source authority
no parser/resolver meaning duplicated across Rust and Hako
no target before declared contract
no publication before body conformance
no Recipe/Builder/provider fallback
```
