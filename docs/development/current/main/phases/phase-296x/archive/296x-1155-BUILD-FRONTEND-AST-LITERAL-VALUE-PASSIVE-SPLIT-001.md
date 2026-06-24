---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend AST passive crate.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1154-BUILD-FRONTEND-AST-LITERAL-VALUE-BRIDGE-DESIGN-001.md
---

# BUILD-FRONTEND-AST-LITERAL-VALUE-PASSIVE-SPLIT-001

## Decision

Move `LiteralValue` and its `Display` implementation into
`hakorune-frontend-ast`.

```text
moved_type=LiteralValue
new_owner=crates/hakorune_frontend_ast/src/literal.rs
display_impl_owner=hakorune-frontend-ast
compat_reexport=src/ast/syntax.rs
historical_import_path_preserved=crate::ast::LiteralValue
literal_variants_preserved=1
behavior_changed=0
```

## Runtime Bridge

Runtime Box conversion remains in the main crate as explicit bridge functions:

```text
runtime_conversion_owner=src/ast/literal_box_bridge.rs
bridge_module_public=1
bridge_functions=literal_to_nyash_box,literal_from_nyash_box
inherent_runtime_conversion_methods_preserved=0
internal_method_callsite_count=0
```

## Checks

```text
ast_external_refs_outside_literal_box_bridge=0
cargo_check_default_green=1
current_state_pointer_guard_green=1
```

## Next

```text
selected_next_task=BUILD-FRONTEND-AST-NODES-PASSIVE-PREFLIGHT-001
purpose=audit remaining AST node passive data versus main-crate dependencies before moving larger node shapes
implementation_allowed=preflight_only
```
