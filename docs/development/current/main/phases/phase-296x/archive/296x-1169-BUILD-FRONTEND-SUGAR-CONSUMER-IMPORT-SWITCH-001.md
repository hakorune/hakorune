---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser sugar consumer import switch.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1168-BUILD-FRONTEND-SUGAR-CONFIG-PASSIVE-SPLIT-001.md
---

# BUILD-FRONTEND-SUGAR-CONSUMER-IMPORT-SWITCH-001

## Result

Parser sugar consumers now read `SugarConfig` and `SugarLevel` from the
frontend grammar crate directly.

```text
parser_direct_syntax_sugar_refs_before=2
parser_direct_syntax_sugar_refs_after=0
consumer_import_target=hakorune_frontend_grammar::sugar_config
cargo_check_default_green=1
behavior_changed=0
```

The historical main-crate facade remains available for tests and compatibility:

```text
compat_facade=src/syntax/sugar_config.rs
facade_removed=0
```

## Remaining Boundary

Parser crate extraction still has one direct prelude seam:

```text
parser_direct_result_option_prelude_refs=1
```

## Next

```text
selected_next_task=BUILD-FRONTEND-RESULT-OPTION-PRELUDE-PASSIVE-SPLIT-001
purpose=move passive Result/Option enum declaration construction into hakorune-frontend-ast
implementation_allowed=passive_split_only
default_feature_change_allowed=0
```

Non-goals:

```text
do_not_change_result_option_prelude_behavior=1
do_not_extract_parser_crate_yet=1
```
