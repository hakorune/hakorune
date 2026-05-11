---
Status: Complete
Date: 2026-05-12
Scope: usize and pre-mimalloc syntax/spec decision.
Related:
  - docs/reference/language/EBNF.md
  - docs/reference/language/LANGUAGE_REFERENCE_2025.md
  - docs/reference/language/types.md
  - docs/reference/runtime/substrate-capabilities.md
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/README.md
---

# 293x-174 Usize Mimalloc Syntax Decision

## Goal

Fix the pre-port language reading before moving deeper into the mimalloc
`.hako` implementation.

The mimalloc source naturally uses size-like vocabulary. Hakorune already
accepts `usize` as numeric substrate annotation text, but the live runtime lane
is still `Integer(i64)`. Treating `usize` as exact unsigned pointer-sized
semantics now would be misleading.

## Decision

- Keep hako_alloc/mimalloc numeric state fields on `i64` for now.
- Do not migrate counters, capacities, page ids, or sentinel-bearing indexes to
  `usize` until exact pointer-sized unsigned semantics are implemented and
  verified.
- Continue to accept numeric substrate names such as `i64` and `usize` as
  annotation metadata. They currently route through the existing integer lane.
- Keep negative sentinel fields, such as direct-page indexes, signed.
- Treat method and `birth` parameter type annotations as syntax-only metadata
  in AST v0. They do not enforce runtime types yet.
- Treat stored field initializer expressions as per-construction values, not
  static shared defaults.

## Additional Syntax Cleanup

The language reference now mirrors the live assignment surface:

```hako
x = expr
me.field = expr
array[index] = expr
```

Compound assignment remains sugar-gated. Plain assignment remains the
canonical form for allocator policy/state code.

## Non-goals

- No exact-width integer runtime semantics.
- No unsigned overflow/range verifier.
- No `usize` migration in `lang/src/hako_alloc/memory`.
- No constructible `RawArray<T>` surface decision.
- No allocator algorithm advancement.

## Proof

This is a docs/spec alignment row. The existing parser and substrate tests own
the corresponding live behavior:

```bash
cargo test -q parser_accepts_typed_params_and_keeps_param_names_in_ast_v0
cargo test -q parse_type_name_to_mir_maps_numeric_substrate_names_to_integer_lane
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
