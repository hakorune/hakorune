---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: Recommended task order after the M192-M213 purge/lifecycle ladder closeout.
Related:
  - docs/development/current/main/design/purge-lifecycle-ladder-closeout-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
---

# Post-M213 Next-Lane Selection SSOT

## Decision

After M213 and D204, do not continue into reclaim execution automatically.

Recommended order:

```text
1. D206 mimalloc port remaining inventory
2. mimalloc migration / parity rows selected from that inventory
3. language minimal-surface implementation rows when they unblock allocator or compiler clarity
4. selfhost migration after mimalloc and language surfaces have stable seams
```

This preserves momentum without mixing allocator behavior, language design, and
selfhost migration in one row.

## Current selected blocker

```text
D206 mimalloc port remaining inventory
```

D206 is docs/inventory first.
It should identify the next concrete mimalloc migration rows and must not
implement reclaim execution, provider activation, hooks, or allocator
replacement.

## Wave 1: Mimalloc port remaining inventory

Purpose:

```text
freeze what is already complete
identify remaining mimalloc-shaped gaps
select the next small implementation row
keep optional provider/replacement ladders inactive
```

Suggested rows:

| Order | Row | Scope |
| --- | --- | --- |
| 1 | `D206 mimalloc port remaining inventory` | Map completed hako_alloc rows against the mimalloc port purpose and list remaining gaps. |
| 2 | `D207 mimalloc next-row selection` | Choose exactly one next implementation row from D206. |
| 3 | `M214 selected mimalloc migration row` | Implement one durable allocator semantic slice with proof/guard. |
| 4 | `D208 mimalloc migration closeout check` | Update docs and decide whether to continue mimalloc or switch lanes. |

Allowed D206 outputs:

```text
remaining feature inventory
inactive surface inventory
recommended next one-row implementation
stop lines
proof/guard expectations
```

Forbidden in D206:

```text
new allocator behavior
reclaim execution
thread scheduling
atomics expansion
provider activation
hooks
process allocator replacement
language syntax implementation
selfhost parser/mirbuilder changes
```

## Wave 2: Mimalloc migration rows

Mimalloc migration rows must stay one semantic slice at a time.

Examples of acceptable row shapes:

```text
one missing policy owner
one bounded execution seam
one read-only inventory
one proof-only EXE hardening row
one verifier-owned invariant transfer
one closeout map
```

Do not bundle:

```text
reclaim execution + atomics
provider activation + allocator replacement
language syntax + allocator behavior
selfhost migration + allocator behavior
```

If a mimalloc row needs a language feature, stop and open the language row
explicitly.

## Wave 3: Language minimal-surface rows

Language rows are parked in:

```text
docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
```

Recommended first language rows when the allocator lane pauses:

| Order | Row |
| --- | --- |
| 1 | `DEL-001 legacy delegation status reconcile` |
| 2 | `LOOP-002 Stage0 LoopRange parser capsule` |
| 3 | `LOOP-003 Stage1 LoopRange lowering` |
| 4 | `BRAND-001 Stage0 brand declaration metadata capsule` |
| 5 | `BRAND-002 Stage1 brand constructor unwrap policy` |
| 6 | `REC-001 Stage0 explicit record literal shape capsule` |
| 7 | `REC-002 Stage1 record construction/read lowering` |
| 8 | `CONTRACT-002 contract syntax metadata capsule` |
| 9 | `TRANS-001 transition metadata capsule` |
| 10 | `USES-001 method-level uses metadata capsule` |

Docs update rule:

```text
when a language feature is implemented:
  update the task breakdown row
  update the minimal surface SSOT if canonical spelling changes
  update docs/reference only after Decision is accepted
```

## Wave 4: Selfhost migration

Selfhost migration should resume after the current allocator/mimalloc lane has
a stable stop point and language surfaces needed by selfhost are either
implemented or explicitly parked.

Entry docs:

```text
docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md
docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md
docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md
```

Recommended selfhost order:

| Order | Row | Scope |
| --- | --- | --- |
| 1 | `SH-001 selfhost restart inventory` | Refresh mirbuilder/parser status from tracked SSOTs. |
| 2 | `SH-002 next Tier-2 promotion candidate` | Select one case, no broad migration. |
| 3 | `SH-003 promote one mirbuilder case` | Use existing promote helper and update coverage docs. |
| 4 | `SH-004 parser handoff blocker check` | Only after mirbuilder coverage justifies parser work. |

Stop lines:

```text
do not use .hako workaround to hide compiler gaps
do not mix parser migration with mirbuilder owner migration
do not reopen broad VM parity
do not run allocator/provider activation as part of selfhost
```

## Lane switch rule

A lane switch requires a docs card that states:

```text
previous lane stop point
new lane blocker token
why now
first proof/guard
inactive surfaces
rollback path
```

Without that card, continue the selected blocker.

