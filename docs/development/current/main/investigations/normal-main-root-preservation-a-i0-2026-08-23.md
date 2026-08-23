Status: Implementation slice — parser root preservation only
Date: 2026-08-23
Decision: NORMAL-MAIN-ROOT-PRESERVATION-A-I0
ParentCurrentCard: docs/development/current/main/investigations/normal-main-app-root-consumer-d0-2026-08-23.md
ProductionCaller: parser final-transform path only; root consumer remains 0
ProductionEdit: parser source preservation and required move chain only
CeremonyTier: I0 — final source authority before Builder root effects
---

# NORMAL-MAIN-ROOT-PRESERVATION-A-I0

## Six-line brief

```text
Decision:
  adopt A: a parser-owned opaque non-Clone final root-preservation token.
Source authority + canonical issuer:
  ParserNormalRootSourceDispositionV1, ParserNormalProgramSourceAuthorityV1,
  and ParserInvocationWitnessV1 are the authority; the sole final issuer is
  ParserNormalRootPreservationIssuerV1::seal_after_transform.
Non-authority:
  Builder raw root expansion, root_is_app_mode, names/ordinals/pointers in MIR,
  Script-A rows, compatibility retry, and any second AST root classifier.
Fail-fast boundary:
  final-transform validation must finish before VerifiedFinalCallableProgramSourceV1
  is issued; any drift is a typed parser reject with zero Builder effects.
Smallest next slice:
  issue the token, validate the source-root prefix and role drift, and move it
  through final source and PreparedNormalDefaultProgramRootV1.
Non-claims:
  root consumer, App/Script lowering, work-plan bool removal, MIR, publication,
  raw classifier retirement, fallback, production switch, and performance.
```

## Authority contract

The final issuer consumes the already-issued root disposition and the same
source-backed transform session. It does not reissue App/Script meaning from
the transformed AST. The token is private to the parser boundary and exposes
only a closed root role to its future named consumer.

```text
Parsed source disposition
  -> one transform session carrying the parser witness
  -> seal_after_transform(initial, final)
  -> ParserNormalRootPreservationV1 (non-Clone)
  -> VerifiedFinalCallableProgramSourceV1
  -> PreparedNormalDefaultProgramRootV1
```

The transform API must not leave a production caller that passes an unrelated
raw AST as a free-standing final-source argument. Tests may construct negative
transform inputs only through a parser-owned test seam.

## Preservation rule

The token proves the source-owned Program prefix was preserved. Exact AST
structural equality is used only inside the parser issuer as a preservation
check; it is not exported as a pairing key or authority to MIR.

```text
initial Program exists
final Program exists
final prefix[0..initial_len] == initial body exactly
AppReady:
  original App seal remains the admitted role
  final suffix contains no second static Main
ScriptReady:
  final suffix contains no static Main introduction
same parser witness/session
```

App and Script suffix rows are not silently merged into the source cohort. The
token records only that the source prefix and root role remain valid. Suffix
policy is left to the later root-consumer Decision.

## State and errors

```text
Ready(App | Script)
  -> RootPreserved(App | Script)
  -> final source

Outside / ScriptTerminal / SourceAuthorityUnavailable / Incomplete /
IntegrityInvalid / DiscardedBeforeA
  -> typed terminal or explicit A discard

PrefixChanged / RootRoleDrift / ForeignTransform / ProgramShapeChanged
  -> FinalCallableProgramSourceRejectV1::ProgramSource
```

No state is represented by a parallel `Option`, default role, bool, or empty
catalog. The A frontdoor keeps its existing explicit `discard_before_a`
boundary and must reject AppReady before discarding it.

## Tasks

```text
A-I0-1  Add parser-owned non-Clone preservation model and sole issuer.
A-I0-2  Close the transform session so the final AST is validated under the
        same parser source product; no free production raw-AST pairing.
A-I0-3  Move the preservation token through Prepared and Verified final source
        into PreparedNormalDefaultProgramRootV1 as a required field.
A-I0-4  Add focused positive/negative evidence: App, Script, unchanged prefix,
        allowed suffix, prefix replacement/insertion, static-Main suffix,
        foreign transform, and AppReady-on-A rejection.
A-I0-5  Add one reusable guard: issuer=1, final-source carry=1, token Clone=0,
        raw root classifier consumer=0, Script-A row import=0, fallback=0.
```

## NoSafeSlice

Stop this I0 if the transform session cannot carry the same parser witness,
if root preservation needs parser anchors exposed to MIR, if suffix rows must
be guessed as App or Script, if a parallel `Option`/default is needed, or if
the change requires root lowering, work-plan, MIR, publication, fallback, or
production switching. The next design card must then revisit the source
transform authority; do not weaken the token into a raw role enum.

## Acceptance

```text
source files touched remain below 760 lines; no file reaches 800
source root issuer remains unique
final source and Prepared root carry exactly one token by move
non-ready final transform has no Builder/session effect
existing parser focused tests plus new preservation tests pass
pointer guard, source-size guard, diff check pass
main is clean and synchronized before closeout
```

This card does not authorize the normal-root consumer. That is the next
separate slice after this parser preservation product is complete.
