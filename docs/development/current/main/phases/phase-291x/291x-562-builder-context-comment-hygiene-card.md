---
Status: Landed
Date: 2026-04-28
Scope: Refresh builder context-helper comments to current SSOT wording
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/builder_debug.rs
  - src/mir/builder/builder_init.rs
  - src/mir/builder/builder_metadata.rs
  - src/mir/builder/vars/lexical_scope.rs
---

# 291x-562: Builder Context Comment Hygiene

## Goal

Remove stale legacy wording from builder context-helper comments.

The affected helpers already delegate to the current context owners. Comments
should state the current SSOT instead of describing removed legacy fields or
legacy synchronization.

## Inventory

Updated comments in:

- `src/mir/builder/builder_metadata.rs`
- `src/mir/builder/builder_debug.rs`
- `src/mir/builder/builder_init.rs`
- `src/mir/builder/vars/lexical_scope.rs`

## Cleaner Boundary

```text
metadata_ctx
  hint/source metadata SSOT

scope_ctx
  debug-region, if-merge, and lexical-scope stack SSOT

binding_ctx
  binding-id SSOT
```

## Boundaries

- BoxShape/docs-in-code only.
- Do not change builder behavior.
- Do not change context ownership.
- Do not remove current direct context field access comments on `MirBuilder`
  struct fields in this card.

## Acceptance

- No `legacy field removed` or `legacy sync` wording remains in live builder
  Rust files.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Replaced stale legacy-removal comments with current context SSOT wording.
- Kept behavior unchanged.

## Verification

```bash
rg -n "legacy field removed|legacy sync" src/mir/builder -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check -q
cargo fmt -- --check
git diff --check
```
