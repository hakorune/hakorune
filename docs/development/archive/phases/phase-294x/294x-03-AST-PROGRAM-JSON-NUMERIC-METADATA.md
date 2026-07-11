---
Status: Complete
Date: 2026-05-12
Scope: AST JSON and Program(JSON) declared param/return metadata transport.
Related:
  - src/macro/ast_json/shared.rs
  - src/macro/ast_json/joinir_compat.rs
  - src/macro/ast_json/roundtrip.rs
  - src/stage1/program_json_v0/lowering.rs
  - src/runner/json_v0_bridge/ast.rs
  - src/tests/parser_header_param_extensions.rs
  - src/stage1/program_json_v0/tests/stage1_sources.rs
---

# 294x-03 AST Program JSON Numeric Metadata

## Goal

Round-trip declared parameter and return type text through JSON surfaces without
changing runtime semantics or claiming exact `usize` behavior.

## Changes

- Added shared AST JSON helpers for `param_decls` serialization and legacy
  names-only fallback.
- Extended AST JSON `FunctionDeclaration` shape with:
  - `param_decls: [{ name, declared_type }]`;
  - `return_type`.
- Kept `params` as the canonical names-only compatibility surface.
- Kept legacy AST JSON readable by reconstructing `param_decls` from `params`
  when the richer metadata is absent.
- Extended Stage1 Program(JSON) helper defs with the same `param_decls` and
  `return_type` metadata.
- Extended JSON v0 bridge structs so Program(JSON) metadata survives
  deserialize/serialize paths, while lowerers still use `params` for function
  arity and variable-map ownership.

## Stop Line

This row does not add:

- MIR exact numeric type facts;
- route facts for numeric params/returns;
- runtime `usize` values;
- verifier range, negative, overflow, or sentinel checks;
- backend exact unsigned lowering;
- hako_alloc field migration.

## Proof

```bash
cargo check --bin hakorune
cargo test -q parser_header_param_extensions --lib
cargo test -q source_to_program_json_v0_emits_helper_defs_for_main_box_methods --lib
cargo check --tests
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
