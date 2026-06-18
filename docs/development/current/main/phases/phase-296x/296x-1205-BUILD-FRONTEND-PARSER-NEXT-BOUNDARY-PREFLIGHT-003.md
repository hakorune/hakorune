---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser next boundary preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1204-BUILD-FRONTEND-PARSER-PARSE-ERROR-PASSIVE-SPLIT-001.md
---

# BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-003

## Result

The next safe parser-side passive split is the token cursor helper:

```text
selected_family=parser_cursor
selected_types=TokenCursor,NewlineMode
selected_destination=crates/hakorune_frontend_parser/src/parser/cursor.rs
runtime_refs=0
MIR_refs=0
NyashParser_owner_required=0
```

Reason:

```text
TokenCursor_dependencies_frontend_owned=1
Token_owner=hakorune_frontend_parser
TokenType_owner=hakorune_frontend_parser
ParseError_owner=hakorune_frontend_parser
```

Not selected:

```text
expr_cursor
reason=active_expression_parser_helper_and_should_follow_cursor_boundary

ParserMetadata
reason=contains_RuneAttr_and_should_move_with_parser_metadata_rune_boundary

NyashParser
reason=active_parser_owner_bundle
```

## Decision

Move `TokenCursor` and `NewlineMode` into `hakorune-frontend-parser` and keep
the historical `crate::parser::cursor::*` path as a compatibility facade.

```text
selected_next_task=BUILD-FRONTEND-PARSER-CURSOR-PASSIVE-SPLIT-001
implementation_allowed=1
NyashParser_move_allowed=0
```

Non-goals:

```text
do_not_move_expr_cursor=1
do_not_move_NyashParser=1
do_not_change_newline_policy=1
```
