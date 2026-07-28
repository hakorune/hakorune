---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser-tokenizer logging facade preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1178-BUILD-FRONTEND-STAGE3-ENV-SHARED-FACADE-001.md
---

# BUILD-FRONTEND-LOGGING-FACADE-PREFLIGHT-001

## Result

Parser/tokenizer logging calls are already centralized:

```text
parser_log_facade=src/parser/log.rs
tokenizer_log_facade=src/tokenizer/log.rs
parser_log_facade_runtime_refs=3
tokenizer_log_facade_runtime_refs=2
parser_tokenizer_direct_runtime_refs_outside_log_facades=0
```

The logging blocker is therefore a facade ownership problem, not scattered
call-site cleanup.

## Decision

Introduce a shared frontend logging facade that preserves the current runtime
logger behavior. Parser/tokenizer log facades should delegate to it.

```text
selected_shape=shared_frontend_log_facade
new_owner=src/frontend_log.rs
runtime_logger_behavior_preserved=1
parser_tokenizer_log_facades_preserved=1
call_site_rewrite_required=0
```

Non-goals:

```text
do_not_replace_runtime_logger_with_eprintln=1
do_not_thread_logger_objects_through_parser=1
do_not_change_cli_verbose_cache=1
do_not_extract_parser_tokenizer_crate=1
```

## Next

```text
selected_next_task=BUILD-FRONTEND-LOGGING-SHARED-FACADE-001
purpose=move parser/tokenizer runtime logging access behind shared frontend_log facade
implementation_allowed=seam_only
```
