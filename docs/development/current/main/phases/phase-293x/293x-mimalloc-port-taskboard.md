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

Slimming contract:

```text
current owner:
  CURRENT_STATE.toml

durable order owner:
  this taskboard's row tables

do not duplicate:
  the current MIMAP row window in sidecar sections
  latest-card history already covered by CURRENT_STATE.toml

allowed here:
  stable policy sidecars
  durable row-order tables
  short pointers to current row/card owners
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

## Guard Manifest Cleanup Sidecar

SSOT:

```text
docs/development/current/main/design/guard-manifest-migration-ssot.md
docs/tools/check-scripts-index.md
```

Decision:

```text
before adding many more allocator rows:
  keep stable shell entrypoints
  move proof app and row guard selection/routing into manifest-backed runners
  prevent app-local test.sh from directly re-encoding guard script choices

not part of this sidecar:
  allocator behavior
  compiler acceptance
  dev_gate / allocator-wide manifest pilot activation
  bulk deletion of k2_wide_* entrypoints
```

| Row | Status | Purpose | Ordering |
| --- | --- | --- | --- |
| `GUARD-MANIFEST-001` | landed | Convert manifest-backed proof app `test.sh` entries to `run_proof_app.sh --only <id>` and add a no-growth guard. | before more guard-heavy MIMAP rows |
| `GUARD-MANIFEST-002` | landed | Select recent hako_alloc segment closeout guards as the first `k2_wide_*` manifest-backed thin-wrapper family. | after GM001 |
| `GUARD-MANIFEST-003` | landed | Move the selected segment closeout guards behind manifest rows while keeping public `k2_wide_*` entrypoints stable. | after GM002 |
| `GUARD-MANIFEST-004` | landed | Select reclaim scheduler closeout guards as the next manifest-backed thin-wrapper family. | after GM003 |
| `GUARD-MANIFEST-005` | landed | Move the selected reclaim scheduler closeout guards behind manifest rows while keeping public `k2_wide_*` entrypoints stable. | after GM004 |
| `GUARD-MANIFEST-006` | landed | Select OSVM fast-path closeout guards as the next manifest-backed thin-wrapper family. | after GM005 |
| `GUARD-MANIFEST-007` | landed | Move the selected OSVM fast-path closeout guards behind manifest rows while keeping public `k2_wide_*` entrypoints stable. | after GM006 |
| `GUARD-MANIFEST-008` | landed | Select the final public hako_alloc closeout wrappers for manifest-backed migration. | after GM007 |
| `GUARD-MANIFEST-009` | landed | Move the final public hako_alloc closeout wrappers behind manifest rows while keeping public `k2_wide_*` entrypoints stable. | after GM008 |
| `GUARD-MANIFEST-010` | landed | Close the manifest-wrapper cleanup burst by deriving hako_alloc closeout wrapper expectations from `hako-alloc-closeout`, then return to MIMAP rows. | selected MIMAP-087A |

## Row Validation Profile Sidecar

SSOT:

```text
docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md
tools/checks/lib/manifest_runner.py
tools/checks/proof_apps.toml
```

Decision:

```text
before adding more segment-map / allocator scalar rows:
  keep proof-first row discipline
  classify rows by validation_profile / row_kind / closeout_pack in manifest
  make daily rows default to L0/L1/L2 unless an EXE-required rule fires
  reserve L3 EXE for backend-route / first-pattern / closeout evidence

not part of this sidecar:
  weakening existing public guards
  bulk guard deletion
  changing dev_gate / allocator-wide defaults
  decomposing every public guard into separate VM/MIR/EXE commands
