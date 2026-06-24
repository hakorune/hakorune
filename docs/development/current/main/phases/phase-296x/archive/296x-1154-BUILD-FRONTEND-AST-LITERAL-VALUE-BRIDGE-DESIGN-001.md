---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend AST passive crate.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1153-BUILD-FRONTEND-AST-ATTRS-PASSIVE-SPLIT-001.md
---

# BUILD-FRONTEND-AST-LITERAL-VALUE-BRIDGE-DESIGN-001

## Problem

`LiteralValue` is passive frontend AST data, but it still lives in the main
crate because `src/ast/literal_box_bridge.rs` attaches runtime Box conversion
as inherent methods:

```text
LiteralValue::to_nyash_box()
LiteralValue::from_nyash_box()
```

If `LiteralValue` moves to `hakorune-frontend-ast`, the main crate cannot
preserve those inherent methods because Rust forbids inherent impls for types
defined in another crate.

```text
orphan_rule_blocks_inherent_method_compat=1
internal_to_nyash_box_callsite_count=0
internal_literal_from_nyash_box_callsite_count=0
```

## Decision

Move the passive data and `Display` implementation to `hakorune-frontend-ast`.
Keep runtime conversion as explicit main-crate bridge functions instead of
methods.

```text
new_data_owner=crates/hakorune_frontend_ast/src/literal.rs
display_impl_owner=hakorune-frontend-ast
runtime_conversion_owner=src/ast/literal_box_bridge.rs
inherent_runtime_conversion_methods_preserved=0
free_function_bridge_enabled=1
```

The compatibility target is the data path, not runtime method syntax:

```text
historical_import_path_preserved=crate::ast::LiteralValue
literal_variants_preserved=1
literal_display_preserved=1
runtime_box_conversion_method_path_preserved=0
```

## Bridge API

The main crate bridge should expose explicit functions:

```rust
pub fn literal_to_nyash_box(value: &LiteralValue) -> Box<dyn NyashBox>;
pub fn literal_from_nyash_box(box_val: &dyn NyashBox) -> Option<LiteralValue>;
```

Optional extension traits are not selected for v0 because they require trait
imports at call sites and would look like method compatibility without actually
preserving the old inherent-method API.

```text
extension_trait_selected=0
reason=would_create_method_like_partial_compatibility
```

## Allowed Implementation Slice

```text
move_literal_value_enum=1
move_literal_display_impl=1
replace_literal_box_bridge_inherent_impl_with_free_functions=1
preserve_crate_ast_literalvalue_reexport=1
behavior_changed=0
```

Forbidden:

```text
runtime_box_semantics_change=0
parser_literal_semantics_change=0
macro_json_literal_shape_change=0
mir_literal_lowering_change=0
```

## Next

```text
selected_next_task=BUILD-FRONTEND-AST-LITERAL-VALUE-PASSIVE-SPLIT-001
implementation_allowed=passive_type_split_plus_bridge_function_rename
```
