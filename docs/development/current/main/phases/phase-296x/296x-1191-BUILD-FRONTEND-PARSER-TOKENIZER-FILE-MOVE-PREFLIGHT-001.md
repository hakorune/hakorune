---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser-tokenizer first file-move preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1190-BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-SCAFFOLD-001.md
---

# BUILD-FRONTEND-PARSER-TOKENIZER-FILE-MOVE-PREFLIGHT-001

## Result

The smallest safe first file-move family is tokenizer kinds:

```text
selected_family=tokenizer_kinds
selected_source=src/tokenizer/kinds.rs
selected_destination=crates/hakorune_frontend_parser/src/tokenizer/kinds.rs
line_count=161
direct_crate_refs=0
runtime_refs=0
parser_refs=0
behavior_change_allowed=0
```

Moved types:

```text
TokenType
Token
TokenizeError
```

Reason:

```text
tokenizer_core_files_with_zero_crate_refs=cursor,engine,kinds,lex_ident,lex_number,lex_string,mod,whitespace
tokenizer_kinds_has_no_NyashTokenizer_impl_dependency=1
tokenizer_kinds_is_public_surface=1
```

`engine` / lexer modules are not selected first because they depend on active
`NyashTokenizer` impl structure and env/log behavior. Moving them now would mix
root crate layout work with active implementation movement.

## Decision

Move tokenizer kinds into the passive frontend parser crate and keep the
historical main-crate path as a compatibility facade.

```text
selected_next_task=BUILD-FRONTEND-TOKENIZER-KINDS-PASSIVE-SPLIT-001
purpose=move TokenType/Token/TokenizeError to hakorune-frontend-parser
implementation_allowed=1
main_crate_facade_required=1
```

Non-goals:

```text
do_not_move_NyashTokenizer=1
do_not_move_tokenizer_engine=1
do_not_move_tokenizer_env_log=1
do_not_change_tokenization_behavior=1
```
