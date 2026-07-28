---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser-tokenizer crate preflight after frontend seams.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1171-BUILD-FRONTEND-PARSER-PRELUDE-CONSUMER-IMPORT-SWITCH-001.md
---

# BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-PREFLIGHT-002

## Result

Direct parser/tokenizer crate extraction is still not selected, but the blocker
class has moved.

```text
parser_tokenizer_rust_file_count=92
parser_tokenizer_total_lines=15144
parser_direct_grammar_refs=0
parser_direct_sugar_refs=0
parser_direct_prelude_refs=0
direct_parser_tokenizer_crate_extraction_allowed=0
behavior_changed=0
implementation_allowed=0
```

The remaining crate-path families are:

```text
parser_ast_facade_refs=121
parser_tokenizer_refs=67
parser_internal_crate_path_refs=259
parser_tokenizer_env_log_facade_refs=present
```

The `crate::ast`, `crate::parser`, and `crate::tokenizer` families are mostly
layout/compatibility concerns. A future parser crate can preserve them by
placing parser/tokenizer under compatible crate-root modules.

The harder extraction blocker is the env/log facade implementation:

```text
parser_env_facade_delegates_to_main_config=1
parser_log_facade_delegates_to_main_runtime=1
tokenizer_env_facade_delegates_to_main_config=1
tokenizer_log_facade_delegates_to_main_runtime=1
```

Those facades are good seams, but they still depend on the main crate.

## Decision

Do not extract the parser/tokenizer crate yet. First decide the env/log
abstraction boundary for a standalone frontend parser crate.

```text
selected_next_task=BUILD-FRONTEND-PARSER-ENV-LOG-ABSTRACTION-PREFLIGHT-001
purpose=choose standalone parser/tokenizer env and logging boundary before crate extraction
implementation_allowed=preflight_only
```

Non-goals:

```text
do_not_extract_parser_or_tokenizer_yet=1
do_not_rewrite_parser_internal_paths=1
do_not_change_env_defaults=1
do_not_change_logging_behavior=1
```
