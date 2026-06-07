---
Status: Active
Date: 2026-06-02
Scope: active mimalloc migration, optimization, and provider-benchmark workstream.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/current-docs-archive-policy-ssot.md
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/design/mimalloc-benchmark-route-taxonomy-ssot.md
  - docs/development/current/main/design/type-abi-route-descriptor-plane-ssot.md
  - docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/design/hako-alloc-mimalloc-port-identity-boundary-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/investigations/mimalloc-current-history-2026-06-02.md
---

# Mimalloc Current Workstream

This is the active restart card. It intentionally stays compact.

Full historical MIM-001..MIM-146 prose was archived to:

```text
docs/development/current/main/investigations/mimalloc-current-history-2026-06-02.md
```

Use that archive for exact old evidence. Use this file for the current decision
surface, next task order, and parking lot.

## Goal

Keep proving that `.hako` mimalloc can be built, packaged, and compared against
C mimalloc without opening product allocator replacement prematurely.

Current focus:

```text
provider/DLL benchmark bridge
same-machine C mimalloc comparison
next owner selection from local evidence
algorithm-port coverage separation
fastmem parser parity before source syntax
safe capability wrapper evidence before shape coverage scoring
```

## Stop Line

- no new numbered row for inventory-only work
- no row-specific `.sh` guard
- no full external benchmark corpus import
- no copied benchmark executables in git
- no provider activation as product default
- no allocator replacement claim
- no hook installation claim
- no production `#[global_allocator]` claim
- no winner claim
- no source syntax expansion unless a tracked reference decision accepts it
- no Rust-only `fastmem` active grammar; `.hako` parser parity is required
- no new report-only smoke scripts unless a new execution boundary opens

## Current Decisions

```text
parity/front:
  direct exact remains the .hako mimalloc optimization front

public/default front:
  compatibility reference only; not the parity owner

provider package:
  handoff artifact and benchmark smoke path only

LD_PRELOAD shim:
  experiment/measurement bridge only

Hakozuna mixed-ws:
  repo-local CRT fixture is connected
  compare same-machine system malloc / C mimalloc / optional Hakorune provider
  do not compare Ubuntu numbers horizontally while CPU differs

record/state direction:
  PageModel remains box owner
  primitive mutable state may become record-shaped residence only through
  RecordStateResidencePlanV0-style metadata/plans

Inline(required):
  small receiver-local leaf helper only
  multi-block hot paths use HotCore/direct-exact plans instead

algorithm-port coverage:
  `.hako` hako_alloc policy/model coverage and benchmark-only replacement-front
  execution coverage are different surfaces. Do not read the fixed-slot
  replacement front as proof that the full `.hako` mimalloc algorithm is wired
  into LD_PRELOAD/product replacement.

hako_alloc identity:
  `hako_alloc` is the `.hako` body/source truth of the mimalloc port, not a
  separate allocator family. The Python-template C replacement front is now
  diagnostic-baseline-only after the MIR-to-LLVM producer readiness gate.
  Bridge evidence exists to prevent drift while any explicit diagnostic
  baseline remains. The active implementation direction is `.hako hako_alloc /
  fastmem -> MIR MemOp -> LLVM/object`, with MIM-PORT-FMEM-001 opening the
  first narrow body migration. Runtime/bootstrap allocation stays separate from
  application/product allocator activation. SSOT:
  `docs/development/current/main/design/hako-alloc-mimalloc-port-identity-boundary-ssot.md`

capability gap:
  current evidence does not point at missing syntax alone. Bitwise/shift syntax
  exists; the accepted next boundary is a contract-bound memory fast-path
  sublanguage (`fastmem ContractName { ... }`) plus verifier/report inventory.
  Allocator page maps are the first consumer, not the only consumer. Later
  safe wrappers such as AddressToken/PageKey/PageMapBridge may sit on top of
  the same MemOps. SSOT:
  `docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md`

parser parity:
  `fastmem ContractName { ... }` is source syntax, not only report vocabulary.
  The next phase is parser parity catch-up, not Rust-only fastmem parsing.
  `.hako` parser catch-up must prove the parse-only surface before any
  lowering, runtime behavior, or replacement-front behavior changes.
  Phase card:
  `docs/development/current/main/phases/phase-296x/296x-416-FASTMEM-PARSER-PARITY-CATCHUP.md`

smoke growth brake:
  `MIM-FMEM-011A/B/C` completed the owner-state foundation. Future report-only
  rows should not add one-off smoke scripts. Behavior rows may extend the
  existing fastmem smoke family, but the next priority is `.hako` mimalloc
  fastmem migration body work, not per-row report-smoke growth.

free-list mutation direction:
  `local_free_head` remains rejected as an ordinary FieldLoad/FieldStore
  lowering target. The selected substrate is a free-list-specific FastMemory
  MemOp family, starting with LocalFreePush / LocalFreePop-style vocabulary and
  source-intrinsic observation. Lowering must consume verifier-owned local
  free-list plans only and must not open remote-owner routing, AtomicRemoteHead,
  TLS transfer, provider activation, hook installation, global allocator claim,
  or winner claim as a side effect.
```

