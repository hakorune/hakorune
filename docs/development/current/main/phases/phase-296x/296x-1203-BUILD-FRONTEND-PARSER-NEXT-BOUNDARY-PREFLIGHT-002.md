---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser next boundary preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1202-BUILD-FRONTEND-PARSER-BUILD-CONFIG-PASSIVE-SPLIT-001.md
---

# BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-002

## Result

The next safe parser-side passive split is the parser error vocabulary:

```text
selected_family=parser_error
selected_type=ParseError
selected_destination=crates/hakorune_frontend_parser/src/parser/error.rs
runtime_refs=0
MIR_refs=0
NyashParser_owner_required=0
```

Reason:

```text
ParseError_is_passive_error_vocabulary=1
ParseError_dependencies_already_frontend_parser_owned=1
TokenType_owner=hakorune_frontend_parser
TokenizeError_owner=hakorune_frontend_parser
```

Not selected:

```text
NyashParser
reason=active_parser_owner_bundle

ParserMetadata
reason=contains_RuneAttr_and_should_move_with_parser_metadata_rune_boundary

TokenCursor,NewlineMode
reason=depends_on_ParseError_and_should_follow_after_error_boundary
```

## Decision

Move `ParseError` into `hakorune-frontend-parser` and keep the historical
`crate::parser::ParseError` path as a compatibility re-export.

```text
selected_next_task=BUILD-FRONTEND-PARSER-PARSE-ERROR-PASSIVE-SPLIT-001
implementation_allowed=1
NyashParser_move_allowed=0
```

Non-goals:

```text
do_not_move_NyashParser=1
do_not_move_ParserMetadata=1
do_not_move_TokenCursor=1
do_not_change_error_messages=1
```
