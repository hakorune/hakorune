---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser next boundary preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1210-BUILD-FRONTEND-PARSER-METADATA-PASSIVE-SPLIT-001.md
---

# BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-006

## Result

The next safe parser-side owner bundle is delegate exposes lowering:

```text
selected_family=parser_delegate_lowering
selected_owner_bundle=lower_delegate_exposes
selected_destination=crates/hakorune_frontend_parser/src/parser/delegate_lowering.rs
runtime_refs=0
MIR_refs=0
NyashParser_owner_required=0
```

Reason:

```text
delegate_lowering_depends_on_AST_and_ParseError_only=1
post_parse_AST_transform=1
main_crate_config_or_runtime_refs=0
```

Not selected:

```text
contracts,common,items
reason=active_NyashParser_impls

NyashParser
reason=active_parser_owner_bundle
```

## Decision

Move delegate exposes lowering into `hakorune-frontend-parser`, keeping the
historical `crate::parser::delegate_lowering::lower_delegate_exposes` path as a
compatibility facade.

```text
selected_next_task=BUILD-FRONTEND-PARSER-DELEGATE-LOWERING-OWNER-BUNDLE-MOVE-001
implementation_allowed=1
NyashParser_move_allowed=0
```

Non-goals:

```text
do_not_move_NyashParser=1
do_not_change_delegate_lowering_semantics=1
do_not_move_items_or_common_parser_impls=1
```
