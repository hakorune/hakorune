---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / tokenizer env and logging seam.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1162-BUILD-FRONTEND-PARSER-CRATE-PREFLIGHT-001.md
---

# BUILD-FRONTEND-TOKENIZER-ENV-LOGGING-SEAM-001

## Result

Tokenizer implementation modules no longer import the main crate config or
runtime logger directly.

```text
tokenizer_env_facade=src/tokenizer/env.rs
tokenizer_log_facade=src/tokenizer/log.rs
tokenizer_direct_config_env_refs_after=0
tokenizer_direct_runtime_log_refs_after=0
cargo_check_default_green=1
behavior_changed=0
```

The new facades are intentionally thin:

```text
env_facade_delegates_to=crate::config::env
log_facade_delegates_to=crate::runtime::get_global_ring0().log
policy_changed=0
logging_behavior_changed=0
```

## Remaining Boundary

Tokenizer and parser extraction are still blocked by non-env seams:

```text
tokenizer_direct_grammar_engine_refs=1
parser_direct_grammar_engine_refs=5
parser_direct_syntax_sugar_refs=present
parser_direct_result_option_prelude_refs=present
parser_tokenizer_dependency=present
```

## Next

```text
selected_next_task=BUILD-FRONTEND-GRAMMAR-ENGINE-SEAM-PREFLIGHT-001
purpose=choose whether grammar engine access becomes a parser/tokenizer facade or a shared frontend dependency
implementation_allowed=preflight_only
default_feature_change_allowed=0
```

Non-goals:

```text
do_not_move_tokenizer_crate_yet=1
do_not_move_parser_crate_yet=1
do_not_change_keyword_policy=1
do_not_change_statement_or_binop_acceptance=1
```
