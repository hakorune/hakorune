---
Status: Landed
Date: 2026-04-27
Scope: Prune MIR query re-exports from the MIR root facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - src/mir/mod.rs
  - src/mir/query.rs
---

# 291x-545: MIR Query Root Export Prune

## Goal

Keep `MirQuery` and `MirQueryBox` on the `mir::query` owner module instead of
re-exporting them through the broad MIR root facade.

`MirQuery` is a view API over MIR functions. It is useful infrastructure, but
callers should name the query owner module rather than treating it as core MIR
model vocabulary.

## Inventory

Removed root exports:

- `MirQuery`
- `MirQueryBox`

Migrated root-path consumers:

- `src/mir/join_ir/lowering/loop_form_intake.rs`
- `src/mir/join_ir/lowering/skip_ws.rs`
- `src/mir/join_ir/lowering/loop_scope_shape/builder.rs`
- `src/mir/join_ir/lowering/loop_scope_shape/tests.rs`

Existing owner-path consumers already used `crate::mir::query`.

## Cleaner Boundary

```text
mir::query
  owns MIR read/write/CFG view traits and boxes

mir root
  does not re-export query-view infrastructure
```

## Boundaries

- BoxShape-only.
- Do not change query behavior.
- Do not change JoinIR lowering behavior.
- Do not change LoopForm intake semantics.

## Acceptance

- MIR root no longer re-exports `MirQuery` or `MirQueryBox`.
- Consumers use `mir::query` owner paths.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed query-view infrastructure from the MIR root export surface.
- Preserved existing JoinIR lowering and query behavior.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
