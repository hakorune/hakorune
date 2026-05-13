---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: D210 lane switch from mimalloc inventory closeout to minimal language surface rows.
Related:
  - docs/development/current/main/design/mimalloc-post-m215-closeout-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/design/delegation-no-inheritance-ssot.md
  - docs/development/current/main/design/stage0-stage1-feature-responsibility-split-ssot.md
---

# Language Minimal Lane Switch After M215 SSOT

## Decision

After D209, switch from the mimalloc inventory wave to the minimal language
surface lane.

The first row is documentation reconciliation, not parser implementation:

```text
DEL-001 legacy delegation status reconcile
```

The next implementation row after DEL-001 is:

```text
LOOP-002 Stage0 LoopRange parser capsule
```

## Why now

The mimalloc algorithm/policy surface is complete enough for the current
selfhost-prep objective. The next blocker is language clarity, especially
removing conflicting manual guidance around legacy delegation before adding new
syntax capsules.

## Lane rules

```text
Stage0 rows: parse / metadata / trivial desugar only
Stage1 rows: meaning / verifier facts / lowering / diagnostics
no semantic ownership in Stage0
no duplicate canonical spelling
no silent fallback
```

## Inactive allocator surfaces

Language rows must not reopen:

```text
reclaim execution
thread scheduling
atomic ownership claim
unreserve / OS release
provider activation
hooks
process allocator replacement
```

## Rollback path

If language rows uncover a compiler blocker that is not about the selected
surface, stop and create a new docs card. Do not hide compiler gaps with `.hako`
workarounds.
