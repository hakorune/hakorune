---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser passive data ownership.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1201-BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-001.md
---

# BUILD-FRONTEND-PARSER-BUILD-CONFIG-PASSIVE-SPLIT-001

## Result

`BuildMode` and `ParserBuildConfig` now live in `hakorune-frontend-parser`:

```text
new_owner=crates/hakorune_frontend_parser/src/parser/build_config.rs
compat_facade=src/parser/mod.rs
types_moved=BuildMode,ParserBuildConfig
active_build_cfg_impls_moved=0
behavior_changed=0
```

The main crate keeps the historical `crate::parser::{BuildMode,
ParserBuildConfig}` path as a compatibility re-export. Active build-cfg
predicate/prune logic still lives in the main crate.

## Guardrail

```text
NyashParser_move_allowed=0
ParseError_move_allowed=0
ParserMetadata_move_allowed=0
runtime_refs_added=0
MIR_refs_added=0
```

## Next

The next parser boundary needs a fresh preflight before moving more types:

```text
selected_next_task=BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-002
reason=remaining_parser_passive_candidates_have_different_dependency_shapes
```
