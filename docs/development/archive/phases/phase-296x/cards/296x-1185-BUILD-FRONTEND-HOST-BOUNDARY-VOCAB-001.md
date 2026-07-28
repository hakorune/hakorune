---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / passive frontend host boundary vocabulary.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1184-BUILD-FRONTEND-HOST-ADAPTER-DESIGN-001.md
---

# BUILD-FRONTEND-HOST-BOUNDARY-VOCAB-001

## Result

Added passive frontend host boundary vocabulary:

```text
new_owner=src/frontend_host.rs
new_type=FrontendLogLevel
new_trait=FrontendHostBoundary
new_default=NoopFrontendHost
frontend_env_wiring_changed=0
frontend_log_wiring_changed=0
parser_tokenizer_call_sites_changed=0
cargo_check_default_green=1
behavior_changed=0
```

The vocabulary defines the future boundary:

```text
log(level,message)
warn_alias_once(alias,primary)
```

## Decision

This row is intentionally passive. Runtime logger behavior still lives in
`frontend_log` / `frontend_env`.

## Next

```text
selected_next_task=BUILD-FRONTEND-HOST-BOUNDARY-WIRING-PREFLIGHT-001
purpose=decide whether to wire frontend_env/frontend_log through FrontendHostBoundary now or defer until parser crate scaffold
implementation_allowed=preflight_only
```
