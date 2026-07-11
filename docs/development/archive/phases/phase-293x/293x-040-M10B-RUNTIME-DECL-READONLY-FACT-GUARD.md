---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-040-M10B-RUNTIME-DECL-READONLY-FACT-GUARD
Scope: M10b runtime-decl readonly fact verifier
---

# 293x-040 M10b Runtime-Decl Readonly Fact Guard

## Decision

Before widening strong LLVM attrs, the existing weak runtime-decl attrs must
be internally consistent.

This card adds a narrow verifier rule:

```text
if attrs contains readonly:
  memory must be "read"
```

Conservative omission remains allowed. A row may declare `memory = "read"`
without exporting `readonly`.

## Responsibility

- `docs/development/current/main/design/runtime-decl-manifest-v0.toml` owns
  backend-private runtime-decl memory facts.
- `tools/checks/k2_wide_export_attrs_consistency_guard.sh` owns the fail-fast
  consistency check.
- `.hako` runtime declare emission remains a reader of manifest facts; it does
  not rediscover helper semantics.

## Non-Goals

- No `noalias` / `nonnull` / `dereferenceable` emission.
- No backend alignment export.
- No `readnone` promotion.
- No implementation-body semantic proof.
- No symbol-name-based inference.

## Acceptance

- every manifest row with `readonly` has `memory = "read"`.
- rows without `readonly` are allowed even when memory is read-only.
- strong attrs remain blocked by the M10a guard.

## Gates

```bash
bash tools/checks/k2_wide_export_attrs_consistency_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