```

| Row | Status | Purpose | Ordering |
| --- | --- | --- | --- |
| `ROW-VALIDATION-PROFILE-001` | landed | Add validation profile metadata to the manifest runner and seed the segment-map readiness family without changing guard execution semantics. | before MIMAP-154A implementation |
| `ROW-VALIDATION-PROFILE-002` | landed | Split the segment-map readiness family public guards so manifest `--level L2` can run VM/MIR/preflight without EXE while no-arg guards remain full L3. | before MIMAP-154A implementation |

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
| `MIMAP-062A` | landed | Post-reclaim-scalar-closeout row selection. | selected MIMAP-063A |
| `MIMAP-063A` | landed | Reclaim scheduler boundary inventory. | selected MIMAP-064A |
| `MIMAP-064A` | landed | Reclaim scheduler request marker contract. | selected MIMAP-065A |
| `MIMAP-065A` | landed | Reclaim scheduler marker closeout guard. | selected MIMAP-066A |
| `MIMAP-066A` | landed | Post-scheduler-marker row selection. | selected MIMAP-067A |
| `MIMAP-067A` | landed | Reclaim scheduler substrate proposal-or-park. | selected MIMAP-068A |
| `MIMAP-068A` | landed | Reclaim scheduler request ledger route. | selected MIMAP-069A |
| `MIMAP-069A` | landed | Reclaim scheduler request ledger closeout guard. | selected MIMAP-070A |
| `MIMAP-070A` | landed | Post-scheduler-ledger row selection. | selected MIMAP-071A |
| `MIMAP-071A` | landed | Reclaim scheduler request ledger consume route. | selected MIMAP-072A |
| `MIMAP-072A` | landed | Reclaim scheduler ledger consume closeout guard. | selected MIMAP-073A |
| `MIMAP-073A` | landed | Post-scheduler-consume row selection. | selected MIMAP-074A |
| `MIMAP-074A` | landed | Reclaim scheduler request ledger roundtrip route. | selected MIMAP-075A |
| `MIMAP-075A` | landed | Reclaim scheduler request ledger roundtrip closeout guard. | selected MIMAP-076A |
| `MIMAP-076A` | landed | Post-scheduler-roundtrip row selection. | selected MIMAP-077A |
| `MIMAP-077A` | landed | Reclaim scheduler scalar lane closeout guard. | selected MIMAP-078A |
| `MIMAP-078A` | landed | Post-scheduler-scalar-closeout row selection. | selected MIMAP-079A |
| `MIMAP-079A` | landed | Segment arena bitmap boundary inventory. | selected MIMAP-080A |
| `MIMAP-080A` | landed | Segment arena bitmap inventory closeout guard. | selected MIMAP-081A |
| `MIMAP-081A` | landed | Post-segment-arena-bitmap-inventory row selection. | selected MIMAP-082A |
| `MIMAP-082A` | landed | Segment lifecycle scalar state contract. | selected MIMAP-083A |
| `MIMAP-083A` | landed | Segment lifecycle scalar state closeout guard. | selected MIMAP-084A |
| `MIMAP-084A` | landed | Post-segment-lifecycle-closeout row selection. | selected MIMAP-085A |
| `MIMAP-085A` | landed | Segment page membership scalar contract. | selected MIMAP-086A |
| `MIMAP-086A` | landed | Segment page membership closeout guard. | selected MIMAP-087A |
| `MIMAP-087A` | landed | Post-segment-page-membership-closeout row selection. | selected MIMAP-088A |
| `MIMAP-088A` | landed | Segment allocation readiness scalar contract. | selected MIMAP-089A |
| `MIMAP-089A` | landed | Segment allocation readiness closeout guard. | selected MIMAP-090A |
| `MIMAP-090A` | landed | Post-segment-allocation-readiness row selection. | selected MIMAP-091A |
| `MIMAP-091A` | landed | Segment allocation modeled consume route. | selected MIMAP-092A |
| `MIMAP-092A` | landed | Segment allocation modeled consume closeout guard. | selected MIMAP-093A |
| `MIMAP-093A` | landed | Post-segment-allocation-modeled-consume row selection. | selected MIMAP-094A |
| `MIMAP-094A` | landed | Segment allocation modeled ledger route. | selected MIMAP-095A |
| `MIMAP-095A` | landed | Segment allocation modeled ledger closeout guard. | selected MIMAP-096A |
| `MIMAP-096A` | landed | Post-segment-allocation-modeled-ledger row selection. | selected MIMAP-097A |
| `MIMAP-097A` | landed | Segment allocation modeled ledger release route. | selected MIMAP-098A |
| `MIMAP-098A` | landed | Segment allocation modeled ledger release closeout guard. | selected MIMAP-099A |
| `MIMAP-099A` | landed | Post-segment-allocation-modeled-release row selection. | selected MIMAP-100A |
| `MIMAP-100A` | landed | Segment allocation modeled ledger released-token recycle route. | selected MIMAP-101A |
| `MIMAP-101A` | landed | Segment allocation modeled ledger released-token recycle closeout guard. | selected MIMAP-102A |
| `MIMAP-102A` | landed | Post-segment-allocation-modeled-recycle row selection. | selected HAKO-ALLOC-SRC-CLEAN-001 |
| `HAKO-ALLOC-SRC-CLEAN-001` | landed | Segment counter compound assignment cleanup. | selected MIMAP-103A |
| `MIMAP-103A` | landed | Post-segment-counter-cleanup row selection. | selected MIMAP-104A |
| `MIMAP-104A` | landed | Segment allocation modeled ledger release span facts route. | selected MIMAP-105A |
| `MIMAP-105A` | landed | Post-release-span-facts row selection. | selected MIMAP-ROW-CADENCE-001 |
| `MIMAP-ROW-CADENCE-001` | landed | Mimalloc row validation cadence SSOT. | selected MIMAP-106A |
| `MIMAP-106A` | landed | Post-validation-cadence row selection. | selected MIMAP-107A |
| `MIMAP-107A` | landed | Segment allocation modeled released-span ledger route. | selected MIMAP-108A |
| `MIMAP-108A` | landed | Post-released-span-ledger row selection. | selected MIMAP-109A |
| `MIMAP-109A` | landed | Segment allocation modeled local-free candidate ledger route. | selected MIMAP-110A |
| `MIMAP-110A` | landed | Post-local-free-candidate-ledger row selection. | selected MIMAP-111A |
| `MIMAP-111A` | landed | Segment allocation modeled local-free apply plan route. | selected MIMAP-112A |
| `MIMAP-112A` | landed | Post-local-free-apply-plan row selection. | selected MIMAP-113A |
| `MIMAP-113A` | landed | Segment allocation modeled local-free scalar lane closeout guard. | selected MIMAP-114A |
| `MIMAP-114A` | landed | Post-local-free-scalar-closeout row selection. | selected MIMAP-115A |
| `MIMAP-115A` | landed | Segment allocation modeled local-free page-model apply route. | selected MIMAP-116A |
| `MIMAP-116A` | landed | Post-local-free-page-apply row selection. | selected MIMAP-117A |
| `MIMAP-117A` | landed | Segment allocation modeled local-free page-apply closeout guard. | selected MIMAP-118A |
| `MIMAP-118A` | landed | Post-local-free-page-apply-closeout row selection. | selected MIMAP-119A |
| `MIMAP-119A` | landed | Segment allocation modeled local-free integration route. | selected MIMAP-120A |
| `MIMAP-120A` | landed | Post-local-free-integration row selection. | selected MIMAP-121A |
| `MIMAP-121A` | landed | Segment allocation modeled local-free integration closeout guard. | selected MIMAP-122A |
| `MIMAP-122A` | landed | Post-local-free-integration-closeout row selection. | selected PURE-FIRST-GLOBAL-CALL-001 |
| `PURE-FIRST-GLOBAL-CALL-001` | landed | Same-module static helper global-call route support. | selected MIMAP-123A |
| `MIMAP-123A` | landed | Post-same-module-global-call row selection. | selected ROUTE-FIXPOINT-001 |
| `ROUTE-FIXPOINT-001` | landed | Route refresh fixpoint owner extraction. | selected ROUTE-DIAG-VOCAB-001 |
| `ROUTE-DIAG-VOCAB-001` | landed | Route diagnostics vocabulary SSOT. | selected ROUTE-DIAG-VOCAB-002 |
| `GUARD-MANIFEST-011` | landed | Pure-first route thin wrapper pilot. | selected ROUTE-DIAG-VOCAB-001 |
| `ROUTE-DIAG-VOCAB-002` | landed | Preflight vocabulary drift guard. | selected MIMAP-124A |
| `MIMAP-124A` | landed | Post-route-diagnostics cleanup row selection. | selected RUNTIME-UNWRAP-001 |
| `RUNTIME-UNWRAP-001` | landed | Runtime lock expect messages. | selected WASM-LOG-001 |
| `WASM-LOG-001` | landed | WAT2WASM stable tags. | selected MIMAP-125A |
| `MIMAP-125A` | landed | Post-source-cleanup row selection. | selected MIMAP-126A |
| `MIMAP-126A` | landed | Segment allocation modeled local-free reuse route. | selected MIMAP-127A |
| `MIMAP-127A` | landed | Post-local-free-reuse row selection. | selected MIMAP-128A |
| `MIMAP-128A` | landed | Segment allocation modeled local-free reuse closeout guard. | selected MIMAP-129A |
| `MIMAP-129A` | landed | Post-local-free-reuse-closeout row selection. | selected MIMAP-130A |
| `MIMAP-130A` | landed | Segment allocation modeled local-free reuse ledger route. | selected MIMAP-131A |
| `MIMAP-131A` | landed | Post-local-free-reuse-ledger row selection. | selected MIMAP-132A |
| `MIMAP-132A` | landed | Segment allocation modeled local-free reuse ledger closeout guard. | selected MIMAP-133A |
| `MIMAP-133A` | landed | Post-local-free-reuse-ledger-closeout row selection. | selected MIMAP-134A |
| `MIMAP-134A` | landed | Segment allocation modeled local-free reuse ledger release route. | selected MIMAP-135A |
| `MIMAP-135A` | landed | Post-local-free-reuse-ledger-release row selection. | selected MIMAP-136A |
| `MIMAP-136A` | landed | Segment allocation modeled local-free reuse ledger release closeout guard. | selected MIMAP-137A |
| `MIMAP-137A` | landed | Post-local-free-reuse-ledger-release-closeout row selection. | selected MIMAP-138A |
| `MIMAP-138A` | landed | Segment allocation modeled local-free reuse ledger release apply route. | selected MIMAP-139A |
| `MIMAP-139A` | landed | Segment allocation modeled local-free reuse ledger release apply closeout guard. | selected MIMAP-140A |
| `MIMAP-140A` | landed | Post-local-free-reuse-ledger-release-apply-closeout row selection. | selected GUARD-MANIFEST-012 |
| `GUARD-MANIFEST-012` | landed | Guard manifest batch migration inventory. | selected GUARD-MANIFEST-013 |
| `GUARD-MANIFEST-013` | landed | Declarative guard spec pilot. | selected MIMAP-141A |
| `MIMAP-141A` | landed | Post-guard-spec-pilot row selection. | selected MIMAP-142A |
| `MIMAP-142A` | landed | Segment allocation modeled local-free reuse ledger release-applied recycle proof. | selected MIMAP-143A |
| `GUARD-MANIFEST-014` | landed | Proof app test wrapper backfill. | sidecar before MIMAP-143A closeout |
| `MIMAP-143A` | landed | Segment allocation modeled local-free reuse ledger release-applied recycle closeout guard. | selected MIMAP-144A |
| `MIMAP-144A` | landed | Post-release-applied-recycle-closeout row selection. | selected HAKO-ALLOC-ID-BRAND-001 |
| `HAKO-ALLOC-ID-BRAND-001` | landed | Allocator scalar ID brand application inventory. | selected PURE-FIRST-BRAND-CONSTRUCT-001 |
| `PURE-FIRST-BRAND-CONSTRUCT-001` | landed | Brand constructor MIR acceptance. | selected HAKO-ALLOC-ID-BRAND-002 |
| `HAKO-ALLOC-ID-BRAND-002` | landed | Allocator scalar ID brand first pilot. | selected HAKO-ALLOC-ID-BRAND-003 |
| `HAKO-ALLOC-ID-BRAND-003` | landed | Allocator scalar ID brand pilot closeout guard. | selected MIMAP-145A |
| `MIMAP-145A` | landed | Post-ID-brand-pilot-closeout row selection. | selected HAKO-ALLOC-REPORT-RECORD-001 |
| `HAKO-ALLOC-REPORT-RECORD-001` | landed | Allocator report record cleanup inventory. | selected HAKO-ALLOC-REPORT-RECORD-002 |
| `HAKO-ALLOC-REPORT-RECORD-002` | landed | Local-free integration report record boundary cleanup. | selected MIMAP-146A |
| `MIMAP-146A` | landed | Post-report-record-cleanup row selection. | selected HAKO-ALLOC-RESULT-API-001 |
| `HAKO-ALLOC-RESULT-API-001` | landed | Allocator Result/Option guard-let inventory. | selected PURE-FIRST-GUARDLET-ENUMMATCH-001 |
| `PURE-FIRST-GUARDLET-ENUMMATCH-001` | landed | Direct MIR guard-let EnumMatchExpr acceptance. | selected HAKO-ALLOC-RESULT-API-002 |
| `HAKO-ALLOC-RESULT-API-002` | landed | Allocator local-free Result guard-let pilot. | selected MIMAP-147A |
| `MIMAP-147A` | landed | Post-Result-guard-let-pilot row selection. | selected HAKO-ALLOC-RESULT-API-003 |
| `HAKO-ALLOC-RESULT-API-003` | landed | Allocator local-free remaining Result guard-let boundaries. | selected MIMAP-148A |
| `MIMAP-148A` | landed | Post-local-free-Result-boundary row selection. | selected MIMAP-149A |
| `MIMAP-149A` | landed | Segment allocation blocked-substrate matrix proof. | selected MIMAP-150A |
| `MIMAP-150A` | landed | Post-blocked-substrate-matrix row selection. | selected MIMAP-151A |
| `MIMAP-151A` | landed | Segment-map scalar lookup boundary inventory. | selected MIMAP-152A |
| `MIMAP-152A` | landed | Post-segment-map-scalar-lookup row selection. | selected MIMAP-153A |
| `MIMAP-153A` | landed | Segment-map lookup guarded readiness composition. | selected MIMAP-154A |
| `MIMAP-154A` | landed | Post-lookup-guarded-readiness row selection. | selected MIMAP-155A |
| `MIMAP-155A` | landed | Segment-map readiness validation pack closeout guard. | selected MIMAP-156A |
| `MIMAP-156A` | landed | Post-segment-map-readiness-closeout row selection. | selected MIMAP-157A |
| `MIMAP-157A` | landed | Segment-map accepted readiness modeled consume ledger route. | selected MIMAP-158A |
| `MIMAP-158A` | landed | Segment-map modeled consume ledger diagnostics. | selected MIMAP-159A |
| `MIMAP-159A` | landed | Segment-map modeled consume ledger closeout pack. | selected MIMAP-160A |
| `MIMAP-160A` | landed | Post-segment-map-modeled-consume-ledger-closeout row selection. | selected MIMAP-161A |
| `MIMAP-161A` | landed | Segment-map modeled consume ledger release route. | selected MIMAP-162A |
| `MIMAP-162A` | landed | Segment-map modeled consume ledger release closeout pack. | selected MIMAP-163A |
| `MIMAP-163A` | landed | Post-segment-map-modeled-consume-ledger-release-closeout row selection. | selected MIMAP-164A |
| `MIMAP-164A` | landed | Segment-map modeled consume ledger released-token recycle route. | selected MIMAP-165A |
| `MIMAP-165A` | landed | Post-segment-map-modeled-consume-ledger-released-token-recycle row selection. | selected MIMAP-166A |
| `MIMAP-166A` | landed | Segment-map modeled consume ledger released-token recycle closeout pack. | selected MIMAP-167A |
| `MIMAP-167A` | landed | Post-segment-map-modeled-consume-ledger-released-token-recycle-closeout row selection. | selected MIMAP-168A |
| `MIMAP-168A` | landed | Segment-map modeled consume ledger released-span observation route. | selected MIMAP-169A |
| `MIMAP-169A` | landed | Post-segment-map-modeled-consume-ledger-released-span-observation row selection. | selected MIMAP-170A |
| `MIMAP-170A` | landed | Segment-map modeled consume ledger released-span observation closeout pack. | selected MIMAP-171A |
| `MIMAP-171A` | landed | Post-segment-map-modeled-consume-ledger-released-span-observation-closeout row selection. | selected MIMAP-172A |
| `MIMAP-172A` | landed | Segment-map released-span local-free candidate bridge. | selected MIMAP-173A |
| `MIMAP-173A` | landed | Post-segment-map-released-span-local-free-candidate-bridge row selection. | selected MIMAP-174A |
| `MIMAP-174A` | landed | Segment-map released-span local-free candidate bridge closeout pack. | selected MIMAP-175A |
| `MIMAP-175A` | landed | Post-segment-map-released-span-local-free-candidate-bridge-closeout row selection. | selected MIMAP-176A |
| `MIMAP-176A` | landed | Segment-map local-free apply-plan bridge. | selected MIMAP-177A |
| `MIMAP-177A` | landed | Post-segment-map-local-free-apply-plan-bridge row selection. | selected MIMAP-178A |
| `MIMAP-178A` | landed | Segment-map local-free apply-plan bridge closeout pack. | selected MIMAP-179A |
| `MIMAP-179A` | landed | Post-segment-map-local-free-apply-plan-bridge-closeout row selection. | selected MIMAP-180A |
| `MIMAP-180A` | landed | Segment-map local-free page-apply bridge. | selected MIMAP-181A |
| `MIMAP-181A` | landed | Post-segment-map-local-free-page-apply-bridge row selection. | selected MIMAP-182A |
| `MIMAP-182A` | landed | Segment-map local-free page-apply bridge closeout pack. | selected MIMAP-183A |
| `MIMAP-183A` | landed | Post-segment-map-local-free-page-apply-bridge-closeout row selection. | selected MIMAP-184A |
| `MIMAP-184A` | landed | Segment-map local-free integration bridge. | selected MIMAP-185A |
| `MIMAP-185A` | landed | Post-segment-map-local-free-integration-bridge row selection. | selected MIMAP-186A |
| `MIMAP-186A` | landed | Segment-map local-free integration bridge closeout pack. | selected MIMAP-187A |
| `MIMAP-187A` | landed | Post-segment-map-local-free-integration-bridge-closeout row selection. | selected MIMAP-188A |
| `MIMAP-188A` | landed | Segment-map local-free reuse bridge. | selected MIMAP-189A |
| `MIMAP-189A` | landed | Post-segment-map-local-free-reuse-bridge row selection. | selected MIMAP-190A |
| `MIMAP-190A` | landed | Segment-map local-free reuse bridge closeout pack. | selected MIMAP-191A |
| `MIMAP-191A` | landed | Post-segment-map-local-free-reuse-bridge-closeout row selection. | selected MIMAP-192A |
| `MIMAP-192A` | landed | Segment-map local-free reuse ledger bridge. | selected MIMAP-193A |
| `MIMAP-193A` | landed | Post-segment-map-local-free-reuse-ledger-bridge row selection. | selected MIMAP-194A |
| `MIMAP-194A` | landed | Segment-map local-free reuse ledger bridge closeout pack. | selected MIMAP-195A |
| `MIMAP-195A` | landed | Post-segment-map-local-free-reuse-ledger-bridge-closeout row selection. | selected MIMAP-196A |
| `MIMAP-196A` | landed | Segment-map local-free reuse ledger release bridge. | selected MIMAP-197A |
| `MIMAP-197A` | landed | Post-segment-map-local-free-reuse-ledger-release-bridge row selection. | selected MIMAP-198A |
| `MIMAP-198A` | landed | Segment-map local-free reuse ledger release bridge closeout pack. | selected MIMAP-199A |
| `MIMAP-199A` | landed | Post-segment-map-local-free-reuse-ledger-release-bridge-closeout row selection. | selected MIMAP-200A |
| `MIMAP-200A` | landed | Segment-map local-free reuse ledger release apply bridge. | selected MIMAP-201A |
| `MIMAP-201A` | landed | Post-segment-map-local-free-reuse-ledger-release-apply-bridge row selection. | selected MIMAP-202A |
| `MIMAP-202A` | landed | Segment-map local-free reuse ledger release apply bridge closeout pack. | selected MIMAP-203A |
| `MIMAP-203A` | landed | Post-segment-map-local-free-reuse-ledger-release-apply-bridge-closeout row selection. | selected MIMAP-204A |
| `MIMAP-204A` | landed | Segment-map local-free reuse ledger release-applied recycle bridge. | selected MIMAP-205A |
| `MIMAP-205A` | landed | Post-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge row selection. | selected MIMAP-206A |
| `MIMAP-206A` | landed | Segment-map local-free reuse ledger release-applied recycle bridge closeout pack. | selected MIMAP-207A |
| `MIMAP-207A` | landed | Post-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge-closeout row selection. | selected MIMAP-208A |
| `MIMAP-208A` | landed | Segment-map local-free reuse ledger release-applied recycle second-release diagnostic. | selected MIMAP-209A |
| `MIMAP-209A` | landed | Post-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic row selection. | selected MIMAP-210A |
| `MIMAP-210A` | landed | Segment-map local-free reuse ledger release-applied recycle second-release diagnostic closeout pack. | selected MIMAP-211A |
| `MIMAP-211A` | landed | Post-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-closeout row selection. | selected MIMAP-212A |
| `MIMAP-212A` | landed | Segment-map local-free reuse ledger lifecycle-token pilot. | selected MIMAP-213A |
| `MIMAP-213A` | landed | Post-segment-map-local-free-reuse-ledger-lifecycle-token-pilot row selection. | selected MIMAP-214A |
| `MIMAP-214A` | landed | Segment-map local-free reuse ledger lifecycle-token pilot closeout pack. | selected MIMAP-215A |
| `MIMAP-215A` | landed | Post-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-closeout row selection. | selected MIMAP-216A |
| `MIMAP-216A` | landed | Segment-map local-free reuse ledger lifecycle-token observer diagnostic. | selected MIMAP-217A |
| `MIMAP-217A` | landed | Post-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic row selection. | selected MIMAP-218A |
| `MIMAP-218A` | landed | Segment-map local-free reuse ledger lifecycle-token observer diagnostic closeout pack. | selected MIMAP-219A |
| `MIMAP-219A` | landed | Post-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-closeout row selection. | selected MIMAP-220A |
| `MIMAP-220A` | landed | Segment-map local-free reuse ledger lifecycle-token release-key precondition observer. | selected MIMAP-221A |
| `MIMAP-221A` | landed | Post-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition row selection. | selected MIMAP-222A |
| `MIMAP-222A` | landed | Segment-map local-free reuse ledger lifecycle-token release-key precondition closeout pack. | selected MIMAP-223A |
| `MIMAP-223A` | landed | Post-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-closeout row selection. | selected MIMAP-224A |
| `MIMAP-224A` | landed | Segment-map local-free reuse ledger lifecycle-keyed release shadow pilot. | selected MIMAP-225A |
| `MIMAP-225A` | landed | Post-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow row selection. | selected MIMAP-226A |
| `MIMAP-226A` | landed | Segment-map local-free reuse ledger lifecycle-keyed release shadow closeout pack. | selected MIMAP-227A |
| `MIMAP-227A` | landed | Post-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-closeout row selection. | selected MIMAP-228A |
| `MIMAP-228A` | landed | Source release-ledger lifecycle-key migration pilot. | selected MIMAP-229A |
| `MIMAP-229A` | landed | Source lifecycle-keyed release ledger diagnostics. | selected MIMAP-230A |
| `MIMAP-230A` | landed | Source release-ledger lifecycle-key migration closeout pack. | selected MIMAP-231A |
| `MIMAP-231A` | landed | Post source release-ledger lifecycle-key migration closeout row selection. | selected MIMAP-232A |
| `MIMAP-232A` | landed | Source lifecycle-keyed release apply/recycle continuation bridge. | selected MIMAP-233A |
| `MIMAP-233A` | landed | Source lifecycle-keyed release apply/recycle continuation diagnostics. | selected MIMAP-234A |
| `MIMAP-234A` | landed | Source lifecycle-keyed release apply/recycle continuation closeout pack. | selected MIMAP-235A |
| `MIMAP-235A` | landed | Post source lifecycle-keyed release apply/recycle continuation closeout row selection. | selected MIMAP-236A |
| `MIMAP-236A` | landed | Segment arena backing readiness inventory. | selected MIMAP-237A |
| `MIMAP-237A` | landed | Segment arena backing readiness diagnostics. | selected MIMAP-238A |
| `MIMAP-238A` | landed | Segment arena backing readiness closeout pack. | selected MIMAP-239A |
| `MIMAP-239A` | landed | Post segment arena backing readiness closeout row selection. | selected MIMAP-240A |
| `MIMAP-240A` | landed | Segment arena backing scalar requirement matrix inventory. | selected MIMAP-241A |
| `MIMAP-241A` | landed | Segment arena backing requirement matrix diagnostics. | selected MIMAP-242A |
| `MIMAP-242A` | landed | Segment arena backing requirement matrix closeout pack. | selected MIMAP-243A |
| `MIMAP-243A` | landed | Post segment arena backing requirement matrix closeout row selection. | selected MIMAP-244A |
| `MIMAP-244A` | landed | Segment arena backing no-escape raw pointer capability inventory. | selected MIMAP-245A |
| `MIMAP-245A` | landed | Segment arena backing no-escape address capability diagnostics. | selected MIMAP-246A |
| `MIMAP-246A` | landed | Segment arena backing no-escape address capability closeout pack. | selected MIMAP-247A |
| `MIMAP-247A` | landed | Post segment arena backing no-escape address capability closeout row selection. | selected MIMAP-248A |
| `MIMAP-248A` | landed | Segment arena backing modeled no-escape address residence inventory. | selected MIMAP-249A |
| `MIMAP-249A` | landed | Segment arena backing modeled no-escape address residence diagnostics. | selected MIMAP-250A |
| `MIMAP-250A` | landed | Segment arena backing modeled no-escape address residence closeout pack. | selected MIMAP-251A |
| `MIMAP-251A` | landed | Post segment arena backing modeled no-escape address residence closeout row selection. | selected MIMAP-252A |
| `MIMAP-252A` | landed | Segment arena backing modeled residence arena-binding inventory. | selected MIMAP-253A |
| `MIMAP-253A` | landed | Segment arena backing modeled residence arena-binding diagnostics. | selected MIMAP-254A |
| `MIMAP-254A` | landed | Segment arena backing modeled residence arena-binding closeout pack. | selected MIMAP-255A |
| `MIMAP-255A` | landed | Post segment arena backing modeled residence arena-binding closeout row selection. | selected MIMAP-256A |
| `MIMAP-256A` | landed | Segment arena backing modeled arena slot inventory. | selected MIMAP-257A |
| `MIMAP-257A` | landed | Segment arena backing modeled arena slot diagnostics. | selected MIMAP-258A |
| `MIMAP-258A` | landed | Segment arena backing modeled arena slot closeout pack. | selected MIMAP-259A |
| `MIMAP-259A` | landed | Post segment arena backing modeled arena slot closeout row selection. | selected MIMAP-260A |
| `MIMAP-260A` | landed | Segment arena backing modeled source bridge inventory. | selected MIMAP-261A |
| `MIMAP-261A` | landed | Segment arena backing modeled source bridge diagnostics. | selected MIMAP-262A |
| `MIMAP-262A` | landed | Segment arena backing modeled source bridge closeout pack. | selected MIMAP-263A |
| `MIMAP-263A` | landed | Post segment arena backing modeled source bridge closeout row selection. | selected MIMAP-264A |
| `MIMAP-264A` | landed | Segment arena backing modeled source accounting inventory. | selected MIMAP-265A |
| `MIMAP-265A` | landed | Segment arena backing modeled source accounting diagnostics. | selected MIMAP-266A |
| `MIMAP-266A` | landed | Segment arena backing modeled source accounting closeout pack. | selected HAKO-ALLOC-REPORT-RECORD-003 |
| `HAKO-ALLOC-REPORT-RECORD-003` | landed | Segment arena backing report record carrier inventory. | selected HAKO-ALLOC-REPORT-RECORD-004 |
| `HAKO-ALLOC-REPORT-RECORD-004` | landed | Segment arena backing source accounting diagnostic ReportFields pilot. | selected MIMAP-267A |
| `MIMAP-267A` | landed | Post segment arena backing ReportFields pilot row selection. | selected MIMAP-268A |
| `MIMAP-268A` | landed | Segment arena backing modeled allocation plan inventory. | selected MIMAP-269A |
| `MIMAP-269A` | landed | Segment arena backing modeled allocation plan diagnostics. | selected MIMAP-270A |
| `MIMAP-270A` | landed | Segment arena backing modeled allocation plan closeout pack. | selected MIMAP-271A |
| `MIMAP-271A` | landed | Post segment arena backing modeled allocation plan closeout row selection. | selected MIMAP-272A |
| `MIMAP-272A` | landed | Segment arena backing modeled allocation apply inventory. | selected MIMAP-273A |
| `MIMAP-273A` | landed | Segment arena backing modeled allocation apply diagnostics. | selected MIMAP-274A |
| `MIMAP-274A` | landed | Segment arena backing modeled allocation apply closeout pack. | selected MIMAP-275A |
| `MIMAP-275A` | landed | Post segment arena backing modeled allocation apply closeout row selection. | selected MIMAP-276A |
| `MIMAP-276A` | landed | Segment arena backing modeled allocation ledger inventory. | selected MIMAP-277A |
| `MIMAP-277A` | landed | Segment arena backing modeled allocation ledger diagnostics. | selected MIMAP-278A |
| `MIMAP-278A` | landed | Segment arena backing modeled allocation ledger closeout pack. | selected MIMAP-279A |
| `MIMAP-279A` | landed | Post segment arena backing modeled allocation ledger closeout row selection. | selected MIMAP-280A |
| `MIMAP-280A` | landed | Segment arena backing modeled allocation-ledger release candidate inventory. | selected HAKO-ALLOC-REPORT-RECORD-005 |
| `HAKO-ALLOC-REPORT-RECORD-005` | landed | Allocation-ledger release candidate ReportFields pilot. | selected MIMAP-281A |
| `MIMAP-281A` | landed | Segment arena backing modeled allocation-ledger release candidate diagnostics. | selected MIMAP-282A |
| `MIMAP-282A` | landed | Segment arena backing modeled allocation-ledger release candidate closeout pack. | selected HAKO-ALLOC-USIZE-FIELD-GROUP-001 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-001` | landed | Select release-candidate report byte/capacity fields as the first allocator exact-`usize` field-group pilot after release-candidate closeout. | selected FIELD-GROUP-002 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-002` | landed | Migrate release-candidate report byte/capacity fields only; keep reason/status/token/sentinel fields on `i64`. | selected FIELD-GROUP-003 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-003` | landed | Close out the first allocator exact-`usize` stored field group and keep the migration evidence bounded. | selected FIELD-GROUP-004 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-004` | landed | Migrate release-candidate diagnostic mirror byte fields only; keep diagnostic counters, reasons, tokens, ids, and sentinels on `i64`. | selected FIELD-GROUP-005 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-005` | landed | Close out the release-candidate diagnostic byte mirror field group and keep the evidence bounded. | selected FIELD-GROUP-006 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-006` | landed | Migrate allocation-ledger report byte/capacity fields only; keep counters, reasons, tokens, ids, and sentinels on `i64`. | selected FIELD-GROUP-007 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-007` | landed | Close out the allocation-ledger byte/capacity field group and keep the evidence bounded. | selected FIELD-GROUP-008 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-008` | landed | Migrate allocation-ledger diagnostic mirror byte fields only; keep diagnostic counters, reasons, tokens, ids, and sentinels on `i64`. | selected FIELD-GROUP-009 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-009` | landed | Close out the allocation-ledger diagnostic byte mirror field group and keep the evidence bounded. | selected FIELD-GROUP-010 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-010` | landed | Migrate allocation-apply report byte/capacity fields only; keep counters, reasons, tokens, ids, and sentinels on `i64`. | selected FIELD-GROUP-011 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-011` | landed | Close out the allocation-apply byte/capacity field group and keep the evidence bounded. | selected FIELD-GROUP-012 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-012` | landed | Migrate allocation-apply diagnostic mirror byte fields only; keep diagnostic counters, reasons, tokens, ids, and sentinels on `i64`. | selected FIELD-GROUP-013 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-013` | landed | Close out the allocation-apply diagnostic byte mirror field group and keep the evidence bounded. | selected FIELD-GROUP-014 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-014` | landed | Migrate allocation-plan report byte/capacity fields only; keep counters, reasons, tokens, ids, and sentinels on `i64`. | selected FIELD-GROUP-015 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-015` | landed | Close out the allocation-plan byte/capacity field group and keep the evidence bounded. | selected FIELD-GROUP-016 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-016` | landed | Migrate allocation-plan diagnostic mirror byte fields only; keep diagnostic counters, reasons, tokens, ids, and sentinels on `i64`. | selected FIELD-GROUP-017 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-017` | landed | Close out the allocation-plan diagnostic byte mirror field group and keep the evidence bounded. | selected FIELD-GROUP-018 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-018` | landed | Migrate source-accounting report byte/capacity fields only; keep counters, reasons, tokens, ids, and sentinels on `i64`. | selected FIELD-GROUP-019 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-019` | landed | Close out the source-accounting byte/capacity field group and keep the evidence bounded. | selected FIELD-GROUP-020 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-020` | landed | Migrate source-accounting diagnostic mirror byte fields only; keep diagnostic counters, reasons, tokens, ids, and sentinels on `i64`. | selected FIELD-GROUP-021 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-021` | landed | Close out the source-accounting diagnostic byte mirror field group and keep the evidence bounded. | selected FIELD-GROUP-022 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-022` | landed | Migrate source-bridge report byte/capacity fields only; keep counters, reasons, tokens, ids, alignments, and sentinels on `i64`. | selected FIELD-GROUP-023 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-023` | landed | Close out the source-bridge byte/capacity field group and keep the evidence bounded. | selected FIELD-GROUP-024 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-024` | landed | Migrate source-bridge diagnostic mirror byte fields only; keep diagnostic counters, reasons, tokens, ids, alignments, and sentinels on `i64`. | selected FIELD-GROUP-025 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-025` | landed | Close out the source-bridge diagnostic byte mirror field group and keep the evidence bounded. | selected FIELD-GROUP-026 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-026` | landed | Migrate arena-slot report byte/capacity fields only; keep counters, reasons, tokens, ids, alignments, geometry, and sentinels on `i64`. | selected FIELD-GROUP-027 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-027` | landed | Close out the arena-slot byte/capacity field group and keep the evidence bounded. | selected FIELD-GROUP-028 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-028` | landed | Migrate the residence arena-binding geometry count / page-size group only; keep alignments, counters, reasons, tokens, ids, and sentinels on `i64`. | selected FIELD-GROUP-029 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-029` | landed | Close out the residence arena-binding geometry count / page-size field group and keep the evidence bounded. | selected FIELD-GROUP-030 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-030` | landed | Migrate the requirement-matrix geometry count / page-size group only; keep alignments, counters, reasons, tokens, ids, and sentinels on `i64`. | selected FIELD-GROUP-031 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-031` | landed | Close out the requirement-matrix geometry count / page-size field group and keep the evidence bounded. | selected FIELD-GROUP-032 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-032` | landed | Migrate the readiness geometry count / page-size group only; keep alignments, counters, reasons, tokens, ids, and sentinels on `i64`. | selected FIELD-GROUP-033 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-033` | landed | Close out the readiness geometry count / page-size field group and keep the evidence bounded. | selected FIELD-GROUP-034 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-034` | landed | Select the next exact-`usize` stored field group after closing the arena-backing geometry chain. | selected FIELD-GROUP-035 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-035` | landed | Migrate the segment-map accepted-readiness modeled consume-ledger block/count report group only; keep reasons, ids, indexes, tokens, block-start sentinels, and owner counters on `i64`. | selected FIELD-GROUP-036 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-036` | landed | Close out the segment-map consume-ledger block/count field group and keep the evidence bounded. | selected FIELD-GROUP-037 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-037` | landed | Migrate the segment-map consume-ledger release block/count report group only; keep reasons, ids, indexes, tokens, block-span sentinels, and owner counters on `i64`. | selected FIELD-GROUP-038 |
| `HAKO-ALLOC-USIZE-FIELD-GROUP-038` | selected current | Close out the segment-map consume-ledger release block/count field group and keep the evidence bounded. | after FIELD-GROUP-037 |

