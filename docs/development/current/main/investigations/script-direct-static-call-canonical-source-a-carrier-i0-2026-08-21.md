---
Status: Implementation-complete — transport-only carrier
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-CARRIER-I0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-carrier-d0-2026-08-21.md
ProductionCaller: none; no A/Recipe/physical consumer
ReplacementCell: parser/frontdoor -> canonical compiler source-plan request
Classification: BoxShape (behavior-preserving transport)
NextCard: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-ISSUER-D0
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-CARRIER-I0

## Six-line brief

Decision: Carry the already-issued parser-backed Script input through the
canonical compiler request exactly once; the carrier only transports source
evidence and does not issue A meaning.

Source authority + canonical issuer: `CanonicalParserSourceHandoffV1` is the
sole parser/source issuer. The compiler carrier owns only the move and
lifetime; a future Source-only A issuer remains the sole semantic issuer.

Non-authority: AST/name/ordinal/digest re-pairing, Builder state,
`SealedNormalScriptSourceV1` alone, the existing plan/receipt pair,
`RawScriptBodyRecipeV1`, and an empty/default carrier cannot issue or consume
A meaning.

Fail-fast boundary: frontdoor classification must either move an explicit
carrier or enter a named `DiscardedBeforeA` terminal before Script recipe,
physical entry, Builder state, or child effects. No silent field drop remains.

Smallest next slice: add the compiler-side carrier, pass it in
`CanonicalCoreSourcePlanCompileRequestV1`, and explicitly discard it at the
current no-A compiler boundary for every non-Script family.

Non-claims: no A issuer, resolver/target/result/proof/terminal facts,
Recipe/Join/physical Call/publication, source admission, compatibility/raw
retirement, production switch, ABI/backend, fallback/retry, or performance.

## Exhaustive transport states

| state | owner / issuer | pre-effect behavior | terminal / continuation | fallback policy |
|---|---|---|---|---|
| `NotApplicable` | outer canonical family classifier | no Script carrier effect | Main/Callable route | never fabricate a Script carrier |
| `CompatibilitySource` | parser/source admission | retain typed compatibility origin | compatibility owner or stop | never canonical A |
| `Deferred` | parser/source admission | retain deferred reason | deferred owner or stop | never `NonCandidate` |
| `AdmissionMissing` | parser cohort/admission | stop before carrier | explicit non-canonical terminal | never `HandoffReady` |
| `CohortUnresolved` | parser cohort issuer | no row/carrier effect | typed stop or compatibility owner | never empty success |
| `SourceAuthorityUnavailable` | parser identity/profile/receipt check | stop before compiler candidate | `NoSafeSlice` | no default/rescan |
| `ObservationIncomplete` | parser row issuer | stop before A/Recipe | incomplete terminal | never `NonCandidate` |
| `IntegrityInvalid` | parser/frontdoor co-seal | stop before A/Recipe | integrity terminal | no foreign/duplicate/stale reuse |
| `NonCandidate` | source-family classifier | no A/Recipe effect | named noncandidate terminal | no raw fallback |
| `HandoffReady` | parser rows + profile/receipt co-seal | move once into compiler request | carrier reaches future A or explicit discard | no `script_input: _` |
| `DiscardedBeforeA` | current no-A compiler boundary | no candidate publication | terminal discard | never call it `HandoffConsumed` |
| `HandoffConsumed` | future named A consumer | no replay or second read | A observation begins | impossible without that consumer |
| `DispositionTransported` | future C/B transport owner | no source reinterpretation | detached consumer terminal | not an A/parser state |

Every row has one issuer, one pre-effect behavior, one continuation or
terminal, and one fallback policy. `NoSafeSlice` is a development stop, not a
source disposition. `Option::None`, wildcard matches, `unwrap_or(default)`,
and generic compatibility labels may not merge these rows.

## Carrier shape and route

```text
CanonicalParserSourceHandoffV1
  -> PreparedNormalFileSourcePlanRequestV1
  -> ClassifiedNormalFileSourcePlanV1
  -> CanonicalCoreSourcePlanCompileRequestV1 { plan, admission, receipt, carrier }
  -> Main/Callable: explicit DiscardedBeforeA
  -> Script: explicit DiscardedBeforeA immediately before prepare_script_recipe()
```

