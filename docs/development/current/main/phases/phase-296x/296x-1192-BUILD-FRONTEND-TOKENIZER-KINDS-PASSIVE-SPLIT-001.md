---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / tokenizer kinds passive split.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1191-BUILD-FRONTEND-PARSER-TOKENIZER-FILE-MOVE-PREFLIGHT-001.md
---

# BUILD-FRONTEND-TOKENIZER-KINDS-PASSIVE-SPLIT-001

## Result

Moved tokenizer kind vocabulary into `hakorune-frontend-parser`:

```text
new_owner=crates/hakorune_frontend_parser/src/tokenizer/kinds.rs
compat_facade=src/tokenizer/kinds.rs
types_moved=TokenType,Token,TokenizeError
main_crate_path_preserved=crate::tokenizer::{Token,TokenType,TokenizeError}
behavior_changed=0
tokenizer_engine_moved=0
NyashTokenizer_moved=0
```

The frontend parser crate now owns the passive token data surface while the
active tokenizer implementation remains in the main crate.

## Boundary

This row is BoxShape-only:

```text
parser_acceptance_changed=0
tokenizer_behavior_changed=0
runtime_dependency_added_to_frontend_parser=0
```

## Next

```text
selected_next_task=BUILD-FRONTEND-TOKENIZER-NEXT-MOVE-PREFLIGHT-001
purpose=choose the next tokenizer/parser movement unit after TokenType ownership moved
implementation_allowed=preflight_only
```

Reason:

```text
remaining_tokenizer_zero_crate_ref_files_have_NyashTokenizer_impl_dependency=1
inherent_impl_move_requires_defining_type_owner=1
```

The next move cannot be selected from line count alone; modules such as
`cursor`, `engine`, `lex_number`, `lex_string`, and `whitespace` are inherent
impls for `NyashTokenizer`, so moving them requires either moving the owning
type or introducing a trait/free-function boundary first.
