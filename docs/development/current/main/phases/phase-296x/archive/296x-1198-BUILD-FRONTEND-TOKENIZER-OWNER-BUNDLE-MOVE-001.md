---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / tokenizer owner-bundle move.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1197-BUILD-FRONTEND-TOKENIZER-FACADE-WRAPPER-DESIGN-001.md
---

# BUILD-FRONTEND-TOKENIZER-OWNER-BUNDLE-MOVE-001

## Result

Moved the tokenizer implementation owner bundle into
`hakorune-frontend-parser`:

```text
new_owner=crates/hakorune_frontend_parser/src/tokenizer.rs
moved_files=cursor,engine,env,log,lex_ident,lex_number,lex_string,whitespace
already_moved_files=kinds
main_facade=src/tokenizer/mod.rs
main_facade_shape=wrapper
```

The main crate keeps the public tokenizer API:

```text
crate::tokenizer::NyashTokenizer::new(input)
crate::tokenizer::NyashTokenizer::tokenize()
crate::tokenizer::{Token,TokenType,TokenizeError}
```

The wrapper installs the main runtime host before constructing the extracted
tokenizer:

```text
wrapper_new_installs_runtime_host=1
frontend_parser_depends_on_runtime=0
tokenization_behavior_changed=0
```

## Verification

```text
cargo_check_q=green
cargo_test_hakorune_frontend_parser=green
tokenizer_unit_tests=2_passed
```

## Next

```text
selected_next_task=BUILD-FRONTEND-PARSER-TOKENIZER-POST-TOKENIZER-MOVE-PREFLIGHT-001
purpose=choose the next parser/tokenizer split boundary after tokenizer implementation moved
implementation_allowed=preflight_only
```

Non-goals:

```text
parser_files_moved=0
parser_behavior_changed=0
```
