# 293x-271 DEL-001 Legacy Delegation Status Reconcile

Status: Complete
Decision: accepted
Date: 2026-05-14

## Purpose

DEL-001 reconciles old `from` / `override` delegation documents with the
accepted no-inheritance direction.

This is a docs/manual reconciliation row. It does not add parser behavior.

## Landed docs

| Path | Role |
| --- | --- |
| `docs/development/current/main/design/delegation-legacy-status-reconcile-ssot.md` | DEL-001 canonical/residue decision. |
| `docs/reference/boxes-system/delegation-system.md` | Legacy notice for old `from` documentation. |
| `docs/reference/core-language/override-delegation-syntax.md` | Legacy notice for old override/from proposal. |
| `docs/reference/boxes-system/README.md` | Index wording now marks old delegation page as historical. |

## Decision

Canonical future behavior reuse is explicit field delegation:

```hako
box Child {
    parent: Parent = new Parent()

    delegate parent exposes {
        method
        other as publicOther
    }
}
```

Legacy `box Child from Parent`, `override`, and `from Parent.method()` are not
new canonical spelling.

## Next blocker

```text
LOOP-002 Stage0 LoopRange parser capsule
```
