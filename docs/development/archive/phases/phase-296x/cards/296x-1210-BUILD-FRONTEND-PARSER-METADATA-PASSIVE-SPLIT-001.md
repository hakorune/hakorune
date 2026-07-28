---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser metadata ownership.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1209-BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-005.md
---

# BUILD-FRONTEND-PARSER-METADATA-PASSIVE-SPLIT-001

## Result

`ParserMetadata` now lives in `hakorune-frontend-parser`:

```text
new_owner=crates/hakorune_frontend_parser/src/parser/metadata.rs
compat_facade=src/parser/mod.rs
type_moved=ParserMetadata
metadata_contents_changed=0
metadata_producer_moved=0
```

The main crate keeps the historical `crate::parser::ParserMetadata` path as a
compatibility re-export.

## Guardrail

```text
rune_parser_impls_move_allowed=0
NyashParser_move_allowed=0
runtime_refs_added=0
MIR_refs_added=0
```

## Next

The remaining parser split candidates are active parser modules or parser state
owner bundles, so the next step must be another preflight.

```text
selected_next_task=BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-006
implementation_allowed=0
```
