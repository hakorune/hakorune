---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend sugar config passive split.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1167-BUILD-FRONTEND-PARSER-SYNTAX-PRELUDE-SEAM-PREFLIGHT-001.md
---

# BUILD-FRONTEND-SUGAR-CONFIG-PASSIVE-SPLIT-001

## Result

`SugarConfig` and `SugarLevel` now live in `hakorune-frontend-grammar`.

```text
new_owner=crates/hakorune_frontend_grammar/src/sugar_config.rs
compat_facade=src/syntax/sugar_config.rs
compat_import_path_preserved=1
cargo_check_default_green=1
cargo_test_frontend_grammar_green=1
behavior_changed=0
```

The frontend grammar crate now owns both grammar tables and sugar-level config:

```text
grammar_table_owner=hakorune-frontend-grammar
sugar_config_owner=hakorune-frontend-grammar
```

## Remaining Boundary

Parser still reads sugar config through the main-crate compatibility facade:

```text
parser_direct_syntax_sugar_refs_after=2
external_sugar_config_available=1
```

## Next

```text
selected_next_task=BUILD-FRONTEND-SUGAR-CONSUMER-IMPORT-SWITCH-001
purpose=switch parser sugar config imports to hakorune_frontend_grammar
implementation_allowed=import_switch_only
default_feature_change_allowed=0
```

Non-goals:

```text
do_not_change_sugar_behavior=1
do_not_extract_parser_crate_yet=1
```
