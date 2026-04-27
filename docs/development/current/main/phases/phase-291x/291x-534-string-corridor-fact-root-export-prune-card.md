---
Status: Landed
Date: 2026-04-27
Scope: Prune string-corridor fact metadata root exports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/mod.rs
  - src/mir/placement_effect.rs
  - src/mir/string_kernel_plan.rs
  - src/mir/string_corridor_compat.rs
  - src/mir/passes/string_corridor_sink/mod.rs
  - src/mir/passes/string_corridor_sink/shared.rs
  - src/mir/passes/string_corridor_sink/publication.rs
  - src/mir/compiler/mod.rs
  - src/mir/verification/string_kernel.rs
  - src/mir/printer.rs
  - src/mir/string_corridor_placement/tests.rs
  - src/mir/string_kernel_plan/tests.rs
  - src/runner/mir_json_emit/tests/string_corridor.rs
  - src/runner/mir_json_emit/tests/placement.rs
---

# 291x-534: String-Corridor Fact Root Export Prune

## Goal

Keep core string-corridor fact vocabulary owned by `string_corridor` instead
of the broad MIR root.

The MIR root should expose refresh entry points for orchestration. Consumers
that construct or inspect string-corridor facts, publish policies, placement
facts, or stable-view provenance should import `string_corridor` directly.

## Inventory

Removed root exports:

- `StringCorridorBorrowContract`
- `StringCorridorCarrier`
- `StringCorridorFact`
- `StringCorridorOp`
- `StringCorridorRole`
- `StringOutcomeFact`
- `StringPlacementFact`
- `StringPublishReason`
- `StringPublishReprPolicy`
- `StringStableViewProvenance`

Migrated consumers:

- `src/mir/placement_effect.rs`
- `src/mir/string_kernel_plan.rs`
- `src/mir/string_corridor_compat.rs`
- `src/mir/passes/string_corridor_sink/**`
- `src/mir/compiler/mod.rs`
- `src/mir/verification/string_kernel.rs`
- `src/mir/printer.rs`
- `src/mir/string_corridor_placement/tests.rs`
- `src/mir/string_kernel_plan/tests.rs`
- `src/runner/mir_json_emit/tests/string_corridor.rs`
- `src/runner/mir_json_emit/tests/placement.rs`

Existing owner imports already covered:

- `src/mir/function/types.rs`
- `src/mir/string_corridor_placement` implementation modules.

## Cleaner Boundary

```text
string_corridor
  owns core fact/publish/provenance vocabulary

string_corridor_placement
  owns candidate/plan/proof/publication vocabulary

mir root
  exports refresh_function_string_corridor_facts
  exports refresh_module_string_corridor_facts
```

## Boundaries

- BoxShape-only.
- Do not change fact inference, candidate inference, sink transforms, or
  string-kernel plan derivation.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports core string-corridor fact vocabulary.
- Consumers use `string_corridor` for fact/publish/provenance vocabulary.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed core string-corridor vocabulary from the MIR root export surface.
- Kept string-corridor fact refresh entry points available at the MIR root.
- Preserved route metadata and JSON output.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
