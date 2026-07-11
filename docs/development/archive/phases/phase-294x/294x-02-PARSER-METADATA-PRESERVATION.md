---
Status: Complete
Date: 2026-05-12
Scope: parser / AST declared parameter and return type metadata preservation.
Related:
  - src/ast/mod.rs
  - src/parser/common/params.rs
  - src/tests/parser_header_param_extensions.rs
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/reference/language/EBNF.md
  - docs/reference/language/LANGUAGE_REFERENCE_2025.md
---

# 294x-02 Parser Metadata Preservation

## Goal

Keep source parameter and return type annotations visible in the Rust AST before
later 294x rows add JSON round-trip, MIR exact numeric types, verifier checks,
or runtime `usize` behavior.

## Changes

- Added `ParamDecl` as parameter metadata next to the existing names-only
  `params` surface.
- Added `param_decls` and `return_type_name` to function declarations.
- Preserved declared parameter type text for:
  - box methods;
  - static box methods;
  - top-level functions;
  - static items;
  - `birth` / `init` / `pack` constructors;
  - interface signatures.
- Preserved return annotation text where the parser accepts
  `name(...): Type`.
- Kept `params: Vec<String>` as the canonical names-only surface for existing
  AST consumers.

## Stop Line

This row does not add:

- AST JSON or Program(JSON) numeric metadata round-trip;
- MIR exact numeric type facts;
- runtime `usize` values;
- range, overflow, or negative-value verifier checks;
- backend lowering changes;
- hako_alloc numeric field migration.

## Proof

```bash
cargo check --bin hakorune
cargo test -q parser_header_param_extensions --lib
cargo test -q numeric_substrate --lib
cargo check --tests
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
