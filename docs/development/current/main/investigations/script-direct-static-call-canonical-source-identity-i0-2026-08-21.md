---
Status: Closed — identity transport/validation implemented and focused evidence green
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-SOURCE-IDENTITY-I0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-identity-d0-2026-08-21.md
ProductionCaller: source-plan identity validation only; no direct-static caller
ReplacementCell: preserve one CanonicalParserSourceHandoff identity through source planning
Classification: BoxShape
---

# SCRIPT-DIRECT-STATIC-SOURCE-IDENTITY-I0

## Six-line brief

Decision: Preserve the existing parser-lineage/canonical-receipt co-seal
through `PreparedNormalSourcePlanInputV1` and validate it once at source-plan
classification. Add only a read-only identity loan; do not issue A or any
semantic/physical product.

Source authority + canonical issuer: the canonical normal-file front door
`CanonicalParserSourceHandoffV1::new` issues `NormalParserSourceLineageV1` and
the front-door receipt together.  The pre-existing selected-normal in-memory
materializer is the only other named parser ingress; this source-plan I0 adds
no issuer.  The source-plan request/classifier is only a validator/transport
consumer and must not recreate lineage or re-read source bytes.

Non-authority: display name/path, AST/pointer/ordinal, digest-only equality,
`NormalSourceIdentityV1`, compatibility success, Builder/`comp_ctx`,
`ValueId`/`MirType`, and the future Source-only A are not identity issuers.

Fail-fast boundary: before `NormalSourcePlanClassifierV1::seal` and before
`prepare_script_recipe()`. Missing lineage, profile/receipt/read-parse drift,
or foreign identity fails without Resolver, Recipe, Join, Builder, or physical
effects. AST-only fixtures remain explicitly non-canonical.

Smallest next slice: add one lineage loan and one receipt-vs-lineage validator
with focused positive/negative tests and a structural guard. Keep every
changed Rust owner below 760 lines; split before semantic growth.

Non-claims: no Source-only A issuer, target/result catalog, Recipe/Join,
physical carrier/Call/publication/Return, source-admission change, production
switch, raw/compat retirement, ABI/backend, or performance claim.

## Exhaustive implementation states

| state | issuer / authority | pre-effect behavior | terminal / continuation | fallback policy |
|---|---|---|---|---|
| `NotApplicable` | non-canonical profile or non-source-plan caller | identity validator not entered | caller-owned route | never fabricate canonical identity |
| `CanonicalSourceBacked` | existing handoff lineage and receipt match exactly | lend read-only identity to source plan | classifier may continue | no display/digest-only substitution |
| `AstOnlyFixture` | explicit AST-only test constructor | `SourceAuthorityUnavailable` before classification | typed source-plan rejection | never enter canonical frontdoor |
| `CompatibilitySource` | parser disposition carries compatibility cohort and lineage | `CompatibilitySourceUnavailable` before classification | typed compatibility rejection/design stop | never become canonical A/NonCandidate |
| `LineageUnavailable` | parser-backed source lost its lineage before classification | typed reject before effects | source-plan rejection/design stop | no AST rebuild, reparse, or raw fallback |
| `IdentityInvalid` | empty/foreign/mismatched profile, digest, UTF-8 length, counts, or source window | typed reject before effects | terminal source-plan discard | no retry/re-pair/default |
| `Transported` | one move of the already-issued handoff into source-plan state | loan cannot be cloned/replayed | later A may borrow once | no second issuer or fallback |

The implementation must use an exhaustive match. `Option::None`, wildcard,
`unwrap_or`, and a generic compatibility arm may not collapse the states.

## Ownership and exact seam

```text
CanonicalParserSourceHandoffV1::new
  issues lineage + receipt once.

PreparedNormalSourcePlanInputV1
  retains the parser-backed handoff and lends its lineage without cloning.

PreparedNormalFileSourcePlanRequestV1::classify
  validates the lineage against the retained front-door receipt once, then
  invokes the existing source-family classifier.

SealedNormalScriptSourceV1 / future Source-only A
  receive the same read-only identity; neither issues it again in I0.
```

The validator compares source identity, bytes digest, grammar profile,
UTF-8 length, and read/parse counts. The source-plan window is checked by the
existing classifier; a later A must not use the validator as a candidate
observation. No `ValueId`, physical block, AST scan, or semantic product is
created here.

## Focused acceptance

Positive:

- canonical parser-backed input reaches classification with one matching
  lineage/receipt identity;
- the identity loan remains available from the sealed Script source without a
  clone or second parser call;
- existing source-family decisions remain unchanged;
- AST-only tests remain marked non-canonical and do not gain A eligibility;
- identity fields and one-read/one-parse counts remain identical end to end.

Negative:

- missing parser lineage;
- source identity, digest, profile, UTF-8 length, or read/parse count drift;
- foreign handoff paired with a different receipt;
- compatibility input routed as canonical source-backed;
- AST/path/pointer/ordinal-only reconstruction;
- second parse/re-read, clone/replay, or raw fallback after identity failure;
- any Resolver/Recipe/Join/Builder/physical effect before validation.

## Structural guard and NoSafeSlice

Guard:

```text
unlisted production `NormalParserSourceLineageV1::issue` call sites = 0
(the canonical normal-file handoff and pre-existing selected-normal
materializer are the only named ingress issuers; test/fixture constructors
remain `cfg(test)` only)
AST/path/pointer identity reconstruction in validator = 0
source bytes re-read/re-hash during classification = 0
identity failure -> raw/compat fallback = 0
new A/Recipe/Join/physical issuer in this I0 = 0
changed Rust owner >= 800 lines = 0
semantic growth at/above 760 without a split = 0
```

Remain at design stop if lineage must be cloned/reissued, the receipt cannot
be validated against the existing handoff, AST-only/compatibility input must
masquerade as canonical, or validation cannot fail before classification
effects. The next Source-only A task remains separate.

## Closeout evidence (2026-08-21)

- `cargo test --profile quick -p nyash-rust --lib source_plan_input -- --test-threads=1` — 20 passed.
- `cargo check --profile quick -p nyash-rust` — passed; the repository's existing warning census remains baseline-only.
- `bash tools/checks/script_direct_static_canonical_source_identity_guard.sh` — passed.
- `bash tools/checks/routing_classification_completeness_guard.sh` — passed.
- `bash tools/checks/current_state_pointer_guard.sh` and `git diff --check` — passed.
- Touched Rust owners remain below 760 lines; no source reissue, Recipe/Join,
  physical, fallback, production caller, or performance claim was opened.

The next row is the separate `SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-ONLY-A-D0`
design stop.  This I0 does not move the canonical Script consumer or claim
main integration.
