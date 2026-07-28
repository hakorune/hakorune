---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / parser syntax sugar and prelude seam preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1166-BUILD-FRONTEND-GRAMMAR-CONSUMER-IMPORT-SWITCH-001.md
---

# BUILD-FRONTEND-PARSER-SYNTAX-PRELUDE-SEAM-PREFLIGHT-001

## Result

Parser crate extraction is still blocked by two small frontend seams.

```text
syntax_sugar_config_lines=95
result_option_prelude_lines=39
parser_direct_syntax_sugar_refs=2
parser_direct_result_option_prelude_refs=1
direct_parser_crate_extraction_allowed=0
behavior_changed=0
implementation_allowed=0
```

## Decision

Split the two seams along their natural owners:

```text
sugar_config_owner=hakorune-frontend-grammar
sugar_config_reason=frontend grammar/config input with no AST dependency
result_option_prelude_owner=hakorune-frontend-ast
result_option_prelude_reason=passive AST enum declaration data
```

Do not create a new crate for either seam yet:

```text
new_syntax_crate_selected=0
new_prelude_crate_selected=0
```

## Next

```text
selected_next_task=BUILD-FRONTEND-SUGAR-CONFIG-PASSIVE-SPLIT-001
purpose=move SugarConfig/SugarLevel into hakorune-frontend-grammar
implementation_allowed=passive_split_only
default_feature_change_allowed=0
```

Follow-up:

```text
followup_task=BUILD-FRONTEND-RESULT-OPTION-PRELUDE-PASSIVE-SPLIT-001
purpose=move passive Result/Option enum declaration construction into hakorune-frontend-ast
```

Non-goals:

```text
do_not_change_sugar_behavior=1
do_not_change_result_option_prelude_behavior=1
do_not_extract_parser_crate_yet=1
```
