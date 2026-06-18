---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser next boundary preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1212-BUILD-FRONTEND-PARSER-DELEGATE-LOWERING-OWNER-BUNDLE-MOVE-001.md
---

# BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-007

## Result

The next safe parser-side passive transform is sugar desugaring:

```text
selected_family=parser_sugar_transform
selected_entrypoint=apply_sugar
selected_destination=crates/hakorune_frontend_parser/src/parser/sugar.rs
runtime_refs=0
MIR_refs=0
NyashParser_owner_required=0
```

Reason:

```text
sugar_transform_depends_on_AST_and_SugarConfig_only=1
AST_to_AST_transform=1
main_crate_config_or_runtime_refs=0
```

Not selected:

```text
sugar_gate
reason=depends_on_main_crate_parser_env

entry_sugar
reason=depends_on_NyashParser_entrypoints

NyashParser
reason=active_parser_owner_bundle
```

## Decision

Move `apply_sugar` and the AST-to-AST sugar transform into
`hakorune-frontend-parser`, keeping the historical `crate::parser::sugar`
path as a compatibility facade.

```text
selected_next_task=BUILD-FRONTEND-PARSER-SUGAR-TRANSFORM-PASSIVE-SPLIT-001
implementation_allowed=1
NyashParser_move_allowed=0
```

Non-goals:

```text
do_not_move_sugar_gate=1
do_not_move_entry_sugar=1
do_not_change_sugar_semantics=1
```
