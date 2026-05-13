---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: DEL-001 reconciliation of legacy delegation docs and canonical no-inheritance direction.
Related:
  - docs/development/current/main/design/delegation-no-inheritance-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/reference/boxes-system/delegation-system.md
  - docs/reference/core-language/override-delegation-syntax.md
  - src/core/model.rs
---

# Delegation Legacy Status Reconcile SSOT

## Decision

DEL-001 reconciles the old delegation/inheritance-like documentation with the
new canonical direction.

Canonical direction:

```text
no inheritance
no extends as source spelling
no super
no origin
no inherited fields
behavior reuse uses explicit field delegation
future syntax is delegate field exposes { ... }
```

Legacy/residue status:

| Surface | Status | Reading |
| --- | --- | --- |
| `box Child from Parent` | legacy / non-canonical | Historical delegation-like header. Do not use for new language docs. |
| internal `extends` model field | residue | Implementation naming may remain temporarily, but docs should call it delegation residue. |
| `override` | legacy proposal | Do not require for new delegation design. |
| `from Parent.method()` | legacy proposal | Prefer explicit `me.<field>.<method>()` in future design. |
| `super` | rejected | Inheritance mental model; not canonical. |
| `origin` | rejected for MVP | Ambiguous with multiple delegates. |
| inherited fields | rejected | Delegate fields are never imported. |
| wildcard expose | deferred | Explicit expose list only in MVP. |

## Manual correction

Legacy reference pages may remain for historical context, but they must display
a legacy notice and point to the accepted no-inheritance design. They must not
look like current canonical syntax.

Updated pages:

```text
docs/reference/boxes-system/delegation-system.md
docs/reference/core-language/override-delegation-syntax.md
docs/reference/boxes-system/README.md
```

## Next row

DEL-001 does not add parser behavior.

The next implementation row is:

```text
LOOP-002 Stage0 LoopRange parser capsule
```

Delegation parser implementation remains parked at:

```text
DEL-002 Stage0 delegate syntax metadata capsule
```
