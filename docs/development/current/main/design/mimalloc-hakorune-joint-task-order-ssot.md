---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: Recommended task order for continuing Hakorune core development while advancing the `.hako` mimalloc port.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
  - docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-capability-surface-ssot.md
  - docs/development/current/main/design/language-feature-implementation-order-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# Mimalloc / Hakorune Joint Task Order SSOT

## Decision

Continue the mimalloc port and Hakorune core development together, but do not
turn mimalloc into a reason to implement every language/concurrency feature
first.

The order is:

```text
1. finish the current planning row
2. open only the Hakorune core capability that the next allocator row needs
3. prove the allocator row in `.hako`
4. add language ergonomics only when they remove real allocator complexity
5. keep provider/global allocator replacement parked
```

This keeps Hakorune improving as a language/compiler while preventing mimalloc
from pulling in broad user-facing concurrency or provider activation too early.

## Current Recommended Row

`MIMAP-055A` landed the reclaim owner-transfer first execution route before
remote-free drain execution.

Recommended current row:

```text
MIMAP-056A
  reclaim remote-free drain contract inventory
```

Purpose:

```text
name remote-free drain readiness / blocked reasons before drain execution
keep remote-free drain execution, scheduling, page-source, and provider activation closed
keep secure entropy execution parked until a real random route is accepted
```

Stop lines:

```text
no remote-free drain execution
no thread scheduling
no page-source call
no OSVM unreserve / release
no provider activation
```

## Recommended Task Order

| Order | Track | Row shape | Why next |
| --- | --- | --- | --- |
| 1 | planning | `MIMAP-049B` selects one next row | landed |
| 2 | Hakorune core | `RANDOM-CAP-001 uses random capability decision` | landed |
| 3 | Hakorune core | `RANDOM-CAP-002 random route fail-fast/preflight` | landed |
| 4 | allocator | `MIMAP-050A secure entropy route proposal or park row` | landed; parked entropy execution |
| 5 | allocator | `MIMAP-051A reclaim owner-transfer contract inventory` | landed; named explicit preconditions before execution |
| 6 | planning | `MIMAP-051B post-reclaim-contract row selection` | landed; selected USES-002A |
| 7 | Hakorune core | `USES-002A declared uses capability plan mapping` | landed; low-level declared uses are MIR-visible |
| 8 | planning | `MIMAP-052A reclaim execution preflight proposal` | landed; selected dedicated reclaim marker |
| 9 | Hakorune core / allocator gate | `MIMAP-052B reclaim execution intent marker preflight` | landed; fail-fast marker exists |
| 10 | planning | `MIMAP-053A reclaim execution support row selection` | landed; selected atomic-claim contract |
| 11 | allocator prerequisite | `MIMAP-054A reclaim atomic-claim contract` | landed; claim contract is named |
| 12 | allocator | `MIMAP-055A reclaim owner-transfer first execution route` | landed; one guarded modeled owner transfer |
| 13 | allocator prerequisite | `MIMAP-056A reclaim remote-free drain contract inventory` | current; no-execution drain readiness contract |
| 14 | Hakorune language | brands/type aliases for allocator scalar IDs | reduces page/block/ptr/generation mix-ups without changing allocator behavior |
| 15 | Hakorune language | record literal / report object cleanup | replaces wide scalar report methods when current compiler support is enough |
| 16 | Hakorune language | Result/Option + guard-let ergonomics | improves allocator failure APIs after semantics are stable |
| 17 | optional runtime | provider/host allocator replacement ladder | explicit future option only; not a mimalloc completion prerequisite |

## What Does Not Block Current Mimalloc Rows

These are useful Hakorune features, but they are not prerequisites for the next
allocator rows unless a focused card proves they are the smallest blocker:

```text
source-level worker_local
Channel
co / task_scope expansion
true parallel language semantics
lock<T> / sync box user-facing semantics
provider activation
#[global_allocator]
full Stage1 parser/mirbuilder rewrite in .hako
```

## Hakorune Core Work That Can Run Between Allocator Rows

Prefer these core rows when allocator work hits a representation or diagnostic
limit:

| Core row family | Opens | Keep out |
| --- | --- | --- |
| capability checker | `uses osvm` / `uses atomic` / `uses rawbuf` / later `uses random` / `uses tls` verifier facts | backend helper-name guessing |
| route preflight | early unsupported-route diagnostics | late C shim discovery as primary UX |
| brand/type alias | scalar identity for page/block/ptr/generation IDs | Stage0 typechecker |
| record/report objects | compact read-only proof reports | packed/backend lowering unless explicitly selected |
| contract/assert | runtime fail-fast facts | broad invariant system in one row |
| Result/Option | allocator failure API normalization | null sugar or hidden fallback |

## Provider Replacement Rule

Completing the `.hako` mimalloc port means Hakorune can express and prove more
allocator behavior. It does not mean the runtime switches malloc/free.

Keep these parked until an explicit optional provider row reopens them:

```text
provider activation
host allocator replacement
hooks
#[global_allocator]
malloc/new-delete/posix override
```

## Row Selection Rule

At each `MIMAP-*B` planning row:

```text
if the next allocator behavior needs a missing Hakorune capability:
  select the smallest Hakorune core capability row first

else if allocator behavior is already expressible and guarded:
  select the allocator row

else if the blocker is readability/maintainability only:
  select one BoxShape cleanup row

never:
  select provider replacement as a side effect
  bundle language/concurrency expansion with allocator behavior
```
