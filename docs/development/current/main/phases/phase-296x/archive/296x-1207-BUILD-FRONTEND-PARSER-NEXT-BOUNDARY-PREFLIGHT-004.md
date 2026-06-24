---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser next boundary preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1206-BUILD-FRONTEND-PARSER-CURSOR-PASSIVE-SPLIT-001.md
---

# BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-004

## Result

The next safe parser-side move is the cursor expression helper owner bundle:

```text
selected_family=parser_expr_cursor
selected_owner_bundle=ExprParserWithCursor,precedence,primary,record
selected_destination=crates/hakorune_frontend_parser/src/parser/expr_cursor.rs
runtime_refs=0
MIR_refs=0
NyashParser_owner_required=0
```

Reason:

```text
expr_cursor_depends_on_AST_and_tokenizer_only=1
TokenCursor_owner=hakorune_frontend_parser
ParseError_owner=hakorune_frontend_parser
main_crate_config_or_runtime_refs=0
```

Not selected:

```text
ParserMetadata
reason=contains_RuneAttr_and_should_move_with_parser_metadata_rune_boundary

NyashParser
reason=active_parser_owner_bundle
```

## Decision

Move `ExprParserWithCursor` and its helper modules into
`hakorune-frontend-parser`, keeping the historical
`crate::parser::expr_cursor::ExprParserWithCursor` path as a compatibility
facade.

```text
selected_next_task=BUILD-FRONTEND-PARSER-EXPR-CURSOR-OWNER-BUNDLE-MOVE-001
implementation_allowed=1
NyashParser_move_allowed=0
```

Non-goals:

```text
do_not_move_NyashParser=1
do_not_move_standard_expression_parser=1
do_not_change_expression_precedence=1
```
