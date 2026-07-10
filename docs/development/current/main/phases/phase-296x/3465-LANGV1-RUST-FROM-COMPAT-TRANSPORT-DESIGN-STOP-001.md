# 3465 - LANGV1-RUST-FROM-COMPAT-TRANSPORT-DESIGN-STOP-001

## Status

Decision accepted. 3463 and 3464 complete the Rust typed profile,
statement-try, and peek seams. Implementation is authorized only by 3466.

## Accepted Decision

Adopt A: a separate migration-only transport adapter.

```text
transport representation = distinct MigrationTransport record
grammar evidence = span-free ParseWitness referencing the transport record
semantic parser result = AST or typed ParseError::TransportOnly
CompatibilityTransport AST node = forbidden
Compat2025 source-from semantic AST success = forbidden
Option Some/None internal representation = unchanged
```

Canonical semantic parsing rejects the two source forms with their
form-specific tags. Compat2025 semantic parsing recognizes a closed transport
form but fails before AST publication with
`parser/from_compat_transport_only`. Only the migration adapter may return
accepted transport evidence.

Accepted decision claims:

```text
from_compat_transport_owner_decision = separate_migration_transport_adapter
from_compat_transport_adapter_required = 1
migration_transport_record_required = 1
parse_witness_references_migration_transport = 1
canonical_box_from_reject_required = 1
canonical_from_call_reject_required = 1
compat2025_from_semantic_entry_transport_only_error_required = 1
compat_transport_ast_forbidden = 1
from_semantic_lowering_forbidden = 1
option_sugar_unchanged_required = 1
```

## Contract

The accepted grammar registry classifies both forms identically:

```text
box Child from Parent = Canonical rejected, Compat2025 transport-only
from Parent.method() = Canonical rejected, Compat2025 transport-only
```

Transport-only means the parser may preserve migration syntax evidence, but it
must not enter canonical AST semantics, MIR, runtime, or backend lowering.

## Inventory

Current Rust parsing violates that separation:

```text
box Child from Parent
  -> BoxDeclaration.extends
  -> live inheritance/delegation semantics

from Parent.method()
  -> ASTNode::FromCall
  -> live call semantics

Option Some/None sugar
  -> also uses ASTNode::FromCall as an internal representation
```

The public parser API currently returns only `ASTNode`. It has no parse result
that can report accepted compatibility transport while refusing semantic AST
entry. A profile check alone therefore cannot implement the registry row.

## Decision Required

Choose one transport owner.

### A. Separate migration transport adapter (recommended)

Keep the semantic parser AST-free of compatibility transport. Add a dedicated
grammar migration adapter that recognizes the two closed `from` spellings and
emits a span-free `CompatibilityTransport` witness. Canonical semantic parsing
rejects with the form-specific tag. Compat2025 compiler entry recognizes the
transport but fails before AST publication with
`parser/from_compat_transport_only`.

This keeps migration evidence and executable semantics physically separate.

### B. CompatibilityTransport AST node

Add an AST node and require every semantic consumer to reject it.

Not recommended: it broadens the canonical AST and creates many downstream
places where transport could accidentally gain semantics.

### C. Preserve current semantic AST under Compat2025

Rejected. This silently turns transport-only into inheritance/super-call
execution and contradicts the accepted registry.

## Consultation Packet

```text
We are at LANGV1-RUST-FROM-COMPAT-TRANSPORT-DESIGN-STOP-001.

Accepted contract:
- box Child from Parent and from Parent.method() reject in Canonical.
- Compat2025 accepts them only as CompatibilityTransport.
- transport must not enter canonical semantic lowering.

Current Rust shape:
- box-from immediately populates BoxDeclaration.extends.
- from-call immediately creates ASTNode::FromCall.
- ASTNode::FromCall is also reused internally by Option Some/None sugar.
- the public parser returns ASTNode only; no transport result boundary exists.

Recommended A:
- add a separate migration-only transport adapter;
- emit span-free CompatibilityTransport witnesses;
- Canonical semantic parse rejects with form-specific stable tags;
- Compat2025 compiler parse recognizes transport then fails before AST
  publication with parser/from_compat_transport_only;
- no AST transport node and no semantic lowering.

Please decide:
1. Accept or reject A.
2. Whether transport output belongs beside ParseWitness or in a distinct
   MigrationTransport record referenced by ParseWitness.
3. Whether the public semantic parser returns a typed TransportOnly error or a
   broader ParseOutput enum without publishing AST.
4. Exact fixture matrix and conditions for retiring the old semantic routes.

Return claims, non-claims, stable tags, source authority, fail-fast boundary,
and the minimum first implementation slice.

Do not authorize a CompatibilityTransport AST node unless unavoidable, old
from semantic execution under Compat2025, Option sugar changes, Hako changes,
runtime/backend changes, or selfhost migration.
```

## Source Authority

```text
grammar status = grammar/unified-grammar.toml
transport law = docs/reference/language/grammar-contract.md
semantic parser evidence = box declaration parser and from-call parser
normalized witness boundary = hakorune-frontend-grammar ParseWitness
```

## Non-Claims

```text
rust_from_migrated = 0
from_compat_transport_implemented = 0
compat_transport_ast_authorized = 0
from_semantic_lowering = 0
option_sugar_changed = 0
hako_parser_behavior_changed = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Next

3466 implements one code-facing transport boundary for both forms. Do not
split box-from and from-call into separate design/rerun cards.
