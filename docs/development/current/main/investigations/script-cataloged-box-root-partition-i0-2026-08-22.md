---
Status: implementation-ready; one bounded source-window BoxShape
Task: SCRIPT-CATALOGED-BOX-ROOT-PARTITION-I0
Date: 2026-08-22
Priority: make cataloged non-Main static-box declarations an explicit source transfer before Script root lookup
Parent: SCRIPT-CATALOGED-BOX-ROOT-PARTITION-D0
PreviousCard: script-cataloged-box-root-partition-d0-2026-08-22
NextCard: MIR-LOOP-COMPARE-LIVE-PUBLICATION-BOUNDARY-D0
---

# Script cataloged-box root partition I0

## Six-line brief

```text
Decision: extend the existing neutral source-window issuer so a valid CatalogedNonMainStaticBox is issued as StaticCallableCatalogTransfer; the existing full ProgramBody window remains the only window, and lookup/resolver consume its source-issued transfer row without observer-side skipping.
Source authority + canonical issuer: `PreparedCanonicalScriptNeutralProgramWindowV1::issue_from_program_loan` consumes the same parser loan and `VerifiedNormalCallableSemanticPackageV1::declaration_catalog`; it validates the cataloged box/method partition and is the sole issuer of the transfer semantic row.
Non-authority: `ScriptDirectStaticCallLookupIssuerV1`, `resolve_stmt`, AST/name/ordinal matching, the parser composite issuer for multi-method expansion, Builder runtime state, old Recipe/raw/compatibility paths, and publication owners.
Fail-fast boundary: source-window issuance before lookup, target installation, or Builder effects. Missing/foreign/duplicate catalog relation rejects; a valid cataloged declaration is transferred and never reaches root `resolve_stmt`.
Smallest next slice: apply the transfer to `parser_scan_loop_box.hako`, prove `ScriptDirectStaticCallLookupV1` has `CompleteEmpty` root coverage, and preserve the existing callable package/selected Dynamic body owner for method rows.
Non-claims: no multi-method composite provider admission, no method-body lowering change, no A/C/Recipe/Join/physical/publication change, no fallback, no backend, no generic retirement, and no performance work.
```

## Implementation boundary

This is a source-window BoxShape. It changes one existing classifier branch;
it does not create a new semantic `Verified*` or `Prepared*` receipt.

The branch is ordered as:

```text
parser HRTB loan
  -> existing composite provider check
  -> existing source item admission
  -> same-package callable catalog validation for CatalogedNonMainStaticBox
  -> StaticCallableCatalogTransfer source semantic row
  -> existing neutral window / resolver / lookup
```

The existing `ScriptRootSemanticDispositionV1::Transferred` vocabulary and
`ScriptTransferredBoundaryV1::StaticCallableCatalogTransfer` are reused. The
existing runtime work classifier continues to own the retained static-box
runtime terminal, and the callable package continues to own all method-body
rows. No second method-body observer is introduced.

## Exact source relation to validate

For each non-Main, non-sync, non-record static `BoxDeclaration` in the parser
loan:

```text
source statement index
  -> same-package declaration catalog has the box's static method keys
  -> selected callable source inventory has ProgramBoxMethod rows at that index
  -> method names/arity and declaration shape are unchanged
```

The implementation must use existing catalog/inventory APIs and the same
parser-owned source package. It must not reconstruct a key from a name or
ordinal in a later observer. If the relation cannot be proven, the issuer
returns a typed source-window error before any Builder effect.

The multi-method `ParserScanLoopBox` is outside the parser-composite I0
provider cohort. This I0 only transfers its existing callable catalog rows;
it does not turn the box into a parser-composite provider or issue a root
target/candidate.

## Expected fixture transition

Before I0:

```text
neutral window: CatalogedNonMainStaticBox -> Deferred
lookup: BoxDeclaration -> UnsupportedStatement
stage: ScriptSemanticSeal
```

After I0:

```text
neutral window: CatalogedNonMainStaticBox -> StaticCallableCatalogTransfer
lookup: declaration row is source-transferred and not observed as root code
root coverage: CompleteEmpty
```

The unchanged `parser_scan_loop_box.hako` lifecycle test is then expected to
reach the next existing blocker (or a successful root lifecycle). Its old
`ScriptSemanticSeal` assertion must be updated only after the observed error
is rerun; no guessed stage or error string is accepted.

## Finite state table

| State | Meaning | Effect | Next |
| --- | --- | ---: | --- |
| `CatalogedBoxObserved` | source row is a non-Main/non-sync/non-record static box | none | catalog validation |
| `CatalogedBoxTransferred` | same-package catalog and selected method rows cohere | none | neutral window / root lookup |
| `RootLookupCompleteEmpty` | no executable root call remains after transfer rows | none | existing root lifecycle |
| `CompositeProvider` | exact one-provider/one-root-call parser cohort | none | existing composite route |
| `Incomplete` | a required method/catalog/source row is absent | none | unpublished reject |
| `IntegrityInvalid` | foreign, duplicate, stale, or contradictory source relation | none | unpublished reject |
| `RejectedBeforeEffect` | typed source-window failure | none | discard only |
| `NoSafeSlice` | transfer would require another authority or method-body scan | none | return to design_stop |

There is no wildcard/default/empty fallback arm. `RootLookupCompleteEmpty`
means the source issuer proved that all root rows are transfers or transparent
rows; it does not mean the method partition was forgotten.

## Focused acceptance

Positive:

```text
normal_script_neutral_window_tests::parser_scan_loop_box_catalog_transfer
normal_script_direct_static_lookup_tests::cataloged_box_has_complete_empty_root_coverage
normal_default_root_catalog_lifecycle_tests::parser_scan_package_reaches_the_existing_physical_blocker_without_fallback
```

Required observations:

```text
BoxDeclaration never reaches root `resolve_stmt`
root lookup coverage is explicit CompleteEmpty
all ParserScanLoopBox method rows remain in the existing callable catalog
same parser invocation is retained
no target/candidate/Recipe/Join/physical/publication effect is added
```

Negative:

```text
foreign package catalog -> IntegrityInvalid
missing method row -> Incomplete
duplicate source row -> IntegrityInvalid
Main/sync/record/instance box -> existing explicit non-transfer arm
composite one-method provider + root call -> existing composite transfer unchanged
partition failure -> no lookup, target install, or Builder effect
```

The source-reownership/composite guard, current-state pointer guard, focused
neutral/lookup tests, `cargo check --lib`, and `git diff --check` are the
minimum evidence. Production files remain below the 760-line split trigger
and 800-line hard boundary.

## Explicit non-claims and return edge

This I0 does not authorize:

```text
publication observation or ExternalCommit
selected Dynamic live cutover
generic Loop retirement
old Compare leaf deletion
A/C changes
multiple-method parser-composite issuance
method-body re-lowering or AST re-scan
legacy/raw/compatibility retry
```

Once this I0 is green, the next design card is
`MIR-LOOP-COMPARE-LIVE-PUBLICATION-BOUNDARY-D0`. The publication path must
reuse `ModuleDraftCollectorV1`, `PreparedNormalCollectorDrainLifecycleV1`,
and `PreparedBuilderExternalCommitV1::commit` exactly once; the admission
receipt remains transient and is not promoted to a second publication ledger.
