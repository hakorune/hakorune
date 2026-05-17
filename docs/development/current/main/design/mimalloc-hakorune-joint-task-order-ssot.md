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

`MIMAP-132A` is current after `MIMAP-131A` selected the local-free reuse ledger
closeout guard.

Recommended current row:

```text
MIMAP-132A
  segment allocation modeled local-free reuse ledger closeout guard
```

Purpose:

```text
freeze the MIMAP-130A owner/proof/manifest/index wiring before broader allocator progress
keep real segment free, segment-map lookup, page-source, OSVM release, and provider activation closed
keep secure entropy execution parked until a real random route is accepted
```

Stop lines:

```text
no real thread scheduling
no worker spawning
no real segment free
no segment-map lookup
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
| 13 | allocator prerequisite | `MIMAP-056A reclaim remote-free drain contract inventory` | landed; no-execution drain readiness contract |
| 14 | allocator | `MIMAP-057A reclaim remote-free drain first execution route` | landed; one modeled drain entry |
| 15 | allocator | `MIMAP-058A reclaim post-drain owner-transfer integration route` | landed; compose drain and transfer order |
| 16 | planning | `MIMAP-059A post-reclaim-integration row selection` | landed; selected MIMAP-060A |
| 17 | allocator | `MIMAP-060A reclaim completion marker route` | landed; selected MIMAP-061A |
| 18 | closeout | `MIMAP-061A reclaim scalar lane closeout guard` | landed; selected MIMAP-062A |
| 19 | planning | `MIMAP-062A post-reclaim-scalar-closeout row selection` | landed; selected MIMAP-063A |
| 20 | allocator prerequisite | `MIMAP-063A reclaim scheduler boundary inventory` | landed; selected MIMAP-064A |
| 21 | allocator contract | `MIMAP-064A reclaim scheduler request marker contract` | landed; selected MIMAP-065A |
| 22 | closeout | `MIMAP-065A reclaim scheduler marker closeout guard` | landed; selected MIMAP-066A |
| 23 | planning | `MIMAP-066A post-scheduler-marker row selection` | landed; selected MIMAP-067A |
| 24 | planning | `MIMAP-067A reclaim scheduler substrate proposal-or-park` | landed; selected MIMAP-068A |
| 25 | allocator | `MIMAP-068A reclaim scheduler request ledger route` | landed; selected MIMAP-069A |
| 26 | closeout | `MIMAP-069A reclaim scheduler request ledger closeout guard` | landed; selected MIMAP-070A |
| 27 | planning | `MIMAP-070A post-scheduler-ledger row selection` | landed; selected MIMAP-071A |
| 28 | allocator | `MIMAP-071A reclaim scheduler request ledger consume route` | landed; selected MIMAP-072A |
| 29 | closeout | `MIMAP-072A reclaim scheduler ledger consume closeout guard` | landed; selected MIMAP-073A |
| 30 | planning | `MIMAP-073A post-scheduler-consume row selection` | landed; selected MIMAP-074A |
| 31 | allocator | `MIMAP-074A reclaim scheduler request ledger roundtrip route` | landed; selected MIMAP-075A |
| 32 | closeout | `MIMAP-075A reclaim scheduler request ledger roundtrip closeout guard` | landed; selected MIMAP-076A |
| 33 | planning | `MIMAP-076A post-scheduler-roundtrip row selection` | landed; selected MIMAP-077A |
| 34 | closeout | `MIMAP-077A reclaim scheduler scalar lane closeout guard` | landed; selected MIMAP-078A |
| 35 | planning | `MIMAP-078A post-scheduler-scalar-closeout row selection` | landed; selected MIMAP-079A |
| 36 | allocator inventory | `MIMAP-079A segment arena bitmap boundary inventory` | landed; selected MIMAP-080A |
| 37 | closeout | `MIMAP-080A segment arena bitmap inventory closeout guard` | landed; selected MIMAP-081A |
| 38 | planning | `MIMAP-081A post-segment-arena-bitmap-inventory row selection` | landed; selected MIMAP-082A |
| 39 | allocator contract | `MIMAP-082A segment lifecycle scalar state contract` | landed; selected MIMAP-083A |
| 40 | closeout | `MIMAP-083A segment lifecycle scalar state closeout guard` | landed; selected MIMAP-084A |
| 41 | planning | `MIMAP-084A post-segment-lifecycle-closeout row selection` | landed; selected MIMAP-085A |
| 42 | allocator contract | `MIMAP-085A segment page membership scalar contract` | landed; selected MIMAP-086A |
| 43 | closeout | `MIMAP-086A segment page membership closeout guard` | landed; selected MIMAP-087A |
| 44 | planning | `MIMAP-087A post-segment-page-membership-closeout row selection` | landed; selected MIMAP-088A |
| 45 | allocator contract | `MIMAP-088A segment allocation readiness scalar contract` | landed; selected MIMAP-089A |
| 46 | closeout | `MIMAP-089A segment allocation readiness closeout guard` | landed; selected MIMAP-090A |
| 47 | planning | `MIMAP-090A post-segment-allocation-readiness row selection` | landed; selected MIMAP-091A |
| 48 | allocator | `MIMAP-091A segment allocation modeled consume route` | landed; selected MIMAP-092A |
| 49 | closeout | `MIMAP-092A segment allocation modeled consume closeout guard` | landed; selected MIMAP-093A |
| 50 | planning | `MIMAP-093A post-segment-allocation-modeled-consume row selection` | landed; selected MIMAP-094A |
| 51 | allocator | `MIMAP-094A segment allocation modeled ledger route` | landed; selected MIMAP-095A |
| 52 | closeout | `MIMAP-095A segment allocation modeled ledger closeout guard` | landed; selected MIMAP-096A |
| 53 | planning | `MIMAP-096A post-segment-allocation-modeled-ledger row selection` | landed; selected MIMAP-097A |
| 54 | allocator | `MIMAP-097A segment allocation modeled ledger release route` | landed; selected MIMAP-098A |
| 55 | closeout | `MIMAP-098A segment allocation modeled ledger release closeout guard` | landed; selected MIMAP-099A |
| 56 | planning | `MIMAP-099A post-segment-allocation-modeled-release row selection` | landed; selected MIMAP-100A |
| 57 | allocator | `MIMAP-100A segment allocation modeled ledger released-token recycle route` | landed; selected MIMAP-101A |
| 58 | closeout | `MIMAP-101A segment allocation modeled ledger released-token recycle closeout guard` | landed; selected MIMAP-102A |
| 59 | planning | `MIMAP-102A post-segment-allocation-modeled-recycle row selection` | landed; selected HAKO-ALLOC-SRC-CLEAN-001 |
| 60 | cleanup | `HAKO-ALLOC-SRC-CLEAN-001 segment counter compound assignment cleanup` | landed; selected MIMAP-103A |
| 61 | planning | `MIMAP-103A post-segment-counter-cleanup row selection` | landed; selected MIMAP-104A |
| 62 | allocator | `MIMAP-104A segment allocation modeled ledger release span facts route` | landed; selected MIMAP-105A |
| 63 | planning | `MIMAP-105A post-release-span-facts row selection` | landed; selected MIMAP-ROW-CADENCE-001 |
| 64 | process cleanup | `MIMAP-ROW-CADENCE-001 mimalloc row validation cadence SSOT` | landed; selected MIMAP-106A |
| 65 | planning | `MIMAP-106A post-validation-cadence row selection` | landed; selected MIMAP-107A |
| 66 | allocator | `MIMAP-107A segment allocation modeled released-span ledger route` | landed; selected MIMAP-108A |
| 67 | planning | `MIMAP-108A post-released-span-ledger row selection` | landed; selected MIMAP-109A |
| 68 | allocator | `MIMAP-109A segment allocation modeled local-free candidate ledger route` | landed; selected MIMAP-110A |
| 69 | planning | `MIMAP-110A post-local-free-candidate-ledger row selection` | landed; selected MIMAP-111A |
| 70 | allocator | `MIMAP-111A segment allocation modeled local-free apply plan route` | landed; selected MIMAP-112A |
| 71 | planning | `MIMAP-112A post-local-free-apply-plan row selection` | landed; selected MIMAP-113A |
| 72 | closeout | `MIMAP-113A segment allocation modeled local-free scalar lane closeout guard` | landed; selected MIMAP-114A |
| 73 | planning | `MIMAP-114A post-local-free-scalar-closeout row selection` | landed; selected MIMAP-115A |
| 74 | allocator | `MIMAP-115A segment allocation modeled local-free page-model apply route` | landed; selected MIMAP-116A |
| 75 | planning | `MIMAP-116A post-local-free-page-apply row selection` | landed; selected MIMAP-117A |
| 76 | closeout | `MIMAP-117A segment allocation modeled local-free page-apply closeout guard` | landed; selected MIMAP-118A |
| 77 | planning | `MIMAP-118A post-local-free-page-apply-closeout row selection` | landed; selected MIMAP-119A |
| 78 | allocator | `MIMAP-119A segment allocation modeled local-free integration route` | landed; selected MIMAP-120A |
| 79 | planning | `MIMAP-120A post-local-free-integration row selection` | landed; selected MIMAP-121A |
| 80 | closeout | `MIMAP-121A segment allocation modeled local-free integration closeout guard` | landed; selected MIMAP-122A |
| 81 | planning | `MIMAP-122A post-local-free-integration-closeout row selection` | landed; selected PURE-FIRST-GLOBAL-CALL-001 |
| 82 | Hakorune compiler | `PURE-FIRST-GLOBAL-CALL-001 same-module static helper global-call route support` | landed; selected MIMAP-123A |
| 83 | planning | `MIMAP-123A post-same-module-global-call row selection` | landed; selected ROUTE-FIXPOINT-001 |
| 84 | Hakorune compiler cleanup | `ROUTE-FIXPOINT-001 route refresh fixpoint owner extraction` | landed; selected ROUTE-DIAG-VOCAB-001 |
| 85 | Hakorune compiler cleanup | `ROUTE-DIAG-VOCAB-001 route diagnostics vocabulary SSOT` | landed; selected ROUTE-DIAG-VOCAB-002 |
| 86 | guard cleanup | `GUARD-MANIFEST-011 pure-first route thin wrapper pilot` | landed; selected ROUTE-DIAG-VOCAB-001 |
| 87 | Hakorune compiler cleanup | `ROUTE-DIAG-VOCAB-002 preflight vocabulary drift guard` | landed; selected MIMAP-124A |
| 88 | planning | `MIMAP-124A post-route-diagnostics cleanup row selection` | landed; selected RUNTIME-UNWRAP-001 |
| 89 | source cleanup | `RUNTIME-UNWRAP-001 runtime lock expect messages` | landed; selected WASM-LOG-001 |
| 90 | source cleanup | `WASM-LOG-001 WAT2WASM stable tags` | landed; selected MIMAP-125A |
| 91 | planning | `MIMAP-125A post-source-cleanup row selection` | landed; selected MIMAP-126A |
| 92 | allocator | `MIMAP-126A segment allocation modeled local-free reuse route` | landed; selected MIMAP-127A |
| 93 | planning | `MIMAP-127A post-local-free-reuse row selection` | landed; selected MIMAP-128A |
| 94 | closeout | `MIMAP-128A segment allocation modeled local-free reuse closeout guard` | landed; selected MIMAP-129A |
| 95 | planning | `MIMAP-129A post-local-free-reuse-closeout row selection` | landed; selected MIMAP-130A |
| 96 | allocator | `MIMAP-130A segment allocation modeled local-free reuse ledger route` | landed; selected MIMAP-131A |
| 97 | planning | `MIMAP-131A post-local-free-reuse-ledger row selection` | landed; selected MIMAP-132A |
| 98 | closeout | `MIMAP-132A segment allocation modeled local-free reuse ledger closeout guard` | selected current |
| 18 | Hakorune language | brands/type aliases for allocator scalar IDs | reduces page/block/ptr/generation mix-ups without changing allocator behavior |
| 19 | Hakorune language | record literal / report object cleanup | replaces wide scalar report methods when current compiler support is enough |
| 20 | Hakorune language | Result/Option + guard-let ergonomics | improves allocator failure APIs after semantics are stable |
| 21 | optional runtime | provider/host allocator replacement ladder | explicit future option only; not a mimalloc completion prerequisite |

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
