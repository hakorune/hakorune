---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend host boundary wiring preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1185-BUILD-FRONTEND-HOST-BOUNDARY-VOCAB-001.md
---

# BUILD-FRONTEND-HOST-BOUNDARY-WIRING-PREFLIGHT-001

## Result

The passive host boundary vocabulary exists, but current frontend host calls
still bypass it:

```text
frontend_host_boundary_vocab=1
frontend_env_calls_host_boundary=0
frontend_log_calls_host_boundary=0
frontend_env_runtime_ref_for_alias_warning=1
frontend_log_runtime_refs=3
```

The wiring can be done without touching parser/tokenizer call sites:

```text
parser_log_facade_preserved=1
tokenizer_log_facade_preserved=1
parser_env_facade_preserved=1
parser_tokenizer_call_site_rewrite_required=0
```

## Decision

Wire the existing shared `frontend_env` / `frontend_log` facades through a
main-crate runtime adapter that implements `FrontendHostBoundary`.

```text
selected_shape=RuntimeFrontendHost_adapter
adapter_owner=src/frontend_host.rs
frontend_env_direct_runtime_ref_after_target=0
frontend_log_direct_runtime_refs_after_target=0
runtime_logger_behavior_preserved=1
alias_warning_once_semantics_preserved=1
```

Non-goals:

```text
do_not_thread_host_through_NyashParser=1
do_not_thread_host_through_NyashTokenizer=1
do_not_extract_parser_tokenizer_crate=1
do_not_replace_runtime_logger=1
```

## Next

```text
selected_next_task=BUILD-FRONTEND-HOST-BOUNDARY-WIRING-001
purpose=wire frontend_env/frontend_log through RuntimeFrontendHost adapter
implementation_allowed=seam_only
```
