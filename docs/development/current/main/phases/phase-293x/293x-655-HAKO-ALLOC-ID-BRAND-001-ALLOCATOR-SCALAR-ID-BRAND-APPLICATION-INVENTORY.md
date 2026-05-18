# 293x-655 HAKO-ALLOC-ID-BRAND-001 Allocator Scalar ID Brand Application Inventory

Status: selected current
Date: 2026-05-18

## Decision

Use the existing Hakorune brand/type vocabulary as the next narrow row after
`MIMAP-144A`.

This row is not a new brand-semantics design. It inventories where the current
`hako_alloc` modeled allocator path can safely use already-defined allocator ID
brands such as `SegmentId`, `PageId`, `BlockId`, and `Generation`, and records
whether the current Stage1 brand checker is enough for a first source pilot.

## Owner

```text
docs/development/current/main/design/mimalloc-hakorune-brand-type-vocabulary-ssot.md
docs/development/current/main/design/brand-mismatch-checker-ssot.md
lang/src/hako_alloc/memory/
```

## Scope

- Inventory scalar ID families in the current segment allocation modeled
  local-free reuse ledger / release-applied recycle lane.
- Classify each candidate as:
  - already safe for an existing brand/type annotation,
  - blocked by current Stage1 brand checker limits,
  - not worth branding yet because it is a count/unit or transient proof code.
- Pick at most one source pilot if it fits the existing same-program call-arg
  checker.
- If field, return, cross-module, or typed-local propagation is required, do not
  work around it in `.hako`; open a focused compiler row instead.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `001.1` | Inventory allocator ID scalar candidates in `lang/src/hako_alloc/memory/`. | candidate table lists owner file, scalar name, intended brand/type, and blocker. | no source rewrite |
| `001.2` | Check current Stage1 brand support against those candidates. | unsupported needs are named explicitly. | no broad type system work |
| `001.3` | Add one minimal pilot only if existing semantics can verify it. | proof app / guard stays green. | no compiler workaround |
| `001.4` | Otherwise select a focused compiler row. | next row has owner/proof/stop lines. | no silent fallback |

## Current Stage1 Limits

The current brand checker owns same-program call-argument checks for explicit
brand constructors and brand-typed parameters. It does not yet own:

```text
field type propagation
return type checking
typed-local assignment checking
cross-module brand inference
generic substitution
```

This row must respect those limits. It may document a blocker, but it must not
pretend field or return brands are verified when they are not.

## Stop Lines

- No new brand syntax.
- No new brand checker behavior unless a separate compiler row is selected.
- No allocator behavior.
- No real segment allocation/free execution.
- No page-source or OSVM execution.
- No thread scheduling or worker spawning.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No backend `.inc` matcher.
- No silent fallback.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
