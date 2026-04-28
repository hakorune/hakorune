---
Status: Landed
Date: 2026-04-28
Scope: move MIR property getter storage into a PropertyRegistry box
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/properties.rs
  - src/mir/builder/compilation_context.rs
  - src/mir/builder.rs
---

# 291x-652: Property Registry Box

## Goal

Make MIR property getter registration a real box instead of raw map storage in
`CompilationContext`.

This is BoxShape cleanup. It does not change parser emission, getter names, or
property read lowering behavior.

## Evidence

After the previous registry API cleanup, `properties.rs` owned getter naming
policy, but `CompilationContext` still stored:

```text
HashMap<BoxName, HashMap<PropertyName, PropertyKind>>
```

That kept registry shape and insertion details in the broad compilation context
instead of the property owner module.

## Decision

Introduce `PropertyRegistry` in `src/mir/builder/properties.rs`.

`CompilationContext` now owns a single `property_registry` field and delegates
the read-facing methods:

- `register_property_getter_method(...)`;
- `property_getter_method_name(...)`.

## Boundaries

- Keep `CompilationContext` proxy methods so callers do not fan out to the
  registry directly.
- Do not change `__get_*`, `__get_once_*`, or `__get_birth_*` naming.
- Do not change property read lowering in this card.

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

- Added `PropertyRegistry` as the owner of getter map storage.
- Removed raw property getter map storage from `CompilationContext`.
- Kept existing registration/read-facing context methods as thin proxies.
