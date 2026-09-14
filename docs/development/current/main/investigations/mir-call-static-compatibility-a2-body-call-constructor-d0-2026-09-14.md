---
Status: design_open__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-A2-BODY-CALL-CONSTRUCTOR-D0
Date: 2026-09-14
Parent: mir-call-static-compatibility-i0-a1-static-parent-2026-09-14.md
NextCard: none__a2_package_issuer_split_t0
Implementation permission: false; design and taskization only
---

# A2 body, direct-call, and constructor handoff

## Six-line brief

```text
Decision: reuse the existing final-source -> resolver semantic-batch -> normal-package chain and co-seal body, direct-call, constructor, and generated coverage at its existing owner boundaries.
Source authority + canonical issuer: FinalCallableSemanticSyntaxLoanV1 lends parser body/generated rows; ParserConstructorSourceCatalogV1 issues constructor/generated-Birth rows; resolver CallableIndex/canonicalization issues direct-call targets and headers; the existing package issuer joins them.
Non-authority: AST rescans, names/arity, MIR/ValueId, slot ordinals as pairing keys, compatibility fallback, publication/backend, and StringBox-specific readers.
Fail-fast boundary: before semantic-package Ready, reject foreign brand, row/slot cardinality drift, body owner/root mismatch, orphan or duplicate source sites, missing/duplicate target/header, arity mismatch, constructor transform drift, and generated-origin mismatch.
Smallest next slice: split the over-limit package issuer without semantic change, then add focused body/direct-call/constructor coverage guards through the existing batch/package path.
Non-claims: no MixedProgram admission switch, import-lineage changes, publication/caller cutover, compatibility-edge deletion, fallback restoration, StringBox fixes, Windows proof, or R7 closure.
```

## Census boundary

```text
first owner: ParserNormalCallableTransformSessionV1 final source
  -> last terminal: NormalCallableSemanticPackageInstallIssueV1::complete
includes: parser body/generated syntax loan, constructor source catalog,
  resolver body-shape/owner forest, direct-call source-site/target/header
  validation, existing semantic package installation
excludes: source-window admission, A0-2 lineage transport, static-parent set
  issuer (landed), publication/caller switch, compatibility fallback, and
  StringBox-specific lowering
```

This is the next design row because A1 now provides the finite static parent
and method relation set, but it intentionally does not interpret method bodies
or issue direct-call targets. The existing package chain already has these
owners; the missing work is exact co-seal and rejection coverage before a
source-backed semantic package can be claimed for the merged cohort.

## Existing authority chain

```text
VerifiedFinalCallableProgramSourceV1
  -> FinalCallableSemanticSyntaxLoanV1
  -> callable_semantic_batch::issuer
  -> VerifiedResolvedCallableSemanticBatchV1
  -> normal_callable_semantic_package::issuer
  -> VerifiedNormalCallableSemanticPackageV1
  -> existing install/publication consumers
```

The parser syntax loan owns the exact callable identity, final slot, body
source row, and generated callable provenance. `ParserConstructorSourceCatalogV1`
is the only parser constructor/generated-Birth issuer and is consumed by
`with_constructor_semantic_syntax` and
`instance_constructor_semantic::issue_instance_constructor_semantic_batch_v1`.
Generated callable methods remain in the final callable syntax loan; they do
not become constructor rows. The resolver's callable index is the only header
authority, and `resolver_canonicalization.rs` issues direct-call targets and
observations by exact source expression site. The package issuer may validate
and join these products, but it must not re-resolve a name or issue a second
target/header product.

## Required relation contract

For every source-backed callable in the finite accepted window, the handoff
must prove:

```text
parser invocation brand and callable identity agree
final callable row count == resolver declaration/batch count
each body-shape owner has one resolver forest root and matching callable owner
each body/source row is consumed once; no orphan or duplicate site remains
each direct-call observation has one source expression site and one target
each target resolves through the same CallableIndex to one matching header
target callable, declaration identity, namespace, and arity agree
constructor catalog rows match transformed parent/slot coverage exactly
generated callable origin and generated-Birth origin remain separate and exact
```

