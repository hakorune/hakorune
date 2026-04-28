---
Status: Landed
Date: 2026-04-28
Scope: extract MIR property read lowering from generic field lowering
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/property_reads.rs
  - src/mir/builder/fields.rs
  - src/mir/builder.rs
---

# 291x-653: Property Read Lowering Box

## Goal

Separate property read lowering from ordinary field access lowering.

This is BoxShape cleanup. It does not change `FieldGet`, getter call names, or
receiver reuse behavior.

## Evidence

After property registry cleanup, `fields.rs` still owned both:

- ordinary `object.field` slot load lowering;
- unified-member property read routing to synthetic getter calls.

That mixed two different decisions in the same field helper, even though
property reads are intentionally not a `FieldGet`.

## Decision

Move `try_lower_property_read(...)` into a new
`src/mir/builder/property_reads.rs` module.

`fields.rs` still calls it before emitting `FieldGet`, preserving the existing
preemption contract:

```text
property read -> getter call
ordinary field read -> FieldGet
```

## Boundaries

- Do not change receiver lowering; reuse the already-lowered receiver ValueId.
- Do not change the property registry API.
- Do not change weak field read behavior.

## Acceptance

```bash
cargo fmt
cargo test mir_unified_members_property_read --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Added a `property_reads.rs` lowering box.
- Removed property read routing code from `fields.rs`.
- Preserved existing MIR property-read regressions.
