---
Status: Landed
Date: 2026-04-27
Scope: Prune string-corridor candidate metadata root exports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/mod.rs
  - src/mir/placement_effect.rs
  - src/mir/string_kernel_plan.rs
  - src/mir/passes/string_corridor_sink/mod.rs
  - src/mir/compiler/mod.rs
  - src/mir/exact_seed_backend_route.rs
  - src/mir/verification/string_kernel.rs
  - src/mir/printer.rs
  - src/mir/string_corridor_placement/tests.rs
  - src/runner/mir_json_emit/tests/string_corridor.rs
---

# 291x-533: String-Corridor Candidate Root Export Prune

## Goal

Keep string-corridor candidate vocabulary owned by
`string_corridor_placement` instead of the broad MIR root.

The MIR root should expose refresh entry points for orchestration. Consumers
that construct or inspect candidate, plan, proof, boundary, or publication
contract vocabulary should import the owner module directly.

## Inventory

Removed root exports:

- `StringCorridorCandidate`
- `StringCorridorCandidateKind`
- `StringCorridorCandidatePlan`
- `StringCorridorCandidateProof`
- `StringCorridorCandidateState`
- `StringCorridorPublicationBoundary`
- `StringCorridorPublicationContract`

Migrated consumers:

- `src/mir/placement_effect.rs`
- `src/mir/string_kernel_plan.rs`
- `src/mir/passes/string_corridor_sink/mod.rs`
- `src/mir/compiler/mod.rs`
- `src/mir/exact_seed_backend_route.rs`
- `src/mir/verification/string_kernel.rs`
- `src/mir/printer.rs`
- `src/mir/string_corridor_placement/tests.rs`
- `src/mir/string_kernel_plan/tests.rs`
- `src/runner/mir_json_emit/tests/string_corridor.rs`

Existing owner imports already covered:

- `src/mir/function/types.rs`
- `src/runner/mir_json_emit/root.rs`
- `src/mir/string_corridor_placement` implementation modules.

## Cleaner Boundary

```text
string_corridor_placement
  owns candidate/plan/proof/publication vocabulary

mir root
  exports refresh_function_string_corridor_candidates
  exports refresh_module_string_corridor_candidates
```

## Boundaries

- BoxShape-only.
- Do not change candidate inference, relation carry, or string-kernel plan
  derivation.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not move core string-corridor facts in this card.

## Acceptance

- MIR root no longer re-exports string-corridor candidate vocabulary.
- Consumers use `string_corridor_placement` for candidate vocabulary.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed string-corridor candidate vocabulary from the MIR root export
  surface.
- Kept candidate refresh entry points available at the MIR root.
- Preserved route metadata and JSON output.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