Joint Hakorune / mimalloc ordering:

```text
docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md
```

Current row:
`HAKO-ALLOC-USIZE-FIELD-GROUP-038` closes out the segment-map modeled
consume-ledger release report block/count field group:

```text
live_before
live_after
ledger_count_after
ledger_live_count_after
released_blocks
```

Real pointer residence, pointer-derived lookup, real thread scheduling, worker
spawning, source-level concurrency features, real arena backing allocation,
atomic bitmap execution, page-source calls, OSVM unreserve/release, provider
activation, and backend matchers remain closed.

Exact-`usize` follow-up order:

```text
MIMAP-282A closeout first.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-001 selects release-candidate byte/capacity
fields.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-002 migrates that owner-local group only.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-003 closes out that first field group before
selecting the next narrow group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-004 migrates the diagnostic mirror byte fields
that copy already-migrated release-candidate byte facts.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-005 closes out that diagnostic mirror group
before selecting another allocator byte/capacity group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-006 migrates the allocation-ledger report
byte/capacity group that feeds the release-candidate family.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-007 closes out that allocation-ledger group
before selecting another allocator byte/capacity group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-008 migrates the allocation-ledger diagnostic
mirror byte fields that copy already-migrated allocation-ledger byte facts.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-009 closes out that diagnostic mirror group
before selecting another allocator byte/capacity group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-010 migrates the allocation-apply report
byte/capacity group that feeds the allocation-ledger family.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-011 closes out that allocation-apply group
before selecting another allocator byte/capacity group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-012 migrates the allocation-apply diagnostic
mirror byte fields that copy already-migrated allocation-apply byte facts.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-013 closes out that diagnostic mirror group
before selecting another allocator byte/capacity group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-014 migrates the allocation-plan report
byte/capacity group that feeds the allocation-apply family.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-015 closes out that allocation-plan group
before selecting another allocator byte/capacity group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-016 migrates the allocation-plan diagnostic
mirror byte fields that copy already-migrated allocation-plan byte facts.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-017 closes out that diagnostic mirror group
before selecting another allocator byte/capacity group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-018 migrates the source-accounting report
byte/capacity group that feeds the allocation-plan family.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-019 closes out that source-accounting group
before selecting another allocator byte/capacity group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-020 migrates the source-accounting diagnostic
mirror byte fields that copy already-migrated source-accounting byte facts.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-021 closes out that source-accounting
diagnostic mirror group before selecting another allocator byte/capacity group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-022 migrates the source-bridge report
byte/capacity group that feeds the source-accounting family.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-023 closes out that source-bridge group before
selecting another allocator byte/capacity group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-024 migrates the source-bridge diagnostic
mirror byte fields that copy already-migrated source-bridge byte facts.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-025 closes out that source-bridge diagnostic
mirror group before selecting another allocator byte/capacity group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-026 migrates the arena-slot report
byte/capacity group that feeds the source-bridge family.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-027 closes out that arena-slot group before
selecting another allocator exact-`usize` group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-028 migrates the residence arena-binding
geometry count / page-size group that feeds the already-migrated arena-slot
family. This is intentionally not a byte/capacity row.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-029 closes out that residence arena-binding
geometry count / page-size group before selecting another allocator
exact-`usize` field group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-030 migrates the requirement-matrix geometry
count / page-size group that feeds the already-migrated residence arena-binding
family. This is intentionally not a byte/capacity row.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-031 closes out that requirement-matrix
geometry count / page-size group before selecting another allocator
exact-`usize` field group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-032 migrates the readiness inventory geometry
count / page-size group that feeds the already-migrated requirement-matrix
family. This is intentionally not a byte/capacity row.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-033 closes out that readiness geometry count /
page-size group before selecting another allocator exact-`usize` field group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-034 selects the next narrow owner-local field
group before any further migration.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-035 migrates the segment-map
accepted-readiness modeled consume-ledger block/count report group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-036 closes out that segment-map
consume-ledger block/count group before selecting another allocator
exact-`usize` field group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-037 migrates the segment-map consume-ledger
release-side block/count report group.
Then HAKO-ALLOC-USIZE-FIELD-GROUP-038 closes out that segment-map
consume-ledger release-side block/count group before selecting another
allocator exact-`usize` field group.
Reason/status/token/sentinel fields stay i64.
```

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
| `PURE-FIRST-GUARD-RELEASE-BIN-001` | landed | Pure-first guard VM/MIR emit latency cleanup. | MIMAP-192A L2 cut from 119.84s to 16.10s by defaulting active guard VM/MIR paths to release `hakorune`. |

MIMAP row order is not repeated in this sidecar. Read the durable row tables in
this taskboard for historical order, and read `CURRENT_STATE.toml` for the
selected current row and latest landed card.

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
