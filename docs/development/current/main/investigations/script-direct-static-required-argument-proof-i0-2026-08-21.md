---
Status: fast implementation row — one required-ordinal source proof
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-REQUIRED-ARGUMENT-PROOF-I0
Parent: docs/development/current/main/investigations/script-direct-static-required-argument-source-proof-d0-2026-08-21.md
ProductionCaller: selected-normal Script direct-static bridge only
ReplacementCell: selected bridge consumes required ordinals before argument effects
Classification: BoxCount; a new source-bound proof cohort, no language parser change
Execution row: SCRIPT-DIRECT-STATIC-REQUIRED-ARGUMENT-PROOF-I0
---

# SCRIPT-DIRECT-STATIC-REQUIRED-ARGUMENT-PROOF-I0

## Six-line brief

Decision: Issue one source-bound proof for each selected Script direct-static
row's callee-required ordinals. Accept only resolver-inventory-backed integer
`Literal | Unary | Binary` trees at those ordinals; non-required arguments stay
with the existing ordered lowering driver.

Source authority + canonical issuer: `VerifiedResolvedScriptV1` supplies the
Script owner, exact MethodCall/Argument sites, and
`ResolvedExpressionSourceInventoryV1`; `VerifiedScriptDirectStaticJoinHandoffV1`
supplies the already-sealed callee representation and required ordinals. The
new sibling producer
`VerifiedScriptDirectStaticRequiredArgumentProofV1::issue` co-seals only those
facts and issues no AST, ValueId, MirType, or physical identity.

Non-authority: the detached all-argument `ScalarOperandRecipe` as a consumer,
callable-only `CallProofContextV1`, synthetic Script callable keys, AST names or
re-scans, declared type names, Builder type tables, successful Call emission,
the finalizer, compatibility routes, and the detached test kernel.

Fail-fast boundary: issue and validate the proof before the Script claim is
taken and before receiver/argument effects. Missing, unsupported, foreign, or
drifted required representation rejects the Complete Script candidate; no
empty-list conversion, ordinary retry, or legacy fallback is allowed.

Smallest next slice: add the proof sibling, transport it through the existing
Script semantic source/input/state, co-seal it in the claim ledger, and consume
it once at the selected bridge. Keep Call emission, ExactI64 publication,
ordered argument descent, and Script completion owners unchanged.

Non-claims: no variable/initializer-flow/nested-call representation, no source
admission change, no Compatibility/Deferred/RawLegacy repair, no raw-probe
retirement, no `MirInstruction::Call` rewrite, no ABI/backend/performance work,
and no production switch outside selected-normal Script.

## Classification-completeness table

The proof issuer and claim ledger must preserve exactly one state per Join row;
`Option::None`, an empty ordinal list, and a generic compatibility label may
not merge these outcomes.

| state | authority / issuer | before effects | allowed terminal / continuation | fallback |
|---|---|---|---|---|
| `Unavailable` | no Complete Script semantic ledger | no proof/claim | existing compatibility lane only | no Script proof inference |
| `Absent` (`NoCandidate`) | exact ScriptRoot site has no Join row | no proof effects | existing no-row route | no fabricated row |
| `ExactI64Empty` | Join representation is ExactI64 with zero required ordinals | proof is consumed as empty before descent | selected bridge continues | no inferred requirement |
| `ExactI64RequiredProofReady` | new issuer co-seals every required ordinal/site/tree | proof is consumed before descent | selected bridge continues | no callable/MIR inference |
| `RequiredArgumentRepresentationUnavailable` | issuer cannot prove a required source argument | reject before claim/effects | Complete candidate fails | no ordinary/raw retry or empty list |
| `ExactNominalBoxSelected` | Join representation is non-ExactI64 | reject at selected ingress | explicit non-Exact terminal | never coerce to integer |
| `SourceMismatch` | owner/source identity/site/ordinal validation | reject before effects | typed freeze | no AST/name re-pairing |
| `DetachedCandidateOnly` | old all-argument scalar input plus test-only kernel | no production effect | test-only evidence | cannot become consumer by wiring |
| `ConsumerReady` | proof plus selected bridge consumption and claim completion | exact proof consumed once | existing Call/publication/completion | no second consumer |

Every negative witness maps to one row: missing required ordinal,
variable/local-flow argument, nested/static call, field/index, typed or
unsupported literal, foreign source, duplicate proof row, duplicate claim, and
proof-consumer omission are not `Absent` or `ExactI64Empty`.

## Source proof shape

The new product is keyed by the existing Recipe key and retains the same source
owner/identity as the Join. A required row contains only:

```text
call_site
required ordinal
exact Argument(n) site
resolver-inventory-backed scalar tree
```

