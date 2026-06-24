---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser Result-Option prelude consumer import switch.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1170-BUILD-FRONTEND-RESULT-OPTION-PRELUDE-PASSIVE-SPLIT-001.md
---

# BUILD-FRONTEND-PARSER-PRELUDE-CONSUMER-IMPORT-SWITCH-001

## Result

Parser initialization now reads passive Result/Option prelude declarations from
the frontend AST crate directly.

```text
parser_direct_result_option_prelude_refs_before=1
parser_direct_result_option_prelude_refs_after=0
consumer_import_target=hakorune_frontend_ast::result_option_prelude
cargo_check_default_green=1
behavior_changed=0
```

The historical main-crate facade remains for main-crate MIR/stage1 callers:

```text
compat_facade=src/semantics/result_option_prelude.rs
facade_removed=0
mir_builder_facade_refs=1
stage1_facade_refs=1
```

## Remaining Boundary

The parser/tokenizer frontend split now needs a fresh preflight. The prior
blockers were reduced, but the parser still has internal crate-path coupling:

```text
parser_direct_grammar_refs=0
parser_direct_sugar_refs=0
parser_direct_prelude_refs=0
parser_ast_facade_refs=present
parser_tokenizer_dependency=present
parser_internal_crate_path_refs=present
```

## Next

```text
selected_next_task=BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-PREFLIGHT-002
purpose=rerun parser/tokenizer extraction preflight after grammar/sugar/prelude seams were split
implementation_allowed=preflight_only
default_feature_change_allowed=0
```

Non-goals:

```text
do_not_extract_parser_or_tokenizer_yet=1
do_not_touch_mir_or_stage1_callers=1
```
