# 291x-759 CoreLoop StepMode Source Pointer Sync Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `coreloop-stepmode-inline-in-body-ssot.md`
- `CURRENT_STATE.toml`

## Why

The StepMode SSOT still pointed at the retired
`normalizer/simple_while_coreloop_builder.rs` as the source for the
`loop_simple_while explicit-step` pin. That file was removed in 291x-754.

## Decision

Keep the StepMode semantics intact and update only the source pointer.

The active loop-simple-while path is recipe-first:

- `recipe_tree/loop_simple_while_builder.rs`
- `recipe_tree/loop_simple_while_composer.rs`

The legacy fixture/gate token remains documented through the legacy pin
inventory SSOT.

## Landed

- Replaced the deleted normalizer source pointer with active recipe-tree source
  pointers.
- Added a retired-source note for the old builder path.
- Advanced `CURRENT_STATE.toml` to this card.

## Proof

- `rg -n "Source: .*normalizer/simple_while_coreloop_builder\\.rs" docs/development/current/main/design/coreloop-stepmode-inline-in-body-ssot.md`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
