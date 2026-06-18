---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend grammar crate scaffold.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1164-BUILD-FRONTEND-GRAMMAR-ENGINE-SEAM-PREFLIGHT-001.md
---

# BUILD-FRONTEND-GRAMMAR-CRATE-SCAFFOLD-001

## Result

The dependency-light grammar engine and generated tables now live in
`hakorune-frontend-grammar`.

```text
new_crate=hakorune-frontend-grammar
new_crate_path=crates/hakorune_frontend_grammar
new_engine_owner=crates/hakorune_frontend_grammar/src/engine.rs
new_generated_owner=crates/hakorune_frontend_grammar/src/generated.rs
compat_facade=src/grammar/mod.rs
src_grammar_facade_lines=9
build_rs_generated_path=crates/hakorune_frontend_grammar/src/generated.rs
cargo_check_default_green=1
behavior_changed=0
```

The main crate keeps the historical import path:

```text
crate_grammar_import_path_preserved=1
parser_behavior_changed=0
tokenizer_behavior_changed=0
```

## Remaining Boundary

Parser/tokenizer still import grammar through the main-crate compatibility
facade:

```text
parser_tokenizer_crate_grammar_imports_after=6
external_grammar_crate_available=1
```

## Next

```text
selected_next_task=BUILD-FRONTEND-GRAMMAR-CONSUMER-IMPORT-SWITCH-001
purpose=switch parser/tokenizer grammar engine imports to hakorune_frontend_grammar
implementation_allowed=import_switch_only
default_feature_change_allowed=0
```

Non-goals:

```text
do_not_change_keyword_policy=1
do_not_change_statement_or_binop_acceptance=1
do_not_extract_parser_or_tokenizer_yet=1
```