The carrier payload is limited to the existing AST-free parser rows, source
identity/digest/UTF-8/read/parse witness, canonical profile witness, and
complete import/config snapshot already co-sealed by the front door. It does
not contain AST, pointers, Builder/`comp_ctx`, resolver forest, target/result,
Recipe/Join/proof/terminal, `ValueId`, `MirType`, MIR block, physical ID, or
Recipe key.

The compiler-side carrier must not import runner-owned receipt types. The
front door converts its existing handoff into the carrier through one private
co-seal. The request owns the carrier by move; no clone, replay, parser rescan,
or second source issuer is permitted. Main and Callable requests carry an
explicit non-Script state and discard it before their existing route; a
Script request may not use an empty carrier as ordinary raw input.

`DiscardedBeforeA` is a temporary no-consumer terminal for this I0. It is not
semantic success and does not authorize `HandoffConsumed`. A future A issuer
must replace this terminal with its own one-shot consume before any A claim is
made.

## Acceptance

Positive:

- matching canonical pure Script rows and frontdoor receipt arrive in one
  compiler request without reconstructing identity;
- request move is linear and the payload remains AST-free;
- Script reaches the explicit no-A discard immediately before the existing
  recipe, while Main/Callable discard before their existing compiler owners;
- non-Script routes never receive an empty/default Script carrier;
- the compiler request and carrier remain below the 760/800-line limits.

Negative:

- compatibility, deferred, missing-admission, unresolved-cohort, incomplete,
  integrity-invalid, and foreign identity states never become `HandoffReady`;
- missing/foreign profile, digest, UTF-8 length, read/parse count, parser
  lineage, or import/config completeness stops before the old recipe;
- rejected frontdoor/source-plan paths explicitly consume the carrier as
  `DiscardedBeforeA` instead of destructuring `script_input: _`;
- Main/Callable/AST-only/source-free routes cannot borrow a Script carrier;
- no AST rescan, pointer/name/ordinal/digest-only pairing, clone, retry,
  fallback, or premature `HandoffConsumed` is accepted.

## Focused evidence and guard

The focused gate must cover parser-backed Script transport, non-Script explicit
discard, rejected source-plan discard, and request-side source identity
retention. Add a carrier-specific reusable guard or extend the existing parser
handoff guard to assert:

```text
carrier model has an exhaustive state match                         = 1
CanonicalCoreSourcePlanCompileRequestV1 owns the carrier              = 1
frontdoor no longer drops `script_input: _`                          = 0
`HandoffConsumed` issued without a named A consumer                   = 0
carrier imports runner receipt authority                              = 0
AST/Builder/Recipe/Join/physical fields in carrier                  = 0
source-plan/canonical dispatch growth over 760                       = 0
```

Run the focused Rust tests, `cargo check`, current-state pointer guard,
routing-classification completeness guard, and diff/line-count checks. Any
missing carrier identity or new source-to-Recipe correspondence returns the
lane to design stop; it is not repaired with a default state.

## NoSafeSlice conditions

Remain stopped if the carrier needs a runner-owned semantic receipt, can be
paired after move, is optional/defaulted, is consumed without a named A owner,
forces Main/Callable through an empty Script field, requires parser/compiler
rescan, or requires semantic growth in a source already at the 760/800 limit.

## Implementation evidence (2026-08-21)

- `CARGO_BUILD_JOBS=4 cargo test --quiet --profile quick --lib
  canonical_core_dispatch` — 6 passed, 0 failed.
- `CARGO_BUILD_JOBS=4 cargo test --quiet --profile quick --lib
  canonical_script_source_a_input` — 2 passed, 0 failed.
- `CARGO_BUILD_JOBS=4 cargo check --lib` — passed; existing repository
  warnings remain baseline-only.
- `bash tools/checks/script_direct_static_canonical_source_a_carrier_guard.sh`
  — passed.
- `bash tools/checks/current_state_pointer_guard.sh` and
  `bash tools/checks/routing_classification_completeness_guard.sh` — passed.
- `git diff --check` — passed; all touched Rust owners remain below the
  760-line design trigger and 800-line hard stop.

The compiler request now owns the move-only carrier. Main/Callable and Script
all close it at an explicit no-A boundary; no A issuer, Recipe, Join, physical
Call, fallback, compatibility/raw retirement, production switch, or
performance claim was opened.
