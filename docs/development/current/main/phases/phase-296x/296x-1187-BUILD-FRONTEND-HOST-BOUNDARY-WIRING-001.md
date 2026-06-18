---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend host boundary wiring.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1186-BUILD-FRONTEND-HOST-BOUNDARY-WIRING-PREFLIGHT-001.md
---

# BUILD-FRONTEND-HOST-BOUNDARY-WIRING-001

## Result

`frontend_env` and `frontend_log` now route host behavior through
`RuntimeFrontendHost`, which implements `FrontendHostBoundary`:

```text
adapter_owner=src/frontend_host.rs
runtime_adapter=RuntimeFrontendHost
frontend_env_direct_runtime_refs_after=0
frontend_log_direct_runtime_refs_after=0
frontend_host_runtime_refs=2
parser_tokenizer_direct_config_refs_after=0
parser_tokenizer_direct_runtime_refs_after=0
cargo_check_default_green=1
behavior_changed=0
```

Parser/tokenizer call sites were not rewritten:

```text
parser_log_facade_preserved=1
tokenizer_log_facade_preserved=1
parser_env_facade_preserved=1
```

## Decision

The frontend host seam is now explicit enough to rerun parser/tokenizer crate
extraction preflight. Direct extraction may still be blocked by root-module
layout or public API shape, but not by scattered config/runtime access.

## Next

```text
selected_next_task=BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-PREFLIGHT-004
purpose=rerun parser/tokenizer crate extraction preflight after host boundary wiring
implementation_allowed=preflight_only
```
