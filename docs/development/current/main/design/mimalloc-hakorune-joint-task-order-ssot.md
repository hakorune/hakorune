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
  - docs/development/current/main/design/typed-numeric-memory-substrate-task-order-ssot.md
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

Typed numeric / memory substrate ordering is now owned by:

```text
docs/development/current/main/design/typed-numeric-memory-substrate-task-order-ssot.md
```

Use that SSOT when a mimalloc row wants to move beyond owner-local
non-negative `usize` field migrations. In particular, raw pointer residence,
arena backing execution, bitmap word arithmetic, atomics, and TLS require the
typed numeric / memory substrate gates there before they can become allocator
behavior rows.

## Current Recommended Row

`MIMAP-281A` is current after HAKO-ALLOC-REPORT-RECORD-005 closed the
release-candidate ReportFields sidecar.

Recommended current row:

```text
MIMAP-281A
  segment arena backing modeled allocation-ledger release candidate diagnostics
```

Purpose:

```text
observe MIMAP-280A release-candidate counters and last-candidate facts
keep cross-function Result direct ABI and runtime sum materialization closed
keep real pointer residence, real arena backing allocation, segment-map lookup, page-source, OSVM release, and provider activation closed
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
| 98 | closeout | `MIMAP-132A segment allocation modeled local-free reuse ledger closeout guard` | landed; selected MIMAP-133A |
| 99 | planning | `MIMAP-133A post-local-free-reuse-ledger-closeout row selection` | landed; selected MIMAP-134A |
| 100 | allocator | `MIMAP-134A segment allocation modeled local-free reuse ledger release route` | landed; selected MIMAP-135A |
| 101 | planning | `MIMAP-135A post-local-free-reuse-ledger-release row selection` | landed; selected MIMAP-136A |
| 102 | closeout | `MIMAP-136A segment allocation modeled local-free reuse ledger release closeout guard` | landed; selected MIMAP-137A |
| 103 | planning | `MIMAP-137A post-local-free-reuse-ledger-release-closeout row selection` | landed; selected MIMAP-138A |
| 104 | allocator | `MIMAP-138A segment allocation modeled local-free reuse ledger release apply route` | landed; selected MIMAP-139A |
| 105 | closeout | `MIMAP-139A segment allocation modeled local-free reuse ledger release apply closeout guard` | landed; selected MIMAP-140A |
| 106 | planning | `MIMAP-140A post-local-free-reuse-ledger-release-apply-closeout row selection` | landed; selected GUARD-MANIFEST-012 |
| 107 | guard cleanup | `GUARD-MANIFEST-012 batch migration inventory` | landed; selected GUARD-MANIFEST-013 |
| 108 | guard cleanup | `GUARD-MANIFEST-013 declarative guard spec pilot` | landed; selected MIMAP-141A |
| 109 | planning | `MIMAP-141A post-guard-spec-pilot row selection` | landed; selected MIMAP-142A |
| 110 | allocator | `MIMAP-142A release-applied local-free reuse ledger token recycle proof` | landed; selected MIMAP-143A |
| 111 | closeout | `MIMAP-143A release-applied local-free reuse ledger token recycle closeout guard` | landed; selected MIMAP-144A |
| 112 | planning | `MIMAP-144A post-release-applied-recycle-closeout row selection` | landed; selected HAKO-ALLOC-ID-BRAND-001 |
| 113 | Hakorune language / allocator boundary | `HAKO-ALLOC-ID-BRAND-001 allocator scalar ID brand application inventory` | landed; selected PURE-FIRST-BRAND-CONSTRUCT-001 |
| 114 | Hakorune compiler | `PURE-FIRST-BRAND-CONSTRUCT-001 brand constructor MIR acceptance` | landed; selected HAKO-ALLOC-ID-BRAND-002 |
| 115 | Hakorune language / allocator boundary | `HAKO-ALLOC-ID-BRAND-002 allocator scalar ID brand first pilot` | landed; selected HAKO-ALLOC-ID-BRAND-003 |
| 116 | guard cleanup | `HAKO-ALLOC-ID-BRAND-003 allocator scalar ID brand pilot closeout guard` | landed; selected MIMAP-145A |
| 117 | planning | `MIMAP-145A post-ID-brand-pilot-closeout row selection` | landed; selected HAKO-ALLOC-REPORT-RECORD-001 |
| 118 | Hakorune language / allocator cleanup | `HAKO-ALLOC-REPORT-RECORD-001 allocator report record cleanup inventory` | landed; selected HAKO-ALLOC-REPORT-RECORD-002 |
| 119 | Hakorune language / allocator cleanup | `HAKO-ALLOC-REPORT-RECORD-002 local-free integration report record boundary cleanup` | landed; selected MIMAP-146A |
| 120 | planning | `MIMAP-146A post-report-record-cleanup row selection` | landed; selected HAKO-ALLOC-RESULT-API-001 |
| 121 | Hakorune language / allocator boundary | `HAKO-ALLOC-RESULT-API-001 allocator Result/Option guard-let inventory` | landed; selected PURE-FIRST-GUARDLET-ENUMMATCH-001 |
| 122 | Hakorune compiler | `PURE-FIRST-GUARDLET-ENUMMATCH-001 direct MIR guard-let EnumMatchExpr acceptance` | landed; selected HAKO-ALLOC-RESULT-API-002 |
| 123 | Hakorune language / allocator boundary | `HAKO-ALLOC-RESULT-API-002 allocator local-free Result guard-let pilot` | landed; selected MIMAP-147A |
| 124 | planning | `MIMAP-147A post-Result-guard-let-pilot row selection` | landed; selected HAKO-ALLOC-RESULT-API-003 |
| 125 | Hakorune language / allocator boundary | `HAKO-ALLOC-RESULT-API-003 allocator local-free remaining Result guard-let boundaries` | landed; selected MIMAP-148A |
| 126 | planning | `MIMAP-148A post-local-free-Result-boundary row selection` | landed; selected MIMAP-149A |
| 127 | allocator | `MIMAP-149A segment allocation blocked-substrate matrix proof` | landed; selected MIMAP-150A |
| 128 | planning | `MIMAP-150A post-blocked-substrate-matrix row selection` | landed; selected MIMAP-151A |
| 129 | allocator | `MIMAP-151A segment-map scalar lookup boundary inventory` | landed; selected MIMAP-152A |
| 130 | planning | `MIMAP-152A post-segment-map-scalar-lookup row selection` | landed; selected MIMAP-153A |
| 131 | allocator | `MIMAP-153A segment-map lookup guarded readiness composition` | landed; selected MIMAP-154A |
| 132 | planning | `MIMAP-154A post-lookup-guarded-readiness row selection` | landed; selected MIMAP-155A |
| 133 | allocator validation | `MIMAP-155A segment-map readiness validation pack closeout guard` | landed; selected MIMAP-156A |
| 134 | planning | `MIMAP-156A post-segment-map-readiness-closeout row selection` | landed; selected MIMAP-157A |
| 135 | allocator | `MIMAP-157A segment-map accepted readiness modeled consume ledger route` | landed; selected MIMAP-158A |
| 136 | allocator | `MIMAP-158A segment-map modeled consume ledger diagnostics` | landed; selected MIMAP-159A |
| 137 | closeout | `MIMAP-159A segment-map modeled consume ledger closeout pack` | landed; selected MIMAP-160A |
| 138 | planning | `MIMAP-160A post-segment-map-modeled-consume-ledger-closeout row selection` | landed; selected MIMAP-161A |
| 139 | allocator | `MIMAP-161A segment-map modeled consume ledger release route` | landed; selected MIMAP-162A |
| 140 | closeout | `MIMAP-162A segment-map modeled consume ledger release closeout pack` | landed; selected MIMAP-163A |
| 141 | planning | `MIMAP-163A post-segment-map-modeled-consume-ledger-release-closeout row selection` | landed; selected MIMAP-164A |
| 142 | allocator | `MIMAP-164A segment-map modeled consume ledger released-token recycle route` | landed; selected MIMAP-165A |
| 143 | planning | `MIMAP-165A post-segment-map-modeled-consume-ledger-released-token-recycle row selection` | landed; selected MIMAP-166A |
| 144 | closeout | `MIMAP-166A segment-map modeled consume ledger released-token recycle closeout pack` | landed; selected MIMAP-167A |
| 145 | planning | `MIMAP-167A post-segment-map-modeled-consume-ledger-released-token-recycle-closeout row selection` | landed; selected MIMAP-168A |
| 146 | allocator | `MIMAP-168A segment-map modeled consume ledger released-span observation route` | landed; selected MIMAP-169A |
| 147 | planning | `MIMAP-169A post-segment-map-modeled-consume-ledger-released-span-observation row selection` | landed; selected MIMAP-170A |
| 148 | closeout | `MIMAP-170A segment-map modeled consume ledger released-span observation closeout pack` | landed; selected MIMAP-171A |
| 149 | planning | `MIMAP-171A post-segment-map-modeled-consume-ledger-released-span-observation-closeout row selection` | landed; selected MIMAP-172A |
| 150 | allocator | `MIMAP-172A segment-map released-span local-free candidate bridge` | landed; selected MIMAP-173A |
| 151 | planning | `MIMAP-173A post-segment-map-released-span-local-free-candidate-bridge row selection` | landed; selected MIMAP-174A |
| 152 | closeout | `MIMAP-174A segment-map released-span local-free candidate bridge closeout pack` | landed; selected MIMAP-175A |
| 153 | planning | `MIMAP-175A post-segment-map-released-span-local-free-candidate-bridge-closeout row selection` | landed; selected MIMAP-176A |
| 154 | allocator | `MIMAP-176A segment-map local-free apply-plan bridge` | landed; selected MIMAP-177A |
| 155 | planning | `MIMAP-177A post-segment-map-local-free-apply-plan-bridge row selection` | landed; selected MIMAP-178A |
| 156 | closeout | `MIMAP-178A segment-map local-free apply-plan bridge closeout pack` | landed; selected MIMAP-179A |
| 157 | planning | `MIMAP-179A post-segment-map-local-free-apply-plan-bridge-closeout row selection` | landed; selected MIMAP-180A |
| 158 | allocator | `MIMAP-180A segment-map local-free page-apply bridge` | landed; selected MIMAP-181A |
| 159 | planning | `MIMAP-181A post-segment-map-local-free-page-apply-bridge row selection` | landed; selected MIMAP-182A |
| 160 | closeout | `MIMAP-182A segment-map local-free page-apply bridge closeout pack` | landed; selected MIMAP-183A |
| 161 | planning | `MIMAP-183A post-segment-map-local-free-page-apply-bridge-closeout row selection` | landed; selected MIMAP-184A |
| 162 | allocator | `MIMAP-184A segment-map local-free integration bridge` | landed; selected MIMAP-185A |
| 163 | planning | `MIMAP-185A post-segment-map-local-free-integration-bridge row selection` | landed; selected MIMAP-186A |
| 164 | closeout | `MIMAP-186A segment-map local-free integration bridge closeout pack` | landed; selected MIMAP-187A |
| 165 | planning | `MIMAP-187A post-segment-map-local-free-integration-bridge-closeout row selection` | landed; selected MIMAP-188A |
| 166 | allocator | `MIMAP-188A segment-map local-free reuse bridge` | landed; selected MIMAP-189A |
| 167 | planning | `MIMAP-189A post-segment-map-local-free-reuse-bridge row selection` | landed; selected MIMAP-190A |
| 168 | closeout | `MIMAP-190A segment-map local-free reuse bridge closeout pack` | landed; selected MIMAP-191A |
| 169 | planning | `MIMAP-191A post-segment-map-local-free-reuse-bridge-closeout row selection` | landed; selected MIMAP-192A |
| 170 | allocator | `MIMAP-192A segment-map local-free reuse ledger bridge` | landed; selected MIMAP-193A |
| 171 | planning | `MIMAP-193A post-segment-map-local-free-reuse-ledger-bridge row selection` | landed; selected MIMAP-194A |
| 172 | closeout | `MIMAP-194A segment-map local-free reuse ledger bridge closeout pack` | landed; selected MIMAP-195A |
| 173 | planning | `MIMAP-195A post-segment-map-local-free-reuse-ledger-bridge-closeout row selection` | landed; selected MIMAP-196A |
| 174 | allocator | `MIMAP-196A segment-map local-free reuse ledger release bridge` | landed; selected MIMAP-197A |
| 175 | planning | `MIMAP-197A post-segment-map-local-free-reuse-ledger-release-bridge row selection` | landed; selected MIMAP-198A |
| 176 | closeout | `MIMAP-198A segment-map local-free reuse ledger release bridge closeout pack` | landed; selected MIMAP-199A |
| 177 | planning | `MIMAP-199A post-segment-map-local-free-reuse-ledger-release-bridge-closeout row selection` | landed; selected MIMAP-200A |
| 178 | allocator | `MIMAP-200A segment-map local-free reuse ledger release apply bridge` | landed; selected MIMAP-201A |
| 179 | planning | `MIMAP-201A post-segment-map-local-free-reuse-ledger-release-apply-bridge row selection` | landed; selected MIMAP-202A |
| 180 | closeout | `MIMAP-202A segment-map local-free reuse ledger release apply bridge closeout pack` | landed; selected MIMAP-203A |
| 181 | planning | `MIMAP-203A post-segment-map-local-free-reuse-ledger-release-apply-bridge-closeout row selection` | landed; selected MIMAP-204A |
| 182 | allocator | `MIMAP-204A segment-map local-free reuse ledger release-applied recycle bridge` | landed; selected MIMAP-205A |
| 183 | planning | `MIMAP-205A post-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge row selection` | landed; selected MIMAP-206A |
| 184 | closeout | `MIMAP-206A segment-map local-free reuse ledger release-applied recycle bridge closeout pack` | landed; selected MIMAP-207A |
| 185 | planning | `MIMAP-207A post-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge-closeout row selection` | landed; selected MIMAP-208A |
| 186 | allocator | `MIMAP-208A segment-map local-free reuse ledger release-applied recycle second-release diagnostic` | landed; selected MIMAP-209A |
| 187 | planning | `MIMAP-209A post-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic row selection` | landed; selected MIMAP-210A |
| 188 | closeout | `MIMAP-210A segment-map local-free reuse ledger release-applied recycle second-release diagnostic closeout pack` | landed; selected MIMAP-211A |
| 189 | planning | `MIMAP-211A post-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-closeout row selection` | landed; selected MIMAP-212A |
| 190 | allocator | `MIMAP-212A segment-map local-free reuse ledger lifecycle-token pilot` | landed; selected MIMAP-213A |
| 191 | planning | `MIMAP-213A post-segment-map-local-free-reuse-ledger-lifecycle-token-pilot row selection` | landed; selected MIMAP-214A |
| 192 | closeout | `MIMAP-214A segment-map local-free reuse ledger lifecycle-token pilot closeout pack` | landed; selected MIMAP-215A |
| 193 | planning | `MIMAP-215A post-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-closeout row selection` | landed; selected MIMAP-216A |
| 194 | allocator | `MIMAP-216A segment-map local-free reuse ledger lifecycle-token observer diagnostic` | landed; selected MIMAP-217A |
| 195 | planning | `MIMAP-217A post-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic row selection` | landed; selected MIMAP-218A |
| 196 | closeout | `MIMAP-218A segment-map local-free reuse ledger lifecycle-token observer diagnostic closeout pack` | landed; selected MIMAP-219A |
| 197 | planning | `MIMAP-219A post-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-closeout row selection` | landed; selected MIMAP-220A |
| 198 | allocator | `MIMAP-220A segment-map local-free reuse ledger lifecycle-token release-key precondition observer` | landed; selected MIMAP-221A |
| 199 | planning | `MIMAP-221A post-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition row selection` | landed; selected MIMAP-222A |
| 200 | closeout | `MIMAP-222A segment-map local-free reuse ledger lifecycle-token release-key precondition closeout pack` | landed; selected MIMAP-223A |
| 201 | planning | `MIMAP-223A post-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-closeout row selection` | landed; selected MIMAP-224A |
| 202 | allocator | `MIMAP-224A segment-map local-free reuse ledger lifecycle-keyed release shadow pilot` | landed; selected MIMAP-225A |
| 203 | planning | `MIMAP-225A post-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow row selection` | landed; selected MIMAP-226A |
| 204 | closeout | `MIMAP-226A segment-map local-free reuse ledger lifecycle-keyed release shadow closeout pack` | landed; selected MIMAP-227A |
| 205 | planning | `MIMAP-227A post-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-closeout row selection` | landed; selected MIMAP-228A |
| 206 | allocator | `MIMAP-228A source release-ledger lifecycle-key migration pilot` | landed; selected MIMAP-229A |
| 207 | allocator | `MIMAP-229A source lifecycle-keyed release ledger diagnostics` | landed; selected MIMAP-230A |
| 208 | closeout | `MIMAP-230A source release-ledger lifecycle-key migration closeout pack` | landed; selected MIMAP-231A |
| 209 | planning | `MIMAP-231A post source release-ledger lifecycle-key migration closeout row selection` | landed; selected MIMAP-232A |
| 210 | allocator | `MIMAP-232A source lifecycle-keyed release apply/recycle continuation bridge` | landed; selected MIMAP-233A |
| 211 | allocator | `MIMAP-233A source lifecycle-keyed release apply/recycle continuation diagnostics` | landed; selected MIMAP-234A |
| 212 | closeout | `MIMAP-234A source lifecycle-keyed release apply/recycle continuation closeout pack` | landed; selected MIMAP-235A |
| 213 | planning | `MIMAP-235A post source lifecycle-keyed release apply/recycle continuation closeout row selection` | landed; selected MIMAP-236A |
| 214 | allocator inventory | `MIMAP-236A segment arena backing readiness inventory` | landed; selected MIMAP-237A |
| 215 | allocator diagnostic | `MIMAP-237A segment arena backing readiness diagnostics` | landed; selected MIMAP-238A |
| 216 | closeout | `MIMAP-238A segment arena backing readiness closeout pack` | landed; selected MIMAP-239A |
| 217 | planning | `MIMAP-239A post-segment-arena-backing-readiness-closeout row selection` | landed; selected MIMAP-240A |
| 218 | allocator inventory | `MIMAP-240A segment arena backing scalar requirement matrix inventory` | landed; selected MIMAP-241A |
| 219 | allocator diagnostic | `MIMAP-241A segment arena backing requirement matrix diagnostics` | landed; selected MIMAP-242A |
| 220 | closeout | `MIMAP-242A segment arena backing requirement matrix closeout pack` | landed; selected MIMAP-243A |
| 221 | planning | `MIMAP-243A post-segment-arena-backing-requirement-matrix-closeout row selection` | landed; selected MIMAP-244A |
| 222 | allocator inventory | `MIMAP-244A segment arena backing no-escape raw pointer capability inventory` | landed; selected MIMAP-245A |
| 223 | allocator diagnostic | `MIMAP-245A segment arena backing no-escape address capability diagnostics` | landed; selected MIMAP-246A |
| 224 | closeout | `MIMAP-246A segment arena backing no-escape address capability closeout pack` | landed; selected MIMAP-247A |
| 225 | planning | `MIMAP-247A post-segment-arena-backing-no-escape-address-capability-closeout row selection` | landed; selected MIMAP-248A |
| 226 | allocator inventory | `MIMAP-248A segment arena backing modeled no-escape address residence inventory` | landed; selected MIMAP-249A |
| 227 | allocator diagnostic | `MIMAP-249A segment arena backing modeled no-escape address residence diagnostics` | landed; selected MIMAP-250A |
| 228 | closeout | `MIMAP-250A segment arena backing modeled no-escape address residence closeout pack` | landed; selected MIMAP-251A |
| 229 | planning | `MIMAP-251A post-segment-arena-backing-modeled-no-escape-address-residence-closeout row selection` | landed; selected MIMAP-252A |
| 230 | allocator inventory | `MIMAP-252A segment arena backing modeled residence arena-binding inventory` | landed; selected MIMAP-253A |
| 231 | allocator diagnostic | `MIMAP-253A segment arena backing modeled residence arena-binding diagnostics` | landed; selected MIMAP-254A |
| 232 | closeout | `MIMAP-254A segment arena backing modeled residence arena-binding closeout pack` | landed; selected MIMAP-255A |
| 233 | planning | `MIMAP-255A post-segment-arena-backing-modeled-residence-arena-binding-closeout row selection` | landed; selected MIMAP-256A |
| 234 | allocator inventory | `MIMAP-256A segment arena backing modeled arena slot inventory` | landed; selected MIMAP-257A |
| 235 | allocator diagnostic | `MIMAP-257A segment arena backing modeled arena slot diagnostics` | landed; selected MIMAP-258A |
| 236 | closeout | `MIMAP-258A segment arena backing modeled arena slot closeout pack` | landed; selected MIMAP-259A |
| 237 | planning | `MIMAP-259A post-segment-arena-backing-modeled-arena-slot-closeout row selection` | landed; selected MIMAP-260A |
| 238 | allocator inventory | `MIMAP-260A segment arena backing modeled source bridge inventory` | landed; selected MIMAP-261A |
| 239 | allocator diagnostic | `MIMAP-261A segment arena backing modeled source bridge diagnostics` | landed; selected MIMAP-262A |
| 240 | closeout | `MIMAP-262A segment arena backing modeled source bridge closeout pack` | landed; selected MIMAP-263A |
| 241 | planning | `MIMAP-263A post-segment-arena-backing-modeled-source-bridge-closeout row selection` | landed; selected MIMAP-264A |
| 242 | allocator inventory | `MIMAP-264A segment arena backing modeled source accounting inventory` | landed; selected MIMAP-265A |
| 243 | allocator diagnostic | `MIMAP-265A segment arena backing modeled source accounting diagnostics` | landed; selected MIMAP-266A |
| 244 | closeout | `MIMAP-266A segment arena backing modeled source accounting closeout pack` | landed; selected HAKO-ALLOC-REPORT-RECORD-003 |
| 245 | Hakorune language / allocator cleanup | `HAKO-ALLOC-REPORT-RECORD-003 segment arena backing report record carrier inventory` | landed; selected HAKO-ALLOC-REPORT-RECORD-004 |
| 246 | Hakorune language / allocator cleanup | `HAKO-ALLOC-REPORT-RECORD-004 segment arena backing source accounting diagnostic ReportFields pilot` | landed; selected MIMAP-267A |
| 247 | planning | `MIMAP-267A post-segment-arena-backing-reportfields-pilot row selection` | landed; selected MIMAP-268A |
| 248 | allocator inventory | `MIMAP-268A segment arena backing modeled allocation plan inventory` | landed; selected MIMAP-269A |
| 249 | allocator diagnostic | `MIMAP-269A segment arena backing modeled allocation plan diagnostics` | landed; selected MIMAP-270A |
| 250 | closeout | `MIMAP-270A segment arena backing modeled allocation plan closeout pack` | landed; selected MIMAP-271A |
| 251 | planning | `MIMAP-271A post-segment-arena-backing-modeled-allocation-plan-closeout row selection` | landed; selected MIMAP-272A |
| 252 | allocator inventory | `MIMAP-272A segment arena backing modeled allocation apply inventory` | landed; selected MIMAP-273A |
| 253 | allocator diagnostic | `MIMAP-273A segment arena backing modeled allocation apply diagnostics` | landed; selected MIMAP-274A |
| 254 | closeout | `MIMAP-274A segment arena backing modeled allocation apply closeout pack` | landed; selected MIMAP-275A |
| 255 | planning | `MIMAP-275A post-segment-arena-backing-modeled-allocation-apply-closeout row selection` | landed; selected MIMAP-276A |
| 256 | allocator inventory | `MIMAP-276A segment arena backing modeled allocation ledger inventory` | landed; selected MIMAP-277A |
| 257 | allocator diagnostic | `MIMAP-277A segment arena backing modeled allocation ledger diagnostics` | landed; selected MIMAP-278A |
| 258 | closeout | `MIMAP-278A segment arena backing modeled allocation ledger closeout pack` | landed; selected MIMAP-279A |
| 259 | planning | `MIMAP-279A post-segment-arena-backing-modeled-allocation-ledger-closeout row selection` | landed; selected MIMAP-280A |
| 260 | allocator inventory | `MIMAP-280A segment arena backing modeled allocation-ledger release candidate inventory` | landed; selected HAKO-ALLOC-REPORT-RECORD-005 |
| 261 | Hakorune language / allocator cleanup | `HAKO-ALLOC-REPORT-RECORD-005 allocation-ledger release candidate ReportFields pilot` | landed; selected MIMAP-281A |
| 262 | allocator diagnostic | `MIMAP-281A segment arena backing modeled allocation-ledger release candidate diagnostics` | landed; selected MIMAP-282A |
| 263 | closeout | `MIMAP-282A segment arena backing modeled allocation-ledger release candidate closeout pack` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-001 |
| 264 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-001 select allocator byte/capacity field-group pilot` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-002 |
| 265 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-002 release-candidate byte/capacity migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-003 |
| 266 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-003 release-candidate byte/capacity closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-004 |
| 267 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-004 release-candidate diagnostic byte mirror migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-005 |
| 268 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-005 release-candidate diagnostic byte mirror closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-006 |
| 269 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-006 allocation-ledger byte/capacity migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-007 |
| 270 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-007 allocation-ledger byte/capacity closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-008 |
| 271 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-008 allocation-ledger diagnostic byte mirror migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-009 |
| 272 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-009 allocation-ledger diagnostic byte mirror closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-010 |
| 273 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-010 allocation-apply byte/capacity migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-011 |
| 274 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-011 allocation-apply byte/capacity closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-012 |
| 275 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-012 allocation-apply diagnostic byte mirror migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-013 |
| 276 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-013 allocation-apply diagnostic byte mirror closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-014 |
| 277 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-014 allocation-plan byte/capacity migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-015 |
| 278 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-015 allocation-plan byte/capacity closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-016 |
| 279 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-016 allocation-plan diagnostic byte mirror migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-017 |
| 280 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-017 allocation-plan diagnostic byte mirror closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-018 |
| 281 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-018 source-accounting byte/capacity migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-019 |
| 282 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-019 source-accounting byte/capacity closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-020 |
| 283 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-020 source-accounting diagnostic byte mirror migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-021 |
| 284 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-021 source-accounting diagnostic byte mirror closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-022 |
| 285 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-022 source-bridge byte/capacity migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-023 |
| 286 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-023 source-bridge byte/capacity closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-024 |
| 287 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-024 source-bridge diagnostic byte mirror migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-025 |
| 288 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-025 source-bridge diagnostic byte mirror closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-026 |
| 289 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-026 arena-slot byte/capacity migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-027 |
| 290 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-027 arena-slot byte/capacity closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-028 |
| 291 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-028 residence arena-binding geometry count/page-size migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-029 |
| 292 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-029 residence arena-binding geometry count/page-size closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-030 |
| 293 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-030 requirement-matrix geometry count/page-size migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-031 |
| 294 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-031 requirement-matrix geometry count/page-size closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-032 |
| 295 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-032 readiness geometry count/page-size migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-033 |
| 296 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-033 readiness geometry count/page-size closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-034 |
| 297 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-034 next exact-usize field-group selection` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-035 |
| 298 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-035 segment-map consume-ledger block/count migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-036 |
| 299 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-036 segment-map consume-ledger block/count closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-037 |
| 300 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-037 segment-map consume-ledger release block/count migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-038 |
| 301 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-038 segment-map consume-ledger release block/count closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-039 |
| 302 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-039 next exact-usize field-group selection` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-040 |
| 303 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-040 local-free reuse ledger count migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-041 |
| 304 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-041 local-free reuse ledger count closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-042 |
| 305 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-042 next exact-usize field-group selection` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-043 |
| 306 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-043 local-free reuse ledger release-apply count migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-044 |
| 307 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-044 local-free reuse ledger release-apply count closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-045 |
| 308 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-045 local-free reuse ledger release-apply primary counter migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-046 |
| 309 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-046 local-free reuse ledger release-apply primary counter closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-047 |
| 310 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-047 local-free reuse ledger release-apply shape/lookup reject counter migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-048 |
| 311 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-048 local-free reuse ledger release-apply shape/lookup reject counter closeout` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-049 |
| 312 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-049 local-free reuse ledger release-apply execution/capability reject counter migration` | landed; selected HAKO-ALLOC-USIZE-FIELD-GROUP-050 |
| 313 | Hakorune language / allocator cleanup | `HAKO-ALLOC-USIZE-FIELD-GROUP-050 local-free reuse ledger release-apply execution/capability reject counter closeout` | landed; selected HAKO-ALLOC-REPORT-RECORD-006 |
| 314 | Hakorune language / allocator cleanup | `HAKO-ALLOC-REPORT-RECORD-006 local-free reuse ledger release-apply ReportFields pilot` | landed; selected HAKO-ALLOC-REPORT-RECORD-007 |
| 315 | Hakorune language / allocator cleanup | `HAKO-ALLOC-REPORT-RECORD-007 local-free reuse ledger release-apply ReportFields closeout` | selected current |
| 125 | optional runtime | provider/host allocator replacement ladder | explicit future option only; not a mimalloc completion prerequisite |

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

## Report Record Cleanup Timing

Use report-record cleanup rows as short BoxShape stops when all of these hold:

```text
1. a closeout row has just bundled a behavior family
2. the next behavior row would otherwise add more all-i64 report boxes
3. the report is a diagnostic/proof payload, not an identity owner
4. one owner-local ReportFields record payload can improve the boundary without
   opening record pass/return/store escape
```

For the current arena-backing lane, the next such stop is:

```text
after:
  MIMAP-266A segment arena backing modeled source accounting closeout pack

before:
  MIMAP-268A segment arena backing modeled allocation plan inventory

row:
  HAKO-ALLOC-REPORT-RECORD-003 segment arena backing report record carrier inventory
```

If that inventory shows the real blocker is cross-function record return or
backend record materialization, select a focused compiler/language sidecar
instead of rewriting `.hako` reports around the missing capability.

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
