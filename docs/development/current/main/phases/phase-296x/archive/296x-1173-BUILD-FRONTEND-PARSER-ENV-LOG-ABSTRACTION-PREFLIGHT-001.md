---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser-tokenizer env and logging abstraction preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1172-BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-PREFLIGHT-002.md
---

# BUILD-FRONTEND-PARSER-ENV-LOG-ABSTRACTION-PREFLIGHT-001

## Result

Standalone parser/tokenizer env/log abstraction is not ready to implement yet.

The current facades are useful seams:

```text
parser_env_facade=src/parser/env.rs
parser_log_facade=src/parser/log.rs
tokenizer_env_facade=src/tokenizer/env.rs
tokenizer_log_facade=src/tokenizer/log.rs
```

But parser modules still contain direct `std::env::var` reads outside the
parser env facade:

```text
parser_direct_std_env_reads=present
parser_env_facade_complete=0
env_log_abstraction_allowed=0
behavior_changed=0
implementation_allowed=0
```

Examples include grammar-diff tracing, parser block tracing, method-body
strictness, parse-local debugging, map literal enablement, and `this`
deprecation warning switches.

## Decision

Do not introduce a standalone env/log abstraction yet. First consolidate parser
direct env reads into `src/parser/env.rs`, preserving existing defaults and
behavior.

```text
selected_next_task=BUILD-FRONTEND-PARSER-DIRECT-ENV-SEAM-001
purpose=move parser direct std::env reads behind parser env facade
implementation_allowed=seam_only
default_feature_change_allowed=0
```

After that, rerun env/log abstraction preflight and decide whether the parser
crate gets:

```text
option_a=standalone default ParserEnv/ParserLogger value objects
option_b=trait-backed host adapter
option_c=compile-time facade plus main-crate adapter
```

Non-goals:

```text
do_not_change_env_defaults=1
do_not_change_logging_behavior=1
do_not_extract_parser_or_tokenizer_yet=1
```
