---
Status: Active
Date: 2026-05-27
Scope: taskboard for phase-296x mimalloc benchmark contract lane.
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/design/provider-abi-v1-ssot.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/README.md
---

# 296x-90 Mimalloc Benchmark Taskboard

## Rule

Benchmark contract work comes before DLL/provider work. Do not use this phase
to activate a provider, replace the process allocator, install hooks, or make
winner claims.

Smoke growth brake:

```text
report_only_rows_should_not_add_new_smoke_scripts=1
behavior_rows_may_extend_one_existing_fastmem_smoke=1
new_smoke_script_requires_new_execution_boundary=1
fastmem_migration_body_work_must_not_wait_for_per_row_report_smokes=1
```

Keep the daily gate compact. Prefer extending the existing FastMemory smoke
family when a row only promotes report/check evidence. Add a new smoke script
only when a new execution boundary would otherwise be unguarded.

## Current Truth

- Phase-295x landed the first `.hako` mimalloc comparison/remote-free pass.
- The external `hakmem` corpus exists at:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

- The corpus includes benchmark binaries, source, historical `benchres.csv`,
  `hakozuna_compare` logs, perf data, and strace data.
- Phase-296x should make those assets usable through stable Hakorune-side
  contracts before DLL/provider work begins.

## Current Blocker

