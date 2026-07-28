Status: Done
Date: 2026-06-18
Scope: isolate RuneAttr Profile validation from the main crate registry
Related:
  - docs/development/current/main/phases/phase-296x/296x-1151-BUILD-FRONTEND-AST-SYNTAX-PASSIVE-SPLIT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-FRONTEND-AST-ATTRS-PROFILE-SEAM-001

## Change

```text
output_contract=build-frontend-ast-attrs-profile-seam-v0

new_module=src/ast/rune_profile_bridge.rs
attrs_direct_rune_profile_registry_refs=0
profile_registry_bridge_refs=2
behavior_changed=0
profile_validation_changed=0
```

`RuneAttr` profile validation no longer reads `crate::rune_profile_registry`
directly from `attrs.rs`. The dependency is isolated behind
`ast/rune_profile_bridge.rs`, preparing `RuneAttr` / `DeclarationAttrs` for a
passive AST crate split.

## Proof

```bash
cargo check -q
rg -n "rune_profile_registry|crate::" src/ast/attrs.rs src/ast/rune_profile_bridge.rs
rg -n "crate::(parser|tokenizer|config|runtime|backend|mir|box_trait|boxes|rune_profile_registry)" \
  src/ast -g '!src/ast/literal_box_bridge.rs' -g '!src/ast/rune_profile_bridge.rs'
```

## Result

```text
cargo_check_default_green=1
ast_external_refs_outside_bridges=0
selected_next_task=BUILD-FRONTEND-AST-ATTRS-PASSIVE-SPLIT-001
summary=ok
```

## Stop Lines

```text
do_not_move_attrs_in_seam_row=1
do_not_change_rune_profile_validation=1
do_not_change_parser_rune_acceptance=1
```
