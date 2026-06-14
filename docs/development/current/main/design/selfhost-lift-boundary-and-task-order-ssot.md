---
Status: SSOT
Decision: accepted-for-tasking
Date: 2026-06-14
Scope: Decide which runtime/kernel responsibilities should move upward into
.hako or MIRBuilder to make self-hosting and de-Rust work easier, while keeping
machine boundaries in substrate.
Related:
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/development/current/main/design/stage2-collection-substrate-cleanup-ssot.md
  - docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
  - docs/development/current/main/design/arc-retirement-and-ownership-substrate-ssot.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/workstreams/arc-retirement-current.md
---

# Selfhost Lift Boundary And Task Order (SSOT)

## Decision

Use one stable rule when deciding what should move out of Rust runtime/kernel
code.

```text
Lift upward:
  policy
  algorithm
  user-visible semantics
  route vocabulary
  verifier-checkable contracts

Keep substrate:
  OS syscalls
  raw allocation/free
  atomics/TLS
  FFI/dlopen/function pointers
  raw object identity storage
  GC/root/weak/finalizer machine boundary
```

Short form:

```text
meaning -> .hako
shape / route / ownership events -> MIRBuilder/CorePlan
machine boundary -> substrate
```

This is not a mandate to rewrite every Rust file in `.hako`. The target is a
thin Rust substrate with `.hako` owning semantics and MIRBuilder owning
lowerable structure.

## Layer Rule

```text
.hako:
  visible API behavior
  collection algorithms
  scheduling and failure policy
  ownership/cancel/failure semantics
  allocator / GC recipes

MIRBuilder / CorePlan:
  route plans
  ownership events
  verifier-visible facts
  lowering metadata

substrate:
  raw memory
  raw threads
  raw atomics
  FFI/plugin invocation
  OS handles
  object table mechanics
```

Do not turn descriptor or tooling layers into execution truth.

```text
TypeAbiCatalog:
  projection/tooling only

BoxCallableRegistry:
  callable truth

ThreadApi:
  runtime substrate only

hako_mem_* / hako_atomic_*:
  substrate leaves
```

## Priority Order

### A. Callable And Object-Model Foundation

Do first because self-hosted runtime code still needs a stable way to describe
what a Box can call without reading Rust side tables directly.

```text
BOXCALL-PROVIDER-SOURCE-001:
  store provider source as registry entry provenance while keeping RoutePlan
  execution derived from the entry target only

BOXCALL-CATALOG-001:
  reconcile String / Array / Map existing surface catalogs into
  BoxCallableRegistry provider rows

BUFFER-CATALOG-001:
  add a Buffer surface catalog before Buffer is reconciled into
  BoxCallableRegistry provider rows

BUFFER-PROVIDER-ROWS-001:
  seed BufferBox provider rows from the Buffer surface catalog while keeping
  VM handler dispatch as the current execution owner

BOXCALL-ROUTEPLAN-001:
  keep MethodCallRoutePlan / NewBoxRoutePlan / DropBoxRoutePlan as semantic
  plan data; runtime later attaches executable function pointers

TYPE-REGISTRY-PROVIDER-001:
  make type_registry a builtin provider / seed source, not execution truth

PLUGIN-PROVIDER-SNAPSHOT-001:
  make PluginLoader exports pure provider snapshots into BoxCallableRegistry
  and keep TypeBox ABI v2 unchanged
```

Concrete order inside this group:

```text
1. BOXCALL-PROVIDER-SOURCE-001
   provenance is stored beside the target, proving provider source is not an
   execution route

2. BOXCALL-CATALOG-001
   existing String / Array / Map catalogs are reconciled first because they
   already exist

3. BUFFER-CATALOG-001
   Buffer gets a surface catalog before any provider-row reconciliation

4. BUFFER-PROVIDER-ROWS-001
   Buffer provider rows are added only after the catalog exists

5. BOXCALL-ROUTEPLAN-001
   route plan data stays semantic; executable pointers attach later

6. TYPE-REGISTRY-PROVIDER-001 and PLUGIN-PROVIDER-SNAPSHOT-001
   provider surfaces are narrowed after catalog rows are visible
```

Stop line:

```text
do not make TypeAbiCatalog callable truth
do not mix internal slot ids with plugin method ids
do not expose PluginLoader internals as a broad public API
do not route hot execution through TypeAbiPack / TypeAbiCatalog
```

### B. Collection Visible Semantics

Do next because this removes high-value Rust semantic ownership while leaving
storage mechanics below the boundary.

The lane is not a broad rewrite of collection storage. It is a visible
semantics lift. Each collection must first expose what users can observe, then
move that policy into `.hako` or `.hako`-owned tests, while Rust keeps raw
storage mechanics until a separate substrate row replaces them.

Task ladder:

