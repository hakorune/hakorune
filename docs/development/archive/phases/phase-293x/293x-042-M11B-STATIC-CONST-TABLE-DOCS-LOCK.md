---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-042-M11B-STATIC-CONST-TABLE-DOCS-LOCK
Scope: M11b docs-first static const table boundary
---

# 293x-042 M11b Static Const Table Docs Lock

## Decision

Before implementing M11b, the source/static-data boundary is fixed as a
docs-first contract.

M11b is split into:

```text
M11b-decl:
  source static const table declaration

M11b-load:
  static table read route

M11b-eval:
  const expression / const fn evaluation
```

Only `M11b-decl` is the next implementation target.

## First Source Shape

Reserved first accepted shape:

```hako
static const SIZE_CLASS: u16[] = [
  8, 16, 24, 32,
]
```

This is not live yet. The implementation card must update both parser fronts
or add explicit fail-fast behavior for the not-yet-active front.

## Responsibility

- Source parsers own syntax acceptance.
- MIR/module metadata owns `static_data_plans`.
- Backend emitters read static data rows.
- Runtime must not allocate `ArrayBox` / `MapBox` for fixed static tables.
- `.inc` / C shim must not infer allocator table meaning from symbol names.

## Updated Docs

- `docs/development/current/main/design/static-const-table-syntax-ssot.md`
- `docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md`
- `docs/development/current/main/design/substrate-capability-ladder-ssot.md`
- `docs/reference/runtime/substrate-capabilities.md`
- `docs/reference/language/EBNF.md`
- `docs/reference/language/types.md`
- `docs/reference/language/README.md`

## Next Implementation Card

`M11b-decl` should land as one narrow row:

```text
parse source static const table
-> preserve module metadata
-> produce MIR static_data_plans
-> emit LLVM readonly global
```

No table read, const eval, or const fn should be included in that first
implementation card.

## Gates

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
