---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / shared frontend Stage-3 env facade.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1177-BUILD-FRONTEND-PARSER-STAGE3-ENV-SEAM-PREFLIGHT-001.md
---

# BUILD-FRONTEND-STAGE3-ENV-SHARED-FACADE-001

## Result

Stage-3 feature parsing and legacy alias warnings now have one shared frontend
owner:

```text
new_owner=src/frontend_env.rs
parser_stage3_config_delegate_after=0
tokenizer_stage3_config_delegate_after=0
parser_tokenizer_duplicate_stage3_logic=0
stage3_alias_warning_behavior_preserved=1
cargo_check_default_green=1
```

Parser/tokenizer config dependencies after this row:

```text
parser_env_facade_main_config_refs_after=1
tokenizer_env_facade_main_config_refs_after=0
remaining_parser_config_ref=cli_verbose_enabled
parser_log_facade_runtime_refs_after=3
tokenizer_log_facade_runtime_refs_after=2
frontend_env_runtime_ref_for_alias_warning=1
```

## Decision

This is still a seam, not parser crate extraction.

```text
parser_tokenizer_crate_extraction_allowed=0
runtime_logger_behavior_changed=0
cli_verbose_cache_behavior_changed=0
```

## Next

The remaining blocker is logging/verbose host access:

```text
selected_next_task=BUILD-FRONTEND-LOGGING-FACADE-PREFLIGHT-001
purpose=decide parser/tokenizer logging and CLI verbose host seam before parser crate extraction
implementation_allowed=preflight_only
```
