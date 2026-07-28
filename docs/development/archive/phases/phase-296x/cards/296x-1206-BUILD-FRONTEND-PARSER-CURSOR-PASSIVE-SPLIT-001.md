---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser cursor ownership.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1205-BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-003.md
---

# BUILD-FRONTEND-PARSER-CURSOR-PASSIVE-SPLIT-001

## Result

`TokenCursor` and `NewlineMode` now live in `hakorune-frontend-parser`:

```text
new_owner=crates/hakorune_frontend_parser/src/parser/cursor.rs
compat_facade=src/parser/cursor.rs
types_moved=TokenCursor,NewlineMode
newline_policy_changed=0
active_parser_impls_moved=0
```

The main crate keeps the historical `crate::parser::cursor::{TokenCursor,
NewlineMode}` path as a compatibility re-export.

## Guardrail

```text
expr_cursor_move_allowed=0
NyashParser_move_allowed=0
ParserMetadata_move_allowed=0
runtime_refs_added=0
MIR_refs_added=0
```

## Next

The next parser boundary needs another preflight. Likely candidates are
`expr_cursor` helpers or parser metadata/rune boundaries, but they have
different dependency shapes and must not be moved opportunistically.

```text
selected_next_task=BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-004
implementation_allowed=0
```
