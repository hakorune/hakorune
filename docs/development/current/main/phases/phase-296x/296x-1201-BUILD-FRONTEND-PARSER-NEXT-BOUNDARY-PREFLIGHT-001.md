---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser next boundary preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1200-BUILD-FRONTEND-BUILD-GATE-REPORT-PASSIVE-SPLIT-001.md
---

# BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-001

## Result

The next safe parser-side passive split is parser build configuration:

```text
selected_family=parser_build_config
selected_types=BuildMode,ParserBuildConfig
selected_destination=crates/hakorune_frontend_parser/src/parser/build_config.rs
runtime_refs=0
MIR_refs=0
NyashParser_owner_required=0
```

Reason:

```text
ParserBuildConfig_is_passive_data=1
BuildMode_is_passive_data=1
active_build_cfg_impls_stay_main_crate=1
```

Not selected:

```text
ParserMetadata
reason=contains_RuneAttr_and_should_move_with_parser_metadata_rune_boundary
```

## Decision

Move `BuildMode` and `ParserBuildConfig` into `hakorune-frontend-parser` and
keep the main-crate parser path as a compatibility facade.

```text
selected_next_task=BUILD-FRONTEND-PARSER-BUILD-CONFIG-PASSIVE-SPLIT-001
implementation_allowed=1
NyashParser_move_allowed=0
```

Non-goals:

```text
do_not_move_NyashParser=1
do_not_move_ParseError=1
do_not_move_ParserMetadata=1
do_not_change_build_cfg_behavior=1
```