## Current Task Order

The parser parity catch-up and owner-state foundation are complete for the
fastmem source-syntax pilot. The docs-slim cleanup row is complete; the
implementation ladder through `MIM-PORT-FMEM-129` is already represented in
phase cards.

```text
next_task:
  phase-296x next lane selection pending

implementation_sequence:
  MIR-FMEM-008D-PRE..MIR-FMEM-008E (landed)
  FASTMEM-REFERENCE-CLOSEOUT-AFTER-PRODUCER-BODY-296X-001 (landed)
  MIM-PORT-FMEM-001..MIM-PORT-FMEM-129 (landed in phase cards)

## Task Granularity / Worker Handoff

Keep the AtomicRemoteHead lane in four buckets. Do not split these further
unless a new verifier or lowering boundary appears.

```text
bucket_1:
  MIM-PORT-FMEM-031 AtomicRemoteHead CAS lowering producer selection
  MIM-PORT-FMEM-032 AtomicRemoteHead CAS lowering report/check preflight
  reason:
    one selection decision and one report contract; keep together

bucket_2:
  MIM-PORT-FMEM-033 AtomicRemoteHead CAS lowering producer pilot
  reason:
    one minimal implementation slice for the single-attempt CAS path

bucket_3:
  MIM-PORT-FMEM-034 AtomicRemoteHead Route/Drain Selection
  MIM-PORT-FMEM-035 AtomicRemoteHead Retry Policy Preflight
  MIM-PORT-FMEM-036 AtomicRemoteHead Retry Lowering Producer Pilot
  reason:
    retry evidence and bounded lowering stay coupled until drain/exchange is
    selected

bucket_4:
  MIM-PORT-FMEM-037 AtomicRemoteHead Drain Preflight
  MIM-PORT-FMEM-038 AtomicRemoteHead Drain/Exchange Selection
  reason:
    drain/exchange vocabulary must be pinned before route selection or
    lowering opens

bucket_5:
  MIM-PORT-FMEM-039 AtomicRemoteHead Drain/Exchange Lowering Producer Pilot
  reason:
    one producer implementation row for the exchange primitive only

bucket_6:
  MIM-PORT-FMEM-040 AtomicRemoteHead Drain-to-Local Route Selection
  reason:
    drain-to-local consumes the drained token and deserves its own
    proof/precondition boundary
