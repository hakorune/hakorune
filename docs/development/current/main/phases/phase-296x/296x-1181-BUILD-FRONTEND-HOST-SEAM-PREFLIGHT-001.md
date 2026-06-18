---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend env-log host seam preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1180-BUILD-FRONTEND-LOGGING-SHARED-FACADE-001.md
---

# BUILD-FRONTEND-HOST-SEAM-PREFLIGHT-001

## Result

Parser/tokenizer modules no longer own scattered host access. Remaining host
access is centralized:

```text
parser_env_remaining_config_ref=cli_verbose_enabled
frontend_env_runtime_ref_for_alias_warning=1
frontend_log_runtime_refs=3
parser_tokenizer_direct_runtime_refs=0
parser_tokenizer_direct_config_refs=0
```

The remaining seams have different risk levels:

```text
cli_verbose=low_risk_env_read
alias_warning=runtime_logger_semantics
frontend_log=runtime_logger_semantics
```

## Decision

Do not introduce a full host adapter yet. First remove the last low-risk config
delegate (`cli_verbose_enabled`) by using the same process env contract locally.

```text
selected_next_shape=cli_verbose_local_env_seam
host_adapter_introduction=defer
parser_tokenizer_crate_extraction_allowed=0
```

The runtime logger seams should remain centralized in `frontend_env` /
`frontend_log` until there is a concrete parser crate extraction row.

## Next

```text
selected_next_task=BUILD-FRONTEND-CLI-VERBOSE-LOCAL-SEAM-001
purpose=replace parser env cli_verbose config delegate with local env read
implementation_allowed=seam_only
```
