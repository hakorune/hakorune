---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser delegate lowering ownership.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1211-BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-006.md
---

# BUILD-FRONTEND-PARSER-DELEGATE-LOWERING-OWNER-BUNDLE-MOVE-001

## Result

Delegate exposes lowering now lives in `hakorune-frontend-parser`:

```text
new_owner=crates/hakorune_frontend_parser/src/parser/delegate_lowering.rs
compat_facade=src/parser/delegate_lowering.rs
entrypoint=lower_delegate_exposes
delegate_lowering_semantics_changed=0
active_NyashParser_impls_moved=0
```

The main crate keeps the historical
`crate::parser::delegate_lowering::lower_delegate_exposes` path as a
compatibility re-export.

## Guardrail

```text
NyashParser_move_allowed=0
items_or_common_parser_impls_move_allowed=0
runtime_refs_added=0
MIR_refs_added=0
```

## Next

The remaining parser split work is now mostly active `NyashParser` impl
bundles. The next task is a preflight that selects whether to move a small
active bundle or stop this parser split series.

```text
selected_next_task=BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-007
implementation_allowed=0
```
