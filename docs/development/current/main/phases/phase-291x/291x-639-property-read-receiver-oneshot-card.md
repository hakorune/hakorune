---
Status: Landed
Date: 2026-04-28
Scope: make unified member property reads reuse the already-lowered receiver
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/fields.rs
  - src/tests/mir_unified_members_property_read.rs
---

# 291x-639: Property Read Receiver One-Shot

## Goal

Keep unified member property reads on a one-shot receiver lowering path.

This is BoxShape cleanup. It does not add syntax, change property registration,
or add a new MIR instruction.

## Evidence

`build_field_access` already lowers `object.field` receiver into `object_value`
to inspect origin/type facts. When the field is a unified member property, it
then re-entered `build_method_call(object_ast, getter, [])`, which can lower the
receiver AST a second time.

For side-effecting receivers, that is a correctness risk. Even for pure
receivers, it makes the field-read path harder to reason about.

## Decision

Property read lowering must reuse the `ValueId` produced by the field-access
receiver lowering. The property path calls the standard method-call emitter with
that receiver value and an empty argument slice.

The field-access owner keeps syntax lowering; property getter registration stays
in `CompilationContext` / `PropertyKind`.

## Boundaries

- Keep property getter names unchanged.
- Keep ordinary stored field reads as `FieldGet`.
- Do not add `PropertyRead` MIR in this card.
- Do not change weak-field semantics.
- Do not widen receiver normalization outside property reads.

## Acceptance

```bash
cargo fmt
cargo test mir_unified_members_property_read --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Extracted property-read lowering from the ordinary `FieldGet` path.
- Reused the already-lowered receiver `ValueId` when lowering a property read
  into a synthetic getter call.
- Kept ordinary stored fields on the existing `FieldGet` path.
- Added a MIR regression test for `(new PropBox()).value`: exactly one
  `NewBox PropBox`, one getter call, and no `FieldGet value`.