```text
COLL-VISIBLE-000:
  docs-only lane card
  define collection visible semantics as the next lane after BoxCallable
  choose Buffer as first pilot

BUFFER-VISIBLE-INVENTORY-001:
  list Buffer visible methods, aliases, return policy, mutation policy, and
  substrate-owned storage mechanics

BUFFER-VISIBLE-CONTRACT-002:
  pin length/read/readAll/write/clear/slice/append behavior with fixtures and
  hako_check report fields

BUFFER-HAKO-CORE-003:
  add the first `.hako` visible owner for Buffer policy
  keep byte storage and allocation in substrate

BUFFER-NUMERIC-LE-004:
  pin typed read/write little-endian, bounds, and failure policy
  do not widen Buffer storage layout in the same row

STRING-VISIBLE-INVENTORY-001:
  split String visible policy from storage: byte/codepoint mode, substring
  clamp, indexOf, lastIndexOf, concat length policy

STRING-HAKO-POLICY-002:
  move the first String visible policy into `.hako` ownership with fixtures
  and keep low-level byte storage in substrate

MAP-VISIBLE-CONTRACT-001:
  pin missing-key, key normalization, delete/clear return, insertion order or
  sorted keys/values behavior

ARRAY-VISIBLE-CONTRACT-001:
  pin OOB/null/append-at-end set behavior and visible length semantics
  without changing inline lane representation

COLL-VISIBLE-CLOSEOUT-001:
  report which collection policies moved upward and which substrate mechanics
  intentionally remain below
```

Stop line:

```text
do not begin with String/Array/Map before the Buffer pilot card exists
do not move Vec / HashMap / RwLock / Arc mechanics into .hako
do not move Array inline lane representation as a semantics task
do not turn method-shaped compat exports into final substrate names
do not widen Array / Map work without a concrete blocker or selected front
do not claim de-Rust progress unless visible semantics are owned above Rust
and raw storage remains explicitly substrate-owned
```

### C. Concurrency Semantics

Do after the callable and collection seams are readable. The current thread
substrate is useful, but source-visible concurrency must remain structured and
safety-gated.

```text
CONC-FUTURE-SEM-001:
  make co / nowait / await / TaskGroup ownership, cancellation, and failure
  taxonomy the language-level owner

CONC-SCHED-ROUTE-VOCAB-001:
  keep inline_resolved_future / cooperative_task / worker_pool_task as route
  vocabulary and report/check data, not default worker activation

CONC-SYNCBOX-MIR-001:
  add sync box MIR metadata/lowering only after reference serialized entry and
  wait-forbidden verifier guards are green

CONC-CHANNEL-MIR-001:
  add canonical Channel<T> await-visible send/recv/close route lowering;
  keep blocking reference helpers private

CONC-CONTEXT-MIR-001:
  add explicit co/task_scope child context snapshot lowering

THREAD-SAFETY-001:
  enforce HakoSend / HakoShare / ThreadRoot before any worker_scope lowering

WORKER-SCOPE-001:
  open worker_scope workers=N only after safety gates; workers=N remains an
  upper-bound hint, not an exact OS-thread promise
```

Stop line:

```text
do not reinterpret nowait as OS thread spawn
do not add raw thread {} source syntax
do not enable worker_pool_task as a source default route
do not reuse legacy P2P ChannelBox as canonical Channel<T>
do not expose ordinary blocking channel calls
```

### D. Arc Retirement / Ownership Substrate

Do this family-by-family after callable truth and object identity seams are
stable. Arc retirement is not a GC implementation lane.

```text
ARC-FAMILY-GATE-001:
  choose one Arc family with measurable replacement criteria

OBJECT-HANDLE-001:
  close ObjectHandle / weak generation / host handle / plugin instance identity
  seams before global Arc replacement claims

RC-MIR-OWNERSHIP-001:
  make retain/release ownership events one MIR owner

BOX-OBJECT-MODEL-001:
  replace clone/share/downcast/type identity/plugin lifetime responsibilities
  with explicit object-model owners

ARC-RETIRE-FAMILY-N:
  retire Arc only for the selected family; do not claim global Arc removal
```

Stop line:

```text
do not treat Arc as only a refcount
do not make TypeAbiCatalog identity truth
do not replace Arc globally before object identity / dispatch / clone-share
  replacements exist
do not use GC recipe work as proof that physical reclamation changed
```

## Ranking

```text
highest immediate value:
  BoxCallable provider/catalog cleanup
  Buffer visible semantics
  String index/clamp/search policy

high value but gated:
  Map visible contract
  Array visible OOB contract
  sync box / Channel / context MIR lowering

large but later:
  Arc family retirement
  worker_scope runtime route
  allocator provider activation
```

## Substrate Keep List

These are not `.hako` semantic owners.

```text
ThreadApi / StdThread / raw JoinHandle
dlopen / plugin function pointers / TLV invoke
hako_mem_* / hako_atomic_* / hako_tls_* / hako_osvm_*
Vec / HashMap storage mechanics
RwLock / Arc mechanics until a family gate replaces them
object handle table / generation / weak table storage
serde JSON parser/stringifier
OS file/socket/time/audio/syscall glue
allocator provider activation and global allocator hooks
```

## Report Vocabulary

Future rows should prefer these fields when proving the boundary.

```text
selfhost_lift_boundary_contract=selfhost-lift-boundary-v0
lift_target_layer=hako|mirbuilder|substrate
visible_semantics_owner=hako
route_plan_owner=mirbuilder
machine_boundary_owner=substrate

typeabi_catalog_execution_route_count=0
box_callable_registry_truth_owner=1
threadapi_source_surface_count=0
nowait_os_thread_spawn=0
worker_scope_exact_thread_count_promise=0

arc_global_retirement_claim=0
arc_family_gate_selected=0|1
object_identity_substrate_ready=0|1
```

## Non-Goals

```text
do not reopen exact-front optimization from this SSOT alone
do not make broad language syntax changes
do not move raw substrate primitives into user-visible .hako surface
do not add a monolithic hako.sys unsafe shelf
do not start global Arc replacement
```
