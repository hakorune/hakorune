---
Status: Active
Date: 2026-05-16
Lane: phase-293x mimalloc blueprint / port preparation
Canonical SSOT:
  - docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md
  - docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md
  - docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md
---

# Phase 293x mimalloc Port Taskboard

## Current Status

This board is active. The language-minimal prerequisite set has reached the
safe handoff point for mimalloc blueprint work.

Preferred handoff point:

```text
LOOP-003C/D complete
PACKED-003/004 complete
```

Current row and latest-card pointers live in `CURRENT_STATE.toml`. This
taskboard keeps durable row order and policy boundaries only; do not paste the
live current status here.

```text
read:
  docs/development/current/main/CURRENT_STATE.toml phase_status
  docs/development/current/main/CURRENT_STATE.toml latest_card_path
```

Closed cleanup sidecar:

```text
docs/development/current/main/phases/phase-293x/293x-mir-builder-diet-taskboard.md
```

## Active Source Policy

Upstream mimalloc source is local-only:

```text
.external/upstream/mimalloc/
```

Tracked output is docs only.

## Pure-First MIR Artifact / Diagnostics Sidecar

SSOT:

```text
docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md
```

Decision:

```text
before MIMAP-029B:
  fix the pure-first/selfhost route shape exposed by MIMAP-029A
  preflight and EXE build must consume the exact same MIR JSON artifact
  route unsupported must fail before backend emission when MIR metadata proves it
  long/no-output builds must report the active phase

not part of this sidecar:
  allocator behavior
  route capability widening
  backend helper-name matching
  provider/hook/global allocator activation
```

| Row | Status | Purpose | Ordering |
| --- | --- | --- | --- |
| `MIR-EMIT-SSOT-001` | landed | Split `--mir-in` / `--mir-out` and make pure-first EXE build consume the exact MIR artifact it preflighted. | before route preflight |
| `MIR-ROUTE-PREFLIGHT-001` | landed | Classify missing/unsupported lowering routes from MIR metadata before ny-llvmc / C shim emission. | after artifact exactness |
| `SELFHOST-PROGRESS-001` | landed | Add phase progress / timeout closeout for slow/stuck/unsupported build diagnosis. | after route preflight |
| `MIR-EMIT-SSOT-002` | landed | Make the canonical external source-to-MIR route explicit through `emit_mir_route.sh`. | after progress diagnostics |
| `RETURN-CONTRACT-001` | parked future | Propagate declared return expected type into return expressions such as `ArrayBox.get`. | not a blocker for artifact exactness |

