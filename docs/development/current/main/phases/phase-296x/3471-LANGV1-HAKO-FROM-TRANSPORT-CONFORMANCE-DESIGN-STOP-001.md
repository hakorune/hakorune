# 3471 - LANGV1-HAKO-FROM-TRANSPORT-CONFORMANCE-DESIGN-STOP-001

## Status

Active design consultation stop after 3472 closes the parser/MIR correctness,
compile-cost, corpus-runner, and source-layout prerequisites. Do not implement
Hako `from` acceptance, transport, semantic lowering, or grammar closeout until
this scope decision is accepted.

## Current Evidence

```text
Rust:
  Canonical box-from/from-call -> stable reject
  Compat2025 migration entry -> distinct MigrationTransport
  Compat2025 semantic entry -> typed TransportOnly error before AST

Hako:
  explicit per-call GrammarProfile facade -> landed
  statement try profile seam -> landed
  peek -> Match compatibility alias -> landed
  compile-once 16-row grammar corpus batch -> green
  ParserProgramBox orchestration owner -> landed
  box-from/from-call transport evidence -> missing

Registry:
  box_from_inheritance Compat2025 -> compatibility_transport
  from_super_call Compat2025 -> compatibility_transport
  normalized_shape -> CompatibilityTransport
  semantic_owner -> none
```

## Decision Question

Should Language v1 parser conformance:

```text
A. formally exclude compatibility_transport rows from the required Hako
   semantic-parser witness scope, while retaining Rust migration tooling as
   the only transport producer;

or

B. require a separate Hako migration-only transport adapter for both closed
   from forms before grammar conformance can close?
```

Do not choose a semantic AST route. `CompatibilityTransport` is migration
evidence only and must never enter canonical AST, MIR, runtime, or backend.

## Authority

```text
grammar status and normalization:
  grammar/unified-grammar.toml

fixture contract:
  grammar/language-v1-grammar-contract-corpus.toml

transport law:
  docs/reference/language/grammar-contract.md

Rust implementation evidence:
  crates/hakorune_frontend_parser/src/migration_transport.rs
  src/parser/from_transport_boundary.rs

Hako implementation evidence:
  current ParserBox routes and external adapter health/profile boundaries
```

## Non-Authority

```text
legacy source acceptance
ASTNode::FromCall reuse
BoxDeclaration.extends presence
source path or test count
Rust-only success
missing Hako evidence alone
runtime/backend behavior
```

## Required Answer

The consultation must fix:

1. Whether compatibility transport is part of two-parser conformance or a
   separate migration-tooling contract.
2. If A, the exact formal exclusion rule and why it does not weaken the
   `two independent parsers` law.
3. If B, the owner and output schema for Hako `MigrationTransport` evidence.
4. Whether Hako semantic parsing must reject both forms under Canonical and
   Compat2025, and the stable tags for each profile.
5. The minimum code slice, fixture matrix, fail-fast boundary, and closeout
   conditions for `LANGV1-GRAMMAR-001`.

## Non-Claims

```text
hako_from_migrated = 0
hako_from_transport_implemented = 0
hako_parse_witness_conformance = 0
language_v1_grammar_closeout = 0
compat_transport_ast_authorized = 0
from_semantic_lowering = 0
runtime_backend_changes = 0
selfhost_claim = 0
```

## Stop Rule

Resume this card after 3472 closeout. The A/B decision remains required before
any Hako `from` implementation.
