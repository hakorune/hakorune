---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / passive frontend parser-tokenizer crate scaffold.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1189-BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-SCAFFOLD-DESIGN-001.md
---

# BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-SCAFFOLD-001

## Result

Created a passive crate scaffold:

```text
crate_name=hakorune-frontend-parser
crate_path=crates/hakorune_frontend_parser
root_modules=ast,parser,tokenizer,frontend_env,frontend_log,frontend_host,grammar
root_macro=must_advance
main_crate_dependency_added=1
behavior_changed=0
parser_files_moved=0
tokenizer_files_moved=0
runtime_dependency_added_to_frontend_parser=0
```

The scaffold provides the future crate root shape while keeping the active
parser/tokenizer implementation in the main crate.

## Boundary

The new crate may depend on:

```text
hakorune-frontend-ast
hakorune-frontend-grammar
```

It must not depend on:

```text
nyash-rust main crate
runtime
MIR
backend
Box implementations
```

## Next

```text
selected_next_task=BUILD-FRONTEND-PARSER-TOKENIZER-FILE-MOVE-PREFLIGHT-001
purpose=select the smallest parser/tokenizer file family that can move behind the scaffold
implementation_allowed=preflight_only
direct_bulk_move_allowed=0
```

Non-goals:

```text
do_not_move_all_parser_files_at_once=1
do_not_rewrite_root_refs_by_hand=1
do_not_change_parser_behavior=1
```
