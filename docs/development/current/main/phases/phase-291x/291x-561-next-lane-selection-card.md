---
Status: Landed
Date: 2026-04-28
Scope: next compiler-cleanliness lane selection after builder_calls shell closeout
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-560-builder-calls-shell-closeout-card.md
  - src/mir/builder/builder_debug.rs
  - src/mir/builder/builder_init.rs
  - src/mir/builder/builder_metadata.rs
  - src/mir/builder/vars/lexical_scope.rs
---

# 291x-561: Next Lane Selection

## Goal

Choose the next small compiler-cleanliness lane after the `builder_calls`
compatibility shell closeout landed.

This card is lane selection only. No behavior changed.

## Candidate Review

| Candidate | Current shape | Decision |
| --- | --- | --- |
| builder context-helper stale comments | helpers already use `core_ctx` / `scope_ctx` / `binding_ctx` / `metadata_ctx` as SSOT, but comments still say "legacy sync" or "legacy field removed" | select next |
| direct `MirBuilder` context-field compatibility comments | struct fields are still intentionally exposed to builder submodules | defer; not comment-only |
| route-shape wrapper API cleanup | public wrapper names and tests need a larger API decision | defer |
| Stage-A/runtime compat lane | runner/selfhost compatibility policy | defer; different layer |
| CoreMethodContract -> CoreOp / LoweringPlan | semantic contract lane | defer; not BoxShape cleanup |

## Decision

Select **builder context-helper comment hygiene** as the next lane.

Reason:

- The code already uses context owner fields as SSOT.
- The stale wording suggests compatibility machinery that no longer exists.
- This is a small BoxShape/docs-in-code cleanup and should not affect behavior.

## Next Card

Create `291x-562-builder-context-comment-hygiene` before editing code.

Planned change:

```text
builder_metadata.rs
  "legacy sync" -> metadata_ctx SSOT wording

builder_debug.rs / builder_init.rs / vars/lexical_scope.rs
  remove "legacy field removed" wording from current helper comments
```

## Acceptance

```bash
rg -n "legacy field removed|legacy sync" src/mir/builder -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check -q
cargo fmt -- --check
git diff --check
```
