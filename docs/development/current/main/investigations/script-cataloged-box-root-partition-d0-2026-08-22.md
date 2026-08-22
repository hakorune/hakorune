---
Status: design_stop; exact parser-scan fixture exposes a pre-publication source-window partition gap
Task: SCRIPT-CATALOGED-BOX-ROOT-PARTITION-D0
Date: 2026-08-22
Priority: partition cataloged non-Main static-box declarations from the Script root execution window without a lookup-side skip or fallback
Parent: MIR-LOOP-COMPARE-LIVE-PUBLICATION-BOUNDARY-D0
PreviousCard: mirbuilder-loop-compare-connect0-d0-2026-08-22
NextCard: SCRIPT-CATALOGED-BOX-ROOT-PARTITION-I0 (only after this D0 is accepted)
---

# Script cataloged-box root partition D0

## Six-line brief

```text
Decision: adopt one parser/source-owned partition that separates executable Script root statements from cataloged non-Main static-box declaration/method rows; do not make the lookup observer silently skip BoxDeclaration.
Source authority + canonical issuer: the parser-owned HRTB source loan, the same-invocation callable declaration catalog, and the existing neutral source-window issuer co-seal the root execution rows, static-box declaration sites, and ordered method-body partition. The source-window issuer is the sole partition issuer.
Non-authority: lookup traversal, `resolve_stmt`, AST re-scan, Box/name/ordinal matching, the composite I0 partition alone, Builder current state, selected Dynamic evidence alone, empty/default coverage, compatibility/raw routes, and the live publication owners.
Fail-fast boundary: partition before `resolve_stmt` or target lookup. A declaration reaching the root lookup input is `PartitionMismatch`; a multi-method cataloged box is an explicit `Deferred(MultipleMethodsOutsideCompositeI0)` method partition, not an empty Script row or compatibility retry.
Smallest next slice: `SCRIPT-CATALOGED-BOX-ROOT-PARTITION-I0` for `parser_scan_loop_box.hako`: issue one source-owned partition, give the lookup/resolver only its root execution side, retain the cataloged method partition as an explicit typed outcome, and prove root `CompleteEmpty` without publication effects.
Non-claims: no multiple-method composite A admission, method-body lowering redesign, target/candidate/A/C/Recipe/Join change, Builder/ModuleDrain/ExternalCommit change, legacy fallback, backend, old-leaf retirement, or performance work.
```

## Why this task was opened

The landed selected-Dynamic body/state bridge moved the unchanged parser-scan
fixture past the previous `RootLower` blocker. The existing diagnostic test was
rerun without changing the tree:

```text
CARGO_BUILD_JOBS=4 cargo test --lib \
  normal_default_root_catalog_lifecycle_tests::parser_scan_package_reaches_the_existing_physical_blocker_without_fallback \
  -- --nocapture

observed:
  stage = ScriptSemanticSeal
  error = [mir/script-static-lookup/preflight]
          Lookup(MethodObservation(UnsupportedStatement {
            kind: "BoxDeclaration",
            site: ProgramBody(1),
          }))
```

The fixture contains a valid non-Main static box with several methods and no
Script root call:

```text
using ...
static box ParserScanLoopBox {
  skip_while(...)
  scan_until_newline(...)
  scan_escape(...)
  scan_escape_piece_and_skip(...)
}
```

The current neutral window preserves the full ProgramBody cardinality. The
first parser-composite cohort accepts only one provider method plus one final
root MethodCall, so this fixture is outside that composite cohort. The generic
neutral decision then leaves `CatalogedNonMainStaticBox` as a deferred runtime
responsibility. The static lookup observer receives the full window and sends
the declaration to the shared statement resolver, which reports
`UnsupportedStatement`.

This is not evidence that the declaration is invalid. It is evidence that a
declaration-owned source row leaked into a root-execution observer. It must be
fixed before the live publication evidence can be gathered.

## Accepted design decision

The lookup observer must not decide that a `BoxDeclaration` is non-executable
by itself. A `continue` at the observer would hide a missing source relation
and would make a later method body appear to be covered by a root window.

The source owner must issue a finite partition in one parser invocation:

```text
Parser normal source loan
  -> CanonicalScriptSourceWindowIssuerV1
       -> root execution window
       -> cataloged static-box declaration/method partition
  -> Script lookup consumes root execution window only
  -> Script resolver consumes root execution window only
  -> existing callable declaration/package owner consumes method partition
```

