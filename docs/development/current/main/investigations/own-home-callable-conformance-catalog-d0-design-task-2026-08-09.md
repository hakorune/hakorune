---
Status: bounded I0 landed; general conformance remains at evidence D0 stop
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-facts-query-i0-implementation-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-CONTRACT-CONFORMANCE-D0

## Decision

The conformance architecture is accepted for a bounded exact `return me`
cohort. The landed evidence and conformance I0 are sufficient for that narrow
row. General conformance must still stop at
`CALLABLE-BODY-CONFORMANCE-EVIDENCE-D0` until the four-axis body execution
evidence boundary is accepted. An empty `effects()` list outside the bounded
receipt is not proof that the body cannot write, allocate, perform IO/FFI,
escape a failure, suspend, transfer non-local control, or move/escape Home.

The fixed pipeline is:

```text
declared instance contract catalog
  + selected Query body-owner/facts catalog
  + complete body-conformance evidence
      -> one per-row body conformance product
      -> one same-brand full-coverage conformant catalog
```

The conformance issuer compares existing meaning. It never reissues Query,
Home, semantic signature, ABI, or public type meaning from the body.

## Products and owners

```text
VerifiedDeclaredInstanceMethodContractCatalogV1
  = declaration + selected Query behavior + VerifiedHomeAbi

VerifiedCallableQueryBodyFactsCatalogV1
  = bounded source shape (`return me`) only

VerifiedQueryBodyConformanceEvidenceV1
  = landed bounded structural-safety/Home no-transfer receipt
    for the exact `return me` cohort

VerifiedBodyExecutionEvidenceV1
  = future atomic co-seal of complete coverage, neutral effect, control, and
    Home-flow receipts for one body root

VerifiedCallableBodyConformanceCatalogV1
  = one accepted behavioral conformance row per selected declaration

VerifiedConformantCallableCatalogV1
  = declared catalog + complete conformance coverage, ready for later target
```

The last two products remain distinct. The per-row product records that a
body satisfies an already declared contract; the conformant catalog proves
exactly one row for every selected declaration and no extras. Publication may
consume only the final catalog and performs no semantic recheck.

The first cohort is Query-only. Do not claim a universal non-Query callable
catalog or reuse this owner catalog for other contract families without a new
design decision.

## Identity and coverage

Rows are paired by the existing aggregate-owned declaration identity and
owner link, not by name, vector position, inventory ordinal, `FunctionOrigin`,
or numeric `FunctionOwnerIdV1`. The co-seal must require:

```text
same parser provenance
same resolver/catalog brand
same nominal Box identity
same Box/member source site
same selected declaration identity
exactly one body facts/evidence row per selected Query declaration
no missing, duplicate, foreign, or extra row
```

Sparse Query/non-Query source order remains valid; non-Query rows are
unselected and receive no default facts or conformance.

## Complete evidence boundary

`CALLABLE-BODY-CONFORMANCE-EVIDENCE-D0` is the current general design stop.
Its future aggregate must make absence explicit for broader Query bodies:

```text
no binding write or Home escape
no allocation
no call / IO / FFI
no QMark / throw / panic / failure escape
no await / suspension / task transfer
no non-local control
complete statement, expression, relation, and effect/control coverage
```

The minimal general boundary is four sibling receipts—complete body coverage,
neutral effect events, neutral control/exit events, and language Home-flow
events/state—followed by one same-root `VerifiedBodyExecutionEvidenceV1`
co-seal. The current `BodyEffectKindV1`/shadow inventory does not yet record
Print/IO or every opaque expression, the ordinary function product does not
yet co-seal all QMark/Throw/Await/non-local control, and no body Home event
issuer exists. These are design blockers, not reasons to copy partial vectors
or infer absence from an empty list.

The existing neutral body-shape inventory and landed `return me` evidence are
positive evidence only for their exact bounded cohort. If broader source
coverage is not issued, stop at `NoSafeSlice` and design the missing issuer;
do not emit an empty/default conformance receipt.

## Dispositions

```text
complete evidence issuer not available       -> NoSafeSlice (development)
opaque/incomplete evidence                    -> Unresolved
fully observed body outside Query contract    -> Declined / nonconforming
identity/brand/site/coverage mismatch        -> Rejected
exact declared contract + exact complete body -> Candidate
```

Any missing/duplicate/foreign row aborts the full catalog. A declared
annotation alone is not body conformance. A Candidate body with a later
contract violation is a conformance `Rejected`, not a fallback route.

## Non-claims and retirement

This D0 does not open:

```text
field/state authority beyond the bounded facts cohort
resolver target or source-bound call relation
Recipe / CallSlot
Builder / MIR / CFG / PHI / physical ABI
module publication or production selection
fallback/retry/provider/runtime dispatch
```

The old `source_instance_result_contract` and
`VerifiedCurrentOwnerInstanceResultTargetV1` family is not a conformance
input. Its body-inferred result/target authority must remain retired rather
than coexist with this declaration-first path.

## Implementation order

```text
1. landed bounded CALLABLE-BODY-CONFORMANCE-EVIDENCE-I0
2. landed private per-row bounded Query conformance issuer
3. accept the general four-axis evidence D0 (current stop)
4. implement general evidence axes and full-coverage conformance catalog
5. stop before resolver target / Recipe / MIR

Implementation receipt:
`docs/development/current/main/investigations/own-home-callable-conformance-catalog-i0-implementation-task-2026-08-09.md`.

The general evidence design remains a future `NoSafeSlice` boundary; it is not
a prerequisite for this exact bounded I0.
```

The bounded implementation receipt above is closed. The next execution is not
open: it is the general evidence D0 card named above. No broader conformance
code or fixture is authorized until that D0 fixes the canonical effect/control/
Home issuers, full-coverage receipt, and negative matrix.
