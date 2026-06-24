---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend CLI verbose local seam.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1181-BUILD-FRONTEND-HOST-SEAM-PREFLIGHT-001.md
---

# BUILD-FRONTEND-CLI-VERBOSE-LOCAL-SEAM-001

## Result

The final parser-local `crate::config::env` delegate is removed:

```text
parser_tokenizer_direct_config_refs_after=0
parser_env_cli_verbose_config_delegate_after=0
cli_verbose_env_contract=NYASH_CLI_VERBOSE=1|2
cargo_check_default_green=1
behavior_changed=0
```

Remaining host access is centralized outside parser/tokenizer modules:

```text
frontend_env_runtime_ref_for_alias_warning=1
frontend_log_runtime_refs=3
parser_tokenizer_direct_runtime_refs_after=0
```

## Decision

This closes the scattered parser/tokenizer env/config cleanup. Parser/tokenizer
crate extraction should be re-preflighted before any new implementation.

```text
parser_tokenizer_env_config_scattered_refs_closed=1
parser_tokenizer_runtime_logging_scattered_refs_closed=1
direct_parser_tokenizer_crate_extraction_claim=0
```

## Next

```text
selected_next_task=BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-PREFLIGHT-003
purpose=rerun parser/tokenizer crate extraction preflight after env/log host seams were centralized
implementation_allowed=preflight_only
```
