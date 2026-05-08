---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-041-M11A-STATIC-READONLY-DATA-SEGMENT
Scope: M11a backend-private static readonly data segment
---

# 293x-041 M11a Static Readonly Data Segment

## Decision

M11 is split into:

```text
M11a:
  backend-private static readonly data segment

M11b:
  source-level static const / const eval / const fn
```

This card lands M11a only. It does not add source syntax.

## Responsibility

- `docs/development/current/main/design/static-data-manifest-v0.toml` owns
  backend-private static data rows.
- `tools/backend_static_data_manifest_codegen.py` owns generated `.hako`
  defaults.
- `StaticDataRegistryBox` owns manifest consumption and LLVM global-line
  rendering.
- `LlTextEmitBox` only appends registry-rendered globals.

## Live Row

```text
@.hako_size_class_u16_v0 =
  private unnamed_addr constant [64 x i16] [...], align 2
```

The row is a mimalloc size-class fixture for static data plumbing. It is not
yet the semantic allocator policy owner.

## Non-Goals

- No `static const` parser syntax.
- No const eval.
- No const fn.
- No source-level size-class table declaration.
- No runtime `ArrayBox` / `MapBox` construction for the emitted target table.
- No C shim or `.inc` size-class special case.

## Acceptance

- static-data manifest codegen is drift-checked.
- generated `.hako` defaults expose the size-class fixture row.
- `.hako ll emitter` reads `StaticDataRegistryBox.emit_globals()`.
- guard verifies the rendered LLVM global shape.

## Gates

```bash
bash tools/checks/k2_wide_static_data_first_row_guard.sh
python3 tools/backend_static_data_manifest_codegen.py --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
