---
Status: Landed
Date: 2026-04-24
Scope: Clarify alias ownership across using resolution, imported static-box binding, and static receiver/type-name lowering without changing runtime behavior.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/reference/language/using.md
  - hako.toml
  - src/runner/modes/common_util/resolve/strip/using.rs
  - src/using/simple_registry.rs
  - src/mir/builder/calls/static_resolution.rs
---

# Alias SSOT Cleanup Card

## Decision

Keep alias ownership split into three explicit layers:

```text
Manifest alias / module name
  -> hako.toml ([using], [using.aliases], [module_roots], [modules])

Imported static-box alias binding
  -> runner-side text-merge strip/import flow
  -> alias -> concrete exported static box name

Static receiver / type-name lowering
  -> MIR builder static call resolution only
  -> Alias.method(...)
```

This card is clarity cleanup only. It does not change using syntax, String behavior,
or static-call execution semantics.

## Alias Contract

- `apps.std.string` is an exact manifest alias in `hako.toml`, not a semantic
  owner and not a type name.
- `apps/std/string.hako` currently exports one public static box: `StdStringNy`.
- `using apps.std.string as S` therefore resolves in two steps:
  1. manifest alias `apps.std.string` -> `apps/std/string.hako`
  2. imported static-box alias `S` -> `StdStringNy`
- The imported alias is valid for static calls such as `S.string_length(...)`
  after `using` lines have been stripped by text merge.
- That imported alias does **not** create a namespace root and does **not**
  imply support for `new S.BoxName()` or `new apps.std.string.BoxName()`.
- When a target file exports multiple static boxes, alias binding is only
  accepted when the alias itself matches one exported box name; otherwise the
  runner stays explicit/fail-fast.

## Implementation Slice

- document the three-layer split in the phase front and language reference
- align resolve/using comments with the current `hako.toml` + text-merge flow
- pin the imported static-box alias contract with focused unit tests
- move current pointers from `alias SSOT cleanup pending` to the next cleanup row

## Non-Goals

- do not widen `using` into general namespace/type-root semantics
- do not add `new Alias.BoxName()` or nested alias access
- do not reopen String semantic ownership
- do not change plugin alias policy or module-root lookup behavior
- do not change `MapBox.get(existing-key)` typing in this card

## Acceptance

```bash
cargo test -q resolve_imported_static_box
bash tools/checks/current_state_pointer_guard.sh
./target/release/hakorune --backend vm apps/smokes/std/string_smoke.hako
```

## Exit Condition

The repo has one visible alias story:
manifest alias resolution in `hako.toml`, imported static-box binding in the
runner text-merge path, and static receiver/type-name lowering only in the MIR
builder.
