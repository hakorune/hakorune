# 293x-258 D202 Language Task Breakdown

Status: Complete
Decision: accepted
Date: 2026-05-14

## Purpose

Turn the language minimal-surface discussions into task-sized backlog rows
without changing the active allocator blocker.

Current allocator blocker remains:

```text
M212 bounded purge/decommit scheduler small path
```

## Landed docs

| Path | Role |
| --- | --- |
| `docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md` | Task-sized backlog for loop-only control, no-inheritance delegation, brand/type, record, contracts, enum transition, Result/Option, generic/PackedArray, uses/capability, Span/view, module, and check report rows. |

## Decisions fixed

The backlog is parked until the user explicitly switches to the language lane
or the allocator M212/M213 lane closes.

Suggested first language-lane rows:

```text
DEL-001 legacy delegation status reconcile
LOOP-002 Stage0 LoopRange parser capsule
LOOP-003 Stage1 LoopRange lowering
BRAND-001 Stage0 brand declaration metadata capsule
BRAND-002 Stage1 brand constructor unwrap policy
REC-001 Stage0 explicit record literal shape capsule
REC-002 Stage1 record construction/read lowering
CONTRACT-002 contract syntax metadata capsule
TRANS-001 transition metadata capsule
USES-001 method-level uses metadata capsule
```

## Stop line

This card does not implement language syntax.
It only preserves the discussed design as actionable task rows.

