---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser-tokenizer grammar consumer imports.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1165-BUILD-FRONTEND-GRAMMAR-CRATE-SCAFFOLD-001.md
---

# BUILD-FRONTEND-GRAMMAR-CONSUMER-IMPORT-SWITCH-001

## Result

Parser and tokenizer grammar-engine consumers now read the frontend grammar
crate directly.

```text
parser_tokenizer_crate_grammar_imports_before=6
parser_tokenizer_crate_grammar_imports_after=0
consumer_import_target=hakorune_frontend_grammar::engine
cargo_check_default_green=1
behavior_changed=0
```

The historical main-crate facade remains available for non-parser callers:

```text
compat_facade=src/grammar/mod.rs
facade_removed=0
```

## Remaining Boundary

Parser crate extraction is still blocked by two non-grammar frontend seams:

```text
parser_direct_syntax_sugar_refs=2
parser_direct_result_option_prelude_refs=1
parser_tokenizer_dependency=present
```

## Next

```text
selected_next_task=BUILD-FRONTEND-PARSER-SYNTAX-PRELUDE-SEAM-PREFLIGHT-001
purpose=choose boundaries for parser sugar config and result/option prelude inputs
implementation_allowed=preflight_only
default_feature_change_allowed=0
```

Non-goals:

```text
do_not_extract_parser_or_tokenizer_yet=1
do_not_change_sugar_behavior=1
do_not_change_result_option_prelude_behavior=1
```
