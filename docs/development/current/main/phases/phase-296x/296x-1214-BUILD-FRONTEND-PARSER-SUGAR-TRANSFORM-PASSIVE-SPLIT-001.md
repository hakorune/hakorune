---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser sugar transform ownership.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1213-BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-007.md
---

# BUILD-FRONTEND-PARSER-SUGAR-TRANSFORM-PASSIVE-SPLIT-001

## Result

The AST-to-AST sugar transform now lives in `hakorune-frontend-parser`:

```text
new_owner=crates/hakorune_frontend_parser/src/parser/sugar.rs
compat_facade=src/parser/sugar.rs
entrypoint=apply_sugar
sugar_semantics_changed=0
sugar_gate_moved=0
entry_sugar_moved=0
```

The main crate keeps the historical `crate::parser::sugar::apply_sugar` path as
a compatibility re-export.

## Guardrail

```text
NyashParser_move_allowed=0
sugar_gate_move_allowed=0
entry_sugar_move_allowed=0
runtime_refs_added=0
MIR_refs_added=0
```

## Next

The remaining parser split candidates are active parser entry modules and
parser env/gate wrappers. Another preflight is required before moving them.

```text
selected_next_task=BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-008
implementation_allowed=0
```