```

Worker handoff order for this lane:

1. report/check gap inventory for the next bucket
2. LLVM producer inventory for the next implementation bucket
3. verifier BoxShape cleanup only after a lowering slice lands
4. docs-slim / taskboard sync only if the lane notes grow past the current
   compactness threshold
5. fixture migration and negative-case audit stay sidecar only; they do not
   change the mainline bucket order

Sidecar worker tasks:

- extract or refresh manifest-backed source fixtures for route ladder evidence
- audit `fastmem-check` negative cases for missing proofs, premature open
  flags, and activation/global/winner leakage
- keep docs/ledger cleanup separate from lowering work
- run a report-key consistency pass across `remote-free-*`, `remote-owner-*`,
  and branch CFG profiles

follow_up_cleanup_task:
  FASTMEM-REFERENCE-CLOSEOUT-AFTER-PRODUCER-BODY-296X-001 (landed)

proof_commonality_follow_up:
  DIRECTARRAY-FMEM-COMMON-001

docs_slim_follow_up:
  DOCS-SLIM-FMEM-SSOT-001 (landed)

follow_up_cleanup_trigger:
  MIR-FMEM-008E landed a producer-neutral readiness gate for layout/table plus
  owner-runtime MIR-to-LLVM evidence.

why:
  MIM-FMEM-001/002 fixed the fastmem boundary and added an observation-only
  hako_check report; MIM-FMEM-003 fixed MIR MemOp report vocabulary;
  MIM-FMEM-004 added the inventory verifier; MIM-FMEM-005 locked PageKey
  exact-route docs/report vocabulary; MIM-FMEM-006 connected existing exact
  shift route evidence to the PageKey report; MIM-FMEM-007 locked the
  PageMapBridge plan. PARSER-FMEM-001 through PARSER-FMEM-006 proved the
  narrow dual-parser parse-only parity needed before reopening fastmem source
  syntax. MIM-FMEM-008 through MIM-FMEM-017D then connected source syntax,
  PageMapBridge, typed page metadata, AllocOwnerId/TLS owner state,
  same/remote-free evidence, safe wrappers, shape scoring, product-shaped
  bridge evidence, and producer taxonomy without product activation.
  LLVM-PIPE-001 made the current LLVM runner debt visible as static hako_check
  inventory, LLVM-PIPE-002 added opt-in dynamic pipeline/executor report
  fields, and LLVM-PIPE-003 moved the current runner defaults behind named
  `CompileOptions` / `PipelinePlan` boxes. MIR-FMEM-001 then accepted the
  representation boundary: `MemOp` is the single executable instruction,
  `MemOpKind` is the dialect vocabulary, and `FastMemRegion` is side-table
  metadata rather than begin/end instructions. MIR-FMEM-002 added the code-side
  vocabulary and contracts allowlist while keeping JSON/VM/LLVM/C support
  closed. MIR-FMEM-003 connected parsed fastmem source to function-local
  FastMemRegion metadata and MemOp instructions without opening backend
  execution. 296x-440 then fixed the identity boundary: `hako_alloc` is the
  `.hako` mimalloc-port body, Python-template C is the temporary execution
  bridge, and runtime/bootstrap allocation remains separate from
  application/product allocator activation. MIR-FMEM-004 then added verifier
  gates for FastMemRegion metadata, MemOp kind/arity/effect shape, and
  no-escape before any backend support opens. 296x-442 then realigned the
  producer order: the next required task is the MIR-to-LLVM/object primary
  producer. MIR-to-C is deferred to an optional debug/diff/bootstrap artifact
  lane and must not become semantic SSOT. 296x-443 then fixed the removal
  timing: MIR-FMEM-005 keeps the Python-template C bridge as baseline,
  MIR-FMEM-006 proves producer-neutral parity, and MIR-FMEM-007 retires it.
  296x-444 landed MIR-FMEM-005 by opening MIR JSON transport and Python LLVM
  lowering for the value-only FastMemory MemOp subset. Layout/table MemOps and
  allocator owner TLS runtime MemOps remain closed until their dedicated rows.
  296x-445 landed MIR-FMEM-006 by adding `hako_check fastmem-producer-parity`,
  an explicit allowlist comparison between `python_template_c_bridge` and
  `mir_to_llvm_lowering` reports. 296x-446 landed the first retirement slice:
  Python-template C replacement-front generation now
  requires `--allow-python-template-c-bridge-baseline`, and report producer
  inference no longer maps `replacement_front_c_shim` to
  `python_template_c_bridge` unless the report declares that producer. The
  bridge is now an explicit diagnostic baseline only, not a semantic/runtime
  dependency. 296x-447
  landed MIR-FMEM-007B by moving the retirement guard into
  `tools/allocator/python_template_c_bridge.py` and requiring that guard at
  both CLI validation and bridge build-helper entrypoints. 296x-448 landed
  MIR-FMEM-007C by adding a dev-gate static import guard that prevents normal
  allocator / hako_check tools from direct-importing retired diagnostic payload
  modules. 296x-449 landed MIR-FMEM-007D by keeping the remaining diagnostic
  payloads quarantined until MIR-to-LLVM replacement-front layout/table/owner
  runtime coverage can replace their baseline role. 296x-450 then split
  MIM-FMEM-018 into AllocOwner lifecycle truth first: Active / ExitingFlush /
  Abandoned / Reclaimed are the persistent states, ReclaimAttempt is transient,
  and AllocOwnerId is generation-bearing from v0. 296x-451 landed
  MIM-FMEM-018B by adding lifecycle inventory fields and fastmem-check gates
  without enabling abandoned reclaim behavior. MIM-FMEM-018C now opens
  producer-side lifecycle shadow counters. 296x-452 landed MIM-FMEM-018C by
  adding producer-side lifecycle shadow counters without enabling reclaim
  behavior. 296x-453 landed MIM-FMEM-019 by draining already-published
  AtomicRemoteHead remote frees during owner thread-exit flush while leaving
  abandoned reclaim closed. 296x-454 landed MIM-FMEM-020 by allowing empty
  abandoned owner-page index entries to transition to Reclaimed after remote
  drain and generation bump evidence, while keeping TLS backing transfer
  closed. 296x-455 landed MIR-FMEM-008A by selecting layout/table MemOps
  (`TableIndex`, `FieldLoad`, `FieldStore`) as the next producer slice and
  deferring owner-runtime MemOps (`CurrentAllocOwnerId`, `OwnerEq`). The
  current reference sync records the accepted source/MIR/runtime reading, but a
  second closeout is intentionally parked until implementation completes so
  stale bridge wording can be removed in one pass. MIR-FMEM-008B then landed
  the concrete layout/table proof chain through complete TableIndex proof
  evidence and `fastmem-check` rejection for incomplete proofs. MIR-FMEM-008C
  opened with an LLVM producer preflight. The metadata loader now preserves
  `field_size` and `element_size`; TableIndex result truth was accepted as
  LayoutRef and the Python LLVM producer now stores raw metadata pointers only
  in `fastmem_layout_refs`. FieldLoad consumes LayoutRefs for verified
  readonly scalar/plain-pointer fields. FieldStore consumes LayoutRefs for
  mutable plain fields only; owner, local-free, and atomic/publication fields
  remain closed. `fastmem-check` now requires complete `mir_to_llvm_lowering`
  layout/table candidates to report positive lowered counts for TableIndex,
  FieldLoad, and FieldStore. MIR-FMEM-008D then added CurrentAllocOwnerId /
  OwnerEq producer evidence, and MIR-FMEM-008E added a producer-neutral
  readiness gate that combines layout/table and owner-runtime evidence before
  hako_alloc body migration opens.

completed_this_slice:
  MIM-FMEM-001 FastMemoryContract docs/report lock
  MIM-FMEM-002 hako_check fastmem capability inventory
  MIM-FMEM-003 MIR MemOp region docs/report plan
  MIM-FMEM-004 FastMem verifier implementation
  MIM-FMEM-005 PageKey exact route docs/report lock
  MIM-FMEM-006 PageKey exact route implementation
  MIM-FMEM-007 PageMapBridge plan
  296x-416 Fastmem parser parity catch-up phase cut
  PARSER-FMEM-001 parser parity inventory contract
  PARSER-FMEM-002 parser parity gate surface
  PARSER-FMEM-003 bitwise/shift expression parity
  PARSER-FMEM-004 rune contract-name parity
  PARSER-FMEM-005 fastmem block parse-only dual parser pilot
  PARSER-FMEM-006 fastmem contractless fail-fast parity
  MIM-FMEM-008 fastmem source syntax pilot after parser parity
  MIM-FMEM-009 PageMapBridge benchmark-front pilot
  MIM-FMEM-010 TypedPageMetaHandle plan
  MIM-FMEM-011 AllocOwnerId / TLS arena owner state
  MIM-FMEM-012 same-owner local-free route evidence
  MIM-FMEM-013 AtomicRemoteHead plan
  MIM-FMEM-014 AtomicRemoteHead pilot
  MIM-FMEM-015 safe capability wrapper plan
  MIM-FMEM-016 Mimalloc shape coverage score
  MIM-FMEM-017A Product-shaped bridge report normalization
  MIM-FMEM-017B SizeClassBox bridge evidence
  MIM-FMEM-017C Page-local state bridge evidence
  MIM-FMEM-017D Replacement-front producer taxonomy
  LLVM-PIPE-001 LLVM runner pipeline debt inventory
  LLVM-PIPE-002 LLVM runner pipeline report fields
  LLVM-PIPE-003 CompileOptions / PipelinePlan cleanup
  MIR-FMEM-001 MIRBuilder FastMemRegion/MemOp design consultation
  MIR-FMEM-002 mir/contracts FastMem MemOp vocabulary
  MIR-FMEM-003 MIRBuilder source lowering to FastMemRegion/MemOp metadata
  296x-440 hako_alloc mimalloc port identity boundary docs
  MIR-FMEM-004 FastMem verifier gates over MIR MemOps
  296x-442 FastMemory producer task order realignment
  296x-443 Python-template C bridge retirement gate
  MIR-FMEM-005 MIR-to-LLVM/object primary producer for value-only MemOps
  MIM-FMEM-018A AllocOwner lifecycle state machine
  MIM-FMEM-018B lifecycle report/check fields
  MIM-FMEM-018C lifecycle shadow counters
  MIM-FMEM-019 AtomicRemoteHead drain
  MIM-FMEM-020 abandoned reclaim

task_order:
  MIM-FMEM-001 FastMemoryContract docs/report lock
  MIM-FMEM-002 hako_check fastmem capability inventory
  MIM-FMEM-003 MIR MemOp region docs/report plan
  MIM-FMEM-004 FastMem verifier implementation
  MIM-FMEM-005 PageKey exact route docs/report lock
  MIM-FMEM-006 PageKey exact route implementation
  MIM-FMEM-007 PageMapBridge plan
  PARSER-FMEM-001 parser parity inventory contract
  PARSER-FMEM-002 parser parity gate surface
  PARSER-FMEM-003 general bitwise/shift expression parity
  PARSER-FMEM-004 rune contract-name parity
  PARSER-FMEM-005 fastmem block parse-only dual parser pilot
  PARSER-FMEM-006 fastmem contractless fail-fast parity
  PARSER-FMEM-007 remaining Rust-parser catch-up backlog split
  MIM-FMEM-008 fastmem source syntax pilot after parser parity
  MIM-FMEM-009 PageMapBridge benchmark-front pilot
  MIM-FMEM-010 TypedPageMetaHandle plan
  MIM-FMEM-011 AllocOwnerId / TLS arena owner state
  MIM-FMEM-012 same-owner local-free route evidence
  MIM-FMEM-013 AtomicRemoteHead plan
  MIM-FMEM-014 AtomicRemoteHead pilot
  MIM-FMEM-015 safe capability wrapper plan
  MIM-FMEM-016 Mimalloc shape coverage score
  MIM-FMEM-017A Product-shaped bridge report normalization
  MIM-FMEM-017B SizeClassBox bridge evidence
  MIM-FMEM-017C Page-local state bridge evidence
  MIM-FMEM-017D Replacement-front producer taxonomy
  LLVM-PIPE-001 LLVM runner pipeline debt inventory
  LLVM-PIPE-002 LLVM runner pipeline report fields
  LLVM-PIPE-003 CompileOptions / PipelinePlan cleanup
  MIR-FMEM-001 MIRBuilder FastMemRegion/MemOp design consultation
  MIR-FMEM-002 mir/contracts FastMem MemOp vocabulary
  MIR-FMEM-003 MIRBuilder source lowering to FastMemRegion/MemOp metadata
  296x-440 hako_alloc mimalloc port identity boundary docs
  MIR-FMEM-004 FastMem verifier gates over MIR MemOps
  MIR-FMEM-005 MIR-to-LLVM/object primary producer
  MIR-FMEM-006 producer-neutral parity against python_template_c_bridge
  MIR-FMEM-007 Python template C bridge retirement first slice
  MIR-FMEM-007B Remaining Python template C quarantine/delete inventory
  MIR-FMEM-007C Python template C diagnostic import guard
  MIR-FMEM-007D Python template C diagnostic payload keep/archive decision
  MIR-FMEM-C-ARTIFACT optional MIR-to-C debug/diff/bootstrap artifact producer
  MIM-FMEM-018A AllocOwner lifecycle state machine
  MIM-FMEM-018B lifecycle report/check fields
  MIM-FMEM-018C lifecycle shadow counters
  MIM-FMEM-019 AtomicRemoteHead drain
  MIM-FMEM-020 abandoned reclaim
  MIR-FMEM-008A producer-slice selection

next_mir_producer_rows:
  ContractRegionV0 docs-only
    296x-458 landed: common envelope only; FastMemRegion remains memory wrapper
  MIR-FMEM-008B layout/table producer pilot
    296x-456 symbolic MemOpAccess ids landed
    296x-457 VerifiedMemAccessPlan metadata skeleton landed
    296x-472 complete TableIndex proof/check evidence landed
  MIR-FMEM-008C layout/table LLVM producer preflight and lowering
    296x-476 TableIndex -> LayoutRef pilot landed
    296x-477 FieldLoad from LayoutRef pilot landed
    296x-478 FieldStore from LayoutRef pilot landed
    296x-479 report/check closeout landed
  MIR-FMEM-008D owner-runtime producer pilot
    296x-482..485 landed CurrentAllocOwnerId / OwnerEq lowering and report gate
  MIR-FMEM-008E producer-neutral parity/readiness
    296x-486 landed candidate-only readiness gate combining layout/table and
    owner-runtime evidence

retirement_gate:
  MIR-FMEM-005 does not delete python_template_c_bridge.
  MIR-FMEM-006 proved producer-neutral parity with the same report.kv /
  hako_check contract.
  MIR-FMEM-007 first slice makes Python-template C explicit baseline-only and
  removes report-side hidden producer inference. MIR-FMEM-007B quarantined the
  remaining build helpers behind the same explicit diagnostic baseline guard.
  MIR-FMEM-007C adds a static import guard for remaining diagnostic payload
  files. MIR-FMEM-007D keeps the remaining payloads quarantined until
  MIR-to-LLVM replacement-front layout/table/owner runtime coverage can replace
  their baseline role. MIR-FMEM-008E has now supplied that replacement
  readiness evidence. The remaining payloads are still diagnostic-only until a
  dedicated deletion/archive row; they are not semantic producers or hidden
  fallbacks. Optional MIR-to-C artifact support is a separate generated-backend
  lane.
  Historical command snippets before 296x-446 are archival. If re-run as a
  diagnostic baseline, every Python-template C replacement-front mode must add
  `--allow-python-template-c-bridge-baseline`; do not read older snippets as
  normal runtime recipes.

closed_by_default:
  RawPtr<T>
  pointer arithmetic operators outside fastmem
  address dereference syntax
  implicit pointer-to-integer conversion
  contract-less unsafe {}
  contract-less fastmem {}
  Type ABI hot lookup
  Provider ABI replacement-front hot dispatch
  product allocator activation
  hook install
  global allocator claim
  winner claim
```

