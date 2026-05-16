# 293x-497 EXPRS-COLLECTION-LITERAL-001

Status: selected current
Date: 2026-05-16

## Decision

`EXPRS-COLLECTION-LITERAL-001` is a BoxShape cleanup for MIR builder expression
lowering. It moves collection literal lowering out of `src/mir/builder/exprs.rs`
into a dedicated builder owner.

## Scope

- Add `src/mir/builder/collection_literals.rs`.
- Move ArrayLiteral lowering out of `exprs.rs`.
- Move MapLiteral lowering out of `exprs.rs`.
- Keep `exprs.rs` as the dispatcher/facade for collection literal AST arms.

## Stop Lines

- Do not change ArrayBox/MapBox method names, route certainty, or effect masks.
- Do not change array element inference, type/origin registry writes, or
  constructor birth markers.
- Do not touch indexing/static-data lowering, CheckExpr lowering, parser syntax,
  allocator behavior, provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `COL.1` | Add the `collection_literals` owner module and wire it from `builder.rs`. | build sees the new module. | no behavior change |
| `COL.2` | Move ArrayLiteral lowering. | array focused test is green. | preserve type/origin writes |
| `COL.3` | Move MapLiteral lowering. | map focused test is green. | preserve route/effect masks |
| `COL.4` | Verify and close out. | required evidence is green. | no adjacent refactor |

## Required Evidence

```text
cargo test -q array_value_get_uses_unified_receiver_arg_shape_and_element_return
cargo test -q map_value_set_uses_unified_receiver_arg_shape_and_receipt_string_return
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## Closeout

This row closes when collection literal lowering has a dedicated builder owner
and the accepted ArrayLiteral / MapLiteral behavior is unchanged.
