# 3471 - LANGV1-HAKO-FROM-TRANSPORT-CONFORMANCE-DESIGN-STOP-001

## Status

Complete design consultation after 3472 closes the parser/MIR correctness,
compile-cost, corpus-runner, and source-layout prerequisites.

Decision: accepted.

```text
Decision A:
  Hako semantic-parser conformance explicitly excludes
  CompatibilityTransport rows. Rust migration tooling remains the only
  transport producer.

Decision B:
  Match owns delimiter disambiguation through an explicit delimiter-aware
  expression context.

Decision C:
  The route-family dependency graph is the single MIR convergence owner.
  Changed-function worklists and local invalidation are mechanisms beneath it.

Decision D:
  Scoped config injection is the single process-state test-isolation owner.
  Subprocess execution is a classification oracle, not the owner.
```

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

## Consolidated Consultation Packet

Treat the following as four independent decisions. Do not combine their code
changes merely because they are reviewed in one consultation.

### Decision A - Hako compatibility transport

Choose A or B from the primary decision question above. Specify:

1. the exact conformance inclusion/exclusion law;
2. the Hako semantic-parser behavior in Canonical and Compat2025;
3. stable reject tags and, if B, the migration-only output schema;
4. the smallest code-facing slice and closeout fixture matrix;
5. why no transport record can enter AST, MIR, runtime, or backend semantics.

### Decision B - Match/record delimiter ambiguity

Current evidence:

```text
match value { Ready(x) => x, Idle => 0 }
```

The Hako expression parser currently sees `value {` first as a record literal.
3472 made malformed record fields fail-fast, but did not silently reinterpret
the source as Match. Canonical Match is intended to accept a general
scrutinee expression, so the ambiguity still needs an explicit owner.

Compare at least these options:

```text
A. explicit delimiter-aware expression context passed by Match
B. declared-record inventory gates record-literal recognition
C. a canonical syntax restriction/change such as required parentheses
```

Select the design that preserves one expression grammar, exactly-once source
evaluation, and no source slicing/reparse fallback. Specify the parser API,
authority source, fail-fast tags, positive/negative fixtures, and unsupported
backend rule. A language syntax change requires an accepted specification
decision before implementation.

### Decision C - MIR compile convergence cost

Current measurements:

```text
merged parser module = about 166895 bytes / 27 static boxes / 259 functions
VM execution = about 0.07 seconds
full 16-row compile-once grammar guard after 3472 = about 34 seconds
dominant compile owner = semantic route convergence
first semantic refresh outer iterations = 4
post-canonicalization semantic refresh outer iterations = 2
50/100/250 isolated methods = about 67/79/126 milliseconds
```

Determine whether the next BoxShape should use:

```text
A. changed-function worklist convergence
B. explicit route-family dependency graph
C. scoped post-canonicalization invalidation
D. retain full refresh and optimize one measured route family first
```

Specify the single convergence owner, invalidation proof, deterministic
termination proof, and a regression guard that is more stable than wall-clock
time alone. Do not authorize helper-name shortcuts, fixture-specific caches,
stale metadata, lower iteration limits without convergence proof, or semantic
fallback.

### Decision D - process-global test isolation

Current baseline evidence:

```text
cargo test --lib parallel = 3540 passed / 56 failed / 32 ignored
cargo test --lib serial = 3551 passed / 45 failed / 32 ignored
five directly affected route tests fail with the same values on pre-3472 HEAD
parser feature gates, plugin loader state, and MIR strictness controls appear
in unrelated failures from the same process
```

Choose an isolation boundary among scoped config injection, subprocess-owned
environment tests, or another explicit owner. A global lock may be used only
if its ownership and cleanup guarantees are proven. Specify how to distinguish
real baseline expectation drift from environment contamination, and define the
first cleanup slice without changing production defaults merely to make tests
green.

## Ordered Follow-up Tasks

```text
1. 3473: implement explicit Hako compatibility-transport exclusion
2. implement delimiter-aware Match/record expression ownership
3. implement the route-family dependency graph in shadow mode
4. implement one scoped process-state config boundary
5. rerun the shared grammar corpus and evaluate LANGV1-GRAMMAR-001 closeout
```

Each implementation remains a separate commit scope. No inventory-only,
fixture-only, or rerun-only numbered cards are allowed.

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
match_record_ambiguity_resolved = 0
mir_compile_convergence_closeout = 0
full_lib_test_isolation_closeout = 0
```

## Stop Rule

Resolved. Proceed to 3473 and then follow the ordered code-facing queue in the
language-v1 workstream. Do not open a second consultation-only card for these
decisions.
