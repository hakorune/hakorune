---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend parser passive split series closeout.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1215-BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-008.md
---

# BUILD-FRONTEND-PARSER-SPLIT-SERIES-CLOSEOUT-001

## Result

The current frontend parser passive split series is closed:

```text
parser_split_series_closed=1
active_parser_impl_move_allowed=0
remaining_src_parser_file_count=54
frontend_parser_owned_parser_file_count=12
```

Moved into `hakorune-frontend-parser` during this series:

```text
tokenizer_kinds=TokenType,Token,TokenizeError
tokenizer_owner_bundle=NyashTokenizer
parser_report=BuildGateExplainReport
parser_build_config=BuildMode,ParserBuildConfig
parser_error=ParseError
parser_cursor=TokenCursor,NewlineMode
parser_expr_cursor=ExprParserWithCursor
parser_metadata=ParserMetadata
parser_delegate_lowering=lower_delegate_exposes
parser_sugar_transform=apply_sugar
```

Compatibility facades remain in the main crate for historical paths:

```text
src/tokenizer/mod.rs
src/parser/build_cfg.rs
src/parser/cursor.rs
src/parser/expr_cursor.rs
src/parser/lifecycle.rs
src/parser/delegate_lowering.rs
src/parser/sugar.rs
src/parser/mod.rs
```

## Stop Line

Do not move active parser modules as a drive-by continuation of this series:

```text
do_not_move_NyashParser_without_new_design=1
do_not_move_parser_env_log_wrappers=1
do_not_move_build_cfg_predicate_prune=1
do_not_move_common_items_declarations_without_active_owner_bundle_design=1
```

Remaining parser-side work is a different class of change:

```text
remaining_work_class=active_parser_owner_bundle_or_host_adapter_design
BoxShape_series_complete=1
BoxCount_added=0
behavior_changed=0
```

## Next

Return to build split measurement before choosing another crate boundary:

```text
selected_next_task=BUILD-FRONTEND-PARSER-POST-SPLIT-MEASUREMENT-001
implementation_allowed=0
measurement_allowed=1
```
