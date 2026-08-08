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

## Follow-up parser/source-authority audit (2026-08-08)

The later review does not open a new callable-contract design. Its parser
corrections are folded into the active R6 card:

```text
raw BoxMethodInventoryV1:
  descriptive, cloneable selected/generated placement carrier

parser-owned source seal:
  non-Clone authority carrying brand, exact source site, and relation coverage

resolver:
  consumes only the final seal after build-gate prune/rebase and delegate
postpass; it never trusts inventory ordinals or raw rune strings
```

The parser seal is an explicit boundary, not an inference from provenance:

```text
BoxMethodInventoryV1
  = cloneable ordered placement carrier
        ↓
VerifiedParsedBoxDeclarationV1
  = non-Clone parser-owned source seal
  = complete duplicate-free source/member relation coverage
        ↓
resolver declaration inventory
```

The seal is issued only by the final rich parse product after selected
build-gate pruning/rebase and delegate postpass. AST-only APIs project that
product and discard the seal; they do not issue a second source authority.

The source method coordinate is also permanently separate from selected
placement:

```text
SourceBoxMethodSiteV1
  = as-written Box member path and selected-gate path
BoxMethodInventoryOrdinalV1
  = selected/generated placement only
```

Generated property/delegate rows have generated origin and placement receipts,
but no explicit method source site. Resolver identity never depends on the
generated-row strategy or inventory ordinal.

For callable contracts, the typed syntax path is mandatory:

```text
RuneAttr(name/args)
  -> CallableContractSyntaxViewV1::Query
  -> canonical semantic issuer
```

Resolver code must not parse `"CallableContract"` or `"query"` strings. The
declared contract may reference the same-declaration `VerifiedHomeAbi`, but it
does not restate receiver/parameter/result Home demands. `VerifiedHomeAbi` is
the sole Home authority; the contract co-seal owns only the relation that both
receipts belong to the same declaration and catalog brand.

The pre-target retirement gate is explicit. The old
`source_instance_result_contract` family (call-site lookup, body-inferred
result, rebind/preloop witnesses) must reach non-test caller-zero or be
retired before the declaration-first instance target is opened. A new target
must not coexist with a second target authority.

Finally, body conformance is a complete Verify product, not a publication-time
per-call query:

```text
DeclaredCallableContractCatalog
  + exactly-one same-brand body conformance per body-bearing declaration
  -> VerifiedConformantCallableCatalog
  -> Lower / Seal / Collect
```

Recursive target resolution may use the declared catalog; lowering and module
publication consume only the complete conformant catalog. Missing, duplicate,
foreign, or rejected conformance is fail-fast.

The source method site and all-row inventory placement are separate forever.
Generated property/delegate rows can consume placement slots, so neither
`selected_method_ordinal` nor a sidecar length delta is a declaration identity.
The parser transaction owns the unpublished inventory and typed source-row
relations. R6-S2 therefore must replace the old parallel selected-gate
`&[u32]` merge with a typed prepared append/rebase bridge before deleting the
method sidecar. The AST carrier validates append collisions and placement; it
does not own parser brands or source sites.

The old body-inferred instance result/target family remains a retire-before-new-
target row (`SOURCE-INSTANCE-RESULT-CONTRACT-RETIRE0-R0`). The callable plan
already separates typed `CallableContractSyntaxV1::Query`, same-declaration
`VerifiedHomeAbi`, semantic `I64`, physical ABI projection, and complete body
conformance. Those rows remain later than R6 and are not pulled into the
parser transaction cutover.
