---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / Result-Option prelude passive split.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1169-BUILD-FRONTEND-SUGAR-CONSUMER-IMPORT-SWITCH-001.md
---

# BUILD-FRONTEND-RESULT-OPTION-PRELUDE-PASSIVE-SPLIT-001

## Result

Passive Result/Option enum declaration construction now lives in
`hakorune-frontend-ast`.

```text
new_owner=crates/hakorune_frontend_ast/src/result_option_prelude.rs
compat_facade=src/semantics/result_option_prelude.rs
compat_import_path_preserved=1
cargo_check_default_green=1
cargo_test_frontend_ast_green=1
behavior_changed=0
```

The data is AST-only:

```text
depends_on_parser=0
depends_on_runtime=0
depends_on_mir=0
```

## Remaining Boundary

The parser still reads the compatibility facade once:

```text
parser_direct_result_option_prelude_refs_after=1
external_result_option_prelude_available=1
```

Other main-crate callers may remain on the facade until their own boundaries
need extraction:

```text
mir_builder_facade_refs=1
stage1_facade_refs=1
```

## Next

```text
selected_next_task=BUILD-FRONTEND-PARSER-PRELUDE-CONSUMER-IMPORT-SWITCH-001
purpose=switch parser Result/Option prelude import to hakorune_frontend_ast
implementation_allowed=import_switch_only
default_feature_change_allowed=0
```

Non-goals:

```text
do_not_change_result_option_prelude_behavior=1
do_not_extract_parser_crate_yet=1
do_not_touch_mir_or_stage1_callers=1
```
