---
Status: Landed
Date: 2026-04-27
Scope: Prune escape-barrier vocabulary root exports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/mod.rs
---

# 291x-529: Escape-Barrier Root Export Prune

## Goal

Keep escape-barrier vocabulary owned by `escape_barrier` instead of the broad
MIR root.

The MIR root only needs the `classify_escape_uses` entry point. The returned
vocabulary remains available through `crate::mir::escape_barrier`.

## Inventory

Removed root exports:

- `EscapeBarrier`
- `EscapeUse`

Kept root export:

- `classify_escape_uses`

Current external root-path consumers:

- None.

## Cleaner Boundary

```text
escape_barrier
  owns EscapeBarrier / EscapeUse

mir root
  exports classify_escape_uses only
```

## Boundaries

- BoxShape-only.
- Do not change escape classification.
- Do not change public module visibility.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports `EscapeBarrier`.
- MIR root no longer re-exports `EscapeUse`.
- `classify_escape_uses` remains available at the MIR root.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed unused root-level convenience exports for escape-barrier vocabulary.
- Preserved the root entry point and classification behavior.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
