---
Status: Landed
Date: 2026-04-28
Scope: thin the MIR unified-member property registry surface
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder.rs
  - src/mir/builder/properties.rs
  - src/mir/builder/compilation_context.rs
  - src/mir/builder/decls.rs
  - src/mir/builder/fields.rs
---

# 291x-641: Property Registry Surface

## Goal

Move unified-member property naming policy out of the root MIR builder file and
keep the registry map behind `CompilationContext` methods.

This is BoxShape cleanup. It does not change property syntax, registration, or
lowered call shape.

## Evidence

`PropertyKind` lived in `builder.rs`, making the root builder module carry a
small policy surface unrelated to orchestration. `CompilationContext` also
exposed `property_getters_by_box` publicly even though all current users go
through registration/query helpers.

## Decision

`builder/properties.rs` owns the MIR-side property kind and getter-name policy.
`CompilationContext` owns the registry map and exposes intent-level helpers:

- register a property getter;
- ask for the getter method name for a property read.

## Boundaries

- Keep `__get_*`, `__get_once_*`, and `__get_birth_*` names unchanged.
- Keep parser-side synthetic method emission unchanged.
- Do not add new MIR instructions.
- Do not change property read semantics.

## Acceptance

```bash
cargo fmt
cargo test test_property_getter_registry --lib
cargo test mir_unified_members_property_read --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Moved MIR-side `PropertyKind` and getter-name parsing/emission into
  `builder/properties.rs`.
- Made `CompilationContext::property_getters_by_box` private.
- Added `property_getter_method_name(...)` so property reads do not need direct
  registry/kind access.
- Kept getter names and lowered call shape unchanged.
