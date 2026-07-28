---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser-tokenizer env and logging abstraction preflight v2.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1174-BUILD-FRONTEND-PARSER-DIRECT-ENV-SEAM-001.md
---

# BUILD-FRONTEND-PARSER-ENV-LOG-ABSTRACTION-PREFLIGHT-002

## Result

Direct parser environment reads are centralized, but the parser/tokenizer
facades still depend on main-crate configuration and runtime logging:

```text
parser_direct_std_env_reads_outside_facade=0
parser_env_facade_main_config_refs=11
tokenizer_env_facade_main_config_refs=7
parser_log_facade_runtime_refs=3
tokenizer_log_facade_runtime_refs=2
direct_parser_tokenizer_crate_extraction_allowed=0
```

The remaining env functions fall into two families:

```text
simple_env_flags=most parser/tokenizer toggles can be reproduced locally with std::env reads
alias_warning_flags=parser_stage3_enabled carries legacy alias warning behavior through config::env::warn_alias_once
```

## Decision

Do not introduce a large trait-backed host adapter yet. It would force parser
and tokenizer call sites to carry host plumbing before there is a concrete
crate boundary.

Use staged standalone facades instead:

```text
selected_shape=standalone_facade_simple_flags_first
option_trait_backed_adapter=defer
option_global_runtime_logger_adapter=defer
behavior_changed=0
implementation_allowed=next_row_only
```

The first implementation row may only remove main-crate config references for
simple env flags whose behavior is a direct env read and does not require alias
warning state.

Non-goals:

```text
do_not_change_stage3_alias_warning_behavior=1
do_not_change_runtime_logger_behavior=1
do_not_extract_parser_tokenizer_crate=1
do_not_thread_parser_env_objects_through_NyashParser=1
```

## Next

```text
selected_next_task=BUILD-FRONTEND-PARSER-STANDALONE-ENV-SIMPLE-FLAGS-001
purpose=replace simple parser/tokenizer facade config::env delegates with local env helpers while preserving behavior
implementation_allowed=seam_only
```
