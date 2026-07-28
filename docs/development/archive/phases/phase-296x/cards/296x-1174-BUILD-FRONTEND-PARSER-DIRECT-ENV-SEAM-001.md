---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser direct environment-read seam.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1173-BUILD-FRONTEND-PARSER-ENV-LOG-ABSTRACTION-PREFLIGHT-001.md
---

# BUILD-FRONTEND-PARSER-DIRECT-ENV-SEAM-001

## Result

Parser direct environment reads are now centralized behind the parser-local
facade:

```text
parser_env_facade=src/parser/env.rs
parser_direct_std_env_reads_outside_facade_after=0
parser_env_facade_direct_read_count=9
behavior_changed=0
cargo_check_default_green=1
```

Moved the remaining direct reads for:

```text
NYASH_DEBUG_PARSE_LOCAL
NYASH_DEPRECATE_THIS
NYASH_ENABLE_MAP_LITERAL
NYASH_GRAMMAR_DIFF
NYASH_PARSER_ALLOW_SEMICOLON
NYASH_PARSER_METHOD_BODY_STRICT
NYASH_PARSER_TRACE_BLOCKS
NYASH_SYNTAX_SUGAR_LEVEL
```

into `src/parser/env.rs` consumers. The facade still delegates broader config
switches to the main crate where that was already the existing contract.

## Decision

This row is seam-only. It does not introduce a standalone parser environment
object yet.

```text
env_log_abstraction_allowed=not_yet
parser_tokenizer_crate_extraction_allowed=0
default_feature_change_allowed=0
logging_behavior_changed=0
env_default_changed=0
```

## Next

Rerun the parser env/log abstraction preflight now that direct parser env reads
are centralized:

```text
selected_next_task=BUILD-FRONTEND-PARSER-ENV-LOG-ABSTRACTION-PREFLIGHT-002
purpose=decide whether parser/tokenizer can use standalone ParserEnv/ParserLogger or adapter seams
implementation_allowed=preflight_only
```
