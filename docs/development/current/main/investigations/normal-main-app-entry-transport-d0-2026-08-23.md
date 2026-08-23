Status: Design stop — ready for Decision
Date: 2026-08-23
Decision: NORMAL-GENERAL-PROGRAM-PARSER-MAIN-APP-ENTRY-TRANSPORT-D0
ParentCurrentCard: docs/development/current/main/investigations/normal-main-app-entry-admission-i0-2026-08-23.md
PrerequisiteExecutionRow: NORMAL-GENERAL-PROGRAM-PARSER-MAIN-APP-ENTRY-ADMISSION-I0
ProductionCaller: 0; transport-only parser/source slice
ProductionEdit: none until this D0 is accepted
CeremonyTier: D0 — source-product transport design
---

# NORMAL-GENERAL-PROGRAM-PARSER-MAIN-APP-ENTRY-TRANSPORT-D0

## Six-line brief

```text
Decision:
  choose Candidate A: move the existing parser-issued Main/App disposition
  through the source-backed callable program and transform products without
  creating a second issuer or opening a downstream consumer.
Source authority + canonical issuer:
  issue_parser_main_app_entry_v1 remains the sole issuer; the existing parser
  product, PreparedNormalCallableProgramSourceV1, and VerifiedFinalCallable-
  ProgramSourceV1 are move-only transport owners, not semantic issuers.
Non-authority:
  AST rescans, Main/name/ordinal lookup, NormalParserSourceLineage,
  VerifiedRawRootExpansionV1, root_is_app_mode, NormalCompileRequest,
  Builder, Recipe/Join, MIR, runner, fallback, and compatibility repair.
Fail-fast boundary:
  parser source-backed product -> final callable source; losing or changing
  the disposition during normal-source preparation/transform is a typed reject
  before any Main/App consumer or root/Builder effect.
Smallest next slice:
  carry the required non-Clone disposition through Prepared and VerifiedFinal
  source products, with Ready/Outside/Unavailable/Incomplete/IntegrityInvalid
  preserved exactly and no downstream read.
Non-claims:
  Main/App semantic meaning, ABI/result validation, root selection, ordinary
  or mixed compatibility policy, NormalCompileRequest transport, publication,
  production switch, old-route retirement, and performance.
```

## Current gap

The parser-only I0 currently stores `main_app_entry` on
`ParsedProgramWithCallableParameterSourceV1`, but
`ParserCallableSourceDispositionV1::into_normal_callable_program` deliberately
ignores that field. The existing source-backed move chain therefore preserves
callable/constructor/source-authority products while dropping the new Main/App
disposition before `PreparedNormalCallableProgramSourceV1` is issued.

This D0 treats that as a transport boundary, not as permission to select or
lower an App root. The parser issuer has already made the only source
observation. The next product must preserve that observation or stop; it must
not reconstruct it from the AST after the drop point.

## Proposed move chain

```text
ParsedProgramWithCallableParameterSourceV1.main_app_entry
  -> ParserCallableSourceDispositionV1::SourceBacked
  -> PreparedNormalCallableProgramSourceV1.main_app_entry
  -> into_transform_parts()
  -> VerifiedFinalCallableProgramSourceV1.main_app_entry
  -> later named consumer design stop
```

The `Compatibility` variant has no parser source-backed Main/App product and
must remain an explicit compatibility lane. It must not receive a synthetic
`Outside`, empty row, `Option::None`, or a reconstructed Main fact merely to
make the field shape parallel.

The field is required on the source-backed products and moves by ownership.
`Clone`, `Arc`, raw AST references, and independent parallel fields are not
allowed. The transform validator keeps the disposition unchanged while moving
the existing source authority and constructor products.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| `issue_parser_main_app_entry_v1` | one parser-issued Main/App disposition | semantic Main, ABI, root selection |
| `ParsedProgramWithCallableParameterSourceV1` | same-invocation parser product | downstream route |
| `PreparedNormalCallableProgramSourceV1` | required source-backed transport field | new Main observation |
| `VerifiedFinalCallableProgramSourceV1` | transform-preserved transport field | root/Builder effect |
| normal transform validator | preservation and drift rejection | Main/App reclassification |
| future named Main/App consumer | semantic admission/route after a separate D0 | AST scan, fallback |

`ParserStaticBoxSourceSealV1`, the complete parameter catalog, and the parser
brand remain the source evidence already co-sealed by the existing issuer.
Transport products may carry that disposition but may not issue another one.

## Acceptance evidence

Positive:

```text
one static Main/main/0 AppMainReady
  -> Prepared source
  -> unchanged transform
  -> VerifiedFinal source still carries AppMainReady
```

Typed non-ready evidence must survive the same move chain for the existing
ordinary, non-Main, mixed, multiple-parent, unsupported-member, and non-zero
arity cases. The result must be the same disposition variant and payload, not a
new classification.

Structural guards:

```text
parser Main/App issuer definition                         = 1
Main/App disposition construction outside issuer          = 0
Prepared source required field                             = 1
VerifiedFinal source required field                        = 1
transport clone/default/empty reconstruction               = 0
AST/name/ordinal re-observation after parser I0             = 0
NormalCompileRequest/root/Builder consumer in this slice   = 0
fallback/retry/reselection                                 = 0
source files at or above 800 lines                          = 0
```

Focused tests must inspect source-product transport only. They must not invoke
the normal root, raw expansion, Builder, runner, or compatibility retry.

## NoSafeSlice conditions

Stop before implementation if any of these appears:

```text
the disposition can only be carried by Clone/Arc/Option/default
Prepared or VerifiedFinal cannot preserve it without a second parser/AST scan
Compatibility must be given a synthetic Main/App disposition
the field has to be re-paired with source identity by name/ordinal/path alone
the new transport requires NormalCompileRequest or root_is_app_mode changes
a downstream consumer is needed to keep the intermediate product buildable
the existing transform validator cannot preserve the field atomically
the touched production file reaches the 760-line split trigger
```

## Bounded task sequence

```text
D0  accept the move-only transport contract and exact field owners
I0  add the required field to Prepared and VerifiedFinal source products,
    thread it through the existing parser-to-transform move chain, and add
    transport-only positive/typed-negative tests
R0  add the issuer/field/no-reobserve guard, update parser README/reference,
    record evidence, and close the card
```

No Main/App consumer is selected by this card. After R0, a separate design
card must decide whether an existing root authority can consume the parser
disposition without re-observation; until then the disposition is allowed to
remain an unconsumed source product.
