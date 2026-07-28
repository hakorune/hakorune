---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend parser crate preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1161-BUILD-FRONTEND-AST-SPLIT-CLOSEOUT-001.md
---

# BUILD-FRONTEND-PARSER-CRATE-PREFLIGHT-001

## Result

Direct parser crate extraction is not selected yet.

```text
parser_tokenizer_rust_file_count=90
parser_tokenizer_total_lines=15091
direct_parser_crate_extraction_allowed=0
behavior_changed=0
implementation_allowed=0
```

The AST data boundary is clean after the frontend AST split, but parser and
tokenizer still carry non-AST main-crate seams:

```text
tokenizer_direct_config_env_refs=present
tokenizer_direct_runtime_log_refs=present
tokenizer_direct_grammar_engine_refs=present
parser_tokenizer_dependency=present
parser_direct_grammar_engine_refs=present
parser_direct_syntax_sugar_refs=present
parser_direct_result_option_prelude_refs=present
```

These are crate-boundary blockers, not parser semantics blockers.

## Boundary

This row is preflight only:

```text
parser_behavior_changed=0
tokenizer_behavior_changed=0
ast_boundary_reopened=0
new_crate_created=0
```

## Decision

The next safe slice is to isolate tokenizer config/logging access behind
tokenizer-local facades before any tokenizer or parser crate extraction.

```text
selected_next_task=BUILD-FRONTEND-TOKENIZER-ENV-LOGGING-SEAM-001
purpose=remove tokenizer direct config/runtime logging dependencies
implementation_allowed=seam_only
default_feature_change_allowed=0
```

Non-goals:

```text
do_not_move_parser_crate_yet=1
do_not_move_tokenizer_crate_yet=1
do_not_change_grammar_engine_policy=1
do_not_change_parser_sugar_or_prelude_behavior=1
```
