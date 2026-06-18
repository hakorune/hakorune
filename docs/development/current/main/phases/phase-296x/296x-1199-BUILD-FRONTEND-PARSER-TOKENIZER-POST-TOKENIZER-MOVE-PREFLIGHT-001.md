---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / post-tokenizer move parser-tokenizer preflight.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1198-BUILD-FRONTEND-TOKENIZER-OWNER-BUNDLE-MOVE-001.md
---

# BUILD-FRONTEND-PARSER-TOKENIZER-POST-TOKENIZER-MOVE-PREFLIGHT-001

## Result

Tokenizer implementation is now owned by `hakorune-frontend-parser`. The next
safe parser-side split is a passive report type:

```text
selected_family=build_gate_explain_report
selected_type=BuildGateExplainReport
selected_source=src/parser/build_cfg.rs
selected_destination=crates/hakorune_frontend_parser/src/parser/build_cfg.rs
line_count=report_only
runtime_refs=0
NyashParser_owner_required=0
```

Reason:

```text
BuildGateExplainReport_is_passive_data=1
BuildGateExplainReport_has_no_AST_or_runtime_dependency=1
predicate_prune_impls_stay_main_crate=1
```

Do not move `build_cfg/predicate.rs` or `build_cfg/prune.rs` in this row. Those
modules are active `NyashParser` impls and require the parser owner bundle.

## Decision

Move only `BuildGateExplainReport` to the frontend parser crate and keep
`src/parser/build_cfg.rs` as the historical main-crate facade plus active
submodule owner.

```text
selected_next_task=BUILD-FRONTEND-BUILD-GATE-REPORT-PASSIVE-SPLIT-001
implementation_allowed=1
active_parser_impl_move_allowed=0
```

Non-goals:

```text
do_not_move_NyashParser=1
do_not_move_build_cfg_predicate=1
do_not_move_build_cfg_prune=1
do_not_change_build_gate_behavior=1
```