For the parser-scan fixture the intended outcome is:

```text
root execution window
  = CompleteEmpty

cataloged static-box method partition
  = Deferred(MultipleMethodsOutsideCompositeI0)
```

`CompleteEmpty` is valid only because the source issuer has proven that every
ProgramBody row is either a transparent using row or a cataloged static-box
transfer, and that no executable root expression remains. It is not
`Vec::new()` used as a missing-window default.

The method partition is not silently discarded. It remains an explicit
outside-I0 fact until the existing callable package/selected Dynamic body
consumer proves the selected method rows. This task does not broaden the
multiple-method composite cohort.

## Authority map

| Fact | Sole owner | Consumer in this slice |
| --- | --- | --- |
| parser invocation and ProgramBody source rows | parser source authority | source-window issuer |
| cataloged static-box declaration identity and method rows | same-invocation callable declaration catalog | source-window issuer and existing callable package |
| root-vs-declaration partition | `CanonicalScriptSourceWindowIssuerV1` (design name) | neutral window, resolver, lookup |
| root method-call observations | `ScriptDirectStaticCallLookupIssuerV1` | existing A/C source handoff only for root rows |
| Script root semantic forest | Script resolver using the partitioned root window | existing root lifecycle |
| selected method body/state | existing callable semantic package and selected Dynamic bridge | selected callable lowering |
| publication | collector/drain/external-commit owners | explicitly later; not this task |

The source-window issuer may validate a cataloged box against the package's
already-issued declaration catalog, but it may not issue a target, candidate,
Recipe, Join, ValueId, block, physical instruction, or publication receipt.

## Non-authority and rejected alternatives

The following are explicitly not allowed:

```text
lookup observer: if BoxDeclaration { continue }
resolve_stmt: treat declaration rejection as complete-zero
AST/name/ordinal scan after the HRTB loan
method-body reconstruction from a catalog key
full ProgramBody window passed to a root-only observer
empty lookup/coverage default when a partition row is missing
fallback to old Recipe, raw, compatibility, or ordinary static lowering
```

The existing parser-composite issuer remains the authority for its bounded
one-provider/one-root-call cohort. It reports this multi-method fixture as
outside that cohort; it does not become a general multi-provider/multi-method
issuer in this task.

## Proposed source-only partition shape

The exact Rust names are implementation detail, but the ownership shape is
fixed before code is allowed:

```text
CanonicalScriptSourcePartitionV1
  invocation: ParserInvocationWitnessV1
  root: CanonicalScriptRootExecutionPartitionV1
  cataloged_methods: CanonicalCatalogedStaticBoxMethodPartitionV1
  _seal
```

The product is source/Facts transport only. Its two sides are not parallel
optional products; they are one co-sealed partition.

```text
Root side:
  CompleteRows(root source sites)
  CompleteEmpty(real zero executable root rows)

Catalog side:
  TransferredExistingCallableCatalog(exact declaration/method relation)
  Deferred(MultipleMethodsOutsideCompositeI0)
  Incomplete(missing catalog/source row)
  IntegrityInvalid(foreign/duplicate/order contradiction)
```

The `Deferred` catalog-side outcome is preserved as a typed state. It may not
be converted into root `CompleteEmpty` unless the source issuer has separately
proved that the root side contains no executable rows. The existing callable
package remains the only possible consumer of selected method-body facts; this
task does not invent a second body observer.

## Fail-fast chronology

```text
same parser invocation loan
  -> validate catalog/source relation
  -> classify every ProgramBody row exactly once
  -> co-seal root execution rows and cataloged method partition
  -> reject foreign/missing/duplicate/contradictory rows
  -> hand root side to lookup/resolver
  -> retain method-side outcome for its existing callable owner
  -> no Builder effect, A/C effect, Recipe, physical effect, or publication
```

If a `BoxDeclaration` reaches the old root lookup traversal, the result is a
typed `PartitionMismatch`, not a `NonCandidate`, empty coverage, or fallback.
Any source partition failure occurs before pinned target installation and
before collector admission.

## Finite state table