`MIM-FMEM-008` row card:

```text
docs/development/current/main/phases/phase-296x/296x-417-FASTMEM-SOURCE-SYNTAX-PILOT.md
```

Accepted scope for this row is source-facing inventory/check metadata only:

```text
fastmem_source_inventory_input=1
fastmem_execution_open=0
fastmem_product_lowering_open=0
```

`MIM-FMEM-008` through `MIM-FMEM-017C` are landed. They promoted fastmem source
inventory, PageMapBridge evidence, TypedPageMetaHandle, AllocOwnerId/TLS
owner-state, same-owner local-free route evidence, AtomicRemoteHead plan
vocabulary, non-activating remote push/drain pilot evidence, and safe
capability wrapper plan evidence, then separated speed/shape/safety/coverage
keeper candidacy, normalized product-shaped bridge evidence, and tied the
replacement-front size-class mirror and page-local state evidence to `.hako`
`SizeClassBox` / `HakoAllocPageModel` policy while keeping product activation
closed.

`MIM-FMEM-017D` landed producer-neutral `replacement_front_producer` fields
before MIR lowering work. LLVM runner cleanup is the next separate follow-up
phase and must not be mixed into replacement-front producer taxonomy.

`MIM-FMEM-017C` landed as report/check-only:

```text
replacement_front_page_local_bridge_v0=1
replacement_front_page_local_bridge_bound=1
replacement_front_page_local_bridge_source_truth=hako_alloc.page_box
replacement_front_page_local_required_fields_present=1
replacement_front_page_local_required_methods_present=1
replacement_front_page_local_typed_meta_matches_source=1
replacement_front_page_local_same_owner_route_matches_source=1
replacement_front_page_local_no_remote_free_claim=1
product_activation_ready=0
```