```text
MIM-FMEM-016:
  Landed Mimalloc shape coverage scoring after MIM-FMEM-015 fixed safe
  capability wrappers as FastMemory MemOp aliases.
MIM-FMEM-017:
  MIM-FMEM-017A landed non-activating product-shaped bridge report
  normalization. MIM-FMEM-017B landed SizeClassBox bridge evidence.
  MIM-FMEM-017C landed Page-local state bridge evidence.
  MIM-FMEM-017D landed producer taxonomy.
Producer transition:
  Python-template C is a temporary bridge producer, not semantic SSOT.
  Add producer-neutral fields before MIR lowering work:
  primary path: python_template_c_bridge -> mir_to_llvm_lowering -> bridge retirement.
  optional artifact path: mir_to_c_lowering for debug/diff/bootstrap only.
  MIR-FMEM-005 keeps the Python-template C bridge as baseline evidence.
  MIR-FMEM-006 proves producer-neutral parity.
  MIR-FMEM-007 retires the bridge and must not leave hidden fallback.
  MIRBuilder design consultation is a separate representation-boundary task;
  MIRBuilder represents FastMemRegion/MemOp, Planner selects, Verifier guards,
  Lowering emits.
LLVM-PIPE-001 landed separate runner cleanup inventory:
  Static hako_check inventory now reports env future rewrite, method-id seam,
  JoinIR experiment hook, and PyVM/harness/mock fallback visibility without
  executing any runner route.
  LLVM-PIPE-002 landed opt-in dynamic pipeline/executor report fields.
  LLVM-PIPE-003 landed named CompileOptions / PipelinePlan boxes for current
  runner defaults.
MIR-FMEM-001 landed the representation decision:
  MemOp is the single executable instruction, MemOpKind is the dialect, and
  FastMemRegion is side-table metadata. FastMemRegionBegin/End are rejected as
  normal MIR instructions.
MIR-FMEM-002 landed the code-side vocabulary:
  MemOp is in the kept MIR instruction vocabulary, MemOpKind V0 has a contracts
  allowlist, and JSON/VM/LLVM/C support remains closed.
MIR-FMEM-003 landed MIRBuilder source lowering:
  parsed fastmem source now records FunctionMetadata.fastmem_regions and emits
  MemOp instructions for the V0 source vocabulary. JSON/VM/LLVM/C support
  remains closed. MIR-FMEM-004 opens verifier gates next.
296x-440 documented the hako_alloc identity boundary:
  hako_alloc is the .hako body/source truth of the mimalloc port, not a
  separate allocator family. The replacement-front C shim is the temporary
  execution bridge for the same port, and runtime/bootstrap allocation remains
  separate from application/product allocator activation. MIR-FMEM-004 remains
  the next implementation blocker.
MIR-FMEM-004 landed verifier gates:
  FastMemRegion metadata, MemOp kind/arity/effect shape, and no-escape are now
  checked before backend support opens. 296x-442 realigned the producer order:
  MIR-FMEM-005 opens the MIR-to-LLVM/object primary producer next, while
  MIR-to-C is deferred to an optional debug/diff/bootstrap artifact lane.
  JSON/VM/product activation remain closed.
MIR-FMEM-005 landed the primary producer subset:
  MIR JSON transport and Python LLVM lowering are open for value-only FastMemory
  MemOps. Table/layout and allocator-owner TLS runtime MemOps remain closed.
MIR-FMEM-006 landed producer-neutral parity:
  `hako_check fastmem-producer-parity` now compares the Python-template C
  bridge baseline against the MIR-to-LLVM candidate through an explicit
  producer-neutral allowlist. MIR-FMEM-007 opens bridge retirement next.
MIR-FMEM-007 first slice landed:
  Replacement-front Python-template C generation now requires
  `--allow-python-template-c-bridge-baseline`, and report producer inference no
  longer defaults `replacement_front_c_shim` to `python_template_c_bridge`.
  MIR-FMEM-007B opens the remaining bridge quarantine/delete inventory.
MIR-FMEM-007B landed:
  Python-template C bridge build helpers now require the shared explicit
  diagnostic baseline guard, and remaining template payloads are classified as
  diagnostic implementation. MIR-FMEM-007C opens a static import guard so normal
  allocator tools cannot import diagnostic payloads directly.
MIR-FMEM-007C landed:
  `tools/checks/python_template_c_bridge_import_guard.sh` now blocks normal
  allocator / hako_check tools from direct-importing retired Python-template C
  diagnostic payload modules. MIR-FMEM-007D opens the keep/archive decision for
  remaining diagnostic payloads.
MIR-FMEM-007D landed:
  Remaining Python-template C diagnostic payloads stay quarantined until
  MIR-to-LLVM replacement-front layout/table/owner runtime coverage can replace
  their baseline role. MIM-FMEM-018 thread-exit / abandoned owner lifecycle
  opens next.
MIM-FMEM-018A landed:
  AllocOwner lifecycle is now documented as page ownership truth, not just
  thread-exit cleanup. Active / ExitingFlush / Abandoned / Reclaimed are the
  persistent states, ReclaimAttempt is transient, and AllocOwnerId is
  generation-bearing from v0. MIM-FMEM-018B opens report/check fields next
  without enabling abandoned reclaim behavior.
MIM-FMEM-018B landed:
  `hako_check` inventory now reports AllocOwner lifecycle state, generation,
  thread-exit, abandoned-owner, and reclaim-block fields. `fastmem-check`
  rejects invalid lifecycle transitions, stale generation, reuse without
  generation bump, local_free misuse, and reclaim-with-remote-candidates.
  MIM-FMEM-018C opens producer-side lifecycle shadow counters next.
MIM-FMEM-018C landed:
  The diagnostic replacement-front producer now emits lifecycle shadow counters
  for owner generation, thread-exit flush, abandoned pages, and reclaim-block
  observations. Reclaim behavior remains closed. MIM-FMEM-019 opens
  AtomicRemoteHead drain next.
MIM-FMEM-019 landed:
  Owner thread-exit flush now drains already-published AtomicRemoteHead remote
  frees into the page local free stack and reports drain evidence. Abandoned
  reclaim behavior remains closed. MIM-FMEM-020 opens generation-safe
  abandoned reclaim next.
MIM-FMEM-020 landed:
  Empty abandoned owner-page index entries now produce Reclaimed evidence only
  after remote candidates are drained and generation bump evidence is emitted.
  Cross-owner TLS backing transfer and owner slot reuse remain closed.
  The parent MIR-FMEM-008 producer task was split below; MIR-FMEM-008A has
  since landed and MIR-FMEM-008B opens next.
MIR-FMEM-008A landed:
  Report/check now fixes the next MIR-to-LLVM replacement-front producer slice
  as layout/table MemOps (`TableIndex`, `FieldLoad`, `FieldStore`) and defers
  owner-runtime MemOps (`CurrentAllocOwnerId`, `OwnerEq`). This row changes no
  lowering behavior and keeps product activation, bridge retirement, hooks,
  global allocator claim, and winner claim closed. MIR-FMEM-008B opens the
  layout/table producer pilot next.
MIR-FMEM-008B first slice landed:
  `MemOpAccess` now preserves symbolic `field_id` / `table_id` in MIR and JSON,
  and verifier rejects missing ids before lowering. The next 008B slice opens a
  verifier-owned `VerifiedMemAccessPlan` table so LLVM GEP/load/store lowering
  consumes accepted layout/table truth instead of recomputing offsets or table
  representation.
MIR-FMEM-008B second slice landed:
  Function metadata and MIR JSON now expose FastMemory access-plan rows for
  `TableIndex` / `FieldLoad` / `FieldStore` sites, and Python LLVM metadata
  loads those rows by site for the future lowering consumer. Rows remain
  `symbolic_only` until canonical layout/table contracts verify offsets, table
  representation, alignment, and bounds.
ContractRegionV0 docs-only landed:
  The common region/contract/obligation/verifier-report envelope is documented
  for future profiles, but `FastMemRegion` remains the current memory-profile
  wrapper and `MemOp` / `VerifiedMemAccessPlan` remain memory-specific. This
  row opens no rename, no generic RegionOp, no second profile, and no behavior.
MIR-FMEM-008B contract-resolution slice landed:
  PageMetaLayoutV0 and page_table contracts are the next implementation seam.
  The resolver owns canonical field ids, owner_id compatibility normalization,
  field offsets/types/alignment/mutability/classes, and table representation
  facts. LLVM GEP/load/store lowering remains closed until verified access
  plans exist and no lowerer-side recomputation is needed. Field plans now
  verify through PageMetaLayoutV0, while page_table remains non-lowerable until
  a TableIndex bounds/length policy is selected.
MIR-FMEM-008B proof direction accepted:
  TableIndex requires a verifier-owned VerifiedTableAccess proof with
  length/bounds/overflow evidence before it becomes lowerable. Layout verified
  is not access verified. Optional short LLVM smoke may lower fields only from
  VerifiedElementRef and must not lower page_table[index].field. Page-map
  strategy / PageTableLengthV0 remain deferred until the proof surface is
  stable.
MIR-FMEM-008B commonality boundary accepted:
  DirectArray and FastMemory may share ProofEnvelopeV0 and RangeIndexFact-style
  proof inputs, but not access-plan payloads. DirectArrayExtentFact remains
  DirectArray-specific; FastMemory gets its own VerifiedTableAccessProof,
  table-length, bounds, overflow, alignment, and provenance payload.
Verifier hygiene split active:
  Treat verifier walker / extern-name / test fixture cleanup as BoxShape work.
  Only the low-risk `MirInstruction::extern_name()` utility is eligible now.
  Narrow walker helper and verifier test_support extraction stay parked until
  after the next FastMemory proof slice, and must not mix with TableIndex
  proof vocabulary.
MIR commonality taxonomy active:
  Escape commonality is limited to `src/mir/escape_barrier.rs` cause
  classification; FastMemory still owns no-escape policy, error/report shape,
  and MemOp proof payloads. Allowlist/gate commonality stays at
  `src/mir/contracts` / backend capability entries, and AllocOwnerId / page
  owner / semantic owner remain separate axes. Next worker handoff order is
  ESCAPE-COMMON-001, ESCAPE-COMMON-002, then resume FMEM-TABLE proof work.
ESCAPE-COMMON-001 implemented:
  FastMemory verifier escape checks now consume `classify_escape_uses()` for
  shared cause labels, preserve the `memop-value-escapes` FastMemory violation
  shape, keep ordinary unclassified MIR consumers rejected as `ordinary_use`,
  and propagate MemOp origin through single-input Phi passthroughs. The next
  mainline task can return to VerifiedTableAccessProof / TableIndex bounds work;
  ESCAPE-COMMON-002 is optional test-only follow-up.
FMEM-TABLE-001 implemented:
  `FastMemTableAccessPlan` now carries explicit `FastMemTableAccessProof`
  fields and MIR JSON emits them. `page_table` still reports
  `table-length-unresolved` and remains non-lowerable; bounds and overflow proof
  rows are next and lowering remains closed.
FMEM-TABLE-002 design stop:
  Before consuming `RangeIndexFact` as `BoundsProof::RangeFact`, decide the
  FastMemory-owned table length fact owner. Current recommendation is a
  memory-profile table length fact carrier now, with page-map strategy selection
  deferred until the proof API is stable. Do not mark `TableIndex` lowerable or
  choose one-level/two-level page-map shape in this stop.
FMEM-TABLE-002 worker review:
  B-now/C-later is accepted as the clean direction if "B" means FastMemory
  semantic metadata, not MIRBuilder-invented length. Proposed owner is
  `FastMemTableLengthFact` in function metadata, refreshed by a new
  `fastmem_table_length_fact` module before access-plan refresh. Implementation
  should wait for design acceptance, then start with 002A carrier-only metadata.
FMEM-TABLE-003B landed:
  FastMemory table access plans now prove target-usize overflow and
  offset-within-object from verifier-owned table length, bounds, stride,
  linked field offset, field size, and element size. LLVM lowering stayed
  closed.
FMEM-TABLE-004 landed:
  `fastmem-check` now fails incomplete FastMemory table access proofs through
  explicit incomplete-proof and missing-overflow-proof fields.
MIR-FMEM-008C handoff order active:
  Start with LLVM producer preflight before opening lowering behavior. Worker
  handoff order is Worker A LLVM producer inventory, Worker B report/check gap
  inventory, Worker C optional verifier BoxShape sidecar after the next
  lowering slice, and Worker D design-consult pack only if pointer/value
  representation is ambiguous. SSOT card:
  `docs/development/current/main/phases/phase-296x/296x-473-MIR-FMEM-008C-HANDOFF-ORDER.md`
MIR-FMEM-008C preflight metadata landed:
  The Python LLVM metadata loader now preserves `field_size` and `element_size`
  from verified FastMemory access-plan rows. Actual TableIndex / FieldLoad /
  FieldStore lowering remains closed until the TableIndex result pointer/value
  representation is decided.
MIR-FMEM-008C TableIndex LayoutRef pilot landed:
  Verified TableIndex rows now lower to backend-private `fastmem_layout_refs`
  rather than ordinary `vmap`. FieldLoad / FieldStore remain closed and open
  next as LayoutRef consumers.
MIM-PORT-FMEM-004 landed:
  The PageMeta hako_alloc pilot now has observation-only MIR-to-LLVM producer
  evidence: verified `TableIndex`, `FieldLoad`, and `FieldStore` access plans
  compile through the Python LLVM producer and emit lowered-count KV evidence
  checked by `fastmem-check --inventory`. Owner runtime, remote-free, TLS
  transfer, bridge deletion/archive, and product activation remain closed.
  MIM-PORT-FMEM-005 opens next hako_alloc body slice selection.
MIM-PORT-FMEM-005 landed:
  Worker inventory selected PageMeta `owner_worker_id` read-only scalar
  observation as the safest next body slice. A separate pilot now reads
  `owner_worker_id` through the existing verified layout/table producer path.
  This is not owner-runtime behavior: `CurrentAllocOwnerId`, `OwnerEq`, owner
  mutation, DirectArray/free-list mutation, remote-free, TLS transfer, and
  product activation remain closed. MIM-PORT-FMEM-006 opens `free_head`
  read-only pointer observation with explicit no-escape evidence.
MIM-PORT-FMEM-006 landed:
  A separate PageMeta fastmem pilot now reads `free_head` as read-only
  pointer-shaped metadata through verified layout/table producer evidence.
  It does not return or store `free_head`, does not claim free-list semantics,
  and keeps `local_free_head`, `remote_head`, DirectArray/free-list mutation,
  owner runtime, TLS transfer, and product activation closed.
  MIM-PORT-FMEM-007 opens owner equality-only source observation.
MIM-PORT-FMEM-007 landed:
  A separate PageMeta fastmem pilot now observes current AllocOwnerId and
  owner equality from `.hako` source via `mem.currentAllocOwnerId` and
  `mem.ownerEq`. The equality result is consumed inside the fastmem region as
  the input to a verified `used` FieldStore, so it does not escape as an
  ordinary value and does not select same/remote free routing. Owner mutation,
  `local_free_head`, `remote_head`, AtomicRemoteHead, DirectArray/free-list
  mutation, TLS transfer, and product activation remain closed.
  MIM-PORT-FMEM-008 opens local_free_head / free-list source-body preflight.
MIM-PORT-FMEM-008 landed:
  A PageMeta local_free_head preflight now proves `local_free_head` is visible
  from `.hako` source and MIR metadata, but MIR-to-LLVM lowering rejects it as
  an ordinary FieldLoad with
  `[llvm/fastmem:unsupported-field-load-class] local_free_head`. This keeps
  free-list mutation fail-closed until a dedicated substrate is selected.
  MIM-PORT-FMEM-009 opens free-list mutation substrate selection.
MIM-PORT-FMEM-009 landed:
  The selected substrate is a free-list-specific FastMemory MemOp family, not
  ordinary `local_free_head` FieldLoad/FieldStore class widening. DirectArray
  commonality remains proof-envelope-only. MIM-PORT-FMEM-010 opens
  LocalFreePush / LocalFreePop vocabulary and source-intrinsic observation
  while lowering, remote-owner routing, AtomicRemoteHead, TLS transfer, and
  product activation remain closed.
MIM-PORT-FMEM-010 landed:
  LocalFreePush / LocalFreePop vocabulary is now visible from `.hako` source,
  MIR, and hako_check inventory through a dedicated PageMeta pilot. LLVM
  lowering remains fail-closed on unsupported-kind, and ordinary
  `local_free_head` FieldLoad/FieldStore remains rejected. MIM-PORT-FMEM-011
  opens verifier-owned local free-list plans before any lowering behavior.
MIM-PORT-FMEM-011 landed:
  LocalFreePush / LocalFreePop now produce verifier-owned access-plan rows.
  The rows resolve `local_free_head` metadata but remain non-lowerable because
  same-owner proof and block-next layout/provenance proof are still missing.
  MIM-PORT-FMEM-012 opens LocalFreePush lowering preconditions / pilot without
  opening LocalFreePop, remote-owner routing, AtomicRemoteHead, TLS transfer,
  or product activation as a side effect.
MIM-PORT-FMEM-012A landed:
  `.hako hako_alloc` can now express LocalFreePush preconditions through
  `mem.assumeSameOwner(page, same_owner)` and
  `mem.assumeLocalFreeBlockNext(block)`. When both proof facts exist,
  LocalFreePush becomes a verified/lowerable access-plan row. LLVM lowering,
  LocalFreePop, remote-owner routing, AtomicRemoteHead, TLS transfer, and
  product activation remain closed. MIM-PORT-FMEM-012B opens the first
  LocalFreePush LLVM producer pilot.
MIM-PORT-FMEM-012B landed:
  Verified LocalFreePush plans now lower through the MIR-to-LLVM producer.
  Lowering consumes a PageMeta LayoutRef, verified `local_free_head` access
  material, and `FreeBlockNodeLayoutV0.next` block-next access material. The
  ordinary `local_free_head` FieldLoad/FieldStore routes, LocalFreePop,
  remote-owner routing, AtomicRemoteHead, TLS transfer, and product activation
  remain closed. MIM-PORT-FMEM-013 opens the LocalFreePop route.
MIM-PORT-FMEM-013A landed:
  `.hako hako_alloc` can now express LocalFreePop preconditions through
  `mem.assumeLocalFreeNonEmpty(page)` plus same-owner proof. LocalFreePop plans
  report non-empty evidence but stay non-lowerable with
  `local-free-pop-lowering-closed`. MIM-PORT-FMEM-013B opens the LocalFreePop
  LLVM producer pilot while keeping remote-free, AtomicRemoteHead, TLS
  transfer, and product activation closed.
MIM-PORT-FMEM-013B landed:
  Verified LocalFreePop plans now lower through the MIR-to-LLVM producer by
  consuming a PageMeta LayoutRef, loading `page.local_free_head`, loading
  `FreeBlockNodeLayoutV0.next` from the popped block, storing the next head
  back to `page.local_free_head`, and returning the popped block as a
  pointer-sized scalar. Remote-free, AtomicRemoteHead, TLS transfer, and
  product activation remain closed. MIM-PORT-FMEM-014 opens the next
  page-local route slice selection.
MIR-FMEM-008C FieldLoad LayoutRef pilot landed:
  Verified FieldLoad rows now consume backend-private LayoutRefs and emit
  readonly scalar GEP/load results into ordinary `vmap`. FieldStore, owner
  mutation, atomic/publication fields, Type ABI hot lookup, Provider ABI hot
  dispatch, Python-template C fallback, and product activation remain closed.
MIR-FMEM-008C FieldStore LayoutRef pilot landed:
  Verified FieldStore rows now consume backend-private LayoutRefs and store
  ordinary i64 values only into mutable plain fields. Owner mutation,
  local_free_head, atomic/publication fields, Type ABI hot lookup, Provider ABI
  hot dispatch, Python-template C fallback, and product activation remain
  closed.
MIR-FMEM-008C report/check closeout landed:
  Complete `mir_to_llvm_lowering` layout/table candidates now fail
  `fastmem-check` unless TableIndex, FieldLoad, and FieldStore lowered-count
  evidence is present. Incomplete proof reports still fail on their
  proof-specific counters, and owner-runtime MemOps remain deferred.
DirectArray/FastMemory commonality task inserted:
  `DIRECTARRAY-FMEM-COMMON-001` is a proof-envelope/report adapter task only.
  DirectArray and FastMemory may share ProofEnvelopeV0-style identity and
  RangeIndexFact-style proof inputs, but DirectArray access does not
  auto-generate fastmem regions and access-plan payloads remain separate.
Docs length cleanup task inserted:
  `DOCS-SLIM-FMEM-SSOT-001` landed by slimming the design SSOT after the
  active restart docs crossed the 1000-line maintenance threshold. Keep
  physical doc slimming separate from MIR-FMEM-008D owner-runtime
  implementation.
Reference sync note:
  docs/reference now records the accepted fastmem source surface, MemOp /
  FastMemRegion split, and AllocOwner lifecycle evidence boundary. After the
  MIR-FMEM layout/table/owner runtime producer body is implemented, run the
  follow-up reference closeout task below to retire stale bridge wording and
  confirm product-activation stop lines still read correctly.
MIR-FMEM-008 split:
  MIR-FMEM-008A producer-slice selection:
    landed; selection fields choose layout/table first and owner-runtime later.
  MIR-FMEM-008B layout/table producer pilot:
    landed through complete TableIndex proof and report/check rejection for
    incomplete access proofs. It did not open LLVM lowering.
  MIR-FMEM-008C layout/table LLVM producer pilot:
    first preflight the Python LLVM producer seam, then lower complete
    VerifiedTableAccess rows only. CurrentAllocOwnerId / OwnerEq remain
    deferred.
  MIR-FMEM-008D owner-runtime producer pilot:
    split into PRE/A/B/C: decide owner-runtime input truth and counters, then
    lower CurrentAllocOwnerId observation, then OwnerEq equality, then close
    report/check coverage. No TLS backing transfer, owner slot reuse,
    AtomicRemoteHead, or local/remote free routing.
  MIR-FMEM-008D-PRE landed:
    CurrentAllocOwnerId v0 is an LLVM-producer owner-id observation scalar,
    not TLS backing transfer. OwnerEq is equality only and must not select
    same/remote free routing. 008D-C owns the owner-runtime complete report
    profile.
  MIR-FMEM-008D-A landed:
    CurrentAllocOwnerId now lowers to a producer-local LLVM helper call returning
    an ordinary i64 scalar. It does not touch LayoutRef, TLS backing transfer,
    owner slot reuse, local/remote free routing, or product activation.
  MIR-FMEM-008D-B landed:
    OwnerEq is now fixed as equality-only lowering over ordinary owner-id
    scalars. It does not choose same-owner or remote-owner allocation/free
    routing.
  MIR-FMEM-008D-C landed:
    `fastmem-check` now has a distinct owner-runtime producer profile requiring
    positive CurrentAllocOwnerId and OwnerEq lowered counts while keeping TLS
    transfer, owner slot reuse, AtomicRemoteHead, ABI hot paths, and product
    activation closed.
  MIR-FMEM-008E producer-neutral parity/readiness:
    landed. `fastmem-producer-parity` now requires candidate-only readiness
    evidence for both layout/table and owner-runtime lowered counts before the
    quarantined Python-template C diagnostic baseline can be treated as
    replaceable. Reference closeout opens next.
  MIM-PORT-FMEM-001:
    landed as the first hako_alloc source/body pilot: PageMeta scalar
    TableIndex / FieldLoad / FieldStore shape is now present in `.hako`
    source and visible to hako_check AST inventory. DirectArray/free-list
    lowering and product activation remain closed.
  MIM-PORT-FMEM-002:
    landed as MIR JSON verified FieldLoad/FieldStore access-plan evidence for
    the PageMeta scalar pilot. TableIndex stayed proof-incomplete in that row.
  MIM-PORT-FMEM-003:
    landed explicit fastmem table-length and index-range proof annotations so
    PageMapV0 TableIndex is now a verified access plan without lowering-side
    inference or ABI lookup. MIM-PORT-FMEM-004 opens MIR-to-LLVM producer
    evidence for the verified PageMeta pilot.
```

## Queue

The detailed historical queue is archived to:

```text
docs/development/current/main/investigations/phase-296x-90-taskboard-history-2026-06-08.md
```

Keep the active taskboard focused on the current blocker, the compact queue
summary, and restart entry points.

## Hako Mimalloc Performance Parity Plan

The long parity roadmap details are archived in the history note above. Keep
this active card for current restart pointers only.

## Mini-Agent Restart Queue

The long restart queue detail is archived in the history note above. Keep this
section compact and do not re-expand the historical rows here.