The recursive scalar tree reuses the existing source-fact vocabulary and
operator cohort (`Minus | BitNot`, `Add | Subtract | Multiply | BitAnd |
BitOr | BitXor`). The producer invokes the one existing inventory tree walker;
it does not add a second AST matcher. It visits only required ordinals, so a
non-required argument can remain an ordinary source expression and still be
lowered by the existing driver.

For an ExactI64 row with no required ordinals, the proof stores an explicit
`ExactI64Empty` disposition. For a non-Exact row, the proof stores an explicit
non-Exact disposition only for ledger cardinality/route validation; the selected
claim ingress still rejects it before effects. Unsupported required arguments
are an issuer error, not a partial proof row.

## Owner and transport changes

Only these owners may change:

```text
normal_script_direct_static_join_handoff/required_argument_proof.rs
  sole source-proof issuer and typed issue errors

normal_script_semantic_source.rs
normal_script_semantic_lowering_input.rs
  move-only transport of the proof alongside existing Bundle/Recipe/Join

normal_script_semantic_lowering_state.rs
normal_script_direct_static_claim_ledger.rs
  co-seal proof identity/cardinality and make the claimed token own it

calls/script_direct_static_physical_bridge.rs
  consume the proof before AssociatedMethodCallArgumentsV1::lower_all
```

The lifecycle gets only a thin issuer call and attach step. Existing generic
Call receipt, Script publication, ordered argument driver, Return/signature
writer, compatibility ports, and detached kernel remain untouched.

The claimed row becomes complete only after its required proof is consumed.
On argument, Call, publication, or completion failure the candidate/session is
discarded; there is no rollback, reinsert, retry, or route fallback.

## Acceptance

- A real Complete Script row with zero required ordinals reaches the selected
  bridge and consumes `ExactI64Empty` once.
- A real Complete Script row with required scalar literal/unary/binary
  arguments issues exact ordinal/site/tree rows and consumes them before any
  argument effect.
- Required ordinal order, argument site, source owner, source identity, Join
  key, and tree child sites are checked exactly once.
- Variable, initializer-flow, field/index, nested/static-call, typed/unknown
  or unsupported required arguments fail before claim/effects with no fallback.
- ExactNominalBox, missing/foreign/duplicate proof rows, duplicate claims, and
  proof omission never become `ExactI64Empty`.
- Non-required arguments continue through the existing ordered driver and are
  not reclassified by this proof.
- Existing generic Call receipt, ExactI64 publication, and Script completion
  remain the sole physical owners and occur at most once.
- Focused positive/negative tests, reusable guard, `cargo check`, pointer
  guard, classification guard, and `git diff --check` are green.
- All touched sources remain below the 760-line split trigger and 800-line
  hard stop; no compression workaround is accepted.

## NoSafeSlice conditions

Stop this I0 if any of these occur:

1. the issuer must reopen AST or infer representation from ValueId/MirType;
2. the proof validates all arguments instead of only required ordinals;
3. non-required arguments acquire a new route or source meaning;
4. missing proof can become Absent, empty, compatibility, or ordinary retry;
5. the detached kernel or callable-only proof becomes a second consumer;
6. the claim can complete without proof consumption or can be retried after
   proof consumption;
7. a changed source owner requires a synthetic Script callable key;
8. any owner crosses 760/800 lines or the lifecycle becomes a second issuer;
9. Compatibility, Deferred, RawLegacy, source admission, or production
   promotion is pulled into this selected-normal row.

## Non-claims and retirement edge

This I0 creates one semantic proof product and one selected-normal consumer;
it does not claim global Script coverage. The old non-consuming edge retired by
this row is the selected bridge's prior behavior of ignoring
`required_callee_i64_arguments`. Raw/compatibility lanes remain explicitly
relationless and are not counted as `ConsumerReady`. A later, separately
selected row must decide variable/flow/nested representation and only then can
global raw retirement or canonical Script transport be considered.

## Implementation evidence (2026-08-21)

The required-ordinal proof sibling is now transported from Complete Script
semantic source through lowering input/state into the claim ledger. The ledger
co-seals source identity, owner, Join key/cardinality, and proof-row site; the
non-Clone claimed token cannot complete until the proof is consumed exactly
once. The bridge consumes it before `AssociatedMethodCallArgumentsV1::lower_all`;
the existing generic Call receipt, ExactI64 publication, and completion owners
are unchanged.

The finite-state rule is enforced by the card table and reusable
`routing_classification_completeness_guard.sh`; the Script direct-static guard
also pins `ExactI64Empty`, required proof readiness, unsupported required
arguments, proof omission, and the required-only producer boundary. Focused
proof tests (3), ledger tests (7), and the direct-static family suite (22),
`cargo check
--profile quick`, `cargo test --profile quick --lib direct_static_claim_ledger`,
the reusable guard, current-state pointer guard, classification guard, and
`git diff --check` are green. The repository-wide `cargo fmt --all -- --check`
still reports pre-existing unrelated formatting drift; no formatter rewrite was
used for this row.