Producer transition direction:

```text
diagnostic baseline:
  replacement_front_producer=python_template_c_bridge
  status=explicit_diagnostic_baseline_only
  semantic_ssot=0
  runtime_dependency=0

transition producer:
  replacement_front_producer=mir_to_c_lowering
  C remains only as backend artifact

final primary producer:
  replacement_front_producer=mir_to_llvm_lowering
  primary path has no intermediate C
  fastmem_producer_readiness_v0=1
```

MIRBuilder design consultation is a separate task after the page-local bridge
evidence is visible. The consultation should lock:

```text
MIRBuilder emits FastMemRegion/MemOp/contract/origin metadata.
MIRBuilder does not choose C vs LLVM.
MIRBuilder does not choose page-map or remote-free routes.
Planner selects route/producer.
Verifier guards contract/layout/escape/ABI boundary.
Lowering emits C/LLVM/object artifacts.
report.kv and hako_check stay producer-neutral.
```

Runner cleanup phase split:

```text
LLVM-PIPE-001:
  landed static inventory/report for current LLVM runner debt:
    NYASH_REWRITE_FUTURE env forcing
    method_id_injector no-op mutation seam
    joinir_experiment hook/fallback
    pyvm/harness/mock fallback route visibility

LLVM-PIPE-002:
  landed opt-in runtime report fields for pipeline/executor/fallback evidence.

LLVM-PIPE-003:
  landed named CompileOptions / PipelinePlan boxes for the current runner
  defaults. LoweringPlan remains future cleanup.
```

