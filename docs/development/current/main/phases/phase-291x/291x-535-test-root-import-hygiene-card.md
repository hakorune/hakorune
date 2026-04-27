---
Status: Landed
Date: 2026-04-27
Scope: Test import hygiene after MIR root export pruning
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/string_kernel_plan/tests.rs
  - src/mir/sum_placement_layout.rs
  - src/mir/string_corridor_placement/tests.rs
---

# 291x-535: Test Root Import Hygiene

## Goal

Remove test-only imports that hide semantic owner boundaries after the MIR root
export pruning wave.

The root should remain an orchestration facade and core MIR convenience surface,
not a way for tests to regain broad access to pruned semantic metadata
vocabulary.

## Inventory

Cleaned imports:

- `src/mir/string_kernel_plan/tests.rs`
  - Removed the remaining exact `use crate::mir::*;`.
  - Replaced it with explicit core MIR imports plus `super` imports for
    string-kernel plan vocabulary.
  - Kept string-corridor vocabulary imported from owner modules.
- `src/mir/sum_placement_layout.rs`
  - Replaced test imports of `SumPlacementSelection` and `ThinEntrySurface`
    from the MIR root with owner-module imports.
- `src/mir/string_corridor_placement/tests.rs`
  - Added the explicit `StringCorridorCarrier` owner import required by test
    compilation after the prior root export prune.

## Cleaner Boundary

```text
tests
  import core MIR types explicitly
  import semantic metadata vocabulary from owner modules

mir root
  does not provide wildcard fallback for pruned metadata vocabulary
```

## Boundaries

- BoxShape-only.
- Do not change string-kernel plan derivation.
- Do not change sum-placement layout derivation.
- Do not change production imports or root refresh entry points.
- Do not change JSON field names or lowering behavior.

## Acceptance

- `rg -n "use crate::mir::\\*;" src` has no matches.
- `src/mir/string_kernel_plan/tests.rs` no longer uses the MIR root wildcard.
- `src/mir/sum_placement_layout.rs` tests import semantic vocabulary from owner
  modules.
- `cargo test --no-run -q` passes.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed the final exact `use crate::mir::*;` under `src`.
- Kept pruned semantic metadata vocabulary on owner-module import paths in
  tests.

## Verification

```bash
rg -n "use crate::mir::\\*;" src
cargo test --no-run -q
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
