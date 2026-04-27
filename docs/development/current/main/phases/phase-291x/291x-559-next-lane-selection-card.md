---
Status: Landed
Date: 2026-04-28
Scope: next compiler-cleanliness lane selection after CallTarget owner-path migration
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-558-calltarget-owner-path-migration-card.md
  - src/mir/builder.rs
  - src/mir/builder/builder_calls.rs
  - src/mir/builder/calls/utils.rs
  - tools/checks/mir_builder_calltarget_owner_guard.sh
---

# 291x-559: Next Lane Selection

## Goal

Choose the next small compiler-cleanliness lane after the `CallTarget`
owner-path migration landed.

This card is lane selection only. No behavior changed.

## Candidate Review

| Candidate | Current shape | Decision |
| --- | --- | --- |
| `builder_calls.rs` compatibility shell closeout | file now only forwards `parse_type_name_to_mir` and `extract_string_literal` to `calls/utils.rs` | select next |
| direct `MirBuilder` context-field compatibility comments | broader builder context migration | defer |
| route-shape wrapper API cleanup | public wrapper names and tests need a larger API decision | defer |
| Stage-A/runtime compat lane | runner/selfhost compatibility policy | defer; different layer |
| CoreMethodContract -> CoreOp / LoweringPlan | semantic contract lane | defer; not BoxShape cleanup |

## Decision

Select **`builder_calls.rs` compatibility shell closeout** as the next lane.

Reason:

- The call-system owner modules already exist under `src/mir/builder/calls/`.
- `builder_calls.rs` no longer owns active call vocabulary after `291x-558`.
- Its remaining associated helpers can live in `calls/utils.rs` beside the free
  utility functions they delegate to.
- Removing the module eliminates a compatibility barrel and simplifies the
  builder module map.

## Next Card

Create `291x-560-builder-calls-shell-closeout` before editing code.

Planned change:

```text
src/mir/builder/calls/utils.rs
  own MirBuilder::parse_type_name_to_mir
  own MirBuilder::extract_string_literal

src/mir/builder.rs
  remove mod builder_calls

src/mir/builder/builder_calls.rs
  delete

tools/checks/mir_builder_calltarget_owner_guard.sh
  reject builder_calls module/file regrowth
```

## Acceptance

```bash
rg -n "builder_calls" src/mir/builder -g'*.rs'
bash tools/checks/mir_builder_calltarget_owner_guard.sh
bash tools/checks/current_state_pointer_guard.sh
cargo check -q
cargo fmt -- --check
git diff --check
```
