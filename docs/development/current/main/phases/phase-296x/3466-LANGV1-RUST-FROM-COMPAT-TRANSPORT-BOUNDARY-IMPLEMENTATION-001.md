# 3466 - LANGV1-RUST-FROM-COMPAT-TRANSPORT-BOUNDARY-IMPLEMENTATION-001

## Status

Active implementation card. Implement the accepted 3465 migration transport
boundary for both Rust `from` forms without publishing semantic AST.

## Structural Scope

```text
semantic parser = AST success or typed TransportOnly error
migration adapter = ParseWitness plus distinct MigrationTransport
transport classifier input = tokenizer tokens, not ad-hoc source text
CompatibilityTransport AST node = forbidden
```

Keep one structured closed-form classifier shared by the Rust semantic guard
and migration adapter. The classifier recognizes syntax only; it owns no AST,
MIR, runtime, or backend semantics.

## Closed Forms

The first transport slice recognizes exactly:

```text
box IDENT from IDENT { }
from IDENT . IDENT ( )
```

Whitespace and comments are tokenizer concerns. Bodies, arguments, multiple
parents, aliases, and other variants are outside this first closed set and
fail with `parser/from_transport_not_closed_form`.

## Ordered Work

1. Add a distinct `MigrationTransport` record with:
   `transport_id`, `row_id`, `profile`, `transport_kind`, `spelling_id`, and
   explicit false flags for AST/MIR/runtime/backend entry.
2. Add `MigrationTransportKind::{BoxFromInheritance, FromCall}`.
3. Add a migration result bundle containing one span-free `ParseWitness` and
   one referenced `MigrationTransport`; do not add an AST node.
4. Add a token-based closed-form classifier for both forms.
5. Add `parse_migration_transport_with_config` as the only accepted transport
   entry. It requires explicit Compat2025.
6. Canonical semantic box-from rejects with
   `parser/from_inheritance_legacy` before `BoxDeclaration.extends` publication.
7. Canonical semantic from-call rejects with `parser/from_call_legacy` before
   `ASTNode::FromCall` publication.
8. Compat2025 semantic parsing of either closed form returns typed
   `ParseError::TransportOnly` with `parser/from_compat_transport_only` before
   AST publication.
9. Malformed or expanded forms reject with
   `parser/from_transport_not_closed_form`.
10. Keep Option Some/None sugar and canonical `delegate field exposes`
    unchanged.
11. Update registry, contract docs, corpus, typed projection, and focused guard
    together if the accepted `parser/from_call_legacy` tag replaces the older
    `parser/from_super_call_legacy` spelling.

## Typed Error Contract

```text
ParseError::TransportOnly {
  row_id,
  profile = Compat2025,
  transport_kind,
  stable_reject_tag = parser/from_compat_transport_only
}
```

This error is not a parse success and cannot be converted to AST, MIR, runtime,
or backend input.

## Stable Tags

```text
parser/from_inheritance_legacy
parser/from_call_legacy
parser/from_compat_transport_only
parser/from_transport_semantic_entry_forbidden
parser/from_transport_ast_forbidden
parser/from_transport_mir_forbidden
parser/from_transport_backend_forbidden
parser/from_transport_not_closed_form
parser/profile_required_for_compat
parser/profile_mismatch
```

## Focused Fixtures

```text
Canonical semantic box-from -> form-specific reject; no AST
Canonical semantic from-call -> form-specific reject; no AST
Compat2025 semantic box-from -> typed TransportOnly; no AST
Compat2025 semantic from-call -> typed TransportOnly; no AST
Compat2025 migration box-from -> witness plus BoxFromInheritance record
Compat2025 migration from-call -> witness plus FromCall record
Canonical migration entry -> profile_required_for_compat
malformed/expanded form -> from_transport_not_closed_form
canonical delegate exposes -> unchanged AST
Option Some/None -> unchanged AST
transport record presented to semantic boundary -> fail-fast
```

## Acceptance

```text
from_compat_transport_boundary_implemented = 1
migration_transport_record_implemented = 1
compat2025_from_migration_adapter_implemented = 1
compat2025_from_semantic_transport_only_error_implemented = 1
canonical_from_rejects_implemented = 1
source_box_from_extends_publication = 0
source_from_call_ast_publication = 0
compat_transport_ast_node_count = 0
option_sugar_changed = 0
hako_parser_behavior_changed = 0
runtime_backend_behavior_changed = 0
```

## Non-Claims

```text
rust_from_migrated = 0
from_semantic_lowering = 0
explicit_delegation_normalization = 0
hako_parser_behavior_changed = 0
migration_transport_to_mir = 0
migration_transport_to_backend = 0
migration_transport_to_runtime = 0
implicit_compat_fallback = 0
parser_sharing = 0
broad_parser_rewrite = 0
selfhost_claim = 0
```

## Verification

Add focused unit tests and extend the reusable Rust grammar-profile guard when
the code owners land. Run `cargo check`, the grammar substrate guard, and the
current-state pointer guard. Keep every new source file below 800 lines.

## Next

After the implementation is green, rerun the shared migration/semantic fixture
matrix inside this card and close Rust from migration without a form-specific
rerun card.
