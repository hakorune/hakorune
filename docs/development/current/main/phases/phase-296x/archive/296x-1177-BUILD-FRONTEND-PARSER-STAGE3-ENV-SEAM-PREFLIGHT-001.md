---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser-tokenizer Stage-3 env alias-warning seam preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1176-BUILD-FRONTEND-PARSER-STANDALONE-ENV-SIMPLE-FLAGS-001.md
---

# BUILD-FRONTEND-PARSER-STAGE3-ENV-SEAM-PREFLIGHT-001

## Result

The remaining parser/tokenizer config dependencies are concentrated in
Stage-3 gating and parser logging:

```text
parser_stage3_enabled_config_delegate=1
tokenizer_stage3_enabled_config_delegate=1
parser_block_method_expr_catch_follow_stage3=1
cli_verbose_config_delegate=1
parser_log_runtime_delegate=3
tokenizer_log_runtime_delegate=2
```

Stage-3 cannot be duplicated independently in parser and tokenizer because
legacy aliases emit a warning once:

```text
legacy_aliases=NYASH_PARSER_STAGE3,HAKO_PARSER_STAGE3
current_warning_owner=crate::config::env::warn_alias_once
duplicated_parser_tokenizer_warning_sets_allowed=0
```

## Decision

Introduce one shared frontend env facade for Stage-3 feature parsing and alias
warnings, then let parser/tokenizer facades call it.

```text
selected_shape=shared_frontend_env_stage3_facade
new_owner=src/frontend_env.rs
parser_tokenizer_duplicate_stage3_logic=0
stage3_alias_warning_behavior_preserved=1
runtime_logger_behavior_preserved=1
```

Non-goals:

```text
do_not_change_cli_verbose_cache=1
do_not_change_parser_log_runtime_delegate=1
do_not_change_tokenizer_log_runtime_delegate=1
do_not_extract_parser_tokenizer_crate=1
```

## Next

```text
selected_next_task=BUILD-FRONTEND-STAGE3-ENV-SHARED-FACADE-001
purpose=move Stage-3 feature parsing and alias warning into shared frontend env facade
implementation_allowed=seam_only
```
