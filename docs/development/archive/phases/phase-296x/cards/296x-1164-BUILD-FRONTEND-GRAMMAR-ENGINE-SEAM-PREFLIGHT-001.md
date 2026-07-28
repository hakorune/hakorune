---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend grammar engine seam preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1163-BUILD-FRONTEND-TOKENIZER-ENV-LOGGING-SEAM-001.md
---

# BUILD-FRONTEND-GRAMMAR-ENGINE-SEAM-PREFLIGHT-001

## Result

The grammar engine is a good frontend crate boundary, but it must move with its
generated table ownership.

```text
grammar_rust_file_count=3
grammar_total_lines=168
main_crate_dependency_count=0
parser_grammar_engine_callsite_count=5
tokenizer_grammar_engine_callsite_count=1
direct_facade_only_selected=0
grammar_crate_scaffold_selected=1
behavior_changed=0
implementation_allowed=0
```

The current code is already dependency-light:

```text
engine_owner=src/grammar/engine.rs
generated_table_owner=src/grammar/generated.rs
generated_source=grammar/unified-grammar.toml
generator_owner=build.rs
external_runtime_dependency=once_cell
```

## Decision

Create a small `hakorune-frontend-grammar` crate and keep the historical
`crate::grammar::*` path as a compatibility facade.

The crate split row must update generated table ownership deliberately:

```text
new_crate=hakorune-frontend-grammar
new_generated_owner=crates/hakorune_frontend_grammar/src/generated.rs
compat_facade=src/grammar/mod.rs
build_rs_generated_path_update_required=1
```

## Next

```text
selected_next_task=BUILD-FRONTEND-GRAMMAR-CRATE-SCAFFOLD-001
purpose=move dependency-light grammar engine and generated tables into a frontend grammar crate
implementation_allowed=passive_split_only
default_feature_change_allowed=0
```

Non-goals:

```text
do_not_change_keyword_policy=1
do_not_change_statement_or_binop_acceptance=1
do_not_extract_parser_or_tokenizer_yet=1
do_not_change_unified_grammar_toml=1
```