| State | Meaning | Owner | Effect | Allowed next |
| --- | --- | --- | ---: | --- |
| `SourceWindowReady` | same-invocation source rows and catalog are available | parser/source package | none | partition |
| `Partitioned` | root rows and cataloged method rows are disjoint and complete | source-window issuer | none | root lookup / method owner |
| `RootLookupCompleteEmpty` | no executable root call exists after partition | source-window + lookup issuer | none | existing root lifecycle |
| `RootLookupCompleteRows` | executable root call rows are complete | lookup issuer | none | existing Script A/C path |
| `CatalogedMethodBodyDeferred` | cataloged method partition is outside composite I0 but explicitly observed | source-window issuer | none | existing callable owner or design stop |
| `PartitionMismatch` | declaration leaked into root input or a root row leaked into catalog side | partition validator | none | reject/discard |
| `Incomplete` | required source/catalog row is absent | partition validator | none | reject/discard |
| `IntegrityInvalid` | foreign, duplicate, stale, or contradictory relation | partition validator | none | reject/discard |
| `RejectedBeforePublication` | any typed pre-effect failure | lifecycle transaction | none | unpublished discard only |
| `NoSafeSlice` | owner or exact method-side consumer cannot be proven | design process | none | remain design_stop |

There is no wildcard/default arm and no `Option::None` state that merges
`RootLookupCompleteEmpty`, `CatalogedMethodBodyDeferred`, or missing source.

## Smallest implementation task after D0 acceptance

`SCRIPT-CATALOGED-BOX-ROOT-PARTITION-I0` is the only proposed fast slice:

1. Extend the existing parser/source-window issuer to recognize one exact
   `CatalogedNonMainStaticBox` source row using the same-invocation callable
   catalog; do not widen the composite provider cohort.
2. Co-seal the root execution side and the cataloged method partition. For the
   fixture, issue root `CompleteEmpty` and typed
   `MultipleMethodsOutsideCompositeI0` on the catalog side.
3. Feed only the root side to Script lookup/resolver. The observer must not
   receive a full ProgramBody with a declaration row.
4. Keep the existing callable package/selected Dynamic bridge as the method
   owner. Do not add a second method-body scan or a new physical path.
5. Add focused positive/negative evidence and a reusable guard before any
   publication observation.

## Acceptance evidence

Positive:

```text
parser_scan_loop_box.hako reaches lookup preflight without BoxDeclaration
root lookup returns CompleteEmpty coverage
cataloged multi-method partition is explicit Deferred(outside composite I0)
all four static-box method declarations remain in existing callable catalog
one parser invocation witness binds both partition sides
```

Negative:

```text
foreign catalog/source invocation -> IntegrityInvalid
missing cataloged method row -> Incomplete
duplicate declaration/site -> IntegrityInvalid
root lookup input contains BoxDeclaration -> PartitionMismatch
method partition silently omitted -> reject
Deferred method partition converted to empty root success -> reject
fallback/retry after partition failure -> structural guard failure
```

No-effect proof for every pre-effect rejection:

```text
pinned target unchanged
current module unchanged
collector admission count unchanged
no MIR instruction/type/value/Recipe/Join publication
no DraftAdmission, ModuleDrain, or ExternalCommit
```

Reusable guards should assert that the root observer is called with the
partition-owned root side, that no observer-side BoxDeclaration skip exists,
and that no old/fallback route is added.

## Relationship to live publication D0

The publication owner decision remains simple and unchanged:

```text
ModuleDraftCollectorV1
  -> PreparedNormalCollectorDrainLifecycleV1
  -> PreparedBuilderExternalCommitV1::commit
```

`CollectedDraftAdmissionReceiptV1` is a transient affine admission witness;
it need not be retained through ExternalCommit. The collector, drain, and
external commit are the only publication authorities. That decision is parked
until this root partition task proves that the real fixture reaches the
publication boundary without a source-window leak.

## NoSafeSlice conditions

Remain at `design_stop` if any of these are true:

```text
the partition needs AST/name/ordinal re-pairing
the callable catalog is not same-invocation with the parser source
the root side cannot distinguish real empty from missing rows
the method side needs a second source/body observer
multiple-method composite A admission is required to make this task pass
the lookup observer must own BoxDeclaration semantics
partition rejection can fall through to old Recipe/raw/compatibility
publication must be changed before the root partition is proven
```

This card does not authorize implementation until the authority, finite
states, exact fixture outcome, and existing method-side consumer are accepted
in SSOT. Once I0 is complete, return to
`MIR-LOOP-COMPARE-LIVE-PUBLICATION-BOUNDARY-D0` and gather the existing-owner
DraftAdmission/ModuleDrain/ExternalCommit evidence.
