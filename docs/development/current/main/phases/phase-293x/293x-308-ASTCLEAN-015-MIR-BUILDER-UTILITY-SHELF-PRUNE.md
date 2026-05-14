# 293x-308 ASTCLEAN-015 MIR builder utility shelf prune

Status: complete

## Decision

Decision: accepted.

MIR builder utility shelves should not preserve unused wrapper APIs behind `#[allow(dead_code)]`. Live wrappers stay without allowances; unused helper shelves are deleted.

## Scope

- Remove stale allowances from live weak-ref/barrier helper methods.
- Remove unused pinning helper wrappers while keeping `pin_to_slot` and `insert_copy_after_phis` owners.
- Keep the staged block schedule helper shelf with an `ASTCLEAN-015` rationale because it is consumed by selective emit-guard routes, not by the default parser guard build.
- Remove unused call-emission wrapper methods that only delegated to `CallMaterializerBox`.
- Delete the unused `utils/type_ops.rs` helper module and its module declaration.

## Non-goals

- No MIR instruction semantics change.
- No call materializer behavior change.
- No scheduler behavior change.
- No future type-system semantics introduced in Rust.

## Guard

- `tools/checks/k2_wide_astclean_mir_builder_utility_shelf_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_mir_builder_utility_shelf_guard.sh` passed locally.
