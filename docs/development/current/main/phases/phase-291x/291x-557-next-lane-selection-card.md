---
Status: Landed
Date: 2026-04-28
Scope: next compiler-cleanliness lane selection after LoopPatternKind alias prune
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-556-loop-pattern-kind-alias-prune-card.md
  - src/mir/builder.rs
  - src/mir/builder/builder_calls.rs
  - src/mir/builder/calls/call_target.rs
---

# 291x-557: Next Lane Selection

## Goal

Choose the next small compiler-cleanliness lane after the route detector alias
cleanup landed.

This card is lane selection only. No behavior changed.

## Candidate Review

| Candidate | Current shape | Decision |
| --- | --- | --- |
| `builder_calls::CallTarget` compatibility path | `CallTarget` owner is `builder/calls/call_target.rs`, but several callsites still import it through `builder_calls` | select next |
| direct `MirBuilder` context-field compatibility comments | broader builder context migration; not just import hygiene | defer |
| route-shape wrapper API cleanup | public wrapper names and tests need a larger API decision | defer |
| Stage-A/runtime compat lane | runner/selfhost compatibility policy | defer; different layer |
| CoreMethodContract -> CoreOp / LoweringPlan | semantic contract lane | defer; not BoxShape cleanup |

## Decision

Select **`CallTarget` owner-path migration** as the next lane.

Reason:

- `CallTarget` already has a clear owner module:
  `src/mir/builder/calls/call_target.rs`.
- `src/mir/builder/builder_calls.rs` is a compatibility shell and should not
  keep re-exporting active vocabulary.
- The live callers are limited and can migrate to `crate::mir::builder::CallTarget`
  / local builder owner paths without changing behavior.
- A small guard can prevent `builder_calls::CallTarget` from regrowing.

## Next Card

Create `291x-558-calltarget-owner-path-migration` before editing code.

Planned change:

```text
src/mir/builder.rs
  pub(crate) use calls::CallTarget

src/mir/builder/builder_calls.rs
  remove CallTarget re-export

builder callsites
  builder_calls::CallTarget -> CallTarget owner path
```

## Acceptance

```bash
rg -n "builder_calls::CallTarget|pub use super::calls::call_target::CallTarget" src/mir/builder -g'*.rs'
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
