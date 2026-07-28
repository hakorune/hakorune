---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser-tokenizer standalone simple env flags.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1175-BUILD-FRONTEND-PARSER-ENV-LOG-ABSTRACTION-PREFLIGHT-002.md
---

# BUILD-FRONTEND-PARSER-STANDALONE-ENV-SIMPLE-FLAGS-001

## Result

Simple parser/tokenizer env flags now resolve inside the local facades instead
of delegating to `crate::config::env`:

```text
parser_env_facade_main_config_refs_before=11
parser_env_facade_main_config_refs_after=5
tokenizer_env_facade_main_config_refs_before=7
tokenizer_env_facade_main_config_refs_after=1
parser_log_facade_runtime_refs_after=3
tokenizer_log_facade_runtime_refs_after=2
cargo_check_default_green=1
behavior_changed=0
```

Moved only direct/simple flag behavior:

```text
parser_allow_semicolon
parser_method_body_strict_enabled
parser_static_init_strict_enabled
parser_static_seam_break_on_static_enabled
parser_static_seam_tolerant_enabled
parser_static_trace_enabled
parser_token_cursor_enabled
parser_try_compat_enabled
unified_members
tokenizer grammar_diff
tokenizer parser_allow_semicolon
tokenizer parser_decode_unicode
tokenizer parser_metadata_annotations_enabled
tokenizer strict_12_7
tokenizer tok_trace
```

## Still Blocked

Remaining main-crate dependencies are intentional:

```text
parser_stage3_enabled_still_delegates_to_config=1
stage3_alias_warning_behavior_preserved=1
parser_block_method_expr_catch_still_follow_stage3_alias_warning=1
cli_verbose_cache_behavior_preserved=1
runtime_logger_behavior_preserved=1
```

The next seam must decide whether to preserve legacy alias warnings through a
small local warning primitive, or to keep Stage-3 gating in the main crate until
parser extraction.

## Next

```text
selected_next_task=BUILD-FRONTEND-PARSER-STAGE3-ENV-SEAM-PREFLIGHT-001
purpose=decide the Stage-3/env-alias warning seam before parser crate extraction
implementation_allowed=preflight_only
```
