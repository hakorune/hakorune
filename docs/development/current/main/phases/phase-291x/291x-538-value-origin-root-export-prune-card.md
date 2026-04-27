---
Status: Landed
Date: 2026-04-27
Scope: Prune value-origin query vocabulary from the MIR root facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - src/mir/mod.rs
  - src/mir/value_origin.rs
---

# 291x-538: Value-Origin Root Export Prune

## Goal

Keep value-origin and copy-parent query vocabulary owned by
`src/mir/value_origin.rs` instead of the broad MIR root facade.

The MIR root facade may expose core MIR model types and refresh orchestration
entry points, but provenance/query helper vocabulary should stay on its owner
module. This keeps route planners and passes explicit about consuming generic
origin queries.

## Inventory

Removed root exports:

- `build_value_def_map`
- `resolve_value_origin`
- `resolve_value_origin_from_copy_parents`
- `resolve_value_origin_from_parent_map`
- `CopyParentMap`
- `ParentMap`
- `ValueDefMap`

Migrated representative consumers to `value_origin` owner paths:

- route/contract planners under `src/mir/array_*`
- generic method route facts and plans
- map lookup fusion plan
- string corridor relation, recognizer, placement, and kernel plan
- sum placement and placement/effect seams
- DCE / escape / simplify-cfg passes

## Cleaner Boundary

```text
value_origin
  owns copy-chain and parent-map normalization queries

mir root
  does not re-export provenance/query helper vocabulary
```

Consumers that need origin resolution should import from one of:

```rust
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin};
use super::value_origin::{ParentMap, ValueDefMap};
```

## Boundaries

- BoxShape-only.
- Do not change copy-chain traversal behavior.
- Do not change parent-map cycle handling.
- Do not change route metadata, JSON field names, helper symbols, or lowering.
- Do not add a replacement root facade export for value-origin vocabulary.

## Acceptance

- MIR root no longer re-exports value-origin query vocabulary.
- Consumers use `value_origin` owner paths for query helpers and maps.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed value-origin vocabulary from the MIR root export surface.
- Preserved origin resolution behavior and all route metadata behavior.
- Aligned the root facade with the 291x-537 contract.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