`MIM-FMEM-017D` landed as report/check-only:

```text
replacement_front_producer_taxonomy_v0=1
replacement_front_producer=python_template_c_bridge
replacement_front_backend_artifact=c
replacement_front_python_template_c_semantic_ssot=0
replacement_front_python_template_c_retirement_required=1
replacement_front_mir_memop_enabled=0
replacement_front_mir_fastmem_region_enabled=0
replacement_front_mirbuilder_representation_only=1
replacement_front_mirbuilder_route_decision_count=0
replacement_front_producer_transition_state=current_bridge
product_activation_ready=0
```

After MIR-FMEM-008E, the active candidate evidence is:

```text
replacement_front_producer=mir_to_llvm_lowering
fastmem_producer_readiness_v0=1
fastmem_producer_readiness_scope=layout_table_owner_runtime
memop_table_index_lowered_count>0
memop_field_load_lowered_count>0
memop_field_store_lowered_count>0
memop_current_alloc_owner_id_lowered_count>0
memop_owner_eq_lowered_count>0
```

`LLVM-PIPE-001` landed as static hako_check inventory:

```text
output_contract=hako-check-llvm-pipeline-inventory-v0
mir_future_rewrite_forced=1
method_id_injector_mutation_count=0
joinir_experiment_fallback_policy=original_mir
pyvm_reachable=1
pyvm_daily_route=0
execution_backend_order=pyvm,obj_out,ny_llvmc_exe,mock
llvm_fallback_used=0
llvm_fallback_reason=static_inventory_only
product_activation=0
```

