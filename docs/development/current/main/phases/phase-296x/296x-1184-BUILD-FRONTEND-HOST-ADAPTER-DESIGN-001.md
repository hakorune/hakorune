---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend host adapter design.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1183-BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-PREFLIGHT-003.md
---

# BUILD-FRONTEND-HOST-ADAPTER-DESIGN-001

## Problem

Parser/tokenizer crate extraction is blocked by host behavior that must not
live inside a pure frontend crate:

```text
frontend_env_runtime_ref_for_alias_warning=1
frontend_log_runtime_refs=3
```

The parser/tokenizer modules no longer own direct config/runtime access. The
remaining question is how a future frontend crate reports warnings/logs without
depending on `crate::runtime`.

## Decision

Use a small host boundary vocabulary before any parser crate extraction.

```text
selected_shape=FrontendHostBoundary
host_owns_logging=1
host_owns_alias_warning_sink=1
frontend_owns_feature_parsing=1
parser_struct_threading_now=0
```

The future boundary is:

```text
FrontendHost:
  debug(message)
  warn(message)
  error(message)
  warn_alias_once(alias, primary)
```

Current main crate behavior remains:

```text
main_crate_adapter=runtime_ring0_logger
runtime_logger_behavior_preserved=1
parser_tokenizer_call_sites_unchanged=1
```

## Rationale

Do not thread host objects through `NyashParser` / `NyashTokenizer` yet. That
would touch broad parser construction paths before a concrete extraction row.

Instead:

```text
phase_1=define passive host boundary vocabulary
phase_2=make frontend_env/frontend_log call the boundary through the main-crate adapter
phase_3=extract parser/tokenizer crate only after the adapter seam is stable
```

## Next

```text
selected_next_task=BUILD-FRONTEND-HOST-BOUNDARY-VOCAB-001
purpose=add passive FrontendHostBoundary vocabulary without changing parser/tokenizer call sites
implementation_allowed=passive_vocab_only
```
