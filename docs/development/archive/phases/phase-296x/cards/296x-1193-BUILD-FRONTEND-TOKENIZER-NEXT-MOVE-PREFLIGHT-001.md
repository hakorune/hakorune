---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / tokenizer next movement preflight after kinds split.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1192-BUILD-FRONTEND-TOKENIZER-KINDS-PASSIVE-SPLIT-001.md
---

# BUILD-FRONTEND-TOKENIZER-NEXT-MOVE-PREFLIGHT-001

## Result

After moving `TokenType` / `Token` / `TokenizeError`, the remaining tokenizer
implementation files are not independent passive data files:

```text
remaining_tokenizer_impl_files=cursor,engine,lex_ident,lex_number,lex_string,whitespace
remaining_tokenizer_impl_owner=NyashTokenizer
single_impl_file_move_allowed=0
reason=inherent_impl_requires_defining_type_owner
```

Although several files have zero direct `crate::` references, they are inherent
impl blocks for `NyashTokenizer`. Rust requires inherent impls to live in the
same crate as the defining type, so moving one impl file without the owning type
is not a valid split.

## Decision

Do not pick another line-count-based leaf file. The next safe slice is a
tokenizer owner-bundle design:

```text
selected_next_task=BUILD-FRONTEND-TOKENIZER-OWNER-BUNDLE-DESIGN-001
owner_type=NyashTokenizer
bundle_files=mod,cursor,engine,env,log,lex_ident,lex_number,lex_string,whitespace
kinds_already_moved=1
implementation_allowed=design_only
```

Design questions for the next row:

```text
frontend_env_behavior_preserved=required
frontend_log_host_boundary_preserved=required
runtime_dependency_in_frontend_parser=forbidden
main_crate_facade_required=required
```

Non-goals:

```text
do_not_move_tokenizer_bundle_before_design=1
do_not_drop_alias_warning_or_logging_semantics_silently=1
do_not_introduce_runtime_dependency_into_frontend_parser=1
```