`LLVM-PIPE-002` landed as opt-in runtime runner report:

```text
output_contract=hako-llvm-pipeline-runtime-report-v0
mir_future_rewrite_route=env_forced_llvm_future_externs
pipeline_joinir_experiment_enabled=0
method_id_injector_mutation_count=0
execution_backend=mock
llvm_fallback_used=1
llvm_fallback_reason=harness_unavailable_or_not_requested
mock_fallback_used=1
product_activation=0
```

`LLVM-PIPE-003` landed runner plan cleanup:

```text
pipeline_plan_v0=1
compile_options_v0=1
mir_future_rewrite_option=env_future_externs
method_id_injector_plan_enabled=1
joinir_experiment_hook_plan_enabled=1
runner_behavior_change=0
```

`MIM-FMEM-017B` landed as report/check-only:

```text
replacement_front_size_class_bridge_v0=1
replacement_front_size_class_bridge_bound=1
replacement_front_size_class_bridge_source_truth=hako_alloc.size_class_box
replacement_front_size_class_required_methods_present=1
replacement_front_size_class_policy_constants_covered=1
replacement_front_size_class_policy_huge_sentinel_covered=1
replacement_front_size_class_policy_mirror_matches_source=1
product_activation_ready=0
```

## Algorithm Port Coverage

The detailed algorithm-port ledger has been archived to:

```text
docs/development/current/main/investigations/mimalloc-current-docs-slim-archive-2026-06-08.md
```

Keep the active restart surface compact; use the archive for the historical
bridge decisions, benchmark ledgers, and non-keeper probes.

## Current Evidence Anchors

The detailed evidence anchors have also moved to the archive above.

## Next Task Order

Current restart-critical pointers are already in `CURRENT_STATE.toml` and the
short task order above. Do not duplicate the long historical ladder here.

## Replacement Front Hot-Path Plan

Historical REPL-001..REPL-018 evidence was archived to:

```text
docs/development/current/main/investigations/mimalloc-current-history-2026-06-02.md
```

Current replacement-front truth:

```text
fixed_slot_native_front=available
matched_hako_good_size_slot=available
multi_bin_native_benchmark_front=available_single_thread_v0
page_bin_benchmark_front=available_single_thread_v0
locked_global_multithread_front=positive_local_evidence_v0
thread_local_multithread_front=correctness_smoke_available_not_perf_keeper
product_pages=not_connected
provider_activation=0
production_replacement_active=0
winner_claim=0
```

Next replacement-front order:

1. Keep `--replacement-front-native-bins-mode` benchmark-only and single-thread
   until a thread/page plan is selected.
2. Treat the locked global counterless front as the current local multithread
   performance evidence owner; keep thread-local as correctness/smoke evidence
   until perf/asm selects a concrete thread-local hot cost.
3. Open product pages only after bins evidence says pages are the owner.
4. Reopen `.hako` core or generated-C local optimization only with fresh
   structural owner evidence.
5. Keep detailed evidence in the investigation archive, not this active card.

Product pages v0 boundary:

