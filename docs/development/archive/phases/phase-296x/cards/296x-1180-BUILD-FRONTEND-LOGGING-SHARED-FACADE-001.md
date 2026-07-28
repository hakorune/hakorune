---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / shared frontend logging facade.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1179-BUILD-FRONTEND-LOGGING-FACADE-PREFLIGHT-001.md
---

# BUILD-FRONTEND-LOGGING-SHARED-FACADE-001

## Result

Parser/tokenizer log facades now delegate to a shared frontend logging owner:

```text
new_owner=src/frontend_log.rs
parser_log_facade_runtime_refs_after=0
tokenizer_log_facade_runtime_refs_after=0
frontend_log_runtime_refs=3
parser_tokenizer_direct_runtime_refs_after=0
cargo_check_default_green=1
behavior_changed=0
```

This preserves the current runtime logger route while removing runtime logger
ownership from parser/tokenizer modules.

## Remaining Host Seams

```text
parser_env_remaining_config_ref=cli_verbose_enabled
frontend_env_runtime_ref_for_alias_warning=1
frontend_log_runtime_refs=3
direct_parser_tokenizer_crate_extraction_allowed=0
```

The remaining work is no longer scattered across parser/tokenizer modules. It is
a host-seam decision for frontend env/log support.

## Next

```text
selected_next_task=BUILD-FRONTEND-HOST-SEAM-PREFLIGHT-001
purpose=decide whether frontend env/log host access stays main-crate facade or moves behind a crate-ready host adapter
implementation_allowed=preflight_only
```
