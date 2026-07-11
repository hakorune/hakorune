---
Status: Complete
Date: 2026-05-12
Scope: JoinIR generic loop field-read bound acceptance.
Related:
  - src/mir/builder/control_flow/plan/normalizer/helpers_value.rs
  - lang/src/hako_alloc/memory/page_box.hako
  - tools/checks/k2_wide_mimalloc_page_model_guard.sh
---

# 293x-170 JoinIR Loop Field Bound

## Goal

Raise compiler expressivity so a generic loop condition can read an object field
directly as its bound:

```hako
loop(i < me.capacity) {
    ...
}
```

This removes the local temporary workaround from `HakoAllocPageModel` while
keeping the app source idiomatic. The AST is not rewritten; the normalizer now
lowers `me` / `this` as the current receiver value, allowing the existing
`FieldAccess` value path to emit a field get.

## Non-goals

- No new loop shape beyond field-read value lowering.
- No support for field-carried loop updates such as `me.i = me.i + 1`.
- No allocator provider activation, hook, or process allocator replacement.

## Proof

```bash
cargo test -q lower_value_ast_accepts_me_field_access
bash tools/checks/k2_wide_mimalloc_page_model_guard.sh
```

`k2_wide_mimalloc_page_model_guard.sh` now pins `loop(i < me.capacity)` so the
page-model proof continues to exercise the field-read loop bound.
