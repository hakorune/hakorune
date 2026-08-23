---
Status: Ready to reopen — relation I0 closed
Date: 2026-08-23
Decision: NORMAL-GENERAL-PROGRAM-PARSER-MAIN-APP-ENTRY-ADMISSION-I0
ParentCurrentCard: docs/development/current/main/investigations/normal-main-app-entry-admission-d0-2026-08-23.md
PrerequisiteExecutionRow: NORMAL-GENERAL-PROGRAM-PARSER-MAIN-APP-DIRECT-CALLABLE-RELATION-I0
ProductionCaller: 0 before and after I0
ProductionEdit: none; parser-only admission I0 is the next bounded slice
CeremonyTier: I0 — bounded source admission
---

# NORMAL-GENERAL-PROGRAM-PARSER-MAIN-APP-ENTRY-ADMISSION-I0

## Six-line brief

```text
Decision:
  implement Candidate A as one parser-owned AppMain disposition for exactly
  one top-level static Main with one direct static main/0; keep old raw-root
  selection noncanonical and unchanged.
Source authority + canonical issuer:
  ParserStaticBoxSourceSealV1 plus the same-invocation complete parameter
  catalog; one private ParserMainAppEntryAuthorityIssuerV1::issue_once called
  from ParsedProgramWithCallableParameterSourceV1::new.
Non-authority:
  AST re-scan, Main/name-only lookup, method ordinal, raw
  VerifiedRawRootExpansionV1, root_is_app_mode, semantic Main plans, Builder,
  runner, Recipe/Join, MIR, and fallback.
Fail-fast boundary:
  completed parser callable-source product, before any NormalCompileRequest,
  root state write, Main child/body lowering, Builder effect, or runner entry.
Smallest next slice:
  one non-mixed static Main, one exact direct main/0 row, typed Ready/Outside/
  SourceAuthorityUnavailable/Incomplete/IntegrityInvalid, sibling transport.
Non-claims:
  semantic result/ABI, helper children, fields/init/generated members,
  ordinary/mixed programs, normal ingress, physical lowering, publication,
  old-authority retirement, production switch, and performance.
```

## Implementation contract (authorized after source relation I0)

The planned I0 must add one parser source product, not a semantic or physical
plan. It is not authorized until the missing relation card closes:

```rust
enum ParserMainAppEntryDispositionV1 {
    AppMainReady(ParserMainAppEntrySealV1),
    Outside(ParserMainAppEntryOutsideReasonV1),
    SourceAuthorityUnavailable(ParserMainAppEntryUnavailableV1),
    Incomplete(ParserMainAppEntryIncompleteV1),
    IntegrityInvalid(ParserMainAppEntryIntegrityIssueV1),
}
```

The constructor and seal remain parser-private. The issuer consumes the
already completed `ParserStaticBoxParentSourceDispositionV1` and the complete
`ParserCallableParameterSourceCatalogV1` from the same
`ParsedProgramWithCallableParameterSourceV1::new` call. It may use the
parser-private direct method site accessor added to the existing static seal;
it may not inspect the AST or create a second callable/source relation.

The I0 admission rules are closed:

```text
program cohort = ParserPostpassProgramCohortV1::StaticBox
static parent = Ready
static parent header name = "Main"
static parent member coverage = exactly one DirectMethod
catalog row source site = exact same Box path/member site/brand
catalog row kind = StaticBoxMethod
catalog row syntax name = "main"
catalog parameter count = 0
```

All other complete shapes are `Outside`, while missing and contradictory
evidence retains the distinction between `SourceAuthorityUnavailable`,
`Incomplete`, and `IntegrityInvalid`. No default or empty row is issued.

## Move/transport chain

```text
CompletedParserPostpassV1.static_box_parent_source
  + Complete(ParserCallableParameterSourceCatalogV1)
  -> ParserMainAppEntryAuthorityIssuerV1::issue_once
  -> ParsedProgramWithCallableParameterSourceV1.main_app_entry
```

The new field is a sibling of existing parser dispositions. It must not be
inserted into `ParserBoxSourceSealV1`, `SourceSealedOrdinary`,
`CanonicalScriptCohortAdmissionV1`, or any Builder/normal source-plan product.
Compatibility and selected-build-gate parameter states terminate before a
false `AppMainReady` can be issued.

## Forbidden edges

```text
ParserMainAppEntryDispositionV1 -> NormalCompileRequest
ParserMainAppEntryDispositionV1 -> root_is_app_mode
ParserMainAppEntryDispositionV1 -> VerifiedRawRootExpansionV1
ParserMainAppEntryDispositionV1 -> Main child/body lowering
ParserMainAppEntryDispositionV1 -> Recipe/Join/MIR/runner
entry failure -> Script/legacy fallback
entry name -> AST re-scan
entry ordinal -> source identity
```

## Focused evidence

Positive:

```text
one static Main with one direct main/0 -> AppMainReady
same parser brand/path is retained
ordinary source path -> not AppMainReady
static Utility with one direct method -> Outside
```

Negative:

```text
mixed program -> Outside(ProgramCohort)
multiple static parents -> Outside(MultipleParentRows)
unsupported member -> Outside(UnsupportedMemberKind)
missing callable catalog row -> Incomplete
foreign/duplicate source relation -> IntegrityInvalid
nonzero main arity -> Outside or Incomplete by the closed contract
```

Every issuer reject must prove zero downstream effect. The I0 tests inspect
only parser dispositions and sibling transport; they do not invoke Builder,
normal lowering, runner, or compatibility retry.

## Structural guards

```text
ParserMainAppEntryAuthorityIssuerV1 definition = 1
issuer production call site = 1
ParserMainAppEntryDispositionV1 construction outside issuer = 0
AST scan in issuer = 0
name/ordinal/pointer pairing in issuer = 0
VerifiedRawRootExpansionV1 caller from issuer = 0
root_is_app_mode write from issuer = 0
downstream parser-entry consumer = 0
fallback/retry/reselection edge = 0
source/module files >= 800 = 0
```

## NoSafeSlice / stop conditions

Stop the I0 without adding a compatibility shim if any condition appears:

```text
the direct method site cannot be exposed from the existing parser seal
without re-pairing independent rows
the parameter catalog is not same-invocation or cannot prove exact arity
Main/main requires AST reconstruction or Builder name lookup
the new disposition must be consumed to keep cargo check green
ordinary or mixed postpass policy must change
old raw-root authority must be retired atomically with parser I0
the new module or touched source exceeds the 760-line split trigger
```

## Relation prerequisite closed

`NORMAL-GENERAL-PROGRAM-PARSER-MAIN-APP-DIRECT-CALLABLE-RELATION-D0` is now
closed by the parser-only relation I0. The existing direct callable issuer
remains the sole owner of `CallableDeclarationAnchorV1`; the parameter row
and static-parent direct-member row carry only its comparison identity, and
the static-parent issuer distinguishes relation mismatch from missing,
foreign, and duplicate evidence. No Main/App issuer, Builder effect, or
downstream consumer has been opened yet. The old raw-root authority remains
unchanged for this next slice.

## Commit sequence

```text
1. parser Main/App model + issuer + static-seal accessor + focused tests
2. postpass/product sibling transport + parser README/reference update
3. reusable guard + CURRENT_STATE/card evidence + closeout
```

No production switch, downstream consumer, old-route removal, or fallback
change belongs in these commits.
