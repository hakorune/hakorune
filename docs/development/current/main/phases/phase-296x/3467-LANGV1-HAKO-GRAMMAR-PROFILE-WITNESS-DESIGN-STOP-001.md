# 3467 - LANGV1-HAKO-GRAMMAR-PROFILE-WITNESS-DESIGN-STOP-001

## Status

Decision accepted. Rust profile seams cover `try`, `peek`, and both
transport-only `from` forms. Hako parser acceptance and ParseWitness projection
remain unchanged; 3468 is authorized for adapter health only.

## Accepted Decision

```text
Hako GrammarProfile owner = profile-bearing adapter facade
profile transport = explicit per parse invocation
ParseWitness owner = thin external projection adapter
first implementation slice = adapter health only
adapter bounded deterministic execution = mandatory prerequisite
Hako from evidence = missing and deferred
```

The facade may configure `ParserBox` per call, but neither environment state nor
`NYASH_FEATURES` may select Canonical or Compat2025. Raw Program(JSON) remains
implementation evidence and is never ParseWitness or language authority.

Accepted claims:

```text
hako_grammar_profile_owner_decision = profile_bearing_adapter_facade
hako_profile_per_parse_invocation_required = 1
hako_env_profile_selection_forbidden = 1
hako_implicit_compat_retry_forbidden = 1
hako_parse_witness_owner_decision = external_projection_adapter
hako_raw_program_json_non_authority = 1
hako_adapter_health_prerequisite_required = 1
hako_first_slice_adapter_health_only = 1
hako_try_migration_deferred_until_health_green = 1
hako_peek_migration_deferred_until_try_or_later = 1
hako_from_transport_deferred = 1
```

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

## Rejected Alternatives

Direct ambient `ParserBox` profile ownership was rejected because it would
widen parser state before adapter health is established. Indefinite
raw-evidence-only parking was rejected because it provides no route to explicit
profile activation or ParseWitness projection.

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

3468 implements only the bounded deterministic adapter-health boundary. After
that card is green, open one profile-bearing facade plus statement-try code
slice. Do not split health probes into per-fixture or rerun cards.