Missing or ambiguous data is a typed reject. It must not be repaired from AST
names, method arity, MIR values, `ValueId`, or final slot ordinal.

## Named reject mapping

| Condition | Terminal owner | Required result |
| --- | --- | --- |
| parser brand or callable identity differs | semantic batch/package | typed source/identity reject |
| body owner/root differs, missing, duplicate, or orphan | resolver batch | body coverage reject before package |
| direct-call source site missing or duplicated | resolver/package | `UnissuedDirectCallObservation` or existing site reject |
| target or header missing/foreign/ambiguous | package direct-call co-seal | existing target/header reject |
| target declaration or arity differs | package direct-call co-seal | identity/arity reject |
| constructor transform or parent coverage drifts | parser transform/constructor owner | existing constructor coverage reject |
| generated callable/Birth provenance is mixed or stale | parser final source | generated-origin mismatch reject |
| downstream loan leaves a row unconsumed | package install | existing incomplete-coverage terminal |

Every reject consumes the existing affine product at its named terminal. A
source-backed failure never retries through the AST-only compatibility route.

## Ordered design and implementation slices

| Order | Slice | Exit condition |
| --- | --- | --- |
| A2-D1 | Freeze the finite body/call/constructor owner inventory | each issuer, consumer, and terminal is named; no StringBox/publication rows enter |
| A2-D2 | Split package orchestration from direct-call validation | `normal_callable_semantic_package/issuer.rs` is below 760 lines with behavior unchanged; focused existing package tests remain green |
| A2-D3 | Body/forest co-seal | callable identity, owner/root, body shape, and exact source-site cardinality reject before package Ready |
| A2-D4 | Direct-call co-seal | source site, observation, target, CallableIndex header, declaration identity, and arity are checked once |
| A2-D5 | Constructor/generated coverage | constructor catalog, transformed parent/slot, generated method, and generated-Birth rows remain exact and disjoint |
| A2-D6 | Focused evidence and guard | positive ordinary+static body/direct-call/constructor fixture plus missing/duplicate/foreign/orphan rejects run in existing test owners |

The first executable slice after this design is the BoxShape-only issuer split
(`none__a2_package_issuer_split_t0`). It does not change acceptance or issue a
new semantic product. A later A2-I0 may add the guards once D1 and the exact
caller/terminal table are accepted.

## Line budget and non-claims

`src/mir/normal_callable_semantic_package/issuer.rs` is currently 765 lines,
already beyond the 760-line design boundary and below the 800-line hard stop.
The split must move direct-call validation or package orchestration into a
focused sibling before any semantic handoff change; do not compress lines.
Other owners remain within their current budgets. No production caller switch,
compatibility classification deletion, fallback restoration, publication
change, StringBox correction, Windows proof, or R7 closure belongs to A2-D0.

## Worker audit receipt — 2026-09-14

The read-only audit selected one authority chain and no new static-specific
body/call/constructor receipt. It identified:

1. `FinalCallableSemanticSyntaxLoanV1` -> callable semantic batch as the body
   and generated callable transport;
2. `ParserConstructorSourceCatalogV1` -> constructor semantic batch as the
   constructor/generated-Birth transport; and
3. resolver canonicalization -> `VerifiedCallableIndexV1` headers as the sole
   direct-call target/header route.

The audit requires owner/root/cardinality/orphan guards, exact direct-call
site/target/header/arity checks, and disjoint constructor/generated provenance.
It also found `normal_callable_semantic_package/issuer.rs` at 765 lines, so a
BoxShape-only split is the first executable task. The audit did not authorize
MixedProgram admission, publication, fallback, cutover, StringBox changes, or
legacy-edge deletion.
