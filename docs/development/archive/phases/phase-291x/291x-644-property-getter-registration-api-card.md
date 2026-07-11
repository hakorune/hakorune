---
Status: Landed
Date: 2026-04-28
Scope: centralize MIR property getter method-name registration
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/properties.rs
  - src/mir/builder/compilation_context.rs
  - src/mir/builder/decls.rs
---

# 291x-644: Property Getter Registration API

## Goal

Make MIR property getter registration a single intent-level API.

This is BoxShape cleanup. It does not change getter names, property syntax, or
field-read lowering.

## Evidence

After 291x-641, `properties.rs` owned getter-name classification, but
`decls.rs` still performed the classification and then separately called
`register_property_getter(...)`.

That leaves two external steps for one operation:

```text
method name -> maybe property kind/name -> registry insert
```

## Decision

`CompilationContext` exposes `register_property_getter_method(...)`, which takes
the declared method name and registers recognized synthetic property getters.

`decls.rs` does not inspect `PropertyKind` directly.

## Boundaries

- Keep `__get_*`, `__get_once_*`, and `__get_birth_*` names unchanged.
- Keep `property_getter_method_name(...)` as the read-facing query.
- Do not introduce a separate registry type in this card.
- Do not change property read lowering.

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

- Added `CompilationContext::register_property_getter_method(...)`.
- Moved getter method-name classification out of `decls.rs`.
- Made direct property getter registration private to `CompilationContext`.
- Updated the registry unit test to assert the read-facing getter method-name
  contract for computed/once/birth_once getters and a non-getter miss.
