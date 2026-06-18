---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser expression cursor helper ownership.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1207-BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-004.md
---

# BUILD-FRONTEND-PARSER-EXPR-CURSOR-OWNER-BUNDLE-MOVE-001

## Result

`ExprParserWithCursor` and its helper modules now live in
`hakorune-frontend-parser`:

```text
new_owner=crates/hakorune_frontend_parser/src/parser/expr_cursor.rs
helper_modules_moved=precedence,primary,record
compat_facade=src/parser/expr_cursor.rs
active_standard_expression_parser_moved=0
expression_precedence_changed=0
```

The owner bundle also moved the small direct-birth-call error helper used by
the cursor expression path:

```text
lifecycle_helper_owner=crates/hakorune_frontend_parser/src/parser/lifecycle.rs
lifecycle_compat_facade=src/parser/lifecycle.rs
```

## Guardrail

```text
NyashParser_move_allowed=0
standard_expression_parser_move_allowed=0
ParserMetadata_move_allowed=0
runtime_refs_added=0
MIR_refs_added=0
```

## Next

The next parser boundary needs another preflight. Remaining candidates include
parser metadata/rune boundaries and active parser implementation modules.

```text
selected_next_task=BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-005
implementation_allowed=0
```
