---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser-tokenizer crate extraction preflight v4.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1187-BUILD-FRONTEND-HOST-BOUNDARY-WIRING-001.md
---

# BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-PREFLIGHT-004

## Result

The host seam blocker is closed:

```text
parser_tokenizer_direct_config_refs=0
parser_tokenizer_direct_runtime_refs=0
frontend_env_direct_runtime_refs=0
frontend_log_direct_runtime_refs=0
host_runtime_refs_owner=src/frontend_host.rs
host_runtime_ref_count=2
```

Remaining root references are layout references:

```text
crate_parser_refs=275
crate_ast_refs=121
crate_tokenizer_refs=67
crate_must_advance_refs=6
crate_frontend_env_refs=5
crate_frontend_log_refs=5
crate_frontend_host_refs=6
```

These are compatible with a future frontend crate that provides the same root
modules:

```text
future_crate_root_modules=ast,parser,tokenizer,frontend_env,frontend_log,frontend_host
future_crate_macro=must_advance
```

Direct extraction should still not be attempted as a move-only row because it
would mix crate scaffolding, compatibility root layout, and module relocation.

## Decision

Create a frontend parser-tokenizer crate scaffold design first.

```text
selected_next_task=BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-SCAFFOLD-DESIGN-001
purpose=define crate root layout and compatibility imports before moving parser/tokenizer files
implementation_allowed=design_only
direct_file_move_allowed=0
```

Non-goals:

```text
do_not_rewrite_463_root_refs_by_hand=1
do_not_move_parser_tokenizer_files_yet=1
do_not_change_parser_behavior=1
```
