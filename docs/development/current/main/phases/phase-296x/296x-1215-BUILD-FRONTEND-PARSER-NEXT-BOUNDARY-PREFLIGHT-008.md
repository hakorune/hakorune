---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser next boundary preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1214-BUILD-FRONTEND-PARSER-SUGAR-TRANSFORM-PASSIVE-SPLIT-001.md
---

# BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-008

## Result

No further thin passive parser boundary is selected in this series:

```text
selected_next_boundary=none
remaining_src_parser_file_count=54
frontend_parser_owned_parser_file_count=12
```

Remaining parser candidates are active owner bundles or main-crate adapters:

```text
NyashParser_impl_bundles=active
common_items_contracts_declarations=active_NyashParser_impls
build_cfg_predicate_prune=active_NyashParser_impls
entry_sugar=depends_on_NyashParser_entrypoints
sugar_gate=depends_on_main_crate_parser_env
parser_env_log=main_crate_host_adapters
```

Already moved into `hakorune-frontend-parser` in this parser split series:

```text
TokenType,Token,TokenizeError
NyashTokenizer owner bundle
BuildGateExplainReport
BuildMode,ParserBuildConfig
ParseError
TokenCursor,NewlineMode
ExprParserWithCursor owner bundle
ParserMetadata
delegate exposes lowering
AST-to-AST sugar transform
```

## Decision

Stop the current passive parser split series and close it out before any active
`NyashParser` owner-bundle move.

```text
selected_next_task=BUILD-FRONTEND-PARSER-SPLIT-SERIES-CLOSEOUT-001
implementation_allowed=0
active_parser_impl_move_allowed=0
```

Non-goals:

```text
do_not_move_NyashParser_in_this_series=1
do_not_move_parser_env_log_wrappers=1
do_not_start_active_impl_bundle_without_new_design=1
```
