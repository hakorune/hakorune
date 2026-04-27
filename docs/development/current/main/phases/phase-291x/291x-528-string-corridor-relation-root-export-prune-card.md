---
Status: Landed
Date: 2026-04-27
Scope: Prune string-corridor relation root exports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/mod.rs
  - src/mir/passes/string_corridor_sink/shared.rs
  - src/mir/printer.rs
  - src/runner/mir_json_emit/tests/string_corridor.rs
---

# 291x-528: String-Corridor Relation Root Export Prune

## Goal

Keep string-corridor relation vocabulary owned by `string_corridor_relation`
instead of the broad MIR root.

Relation vocabulary is semantic metadata. The MIR root can keep relation refresh
entry points, while consumers that inspect or construct relations should import
the owner module.

## Inventory

Removed root exports:

- `StringCorridorRelation`
- `StringCorridorRelationKind`
- `StringCorridorWindowContract`

Migrated consumers:

- `src/mir/passes/string_corridor_sink/shared.rs`
- `src/mir/printer.rs`
- `src/runner/mir_json_emit/tests/string_corridor.rs`

Existing owner imports already covered:

- `src/mir/function/types.rs`
- `src/mir/string_corridor_placement/mod.rs`
- `src/mir/string_corridor_placement/relation_carry.rs`

## Cleaner Boundary

```text
string_corridor_relation
  owns StringCorridorRelation* vocabulary

mir root
  exports refresh_function_string_corridor_relations
  exports refresh_module_string_corridor_relations
```

## Boundaries

- BoxShape-only.
- Do not change relation detection.
- Do not change string-corridor sink behavior.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports `StringCorridorRelation*` vocabulary.
- Internal consumers use `crate::mir::string_corridor_relation`.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed relation semantic vocabulary from the MIR root export surface.
- Kept relation refresh entry points available at the MIR root.
- Preserved relation behavior and JSON output.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
