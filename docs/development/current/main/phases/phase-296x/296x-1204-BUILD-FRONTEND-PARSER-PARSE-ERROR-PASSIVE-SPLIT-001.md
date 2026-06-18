---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser error vocabulary ownership.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1203-BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-002.md
---

# BUILD-FRONTEND-PARSER-PARSE-ERROR-PASSIVE-SPLIT-001

## Result

`ParseError` now lives in `hakorune-frontend-parser`:

```text
new_owner=crates/hakorune_frontend_parser/src/parser/error.rs
compat_facade=src/parser/mod.rs
type_moved=ParseError
error_message_changed=0
active_parser_impls_moved=0
```

The main crate keeps the historical `crate::parser::ParseError` path as a
compatibility re-export.

## Guardrail

```text
NyashParser_move_allowed=0
ParserMetadata_move_allowed=0
TokenCursor_move_allowed=0
runtime_refs_added=0
MIR_refs_added=0
```

## Next

With `ParseError` owned by the frontend parser crate, the next preflight can
re-check parser cursor helpers and remaining metadata boundaries.

```text
selected_next_task=BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-003
implementation_allowed=0
```
