---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / build gate explain report passive split.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1199-BUILD-FRONTEND-PARSER-TOKENIZER-POST-TOKENIZER-MOVE-PREFLIGHT-001.md
---

# BUILD-FRONTEND-BUILD-GATE-REPORT-PASSIVE-SPLIT-001

## Result

Moved `BuildGateExplainReport` into `hakorune-frontend-parser`:

```text
new_owner=crates/hakorune_frontend_parser/src/parser/build_cfg.rs
compat_facade=src/parser/build_cfg.rs
type_moved=BuildGateExplainReport
active_build_cfg_predicate_moved=0
active_build_cfg_prune_moved=0
NyashParser_moved=0
behavior_changed=0
```

## Verification

```text
cargo_check_q=green
cargo_test_hakorune_frontend_parser=green
```

## Next

```text
selected_next_task=BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-001
purpose=choose next parser split boundary after tokenizer and build-gate report movement
implementation_allowed=preflight_only
```
