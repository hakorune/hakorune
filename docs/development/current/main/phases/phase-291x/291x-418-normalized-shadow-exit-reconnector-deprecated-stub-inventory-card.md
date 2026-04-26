---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow exit-reconnector deprecated stub inventory
Related:
  - src/mir/control_tree/normalized_shadow/exit_reconnector.rs
  - src/mir/control_tree/normalized_shadow/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-417-normalized-shadow-post-if-fallback-wording-cleanup-card.md
---

# 291x-418: Normalized-Shadow Exit-Reconnector Deprecated Stub Inventory

## Goal

Pick the next small compiler-cleanliness seam after post-if fallback wording
cleanup.

This is a BoxShape inventory. No behavior changed.

## Findings

`exit_reconnector.rs` still exposes a deprecated stub:

```text
ExitReconnectorBox::extract_k_exit_jump_args(...)
```

The stub always returns `None`, is marked `#[allow(dead_code)]`, and points to
`MergeResult.remapped_exit_values` as the current SSOT. Repository search found
no live callers outside the defining module.

The commented old implementation next to the stub is also stale narrative
surface.

## Decision

Remove only the deprecated extraction stub and the commented old implementation.

Do not change:

- `ExitReconnectorBox::reconnect(...)`
- tests for direct variable-map reconnection
- module export shape
- `MergeResult.remapped_exit_values` ownership

## Next Cleanup

`291x-419`: normalized-shadow exit-reconnector deprecated stub cleanup.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "extract_k_exit_jump_args|Deprecated - boundary approach|extract_k_exit_jump_args_old" \
  src/mir/control_tree/normalized_shadow/exit_reconnector.rs
```

The final `rg` should produce no output.
