---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend AST passive crate.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1152-BUILD-FRONTEND-AST-ATTRS-PROFILE-SEAM-001.md
---

# BUILD-FRONTEND-AST-ATTRS-PASSIVE-SPLIT-001

## Decision

Move passive declaration attribute vocabulary into `hakorune-frontend-ast`.

```text
moved_types=RuneAttr,DeclarationAttrs,RuneProfileExpansion
new_owner=crates/hakorune_frontend_ast/src/attrs.rs
new_owner=crates/hakorune_frontend_ast/src/rune_profile.rs
compat_reexport=src/ast/attrs.rs
compat_reexport=src/rune_profile_registry.rs
historical_import_path_preserved=crate::ast::{RuneAttr,DeclarationAttrs}
historical_profile_path_preserved=crate::rune_profile_registry::*
behavior_changed=0
```

## Boundary

Allowed:

```text
passive_type_split=1
compat_reexport=1
```

Forbidden:

```text
parser_behavior_change=0
rune_profile_semantics_change=0
literal_value_move=0
runtime_box_conversion_move=0
```

## Result

```text
attrs_owner=hakorune-frontend-ast
rune_profile_owner=hakorune-frontend-ast
old_ast_rune_profile_bridge_retired=1
ast_external_refs_outside_literal_box_bridge=0
cargo_check_default_green=1
```

`RuneAttr::value_contract_error()` now validates `Profile(...)` against the
frontend AST crate's `rune_profile` table. Main-crate MIR consumers keep the
historical `crate::rune_profile_registry::*` path through a compatibility
facade.

## Next

```text
selected_next_task=BUILD-FRONTEND-AST-LITERAL-VALUE-BRIDGE-DESIGN-001
reason=LiteralValue still owns main-crate runtime Box conversion through inherent methods
implementation_allowed=design_only
```
