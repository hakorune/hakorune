---
Status: Landed
Date: 2026-04-24
Scope: Publish `MapBox.get(existing-key)` result types conservatively after the missing-key contract and compat/source cleanup landed.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-101-mapbox-get-missing-key-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-106-arraybox-element-result-publication-card.md
  - src/mir/builder/types/map_value.rs
  - src/tests/mir_corebox_router_unified.rs
---

# MapBox get(existing-key) Typing Card

## Decision

Follow the `291x-106` ArrayBox precedent conservatively, but keep this MapBox row
literal-key aware:

```text
receiver-local homogeneous Map value facts
  + tracked literal key
  -> publish stored V for MapBox.get("literal")

mixed/untyped receiver facts
missing-key reads
non-literal key reads
  -> keep Unknown
```

This card does not change the landed missing-key text contract. It only narrows
successful `get(existing-key)` results when the builder already has enough
receiver-local evidence to do so safely.

## Current Facts

- `MapBox.get` is already on the Unified value path.
- `291x-101` landed the missing-key contract:
  `[map/missing] Key not found: <key>`.
- `291x-106` landed the ArrayBox element-result publication rule:
  publish `T` only for known `Array<T>` receivers; keep `Unknown` otherwise.
- pre-291x-110 `MapBox.get` intentionally kept `Unknown` for all successful reads
  because stored-value publication is data-dependent.

## Implementation Slice

- add receiver-local MapBox value facts in the MIR builder type context
- observe `MapBox.set(...)` writes with literal keys and concrete value types
- invalidate those facts conservatively on mixed writes and on destructive writes
  (`delete` / `clear`)
- annotate `MapBox.get("literal")` only when both the receiver and the literal
  key fact prove a publishable stored value type
- pin the rule with focused MIR tests for:
  - typed existing-key publication
  - missing-key remaining `Unknown`
  - mixed-value receivers remaining `Unknown`

## Non-Goals

- do not change the missing-key text contract
- do not publish for dynamic or non-literal keys
- do not add union or per-branch Map value typing
- do not reopen compat/source cleanup or router policy
- do not widen this into a general Map dataflow system

## Acceptance

```bash
cargo test --release map_value_get_existing_key_uses_unified_receiver_arg_shape_and_stored_value_return -- --nocapture
cargo test --release map_value_get_missing_key_stays_unknown_after_typed_write -- --nocapture
cargo test --release map_value_get_mixed_value_results_stay_unknown -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_get_missing_vm.sh
```

## Exit Condition

The repo has one explicit contract for `MapBox.get`:

- missing-key reads keep the landed tagged-text contract
- successful literal-key reads publish `V` only when receiver-local homogeneous
  Map facts prove that result safely
- mixed, untyped, or non-literal cases stay `Unknown`
