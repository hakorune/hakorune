# 3467 - LANGV1-HAKO-GRAMMAR-PROFILE-WITNESS-DESIGN-STOP-001

## Status

Design stop. Rust profile seams now cover `try`, `peek`, and both transport-only
`from` forms. Hako parser acceptance and ParseWitness projection remain
unchanged until this card receives an accepted profile-owner decision.

## Source Evidence

```text
tools/language_v1/grammar_contract_hako_adapter.hako
  -> ParserBox.parse_program2(source)
  -> raw Program(JSON) evidence only

lang/src/compiler/parser/parser_box.hako
  -> parser state has no GrammarProfile field or explicit profile entry

lang/src/compiler/parser/stmt/parser_stmt_box/core.hako
  -> statement try dispatches unconditionally

lang/src/compiler/parser/expr/parser_peek_box.hako
  -> emits distinct Peek JSON, not Match witness normalization
```

Current Hako behavior is implementation evidence, never grammar authority.
The adapter's raw Program(JSON) is not a span-free ParseWitness projection.

## Decision Required

Choose the Hako owner and boundary for Canonical/Compat2025 without ambient
environment profile selection or shared parser implementation.

```text
Candidate A: explicit ParserBox grammar-profile state plus a Hako witness adapter
Candidate B: profile-bearing Hako adapter facade that configures ParserBox per call
Candidate C: retain raw evidence only and defer Hako profile activation
```

The decision must state:

1. The one typed/profile representation Hako receives at parser entry.
2. Whether the first code slice is `try` only or covers `try` and `peek`.
3. How Hako emits the shared span-free ParseWitness without exposing parser JSON
   shape as canonical semantics.
4. The fail-fast behavior for missing/unknown profile, forbidden implicit
   Compat2025 retry, and witness drift.
5. Whether the two `from` forms remain missing evidence or receive a distinct
   transport-only Hako adapter in a later slice.

## Required Boundaries

```text
Canonical is the default.
Compat2025 is explicit per parse invocation.
NYASH_FEATURES must not select GrammarProfile.
Rust and Hako parsers remain independent.
Raw Hako Program(JSON) must not be treated as ParseWitness.
Hako compatibility transport must not enter Rust AST, MIR, runtime, or backend.
No implicit Canonical-to-Compat retry.
```

## Non-Claims

```text
hako_grammar_profile_implemented = 0
hako_try_migrated = 0
hako_peek_normalized = 0
hako_from_transport_implemented = 0
hako_parse_witness_conformance = 0
parser_sharing = 0
runtime_backend_behavior_changed = 0
selfhost_claim = 0
```

## Next

After an accepted decision, open one code-facing macro-row implementation card
for the selected Hako profile/witness seam. Do not split it into per-spelling
planning cards.