```text
replacement_front_page_bins_plan_v0=1
replacement_front_page_bins_supported=1
replacement_front_page_bins_consumer_enabled=0 by default; 1 only when
  --replacement-front-page-bins-mode is selected
replacement_front_page_bins_route=not_consumed | benchmark_page_bins
replacement_front_page_bins_owner=benchmark_only
replacement_front_page_bins_threading=single_thread_until_plan_selected
replacement_front_page_bins_product_claim=0
```

The first implementation is a benchmark-only page/bin-backed route that keeps
provider activation, product replacement, hooks, global allocator, and winner
claims closed. It consumes the workload regular bins and adds a page-shaped
owner structure, but it must not claim full `.hako` mimalloc until the coverage
report stops saying `replacement_front_is_full_hako_algorithm=0`.

## Daily Commands

Build the Hakozuna mixed-ws fixture:

```bash
make -C benchmarks/external/hakozuna/mixed-ws
```

Run same-machine C mimalloc comparison:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --out target/hakozuna-mixed-ws-compare/report.out \
  --out-dir target/hakozuna-mixed-ws-compare/artifacts \
  --sample-count 5
```

Add the optional Hakorune provider subject:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --manifest target/.../provider/pkg/hakorune_provider.json \
  --out target/hakozuna-mixed-ws-compare-provider/report.out \
  --out-dir target/hakozuna-mixed-ws-compare-provider/artifacts \
  --sample-count 5
```

Run the benchmark-only replacement-front smoke/evidence subject:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-thread-local-mode \
  --replacement-front-tls-counter-mode \
  --replacement-front-cross-thread-smoke \
  --replacement-front-slot-size 1024 \
  --out target/hakozuna-mixed-ws-replacement-smoke/report.out \
  --out-dir target/hakozuna-mixed-ws-replacement-smoke/artifacts \
  --sample-count 5
```

Run the replacement-front performance distribution subject after smoke:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-lock-mode \
  --replacement-front-skip-hot-counters \
  --replacement-front-slot-size 1024 \
  --threads 2 \
  --out target/hakozuna-mixed-ws-replacement-perf/report.out \
  --out-dir target/hakozuna-mixed-ws-replacement-perf/artifacts \
  --sample-count 7
```

Run the single-thread benchmark-only multi-bin front:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-bins-mode \
  --threads 1 \
  --out target/hakozuna-mixed-ws-native-bins/report.out \
  --out-dir target/hakozuna-mixed-ws-native-bins/artifacts \
  --sample-count 5
```

Run pointer guard after docs pointer edits:

```bash
bash tools/checks/current_state_pointer_guard.sh
```

## Parking Lot

- `MIR-FMEM-008D-PRE` fixed owner-runtime scope: CurrentAllocOwnerId is an
  observation scalar first, OwnerEq is equality only, and 008D must not open
  local/remote free routing, TLS backing transfer, AtomicRemoteHead, or
  mimalloc body migration.
- `MIR-FMEM-008D-A` lowered CurrentAllocOwnerId to a producer-local LLVM helper
  call returning an ordinary i64 scalar. It did not open TLS backing transfer,
  owner reuse, routing, or product activation.
- `MIR-FMEM-008D-B` fixed OwnerEq as equality-only lowering over ordinary
  owner-id scalars. It did not open same-owner local_free, remote-free, or
  allocator lifecycle behavior.
- `MIR-FMEM-008D-C` added an owner-runtime `fastmem-check` profile that requires
  positive CurrentAllocOwnerId and OwnerEq lowered counts while rejecting TLS
  transfer, owner slot reuse, AtomicRemoteHead, ABI hot paths, and activation
  claims.
- `DIRECTARRAY-FMEM-COMMON-001` is queued as proof-envelope/report adapter
  work only. DirectArray access does not auto-generate a fastmem region in the
  current lane.
- `DOCS-SLIM-FMEM-SSOT-001` landed by slimming the design SSOT and moving the
  historical evidence to the investigation/archive owners instead of growing
  this restart card.
- DirectMemory / Span / Bytes / LayoutSpan remain future substrate work.
- `direct {}` remains parked; use RequiredFastPathRegion diagnostics first.
- `DirectArray<T>` generic source form remains parked; v0 source-visible type is
  concrete `DirectArrayI64`.
- `RecordStateResidencePlanV0` stays a narrow box-private primitive residence
  plan, not record-as-box or ordinary-box auto-recordification.
- Mixed-base helper extraction stays parked unless `EffectSummary` /
  `ReceiverSnapshotPublicationPlanV0` evidence selects it again.
- External Ubuntu benchmark numbers remain non-horizontal until CPU and run
  conditions are aligned.