Allocator row sequence after this sidecar is not repeated in this section.
Canonical row order lives in [Rows / First Executable Slices](#first-executable-slices);
the current active row lives in `CURRENT_STATE.toml`.

## Stage1 / Selfhost Ordering Guard

Stage1/selfhost SSOT:

```text
docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
```

Decision:

```text
mimalloc first:
  continue .hako / hako_alloc allocator completeness work
  use Stage1/selfhost route as monitor/proof
  add only narrow Stage1 semantics / MIR facts / substrate routes needed by
  allocator rows

post-mimalloc:
  reopen broad Stage1 .hako owner reduction
  continue mirbuilder-first / parser-after order
  keep selfhost owner-reduction commits separate from allocator behavior rows
```

| Row | Status | Purpose | Ordering |
| --- | --- | --- | --- |
| `MIMAP-STAGE1-ORDER-001` | landed | Pin that broad Stage1 `.hako` compiler migration is not a mimalloc prerequisite. | docs-only |
| `SELFHOST-POST-MIMAP-001` | parked | Reopen broad Stage1 `.hako` owner reduction after mimalloc completeness evidence. | after mimalloc closeout |

## Allocator Concurrency Substrate Cut

SSOT:

```text
docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md
```

Decision:

```text
mimalloc port:
  needs allocator-facing concurrency substrate
  does not require widening user-facing concurrency language features first
  does not replace the ordinary host/process malloc path

required substrate:
  worker/thread identity
  runtime/internal TLS or worker-local cache slots
  atomic load/store/CAS/fetch_add routes
  OS virtual memory reserve/commit/decommit
  thread-safe hako_mem ABI
  remote free / abandoned-owner / page ownership policy

not prerequisites:
  Channel
  task_scope
  scoped request context
  source-level worker_local syntax
  true parallel language semantics
  allocator-provider activation
  hook install
  default process allocator replacement
```

| Row | Status | Purpose | Ordering |
| --- | --- | --- | --- |
| `MIMAP-SUBSTRATE-CONC-001` | landed | Pin the boundary between allocator concurrency substrate and user-facing concurrency language features. | before route inventory |
| `MIMAP-SUBSTRATE-CONC-002` | landed | Inventory and guard existing hako.atomic / hako.tls / hako.osvm / hako.mem route facts so backend lowering reads MIR-owned routes, not raw helper names. | before MIMAP-021C |
| `MIMAP-021C` | landed | Facade page-source allocation-miss fallback. | after MIMAP-SUBSTRATE-CONC-002 |
| `MIMAP-WORKER-001` | landed | Add internal worker/thread identity substrate for allocator owner/cache policy; no source-level worker identity semantics. | after MIMAP-021C |
| `MIMAP-TLS-001` | landed | Add internal TLS / worker-local cache-slot substrate for allocator caches. | after MIMAP-WORKER-001 |
| `MIMAP-ATOMIC-001` | landed | Add allocator-facing atomic load/store/CAS/fetch_add route guard. | after MIMAP-TLS-001 |
| `MIMAP-REMOTE-001` | landed | Model remote free / abandoned-owner / page ownership policy on the existing atomic/TLS substrate. | after MIMAP-ATOMIC-001 |
| `MIMAP-THREADSAFE-ABI-001` | landed | Pin thread-safe `hako_mem` ABI requirements and smoke boundary. | after MIMAP-REMOTE-001 |
| `MIMAP-PAR-STRESS-001` | landed | Native multi-worker substrate stress for per-worker heaps and remote free pressure. | after MIMAP-THREADSAFE-ABI-001 |

## Collection / Automata Dependency Cut

Map/Set/FST work is tracked in:

```text
docs/development/current/main/design/collection-set-map-fst-task-breakdown-ssot.md
```

Decision for this board:

```text
Set:
  not a prerequisite for MIMAP-011

Map:
  existing MapBox / MapCoreBox is enough if a later row needs dynamic lookup

FST:
  not a mimalloc prerequisite
```

## Rows

### Source and Inventory

| Row | Status | Purpose | Expected size |
| --- | --- | --- | --- |
| `MIMAP-001` | landed | Upstream source pin: URL, commit/tag, license, inspected files, local path. | 1 commit |
| `MIMAP-002` | landed | Source concept inventory: segment/page/block/heap/free-list/size-class/os/stats. | 1-2 commits |
| `MIMAP-003` | landed | Lifecycle rewrite blueprint: enum states, transitions, guard points. | 1-2 commits |
| `MIMAP-004` | landed | Substrate/representation gap ledger from source evidence. | 1-2 commits |

### Hakorune Blueprint

| Row | Status | Purpose | Expected size |
| --- | --- | --- | --- |
| `MIMAP-005A` | landed | Define brand/type vocabulary: `Bytes`, `PageId`, `BlockId`, `SegmentId`, `Generation`. | 1 commit |
| `MIMAP-005B` | landed | Define record vocabulary for page/block refs, size-class entries, stats snapshots. | 1 commit |
| `MIMAP-005C` | landed | Define enum/transition lifecycle blueprint for page/segment state. | 1 commit |
| `MIMAP-005D` | landed | Define capability surface: `uses osvm`, `uses atomic`, `uses rawbuf` inventory. | 1 commit |

### First Executable Slices

| Row | Status | Purpose | Expected size |
| --- | --- | --- | --- |
| `MIMAP-006` | landed | Select first near-transcription executable slice. | 1 commit |
| `MIMAP-007` | landed | Size-class / bin map executable pilot. | 2-3 commits |
| `MIMAP-008` | landed | Page/free-list model pilot with direct executable proof and guard. | 1 commit |
| `MIMAP-009` | landed | Decommit/recommit/reuse lifecycle integration pilot. | 1 commit |
| `MIMAP-010` | landed | Page queue lifecycle selection pilot that skips decommitted pages and selects reusable pages explicitly. | 1 commit |
| `MIMAP-B001` | landed | Backend acceptance policy: VM scalar reference, LLVM/EXE MIMAP-011+ primary, VM timeout required. | 1 commit |
| `MIMAP-011` | landed | Allocator facade lifecycle route pilot using lifecycle-aware page selection; LLVM/EXE primary. | 1 commit |
| `MIMAP-012` | landed | Object-backed lifecycle queue LLVM route pilot; `ArrayBox` retains page objects and returns selected page. | 1 commit |
| `MIMAP-013` | landed | Facade composition over object-backed lifecycle queue. | after MIMAP-012 |
| `MIMAP-014A` | landed | Single-page small allocation fast-path over the facade-owned object lifecycle queue. | after MIMAP-013 |
| `MIMAP-014B` | landed | Reusable-page preference, active-page fallback, and allocation miss reason. | after MIMAP-014A |
| `MIMAP-014C` | landed | Allocation fast-path stats observers. | after MIMAP-014B |
| `MIMAP-015A` | landed | Release/free one known block through the facade. | after MIMAP-014C |
| `MIMAP-015B` | landed | Double-release / stale-release fail-fast route. | after MIMAP-015A |
| `MIMAP-016A` | landed | Alignment request metadata and observer result. | after MIMAP-015B |
| `MIMAP-016B` | landed | Aligned allocation success/fail route. | after MIMAP-016A |
| `MIMAP-017A` | landed | Realloc shrink / same-page route. | after release and alignment are stable |
| `MIMAP-017B` | landed | Realloc grow / move route. | after MIMAP-017A |
| `MIMAP-FACADE-CLEAN-001` | landed | Facade result observer / reason-code SSOT cleanup TODO. | after MIMAP-017B |
| `MIMAP-018A` | landed | Stats snapshot observer integration. | after allocation/release counters are stable |
| `MIMAP-019A` | landed | Purge/reclaim/decommit policy route. | after lifecycle and stats observers are stable |
| `MIMAP-020A` | landed | OSVM/page-source capability pilot; adopts the existing M49 page-source owner. | after in-memory facade route is stable |
| `MIMAP-021A` | landed | Post-020 allocator row selection. | after METADATA-CATALOG-004 |
| `MIMAP-021B` | landed | Facade page-source fresh-page attach. | after MIMAP-021A |
| `MIMAP-021C` | landed | Facade page-source allocation-miss fallback. | after MIMAP-SUBSTRATE-CONC-002 |
| `MIMAP-022A` | landed | Post-lifecycle allocator row selection. | after REUSE-LIFECYCLE-001 |
| `MIMAP-022B` | landed | Facade huge-request fail-fast routing before page-source attach/retry. | after MIMAP-022A |
| `MIMAP-022C` | landed | Post-huge-failfast allocator row selection. | after MIMAP-022B |
| `MIMAP-023A` | landed | Facade huge-page model route using the existing M180 huge-page model owner. | after MIMAP-022C |
| `MIMAP-023B` | landed | Post-huge-page-model allocator row selection. | after MIMAP-023A |
| `MIMAP-024A` | landed | Facade huge-release metadata route. | after MIMAP-023B |
| `MIMAP-024B` | landed | Post-huge-release allocator row selection. | after MIMAP-024A |
| `MIMAP-025A` | landed | Facade huge-release fail-fast diagnostics route. | after MIMAP-024B |
| `MIMAP-025B` | landed | Post-huge-release-failfast allocator row selection. | after MIMAP-025A |
| `MIMAP-026A` | landed | Facade huge-release page-map unregister route. | after MIMAP-025B |
| `MIMAP-026B` | landed | Post-huge-unregister allocator row selection. | after MIMAP-026A |
| `MIMAP-027A` | landed | Facade huge-unregister fail-fast diagnostics route. | after MIMAP-026B |
| `MIMAP-027B` | landed | Post-huge-unregister-failfast allocator row selection. | after MIMAP-027A |
| `MIMAP-028A` | landed | Facade huge page-source backing route. | after MIMAP-027B |
| `MIMAP-028B` | landed | Post-backed-huge allocator row selection. | selected MIMAP-029A |
| `MIMAP-029A` | landed | Facade huge decommit-after-unregister success route. | after MIMAP-028B |
| `MIR-EMIT-SSOT-001` | landed | Pure-first MIR artifact exactness: `--mir-in` / `--mir-out` and same artifact preflight/EXE build. | before MIMAP-029B |
| `MIR-ROUTE-PREFLIGHT-001` | landed | Lowering-plan route preflight before ny-llvmc / C shim emission. | after MIR-EMIT-SSOT-001 |
| `SELFHOST-PROGRESS-001` | landed | Selfhost/pure-first phase progress and timeout diagnostics. | after MIR-ROUTE-PREFLIGHT-001 |
| `MIR-EMIT-SSOT-002` | landed | Canonical external source-to-MIR wrapper. | after progress diagnostics |
| `MIMAP-029B` | landed | Post-huge-decommit allocator row selection. | selected MIMAP-030A |
| `MIMAP-030A` | landed | Facade huge decommit fail-fast diagnostics. | after MIMAP-029B |
| `MIMAP-030B` | landed | Post-huge-decommit-failfast allocator row selection. | selected MIMAP-031A |
| `MIMAP-031A` | landed | OSVM unreserve capability inventory / planning row. | selected MIMAP-032A |
| `MIMAP-032A` | landed | OSVM unreserve substrate route. | after MIMAP-031A |
| `MIMAP-032B` | landed | Post-OSVM-unreserve allocator row selection. | selected MIMAP-033A |
| `MIMAP-033A` | landed | Page-source unreserve adapter. | after MIMAP-032B |
| `MIMAP-033B` | landed | Post-page-source-unreserve row selection. | selected MIMAP-034A |
| `MIMAP-034A` | landed | Facade huge unreserve-after-decommit success route. | after MIMAP-033B |
| `MIMAP-034B` | landed | Post-huge-unreserve row selection. | selected MIMAP-035A |
| `MIMAP-035A` | landed | Facade huge unreserve duplicate/stale fail-fast diagnostics. | after MIMAP-034B |
| `MIMAP-035B` | landed | Post-huge-unreserve-failfast row selection. | selected MIMAP-036A |
| `MIMAP-036A` | landed | Post-huge-unreserve closeout guard. | after MIMAP-035B |
| `MIMAP-036B` | landed | Post-huge-unreserve-closeout row selection. | selected MIMAP-037A |
| `MIMAP-037A` | landed | Facade huge backing-set helper cleanup. | after MIMAP-036B |
| `MIMAP-037B` | landed | Post-backing-set-helper row selection. | selected MIMAP-038A |
| `MIMAP-038A` | landed | Object-lifecycle known-page loop cleanup. | after MIMAP-037B |
| `MIMAP-038B` | landed | Post-known-page-loop row selection. | selected MIMAP-039A |
| `MIMAP-039A` | landed | Remote-free retry-bound named owner cleanup. | after MIMAP-038B |
| `MIMAP-039B` | landed | Post-remote-free-retry-bound row selection. | selected MIR-ROW-C |
| `MIR-ROW-C` | landed | Nullable user-box object return sidecar. | after MIMAP-039B |
| `MIMAP-039C` | landed | Post-nullable-object-return row selection. | selected MIMAP-040A |
| `MIMAP-040A` | landed | Object-lifecycle selectPage queue-length loop cleanup. | after MIMAP-039C |
| `MIMAP-040B` | landed | Post-selectPage-loop row selection. | selected PURE-FIRST-DIAG-001 |
| `PURE-FIRST-DIAG-001` | landed | Pure-first acceptance layer/contract diagnostics. | after MIMAP-040B |
| `MIMAP-040C` | landed | Post-diagnostics row selection. | selected MIMAP-041A |
| `MIMAP-041A` | landed | Record report boundary cleanup for bounded purge/decommit scheduler. | after MIMAP-040C |
| `MIMAP-041B` | landed | Post-record-report row selection. | selected MIR-EXTERN-SPEC-001 |
| `MIR-EXTERN-SPEC-001` | landed | Extern-call route spec table cleanup. | after MIMAP-041B |
| `MIR-EXTERN-SPEC-002` | landed | Post-extern-spec row selection. | selected VMHAKO-EXTERN-SPEC-001 |
| `VMHAKO-EXTERN-SPEC-001` | landed | Subset legacy externcall validator uses `ExternCallRouteSpec` for route-backed rows. | after MIR-EXTERN-SPEC-002 |
| `VMHAKO-EXTERN-SPEC-002` | landed | Post-subset-validator row selection. | selected USERBOX-ROUTE-SPLIT-001 |
| `USERBOX-ROUTE-SPLIT-001` | landed | User-box method route fixed-point orchestration cleanup. | after VMHAKO-EXTERN-SPEC-002 |
| `USERBOX-ROUTE-SPLIT-002` | landed | Post-fixpoint row selection. | selected USERBOX-ROUTE-SPLIT-003 |
| `USERBOX-ROUTE-SPLIT-003` | landed | User-box method route materialization owner cleanup. | after USERBOX-ROUTE-SPLIT-002 |
| `USERBOX-ROUTE-SPLIT-004` | landed | Post-materialization row selection. | selected USERBOX-ROUTE-SPLIT-005 |
| `USERBOX-ROUTE-SPLIT-005` | landed | User-box method target collection owner cleanup. | after USERBOX-ROUTE-SPLIT-004 |
| `USERBOX-ROUTE-SPLIT-006` | landed | Post-target-collection row selection. | selected RECORD-VALUES-REG-001 |
| `RECORD-VALUES-REG-001` | landed | Builder-local record registration helper cleanup. | after USERBOX-ROUTE-SPLIT-006 |
| `RECORD-VALUES-REG-002` | landed | Post-record-values-helper row selection. | selected PROOF-APPS-MANIFEST-SCHEMA-001 |
| `PROOF-APPS-MANIFEST-SCHEMA-001` | landed | Proof-apps manifest schema cleanup for M214/M215 rows. | after RECORD-VALUES-REG-002 |
| `PROOF-APPS-MANIFEST-SCHEMA-002` | landed | Post-manifest row selection. | selected EXPRS-INDEXING-001 |
| `EXPRS-INDEXING-001` | landed | MIR builder indexing owner cleanup. | after PROOF-APPS-MANIFEST-SCHEMA-002 |
| `EXPRS-INDEXING-002` | landed | Post-indexing row selection. | selected EXPRS-COLLECTION-LITERAL-001 |
| `EXPRS-COLLECTION-LITERAL-001` | landed | MIR builder collection literal owner cleanup. | after EXPRS-INDEXING-002 |
| `EXPRS-COLLECTION-LITERAL-002` | landed | Post-collection-literal row selection. | selected EXPRS-CHECK-001 |
| `EXPRS-CHECK-001` | landed | MIR builder check expression owner cleanup. | after EXPRS-COLLECTION-LITERAL-002 |
| `EXPRS-CHECK-002` | landed | Post-check row selection. | selected OSVM-EXPORT-VALIDATION-HELPER-001 |
| `OSVM-EXPORT-VALIDATION-HELPER-001` | landed | OSVM export validation helper cleanup. | after EXPRS-CHECK-002 |
| `OSVM-EXPORT-VALIDATION-HELPER-002` | landed | Post-OSVM row selection. | selected GENERIC-METHOD-ROUTE-SPLIT-001 |
| `GENERIC-METHOD-ROUTE-SPLIT-001` | landed | Generic collection read route matcher cleanup. | after OSVM-EXPORT-VALIDATION-HELPER-002 |
| `GENERIC-METHOD-ROUTE-SPLIT-002` | landed | Post-read-route row selection. | selected GENERIC-METHOD-ROUTE-SPLIT-003 |
| `GENERIC-METHOD-ROUTE-SPLIT-003` | landed | Generic string route matcher cleanup. | after GENERIC-METHOD-ROUTE-SPLIT-002 |
| `GENERIC-METHOD-ROUTE-SPLIT-004` | landed | Post-string-route row selection. | selected GLOBAL-STRING-BODY-ANALYSIS-SPLIT-001 |
| `GLOBAL-STRING-BODY-ANALYSIS-SPLIT-001` | landed | Generic string body analysis phase split. | after GENERIC-METHOD-ROUTE-SPLIT-004 |
| `GLOBAL-STRING-BODY-ANALYSIS-SPLIT-002` | landed | Post-generic-string-body row selection. | selected NUMERIC-SUBSTRATE-SPLIT-001 |
| `NUMERIC-SUBSTRATE-SPLIT-001` | landed | Numeric substrate owner-layout cleanup. | after GLOBAL-STRING-BODY-ANALYSIS-SPLIT-002 |
| `NUMERIC-SUBSTRATE-SPLIT-002` | landed | Post-numeric-substrate row selection. | selected TYPED-OBJECT-STORAGE-INFERENCE-SPLIT-001 |
| `TYPED-OBJECT-STORAGE-INFERENCE-SPLIT-001` | landed | Typed-object storage inference owner-layout cleanup. | after NUMERIC-SUBSTRATE-SPLIT-002 |
| `TYPED-OBJECT-STORAGE-INFERENCE-SPLIT-002` | landed | Post-typed-object-storage row selection. | selected CURRENT-DOCS-PHASE-SLIM-001 |
| `CURRENT-DOCS-PHASE-SLIM-001` | landed | Current docs / phase taskboard slim cleanup. | after TYPED-OBJECT-STORAGE-INFERENCE-SPLIT-002 |
| `CURRENT-DOCS-PHASE-SLIM-002` | landed | Post-current-docs-slim row selection. | selected MIMAP-NEXT-BEHAVIOR-SELECTION-001 |
| `MIMAP-NEXT-BEHAVIOR-SELECTION-001` | landed | Select the next single allocator/compiler cleanup or behavior row. | selected MIMAP-042A |
| `MIMAP-042A` | landed | OSVM-backed fast-path bounded purge route. | after MIMAP-NEXT-BEHAVIOR-SELECTION-001 |
| `MIMAP-042B` | landed | Post-fast-path-purge route row selection. | selected MIMAP-043A |
| `MIMAP-043A` | landed | OSVM-backed fast-path recommit/reuse route. | after MIMAP-042B |
| `MIMAP-043B` | landed | Post-fast-path-reuse route row selection. | selected MIMAP-044A |
| `MIMAP-044A` | landed | OSVM-backed fast-path route closeout guard. | after MIMAP-043B |
| `MIMAP-044B` | landed | Post-fast-path-closeout row selection. | selected MIMAP-045A |
| `MIMAP-045A` | landed | OSVM-backed fast-path unreserve route. | after MIMAP-044B |
| `MIMAP-045B` | landed | Post-fast-path-unreserve row selection. | selected MIMAP-046A |
| `MIMAP-046A` | landed | OSVM-backed fast-path unreserve fail-fast diagnostics. | after MIMAP-045B |
| `MIMAP-046B` | landed | Post-fast-path-unreserve-failfast row selection. | selected MIMAP-047A |
| `MIMAP-047A` | landed | OSVM-backed fast-path unreserve closeout guard. | after MIMAP-046B |
| `MIMAP-047B` | landed | Post-fast-path-unreserve-closeout row selection. | selected MIMAP-048A |
| `MIMAP-048A` | landed | OSVM release capability inventory. | after MIMAP-047B |
| `MIMAP-048B` | landed | Post-release-inventory row selection. | selected MIMAP-049A |
| `MIMAP-049A` | landed | Secure entropy source inventory. | after MIMAP-048B |
| `MIMAP-049B` | landed | Post-secure-entropy-inventory row selection. | selected RANDOM-CAP-001 |
| `RANDOM-CAP-001` | landed | Uses random capability decision + fail-fast contract. | after MIMAP-049B |
| `RANDOM-CAP-002` | landed | Random capability unsupported-route preflight. | after RANDOM-CAP-001 |
| `MIMAP-050A` | landed | Secure entropy route proposal-or-park. | parked entropy execution; selected MIMAP-051A |
| `MIMAP-051A` | landed | Reclaim owner-transfer contract inventory. | after MIMAP-050A |
| `MIMAP-051B` | landed | Post-reclaim-contract row selection. | selected USES-002A |
| `USES-002A` | landed | Declared uses capability plan mapping. | after MIMAP-051B |
| `MIMAP-052A` | landed | Reclaim execution preflight proposal. | selected MIMAP-052B |
| `MIMAP-052B` | landed | Reclaim execution intent marker preflight. | selected MIMAP-053A |
| `MIMAP-053A` | landed | Reclaim execution support row selection. | selected MIMAP-054A |
| `MIMAP-054A` | landed | Reclaim atomic-claim contract. | selected MIMAP-055A |
| `MIMAP-055A` | landed | Reclaim owner-transfer first execution route. | selected MIMAP-056A |
| `MIMAP-056A` | landed | Reclaim remote-free drain contract inventory. | selected MIMAP-057A |
| `MIMAP-057A` | landed | Reclaim remote-free drain first execution route. | selected MIMAP-058A |
| `MIMAP-058A` | landed | Reclaim post-drain owner-transfer integration route. | selected MIMAP-059A |
| `MIMAP-059A` | landed | Post-reclaim-integration row selection. | selected MIMAP-060A |
| `MIMAP-060A` | landed | Reclaim completion marker route. | selected MIMAP-061A |
| `MIMAP-061A` | landed | Reclaim scalar lane closeout guard. | selected MIMAP-062A |
| `MIMAP-062A` | selected current | Post-reclaim-scalar-closeout row selection. | after MIMAP-061A |

Joint Hakorune / mimalloc ordering:

```text
docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md
```

Current row after MIMAP-061A: `MIMAP-062A` selects one narrow follow-up after
the scalar reclaim lane closeout. Thread scheduling, page-source calls, OSVM
unreserve/release, provider activation, and backend matchers remain closed.

MIMAP-020A execution order:

```text
020A.1 adopt existing M49 page-source owner:
  HakoAllocPageSourcePolicy
  HakoAllocProductionFacade.pageSource*

020A.2 rerun existing proof:
  bash tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh

020A.3 close the card without new code unless a MIMAP-specific seam is missing.
```

Split policy:

```text
MIMAP-020B:
  only if a mimalloc-facing page-source acceptance seam is missing

USES-002A:
  only if verifier-active method-level `uses osvm` becomes the blocker

allocator-provider ladder:
  only by explicit reopen; provider hooks, host replacement, and
  #[global_allocator] stay inactive
```

### Construction / Lifecycle Policy Rows

| Row | Status | Purpose | Ordering |
| --- | --- | --- | --- |
| `LIFECYCLE-BIRTH-001` | landed | Lock `birth` as a constructor hook fired only by `new`; direct receiver calls stay forbidden. | before parser widening |
| `PARSER-BIRTH-001` | landed | Add a negative parser fixture for `obj.birth(...)` so constructor policy does not regress. | before PARSER-BIRTH-002 |
| `PARSER-BIRTH-002` | landed | Improve direct-`birth` diagnostic with a `use new Box(...)` hint. | before REUSE-LIFECYCLE-001 |
| `NEW-NAMED-ARGS-001` | parked | Design named constructor arguments for `new Box(field: value, ...)`. | later; not a MIMAP-013 blocker |
| `REUSE-LIFECYCLE-001` | landed | Keep reuse as explicit methods such as `reset`, `reactivate`, `configure`, `clear`, and `attach` with contracts/transitions. | before MIMAP-022A |

Policy SSOT:

```text
docs/development/current/main/design/constructor-birth-new-lifecycle-ssot.md
```

Decision for this board:

```text
Do not fix constructor failures by accepting source-level obj.birth(...).
Use new Box(...) for construction and explicit lifecycle methods for reuse.
```

### MIR Object-Loop Acceptance Follow-up Rows

| Row | Status | Purpose | Ordering |
| --- | --- | --- | --- |
| `MIR-INV-MIMAP012` | ready | Pin the MIMAP-012 heavy loop/object shape investigation and minimized hypotheses. | before broadening MIMAP-012 shape |
| `MIR-ROW-A` | landed | Minimal fixture for `loop + if guard + pages.get(i)` with scalar result only; MIR JSON and LLVM/EXE pass. | after MIR-INV-MIMAP012 |
| `MIR-ROW-B` | ready | Add `considerPage(page)` helper call while selected state remains scalar; prove both MIR JSON and LLVM/EXE acceptance. | after MIR-ROW-A |
| `MIR-ROW-C` | landed | Accept nullable object return through loop-carried `select` / `phi`; prove both MIR JSON and LLVM/EXE acceptance before page queue loop cleanup. | selected by MIMAP-039B |
| `MIR-ROW-D` | landed | Refine void-placeholder object route results from same-module route contracts before nested receiver method checks. | triggered by MIMAP-042A |
| `MIR-ROW-A-FIX` | landed | Preserve or recover typed user-box receiver facts after dynamic `ArrayBox.get(i)` so `page.freeCount()` lowers as `HakoAllocPageModel.freeCount/0`, not `RuntimeDataBox.freeCount`. | before MIR-ROW-A closeout |

MIMAP-013 now composes the queue-length object queue from MIMAP-040A. Do not
combine helper-call object loops, facade selected-object exposure, dense proof
reads, and allocator execution in one row.

### MIMAP-013 landed row

Owner boundary:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako
```

Proof and guard:

```text
apps/mimalloc-facade-object-lifecycle-queue-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_object_lifecycle_queue_exe_guard.sh
```

Scope:

```text
HakoAllocObjectLifecycleFacade stores one HakoAllocObjectLifecyclePageQueue.
Facade methods forward page-object add, invoke queue selection, and expose
selected page identity and queue counters through read-only scalar observers.
No OSVM/page-source execution, provider hook, remote-free execution, host
allocator replacement, or backend shortcut is activated.
```

Landed evidence:

```text
bash tools/checks/k2_wide_mimalloc_facade_object_lifecycle_queue_exe_guard.sh
[mimap013-mir-json] ok
[k2-wide-mimalloc-facade-object-lifecycle-queue-exe] ok
```

### MIMAP-014A ready row

Owner boundary:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako
lang/src/hako_alloc/memory/page_box.hako
```

Expected proof and guard:

```text
apps/mimalloc-facade-small-alloc-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_small_alloc_exe_guard.sh
```

Scope:

```text
HakoAllocObjectLifecycleFacade selects one reusable page from its object
lifecycle queue and calls HakoAllocPageModel.acquire(size).
The row returns scalar observer data only: selected page id, block id, reason
code, and summary.
No release/free, realloc, alignment, OSVM/page-source, provider hook,
remote-free execution, host allocator replacement, selected-object return, or
backend shortcut is activated.
```

Stop condition:

```text
If helper-call object-loop shape blocks MIR JSON or LLVM/EXE, stop MIMAP-014A
and land MIR-ROW-B first with a minimized fixture.
If selected-object return is required, stop and land MIR-ROW-C instead of
broadening MIMAP-014A.
```

Acceptance split for every `MIR-ROW-*`:

```text
MIR JSON:
  parser / Stage1 / planner / emit can produce JSON for the shape

LLVM/EXE:
  the emitted route compiles and executes with the expected proof output

VM:
  diagnostic smoke only; VM object-heavy timeout is not a MIMAP blocker
```

Guard policy:

```text
each implemented MIR-ROW-* must add a k2_wide_*.sh guard
the guard must fail if MIR JSON generation fails
the guard must fail if LLVM/EXE execution fails
the guard must not treat VM timeout as success for the EXE route
```

Current MIR-ROW-A evidence:

```text
tools/checks/k2_wide_mimap012_object_loop_row_a_exe_guard.sh

MIR JSON:
  passes

LLVM/EXE:
  passes
  pages.get(i) result recovers HakoAllocPageModel origin
  page.freeCount() routes as a user-box method rather than RuntimeDataBox.freeCount

Guard:
  bash tools/checks/k2_wide_mimap012_object_loop_row_a_exe_guard.sh
  [mimap012-row-a-mir-json] ok
  [k2-wide-mimap012-object-loop-row-a-exe] ok
```

### Collection / Automata Sidecar Rows

| Row | Status | Purpose | Ordering |
| --- | --- | --- | --- |
| `COLL-001` | ready | Map/Set/HashMap naming and placement docs. | sidecar; not blocking MIMAP-011 |
| `COLL-002` | parked | Set semantic wrapper over Map. | after MIMAP-011 unless Set becomes the blocker |
| `COLL-003` | parked | Set proof app and guard. | after COLL-002 |
| `AUTO-001` | ready | FST placement SSOT. | sidecar; not mimalloc prerequisite |
| `AUTO-002` | parked | FST record vocabulary. | after evidence |
| `AUTO-003` | parked | Compiler keyword-table FST pilot. | compiler evidence only |

## Readiness Checklist

- [x] `.external/` is ignored before upstream source is cloned.
- [x] Upstream pin doc records commit/tag and license.
- [x] Source concepts are classified as `near-transcription`, `lifecycle-rewrite`, `substrate-gap`, `representation-gap`, or `deferred-unsafe`.
- [x] Blueprint uses Hakorune canonical surface only.
- [x] Provisional syntax is clearly marked and does not become implementation by accident.
- [ ] Executable slices have proof apps or guards.

## Stop Lines

```text
no vendored mimalloc source
no full port as the first row
no OSVM/provider/global allocator activation
no hooks / #[global_allocator]
no untracked design decision in implementation
no source-level receiver.birth(...) as lifecycle workaround
```


## Active cleanup sidecar

| Row | Status | Scope | Notes |
| --- | --- | --- | --- |
| `CLEAN-WHILE-001` | landed | While deletion readiness inventory. | BoxShape cleanup; do not mix with MIMAP-012. |
| `CLEAN-WHILE-002` | landed | Delete `ASTNode::While` after inventory. | Parser `while` stays canonical Loop. |

## Remaining cleanup sidecar

| Row | Status | Scope | Notes |
| --- | --- | --- | --- |
| `CLEAN-FOR-001` | landed | Decide legacy `parse_for_range_stage3` fate. | Legacy `for` quarantined; canonical source is `loop i in`. |
| `CLEAN-DEAD-001` | landed | Audit first `#[allow(dead_code)]` cluster. | `numeric_substrate` and `type_registry` classified as intentional staging. |
| `CLEAN-STAGE1-LOWERING-001` | landed | Split `expression_to_json_v0` into case helpers without changing Program(JSON v0) output. | BoxShape cleanup before more lowering rows. |
| `METADATA-CATALOG-001` | landed | Classify MIR metadata catalog and add drift guard. | BoxShape cleanup; no MIR JSON schema or backend behavior change. |
| `METADATA-CATALOG-002` | landed | Add metadata state, naming, Stage0 boundary, and CorePlan promotion policy. | BoxShape cleanup; no metadata struct split or backend behavior change. |
| `METADATA-CATALOG-003` | landed | Add metadata promotion matrix and near-term promotion queue. | BoxShape cleanup; queue landed through `METADATA-PROMOTE-006`. |
| `METADATA-PROMOTE-001` | landed | Guard active promotion matrix rows. | BoxShape cleanup; no MIR JSON schema change. |
| `METADATA-PROMOTE-002` | landed | Harden typed-object and static-data verifier contracts. | BoxShape cleanup; no backend behavior change. |
| `METADATA-PROMOTE-003` | landed | Add active function metadata contract rows. | BoxShape cleanup; no lowering behavior change. |
| `METADATA-PROMOTE-004` | landed | Record placement-effect consumer fold-up plan. | BoxShape cleanup; consumer migration remains future work. |
| `METADATA-PROMOTE-005` | landed | Fix PackedArray no-fallback contract before backend activation. | BoxShape cleanup; packed backend lowering remains closed. |
| `METADATA-PROMOTE-006` | landed | Add seed route retirement ledger. | BoxShape cleanup; no seed deletion or CorePlan promotion. |
| `METADATA-CATALOG-004` | landed | Reconcile post-promotion queue docs and taskboard visibility. | BoxShape cleanup after MIMAP-020A; no behavior change. |
| `CLEAN-TOKEN-STAGE3-001` | ready | Commonize the Stage-3 keyword token list in tokenizer ident classification. | Small follow-up; keep separate from lowering refactor if possible. |
| `CLEAN-AST-DECL-001` | parked | Evaluate `Local` / `Outbox` declaration unification. | Broad AST/API cleanup; do not mix with MIMAP-013. |
