---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend parser-tokenizer crate scaffold design.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1188-BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-PREFLIGHT-004.md
---

# BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-SCAFFOLD-DESIGN-001

## Decision

Create a new passive crate scaffold:

```text
crate_name=hakorune-frontend-parser
crate_path=crates/hakorune_frontend_parser
scope=parser_tokenizer_frontend_host_boundary_scaffold
behavior_changed=0
file_move_allowed=0
```

The scaffold must preserve the current parser/tokenizer root shape before any
file move:

```text
root_modules=ast,parser,tokenizer,frontend_env,frontend_log,frontend_host
root_macro=must_advance
```

Reason:

```text
crate_parser_refs=275
crate_ast_refs=121
crate_tokenizer_refs=67
crate_must_advance_refs=6
```

Rewriting these references by hand would be a high-risk churn row. The cleaner
route is to make the future crate root provide the same module names, then move
files behind that root in later rows.

## Boundary

Allowed in the scaffold row:

```text
create_crate=1
add_to_workspace_dependencies=1
add_passive_boundary_marker=1
reexport_frontend_ast=1
reexport_frontend_grammar_for_future_consumers=1
provide_noop_host_boundary_shape=1
```

Forbidden in the scaffold row:

```text
move_parser_files=0
move_tokenizer_files=0
rewrite_parser_tokenizer_imports=0
change_parser_behavior=0
change_tokenizer_behavior=0
depend_on_main_crate=0
depend_on_runtime=0
```

## Scaffold Shape

```text
crates/hakorune_frontend_parser/Cargo.toml
crates/hakorune_frontend_parser/src/lib.rs
crates/hakorune_frontend_parser/src/ast.rs
crates/hakorune_frontend_parser/src/parser.rs
crates/hakorune_frontend_parser/src/tokenizer.rs
crates/hakorune_frontend_parser/src/frontend_env.rs
crates/hakorune_frontend_parser/src/frontend_log.rs
crates/hakorune_frontend_parser/src/frontend_host.rs
```

Initial module responsibilities:

```text
ast=re-export hakorune_frontend_ast public surface
parser=empty marker only until file move
tokenizer=empty marker only until file move
frontend_env=passive marker only
frontend_log=passive marker only
frontend_host=host boundary trait mirror with no runtime dependency
```

## Next

```text
selected_next_task=BUILD-FRONTEND-PARSER-TOKENIZER-CRATE-SCAFFOLD-001
purpose=create passive crate scaffold with no behavior change
implementation_allowed=1
```
