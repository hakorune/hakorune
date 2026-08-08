---
Status: Closed
Date: 2026-08-08
Decision: accepted — behavior-preserving BoxShape split before typed-call growth
Scope: `src/mir/loop_recipe_contract/operation_physical_demand.rs` and `verify.rs`
---

# LOOP-RECIPE-OPERATION-SHAPE-SPLIT-R0

## Current capsule

- `operation_physical_demand.rs` is 781 lines and `verify.rs` is 725 lines.
- The next typed call/value schema must not be added while either file is at
  the 760-line design trigger or near the 800-line hard boundary.
- This is a BoxShape refactor only. It does not add an operation, value class,
  Recipe version, fixture, selector, physical route, or production caller.

## Change

Split each module by responsibility without changing behavior or public
module-facing names. Keep the existing `loop_recipe_contract` facade and
tests stable; use private sibling modules and narrow `pub(super)`/`pub(crate)`
interfaces only where the current implementation already needs them.

Suggested responsibility boundaries:

```text
operation_physical_demand.rs
  demand issuance / ownership / top-level public product
operation_physical_demand_rows.rs
  prepared row, schedule row, read/write row, coverage receipt
operation_physical_demand_schedule.rs
  recursive Recipe traversal and exact schedule construction
operation_physical_demand_projection.rs
  read/write projections and source/effect coverage projection

verify.rs
  verifier facade and artifact entry point
verify_keys.rs
  canonical key/arena checks and lookup helpers
verify_structure.rs
  bindings, values, loop/block/item structure
verify_operations.rs
  operation/value/class checks and carrier/exit checks
```

The exact file names may be reduced if a responsibility is already small;
the invariant is one owner per behavior, not arbitrary line slicing.

## Contract

- Preserve every existing type name, constructor contract, error variant,
  test result, normalized representation, and caller behavior.
- Do not silently change visibility or create a second semantic owner.
- Each resulting Rust source file must remain below 760 lines; 800 lines is a
  hard stop. Do not compress formatting or remove explanations to meet it.
- Keep Recipe keys, selectors, physical IDs, Builder/MIR effects, and source
  authority out of any new observation helper.
- This series may contain 2–5 behavior-neutral commits under Refactor Series
  Mode, but no new accepted shape or fixture may land in the series.

## Verification

1. The focused operation-demand, structural-verifier, and shape-split tests
   remain green. The full `loop_recipe_contract` package currently contains a
   pre-existing red in
   `source_bound_core_rejects_derived_carrier_and_duplicate_effect_mismatch`;
   it reproduces at the pre-split parent commit `e00a374803` and is tracked as
   known baseline debt, not repaired in this BoxShape series.
2. Public re-exports and `operation_physical_demand_tests` compile unchanged.
3. `git diff --check` and `bash tools/checks/current_state_pointer_guard.sh` pass.
4. A line-count check proves every changed Rust source file is `< 760`.
5. No new Recipe schema/operation/value vocabulary appears in the diff.

Baseline-red procedure: if any other red appears, rerun the exact command at
the parent commit before changing this row. A non-reproducing red blocks
closeout; a reproducing red must be recorded with its exact owner and test.

Language reference documentation is not changed by this behavior-neutral
split. The later typed schema/instance-target implementation must update
`docs/reference/mir/loop-recipe-contract.md`, the two module READMEs, and its
focused tests in the same implementation commit.

## Closeout

- Commit: `032db90298`
- Focused evidence: operation-demand 5/5; structural Recipe tests 38/38;
  release `hakorune` build green; line-count, diff, and current-pointer guards
  green.
- Baseline evidence: the one failing source-bound-core assertion reproduces at
  parent `e00a374803`; it remains known baseline debt and is not part of this
  refactor.
- Next blocker: `LOOP-RECIPE-TYPED-SCHEMA-V2-D0`, an explicit design stop for
  the V2 wire/CallSlot and resolver instance-target relation.

## Non-claims and stop

This row does not claim typed `Text`, `Call`, `TextEq`, instance-method target
resolution, `ScanWithInit`, physical emission, production selection, or legacy
deletion. Stop and return to the typed-call design boundary if the split
requires a semantic change, a new operation/value kind, a new public owner,
or a test-only compatibility branch.
