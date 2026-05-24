---
Status: Active
Date: 2026-05-12
Scope: taskboard for exact `usize` / pointer-sized unsigned semantics.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/README.md
---

# 294x-90 Usize Semantics Taskboard

## Rule

One row should add one durable semantic slice. Do not combine metadata
preservation, runtime behavior, backend lowering, and hako_alloc migration in
one commit unless a row explicitly says it is docs-only.

VM rows are semantic reference execution rows, not product-owner rows. They may
consume MIR-owned facts/contracts, but VM-only behavior is not completion for
hako_alloc or mimalloc migration.

## Quick Current Truth

- `294x-10f` landed the VM reference exact numeric value representation.
- Production `hako_alloc` facade-local stats are exact `usize`; remaining
  page/heap/queue/handle fields stay `i64` except for explicitly migrated
  page-map, release-seam, and realloc same-class/no-move counter groups.
- Mimalloc `.hako` algorithm rows may continue, but they must not claim
  production `usize` field migration yet.
- Native exact numeric typed-object slot representation exists in
  `nyash_kernel`.
- Exact numeric signed/unsigned field helper lanes exist in `nyash_kernel`.
- Python LLVM and the pure-first C shim consume exact typed-object field ABI for
  exact-storage plans.
- Python LLVM consumes exact add/sub/mul, compare, and logical-shift route
  facts; div/mod/bitwise/wrapping stay later vocabulary.
- Further production `hako_alloc` migration remains field-group gated.
- Mimalloc `.hako` work should now target a comparison-quality vertical slice,
  not a full allocator-wide port. Required fields and paths should be selected
  by the workload/report slice below; broad field migration, provider
  activation, DLL packaging, and host allocator replacement remain parked.

## Next Implementation Queue

| Order | Row | Status | Implementation Boundary |
| --- | --- | --- | --- |
| 0 | `293x-185` | Complete | Allocate a replacement ptr, model copy count, and release the old ptr only after success. |
| 1 | `293x-186` | Complete | Realloc negative matrix and failure contract, no extra API expansion. |
| 2 | `293x-187` | Complete | Alignment normalization, power-of-two validation, and padded-size policy only. |
| 3 | `293x-188` | Complete | Alignment metadata now attaches to normal page-map-backed small allocations. |
| 4 | `M179-M184` | Next | Huge-page and secure-list rows, one responsibility per row. |
| 5 | `M185-M190` | Planned | Remaining `usize` migration and object-return/failure-handle API parity. |

Roadmap correction: `M186 exact usize facade stats` is already complete as
`294x-19e`. Do not schedule duplicate facade migration; use `M185+` for
remaining field groups and allocator API parity only.

## Mimalloc Comparison Vertical Slice Queue

This queue overrides the tempting "finish every remaining allocator field"
interpretation. The short-term goal is a measurable `.hako` / `hako_alloc`
slice that can be compared against the existing C mimalloc runner evidence,
not a full native mimalloc-compatible allocator.

| Order | Slice | Validation | Boundary |
| --- | --- | --- | --- |
| V0 | Select the comparison workload pack | docs + manifest/static guard | Selected by `294x-53`. Use a small fixed-size alloc/free/reuse workload, a mixed small-size workload, realloc same-class/grow fallback, aligned-small, and huge/OSVM-backed allocation. Do not add provider activation or host replacement. |
| V1 | Close only comparison-required `usize` fields | field-group L2, L3 only when first-pattern requires it | Started by `294x-54` for the OSVM-backed byte-length seam. Migrate request size, block size, capacity, queue count/index, and report counters only when the workload consumes them. Keep ids, pointer payloads, sentinels, and status flags signed until their own contracts are needed. |
| V2 | Hako alloc small-path comparison slice | VM + MIR + route preflight; representative EXE closeout | Started by `294x-55` / `MIMALLOC-COMPARISON-VSLICE-003` as a model-only schema pilot. Compose existing size-class, page model, page queue, page-map release, and local reuse paths into one stable output schema. No remote-free stress, TLS, abandoned heap, or atomic bitmap expansion. |
| V3 | Realloc/aligned comparison slice | same as V2 | Started by `294x-57` / `MIMALLOC-COMPARISON-VSLICE-005` as a model-only schema pilot. Reuse M174-M178 behavior and produce requested bytes, copied bytes, live handles, failure reason, and alignment metadata evidence. No new API surface unless the report schema requires it. |
| V4 | Huge/OSVM comparison slice | MIR + route preflight + representative pure-first EXE | Started by `294x-58` / `MIMALLOC-COMPARISON-VSLICE-006` as an OSVM-backed schema pilot. Reuse M179-M181 and existing OSVM page-source composition for huge requests, reporting reserve/commit/decommit evidence without widening page-source ownership. |
| V5 | C mimalloc vs `.hako` report closeout | representative L3 / allocator-wide only at closeout | Landed by `294x-59` / `MIMALLOC-COMPARISON-VSLICE-007`. Aligns the selected V2/V3/V4 `.hako` output schema with the existing C mimalloc explicit runner planning surface: requested bytes, committed/live bytes or handles, operation counts, failure reasons, and RSS/memory-use evidence where available. `294x-228` refreshed this path as `MIMALLOC-COMPARISON-VSLICE-009` after the exact `usize` field-group drain. |

Defer beyond this queue:

- full size-class table parity;
- true worker/TLS and remote-free stress;
- abandoned heap reclamation;
- atomic bitmap execution;
- provider/DLL/global allocator integration;
- complete replacement of all remaining `i64` allocator fields.

Rule: if a row does not help V0-V5 produce comparable evidence, it should be
parked or batched into a later native-allocator phase.

## Phase Closeout Target

Close phase 294x after the comparison vertical slice has enough exact `usize`
storage to produce stable `.hako` / `hako_alloc` reports and compare them with
the C mimalloc runner evidence.

Do not keep extending this phase to drain:

- report mirrors / `ReportFields` payload mirrors;
- bool/status/reason vocabulary fields;
- signed sentinel-bearing ids, indexes, and deltas;
- broad page/heap/queue/handle state outside the comparison slice;
- provider/DLL packaging, hook installation, host/global allocator replacement,
  worker/TLS, true threads, remote-free stress, or abandoned-heap stress.

Next field-group rows should therefore prefer owner-local monotonic counters
that the comparison slice already reads. If the next candidate is only a mirror
or a broad identity/payload field, park it and move to closeout planning.

## Field Group Ledger

The long landed field-group blocker history moved to:

```text
docs/development/current/main/phases/phase-294x/294x-usize-field-group-ledger.md
```

Current blocker is owned by `CURRENT_STATE.toml` and mirrored below only for
human restart clarity.

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-261-PAGE-HEAP-CAPACITY-001:
  selected current after 294x-261. Migrate only
  `HakoAllocPage.capacity` to exact `usize`. Do not migrate page id, free_top,
  handle ids, method parameter surfaces, requested_sizes array payload
  semantics, page-model production fields, provider/DLL seams, hooks,
  worker/TLS, atomics, or `#[global_allocator]`.
```

## Cleanup Slice Queue

| Order | Row | Status | Boundary |
| --- | --- | --- | --- |
| C0 | `294x-219` | Landed | Inventory and phase-lock for proof app entrypoint cleanup. |
| C1 | `294x-220` | Landed | Manifest-backed proof app `test.sh` delegates now use `tools/checks/lib/proof_app_test_entry.sh`. |
| C2 | `GUARD-WRAPPER-CLEANUP-001` | Later | Classify direct guard delegates before touching them. |
| C3 | `PROOF-APP-TEMPLATE-CLEANUP-001` | Later | Only after entrypoint compatibility stays green. |

## Ladder

| Row | Status | Scope | Done When |
| --- | --- | --- | --- |
| `294x-00` | Complete | phase lock and full visible task inventory | SSOT, README, taskboard, current pointers are in place |
| `294x-01` | Complete | target-width and numeric-kind SSOT in code | target pointer width owner exists; `usize` no longer depends on ad hoc host assumptions |
| `294x-02` | Complete | parser metadata preservation | method, static method, and `birth` params keep declared type metadata; return annotations are preserved where accepted |
| `294x-03` | Complete | AST JSON / Program(JSON) numeric metadata | declared param/return type text round-trips through JSON metadata without changing runtime semantics |
| `294x-04` | Complete | MIR exact numeric type model | signedness/width/pointer-width are represented as side-car MIR metadata distinct from `MirType::Integer` |
| `294x-05` | Complete | exact numeric constants and conversions | constants and dynamic integer conversions range-check into exact numeric metadata |
| `294x-06` | Complete | verifier negative/range fail-fast | statically known exact numeric field writes reject negative and out-of-range values under the MIR verifier |
| `294x-06b` | Complete | dynamic numeric field write guard | runtime-range-sensitive exact numeric fields reject unchecked dynamic values until runtime-check lowering exists |
| `294x-06c` | Complete | runtime-check contract metadata | dynamic exact numeric field writes can be verifier-accepted only with a matching `DynamicIntegerRange` contract |
| `294x-06d` | Complete | VM dynamic range-check execution | the VM interpreter executes existing `DynamicIntegerRange` contracts at `FieldSet` sites and rejects bad dynamic values before mutation |
| `294x-06e` | Complete | dynamic range-check contract refresh | real MIR `FieldSet` producers receive `DynamicIntegerRange` contracts after optimization and before verification |
| `294x-06f` | Complete | backend runtime-check contract fail-fast | unsupported non-VM backend routes reject modules that still carry exact numeric runtime-check contracts |
| `294x-07` | Complete | overflow and checked arithmetic policy | exact numeric add/sub/mul policy is checked/fail-fast; wrapping stays explicit future vocabulary |
| `294x-08` | Complete | unsigned compare and logical shift | exact numeric compare and logical right-shift policy no longer borrow signed i64 semantics |
| `294x-09` | Complete | PHI/Select numeric unification policy | exact numeric facts merge conservatively and fail fast on exact/dynamic or exact/exact mismatches |
| `294x-09a` | Complete | VM reference-executor boundary | VM is a semantic reference executor, not the product/mainline backend owner |
| `294x-09b` | Complete | exact numeric value facts v0 | field reads, copies, and conservative control merges publish MIR-owned exact numeric value facts before VM reference execution |
| `294x-09c` | Complete | exact numeric signature facts v0 | declared params seed MIR-owned exact numeric value facts and declared returns publish function-level exact numeric facts |
| `294x-09d` | Complete | exact numeric add route facts v0 | exact `+` routes are MIR-owned facts before VM reference execution consumes them |
| `294x-09e` | Complete | dev gate quick profile split | daily quick stays slim while allocator-wide owns the full allocator/mimalloc/provider proof ladder |
| `294x-09f` | Complete | quick first-row cargo filter grouping | quick first-row guards group related cargo filters without changing semantic coverage |
| `294x-10` | Complete | VM reference exact `usize` Add route v0 | VM reference execution consumes MIR-owned exact numeric Add route facts without making VM-only behavior a completion criterion |
| `294x-10b` | Complete | VM reference checked arithmetic routes | VM reference execution consumes MIR-owned exact numeric Add/Sub/Mul route facts without VM-owned inference |
| `294x-10c` | Complete | VM reference exact compare routes | VM reference execution consumes MIR-owned exact numeric compare route facts without VM-owned inference |
| `294x-10d` | Complete | VM exact ops module split | exact numeric VM reference execution is split by operation family before more rows land |
| `294x-10e` | Complete | VM reference exact logical shr routes | VM reference execution consumes MIR-owned exact unsigned logical right-shift route facts |
| `294x-10f` | Complete | VM exact numeric runtime value | VM reference exact numeric arithmetic/shift results stay tagged instead of collapsing back to `Integer(i64)` |
| `294x-11` | Complete | literal suffix and const-eval row | `0usize` / exact numeric consts are accepted only with range checks and preserved as MIR exact const facts |
| `294x-12` | Complete | typed-object exact numeric storage | typed-object plans distinguish exact numeric storage names such as `usize` from legacy `i64` while runtime values stay on the integer lane |
| `294x-13` | Complete | backend capability and fail-fast | unsupported non-VM backends reject exact numeric storage/op routes before emission; native lowering remains a later row |
| `294x-14a` | Complete | byte-length usize facade aliases | RawBuf and OSVM byte-length facades expose `usize` names over the non-negative current-lane i64 subset |
| `294x-14` | Complete | low-level capability usize variants | Buf/RawArray/bounds/initialized-range helpers expose provisional `usize` aliases over the non-negative current-lane i64 subset; RawBuf stays byte-buffer only and OSVM byte-length aliases remain from 294x-14a |
| `294x-15` | Complete | raw-layout pointer-sized field row | `usize`/`isize` raw fields are accepted with target layout rules while source syntax/backend execution remain out of scope |
| `294x-16` | Complete | hako_alloc numeric field inventory | every numeric stored field is classified as signed sentinel, signed delta, count, size, capacity, index, or byte length |
| `294x-17` | Complete | sentinel split plan | direct-page stored `-1` sentinel is split into explicit presence state before any `usize` migration |
| `294x-18` | Complete | hako_alloc non-negative field migration probe | capacity/count/byte-length candidates migrate in a proof app while production fields stay signed/current-lane |
| `294x-19` | Blocked | hako_alloc production facade migration | waits for exact typed-object storage plus backend consumption of the exact field ABI |
| `294x-19a` | Complete | native exact numeric typed-object slots | kernel typed-object storage records exact slot kinds and legacy i64 helpers do not mutate exact numeric slots |
| `294x-19b` | Complete | exact numeric field get/set ABI | runtime helpers read/write exact signed/unsigned slots with range/overflow contracts |
| `294x-19c` | Complete | exact field ABI backend consumption | Python LLVM carries typed-object plans, registers exact layouts, creates exact typed-object handles, and lowers exact field get/set helpers |
| `294x-19d` | Complete | exact op backend subset | Python LLVM lowers exact add/sub/mul, compare, and logical-shift route facts with checked traps |
| `294x-19e` | Complete | hako_alloc production facade stats migration | facade-local event counters migrate to exact `usize`; page/heap/queue/handle fields remain `i64` |
| `294x-20` | Complete | mimalloc row resume gate | M167+ mimalloc implementation resumes with clear `usize` support boundaries while page/heap/queue state remains `i64` |

## Required Feature Checklist

### Spec

- [x] Define exact `usize` range owner by target pointer width.
- [x] Define overflow behavior.
- [x] Define logical shift behavior.
- [x] Define unsigned comparison behavior.
- [x] Define conversion from dynamic `Integer(i64)`.
- [x] Define unsupported backend fail-fast tags.
- [x] Define when `i64` remains preferred.

### Parser / AST / JSON

- [x] Preserve method parameter type annotations.
- [x] Preserve static method parameter type annotations.
- [x] Preserve `birth` parameter type annotations.
- [x] Preserve return type annotations or reject them consistently.
- [x] Round-trip declared numeric metadata through AST JSON / Program(JSON).
- [x] Keep Rust and `.hako` parser fronts aligned for the current exact numeric gap set.
  - Rust parser supports: literal suffixes (`0usize`), parameter type annotations,
    return type annotations, field type annotations with exact numeric types.
  - Stage-B `.hako` parser (`lang/src/compiler/parser/`) now supports literal
    suffixes and preserves them as Program(JSON v0) `Int.declared_type`
    metadata (`294x-245`).
  - Stage-B FuncScanner / JSON builder now preserves method parameter type
    annotations as Program(JSON v0) `param_decls[].declared_type` while keeping
    `params` as bare names (`294x-246`).
  - Stage-B FuncScanner / JSON builder now preserves method return type
    annotations as Program(JSON v0) `return_type` metadata (`294x-247`).
  - Stage-B enrichment now preserves user-box field type annotations as
    Program(JSON v0) `user_box_decls[].field_decls[].declared_type` metadata
    (`294x-248`).
  - Next row: return to explicit `hako_alloc` field-group selection.

Field-group selection after parser-front alignment:

- [x] Select `HakoAllocObjectLifecycleReallocResult.last_requested_size` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-249-REALLOC-REQUESTED-SIZE-RESULT-OBSERVER-001`
  (`294x-249`).
  - It is a non-negative requested-size observer initialized to `0`.
  - It does not carry page/block identity, pointer payload, reason vocabulary,
    or ok/bool-like status.
  - Stop line: keep `last_page_id`, `last_block_id`, `last_new_page_id`,
    `last_new_block_id`, `last_reason`, and `last_ok` signed.
- [x] Migrate `HakoAllocObjectLifecycleReallocResult.last_requested_size` to
  exact `usize` (`294x-250`).
  - Stop line preserved page/block id sentinels, reason/ok fields, alignment
    result observers, and huge requested-size observers as signed.
- [ ] Select the next explicit non-negative production field group as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-002`.
- [x] Select page-queue numeric inventory drift cleanup before the next code
  migration (`294x-251`).
  - This is metadata cleanup only. It must not change `page_queue_box.hako`
    semantics.
- [ ] Synchronize stale `HakoAllocPageQueue` detailed numeric inventory rows as
  `HAKO-ALLOC-USIZE-NUMERIC-INVENTORY-PAGE-QUEUE-DRIFT-CLEANUP-001`.
- [x] Synchronize stale `HakoAllocPageQueue` detailed numeric inventory rows
  with already-exact storage (`294x-252`).
  - No `.hako` behavior changed.
- [ ] Select the next explicit non-negative production field group as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-003`.
- [x] Select `HakoAllocPage.current_used` and `HakoAllocPage.peak_used` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-253-PAGE-HEAP-OCCUPANCY-001` (`294x-253`).
  - Stop line: keep handle/page ids, block size, capacity, free_top, and
    requested_bytes signed.
- [x] Migrate `HakoAllocPage.current_used` and `HakoAllocPage.peak_used` to
  exact `usize` (`294x-254`).
  - Stop line preserved handle/page ids, block size, capacity, free_top, and
    requested_bytes as signed.
- [ ] Select the next explicit non-negative production field group as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-004`.
- [x] Select `HakoAllocPage.requested_bytes` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-255-PAGE-HEAP-REQUESTED-BYTES-001`
  (`294x-255`).
  - Stop line: keep handle/page ids, handle requested size, block size,
    capacity, free_top, and requested_sizes array payload semantics signed.
- [x] Migrate `HakoAllocPage.requested_bytes` to exact `usize` (`294x-256`).
  - Stop line preserved handle/page ids, handle requested size, block size,
    capacity, free_top, and requested_sizes array payload semantics as signed.
- [ ] Select the next explicit non-negative production field group as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-005`.
- [x] Select `HakoAllocHandle.requested_size` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-257-PAGE-HEAP-HANDLE-REQUESTED-SIZE-001`
  (`294x-257`).
  - Stop line: keep handle ids, page/block fields, method parameter types, and
    requested_sizes array payload semantics signed/current-lane.
- [x] Migrate `HakoAllocHandle.requested_size` to exact `usize` (`294x-258`).
  - Stop line preserved handle ids, page/block fields, method parameter types,
    and requested_sizes array payload semantics as current-lane.
- [ ] Select the next explicit non-negative production field group as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-006`.
- [x] Select `HakoAllocPage.block_size` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-259-PAGE-HEAP-BLOCK-SIZE-001`
  (`294x-259`).
  - Stop line: keep page id, capacity, free_top, handle ids, method parameter
    surfaces, and requested_sizes array payload semantics current-lane.
- [x] Migrate `HakoAllocPage.block_size` to exact `usize` (`294x-260`).
  - Stop line preserved page id, capacity, free_top, handle ids, method
    parameter surfaces, and requested_sizes array payload semantics.
- [ ] Select the next explicit non-negative production field group as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-007`.
- [x] Select `HakoAllocPage.capacity` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-261-PAGE-HEAP-CAPACITY-001`
  (`294x-261`).
  - Stop line: keep page id, free_top, handle ids, method parameter surfaces,
    and requested_sizes array payload semantics current-lane.

### MIR / Analysis

- [x] Add exact numeric MIR type representation.
- [x] Preserve signedness and width.
- [x] Preserve pointer-width target metadata owner.
- [x] Add exact numeric constants or constant metadata.
- [x] Add conversion/cast vocabulary.
- [x] Add PHI/Select unification rules.
- [x] Publish exact numeric value facts for field reads, copies, and control merges.
- [x] Publish route facts for numeric params and returns.
- [x] Publish exact numeric op route facts for first arithmetic producers.
- [x] Add checked exact numeric add/sub/mul policy helpers.
- [x] Add exact numeric compare and logical right-shift policy helpers.

### Runtime / VM

- [x] Add exact `usize` runtime representation or equivalent tagged numeric value.
- [x] Define VM as semantic reference executor, not product/mainline owner.
- [x] Execute existing `DynamicIntegerRange` contracts in the VM interpreter.
- [x] Attach `DynamicIntegerRange` contracts for real exact numeric field-write
  producers after MIR shape is stable.
- [x] Range-check literal construction before exact numeric const facts are published.
- [ ] Range-check construction beyond exact numeric field-write contracts and typed literals.
- [x] Implement checked add/sub/mul in live VM exact numeric op routes.
- [ ] Implement div/mod with zero checks.
- [ ] Implement bitwise ops.
- [x] Implement logical right shift in live VM exact numeric op routes.
- [x] Implement unsigned compare in live VM exact numeric op routes.
- [x] Define display/debug formatting.
- [x] Emit stable diagnostics for overflow/range/shift failures in VM reference routes.

### Verifier / Guards

- [x] Reject negative statically known field assignment to `usize`.
- [x] Reject `-1` sentinel field assignment to `usize` when statically known.
- [x] Reject unchecked dynamic field assignment when the exact numeric range
  does not cover all dynamic `Integer(i64)` values.
- [x] Publish `DynamicIntegerRange` runtime-check contract metadata for exact
  numeric field writes.
- [x] Execute `DynamicIntegerRange` contracts in the VM interpreter before
  field mutation.
- [x] Keep verifier and contract refresh on one shared exact numeric field-write
  facts owner.
- [x] Reject unsupported backend lowering.
- [x] Guard against silent fallback to `Integer(i64)` for exact numeric
  runtime-check contracts.
- [ ] Keep strict/dev checks before broad production acceptance.

### Storage / Backend

- [x] Add typed-object exact numeric storage names to layout plans.
- [x] Fail fast on unsupported backend routes before exact numeric typed-object
  storage or op-route facts silently use legacy `Integer(i64)` lowering.
- [x] Add backend/runtime native `usize` slots.
- [x] Add field get/set ABI for exact numeric slots.
- [x] Add backend lowering/capability-gate consumption for exact numeric field
  get/set ABI.
- [x] Lower Python LLVM exact add/sub/mul, unsigned compare, and logical-shift
  route facts.
- [ ] Add exact numeric div/mod/bitwise/wrapping backend vocabulary if needed.
- [ ] Decide WASM target behavior.
- [ ] Keep C ABI size_t mapping explicit.
- [x] Accept raw layout pointer-sized fields only through target-resolved
  layout rules.

### Low-Level Capability Surface

- [x] RawBuf byte-length `usize` allocation/reallocation facades over the
  non-negative current-lane i64 subset.
- [x] RawBuf length/capacity `usize` variants stay out of scope because
  RawBuf intentionally owns no len/cap policy.
- [x] RawArray length/capacity/index `usize` variants.
- [x] OSVM page size and byte-length `usize` facades over the non-negative
  current-lane i64 subset.
- [x] Bounds checks over `usize`.
- [ ] Atomic or TLS `usize` rows only if needed by allocator proofs.
- [ ] Existing `*_i64` helpers remain until call sites migrate.

### Hako Alloc / Mimalloc

- [x] Inventory every numeric hako_alloc stored field.
- [x] Split the direct-page stored sentinel and keep not-found return sentinels
  signed until their API shape changes.
- [x] Probe capacity/count/byte-length `usize` fields in an isolated hako_alloc
  proof app before production migration.
- [x] Probe stack-top `usize` decrement/increment paths with explicit
  underflow/overflow rejects in the isolated hako_alloc proof app.
- [x] Probe exact `usize` stack-top values as `ArrayBox.get/set` indexes in the
  isolated hako_alloc proof app.
- [x] Migrate production page-model stack-top/occupancy fields after the
  proof-only stack-top and ArrayBox-index probes.
- [x] Probe exact `usize` capacity bounds with current-lane signed loop/index
  values before production page capacity migration.
- [x] Migrate production page-model capacity/reserved fields after the
  proof-only capacity-bound probe.
- [x] Probe exact `usize` request-size / block-size comparison and
  accepted-request byte-sum accumulation before production page-model
  size/byte fields migrate.
- [x] Migrate production page-model `block_size` / `requested_bytes` fields
  after the proof-only request byte-sum probe.
- [x] Mark production `usize` field migration blocked on non-VM exact numeric
  storage, exact field ABI, and backend ABI consumption.
- [x] Update first production proof apps for the facade stats field group.
- [x] Migrate the first production non-negative field group after exact field
  ABI backend consumption and needed exact op backend subset are green.
- [ ] Migrate remaining production non-negative fields only by explicit
  field-group rows.
  - Current migrated candidate: `HakoAllocPageMap` counter fields (`entry_count`,
    `live_count`, `register_count`, `lookup_count`, `lookup_miss_count`,
    `unregister_count`, `reject_count`). All non-negative counts, no sentinels,
    owner-local to one box. Low-risk per NUMERIC_FIELDS.md.
  - Row: `294x-21-HAKO-ALLOC-USIZE-PAGE-MAP-COUNTERS.md`.
  - Proof: existing page-map proof app verifies behavior-preserving counters.
  - Guard: existing page-map guard checks exact `usize` typed-object storage.
  - Stop line: do not migrate page-map entry pointer/id fields in this group.
  - Follow-on migrated group: `HakoAllocPageMapReleaseSeam` event/reject
    counters (`page_register_count`, `release_count`, `unregister_count`,
    `lookup_miss_count`, `stale_page_count`, `page_release_reject_count`,
    `reject_count`) in
    `294x-22-HAKO-ALLOC-USIZE-PAGE-MAP-RELEASE-COUNTERS.md`.
  - Stop line: keep `page_count` signed until the page-id/page-count
    comparison contract is split.
  - Follow-on migrated group: `HakoAllocPageMapReallocSameClassPath`
    event/reject counters (`same_class_count`, `grow_reject_count`,
    `lookup_miss_count`, `stale_page_count`, `released_block_count`,
    `reject_count`) in
    `294x-23-HAKO-ALLOC-USIZE-PAGE-MAP-REALLOC-SAME-CLASS-COUNTERS.md`.
  - Stop line: keep `last_result_ptr` signed/pointer-shaped until pointer
    result handles are migrated by their own row.
  - Follow-on migrated group: `HakoAllocPageMapReallocAllocCopyReleasePath`
    fallback event/reject counters (`success_count`, `copy_count`,
    `same_class_reject_count`, `alloc_fail_count`, `lookup_miss_count`,
    `stale_page_count`, `released_block_count`, `reject_count`) in
    `294x-24-HAKO-ALLOC-USIZE-PAGE-MAP-REALLOC-ALLOC-COPY-RELEASE-COUNTERS.md`.
  - Stop line: keep `next_ptr`, `last_result_ptr`, and `last_alloc_*`
    signed/pointer-shaped or sentinel-bearing until their own rows.
  - Follow-on migrated group: `HakoAllocPageMapReallocFailureContract`
    failure-matrix event/reject counters (`success_count`,
    `same_class_success_count`, `move_success_count`, `zero_reject_count`,
    `oversized_reject_count`, `alloc_fail_count`, `lookup_miss_count`,
    `stale_page_count`, `released_block_count`, `unexpected_reject_count`,
    `reject_count`) in
    `294x-25-HAKO-ALLOC-USIZE-PAGE-MAP-REALLOC-FAILURE-CONTRACT-COUNTERS.md`.
  - Stop line: keep `last_result_ptr`, `last_failure_kind`, and
    `last_max_block_size` as signed/pointer/status/size observers.
  - Follow-on migrated group: `HakoAllocPageMapAlignedSmallPath`
    event/reject counters (`alloc_count`, `invalid_alignment_count`,
    `oversized_count`, `alloc_fail_count`, `register_fail_count`,
    `reject_count`) in
    `294x-26-HAKO-ALLOC-USIZE-ALIGNED-SMALL-PATH-COUNTERS.md`.
  - Stop line: keep `meta_count`, `next_ptr`, `last_result_ptr`,
    `last_alignment`, and `last_padded_size` signed until metadata-store,
    pointer, alignment, and size observer contracts are split.
  - Follow-on migrated group: `HakoAllocHugeThresholdRouter` route/reject
    counters (`small_route_count`, `small_success_count`,
    `small_reject_count`, `huge_route_count`, `huge_reject_count`,
    `invalid_alignment_count`, `invalid_size_count`, `reject_count`) in
    `294x-27-HAKO-ALLOC-USIZE-HUGE-THRESHOLD-ROUTER-COUNTERS.md`.
  - Stop line: keep route-kind, pointer, size, and threshold observer fields
    signed until their own exactness contracts are split.
  - Follow-on migrated group: `HakoAllocPageQueue` stats counters
    (`add_count`, `select_count`, `direct_hit_count`, `refresh_count`,
    `reject_count`) in `294x-28-HAKO-ALLOC-USIZE-PAGE-QUEUE-COUNTERS.md`.
  - Stop line: keep `bin`, `page_count`, `has_direct_page`, and
    `direct_page_index` signed until queue index/presence contracts are split.
  - Follow-on migrated group: `HakoAllocPageQueue.page_count` in
    `294x-48-HAKO-ALLOC-USIZE-PAGE-QUEUE-PAGE-COUNT.md`.
  - Stop line: keep `bin`, `has_direct_page`, and `direct_page_index` signed.
  - Follow-on migrated group: `HakoAllocPageQueue.direct_page_index` in
    `294x-49-HAKO-ALLOC-USIZE-PAGE-QUEUE-DIRECT-INDEX.md`.
  - Follow-on selected group: `HakoAllocPageQueue.bin` in
    `294x-70-HAKO-ALLOC-USIZE-PAGE-QUEUE-BIN-SELECTION.md`.
  - Follow-on migrated group: `HakoAllocPageQueue.bin` in
    `294x-71-HAKO-ALLOC-USIZE-PAGE-QUEUE-BIN.md`.
  - Stop line: keep `has_direct_page` signed; heap-level bin mirrors and
    size-class return shapes remain separate rows.
  - Follow-on migrated group: `HakoAllocPageModel` local page counters
    (`alloc_count`, `local_free_count`, `reject_count`) in
    `294x-29-HAKO-ALLOC-USIZE-PAGE-MODEL-LOCAL-COUNTERS.md`.
  - Stop line: keep page identity, size/capacity, stack-top, live-count,
    collection, lifecycle, and byte-length fields signed until their own
    contracts are split.
  - Follow-on migrated group: `HakoAllocPageModel` local-free collection
    counters (`local_free_collect_count`, `local_free_collected_blocks`) in
    `294x-30-HAKO-ALLOC-USIZE-PAGE-MODEL-COLLECTION-COUNTERS.md`.
  - Stop line: keep stack-top, live-count, lifecycle, and byte-length fields
    signed until their own contracts are split.
  - Follow-on migrated group: `HakoAllocPageModel` lifecycle event/reject
    counters (`retire_count`, `decommit_count`, `recommit_count`,
    `reuse_count`, `lifecycle_reject_count`, `reactivate_count`,
    `reactivate_reject_count`) in
    `294x-31-HAKO-ALLOC-USIZE-PAGE-MODEL-LIFECYCLE-COUNTERS.md`.
  - Stop line: keep `retired` / `decommitted` lifecycle state flags,
    stack-top/live-count, identity, size/capacity, and byte-length fields
    signed until their own contracts are split.
  - Follow-on migrated group: `HakoAllocAlignedSmallMetaStore.count` and
    `HakoAllocPageMapAlignedSmallPath.meta_count` in
    `294x-32-HAKO-ALLOC-USIZE-ALIGNED-SMALL-META-COUNT.md`.
  - Stop line: keep aligned-small pointer, alignment, and padded-size
    observers signed until their own contracts are split.
  - Follow-on migrated group: `HakoAllocHugePageMetaStore` metadata counters
    (`count`, `live_count`) in
    `294x-33-HAKO-ALLOC-USIZE-HUGE-META-STORE-COUNTERS.md`.
  - Stop line: keep huge-page pointer, id, requested-size, committed-size, and
    live-flag payload / observer fields signed until their own contracts are
    split.
  - Follow-on migrated group: `HakoAllocHugePageModel` metadata mirrors
    (`huge_count`, `live_count`) in
    `294x-34-HAKO-ALLOC-USIZE-HUGE-MODEL-META-MIRRORS.md`.
  - Stop line: keep huge-model event/reject counters, pointer/id/size/status
    observers, and facade report fields signed until their own rows.
  - Follow-on migrated group: `HakoAllocHugePageModel` event/reject counters
    (`allocate_count`, `release_count`, `release_reject_count`,
    `zero_reject_count`, `commit_reject_count`, `register_fail_count`,
    `reject_count`) in
    `294x-35-HAKO-ALLOC-USIZE-HUGE-MODEL-EVENT-COUNTERS.md`.
  - Follow-on selected group: `HakoAllocHugePageModel.next_page_id` in
    `294x-68-HAKO-ALLOC-USIZE-HUGE-MODEL-NEXT-PAGE-ID-SELECTION.md`.
  - Follow-on migrated group: `HakoAllocHugePageModel.next_page_id` in
    `294x-69-HAKO-ALLOC-USIZE-HUGE-MODEL-NEXT-PAGE-ID.md`.
  - Stop line: keep huge-model pointer/id/size/status observers and facade
    report fields signed until their own rows.
  - Follow-on migrated group: `HakoAllocHugeReleaseSeam` event/reject counters
    (`release_count`, `unregister_count`, `lookup_miss_count`,
    `not_huge_count`, `model_reject_count`, `reject_count`) in
    `294x-36-HAKO-ALLOC-USIZE-HUGE-RELEASE-SEAM-COUNTERS.md`.
  - Stop line: keep huge release seam sentinel/status observer fields signed
    until their own rows.
  - Follow-on migrated group: `HakoAllocFastPathHeap` event/reject counters
    (`alloc_count`, `release_count`, `fallback_count`, `page_create_count`,
    `reject_count`) in
    `294x-37-HAKO-ALLOC-USIZE-FAST-PATH-HEAP-COUNTERS.md`.
  - Follow-on migrated group: `HakoAllocFastPathHeap` size/capacity metadata
    (`block_size`, `page_capacity`) in
    `294x-50-HAKO-ALLOC-USIZE-FAST-PATH-HEAP-SIZE-CAPACITY.md`.
  - Stop line: keep fast-path route/index metadata and handle id/size fields
    signed until their own rows.
  - Follow-on migrated group: `HakoAllocFastPathHandle.requested_size` in
    `294x-51-HAKO-ALLOC-USIZE-FAST-PATH-HANDLE-REQUESTED-SIZE.md`.
  - Stop line: keep fast-path handle page/block id fields signed until
    id/index contracts are split.
  - Follow-on migrated group: `HakoAllocOsVmBackedFastPathHeap` event/source
    counters (`alloc_count`, `release_count`, `fallback_count`,
    `page_create_count`, `reject_count`, `reserve_count`, `commit_count`,
    `decommit_count`, `source_reject_count`) in
    `294x-38-HAKO-ALLOC-USIZE-OSVM-BACKED-FAST-PATH-COUNTERS.md`.
  - Stop line: keep OSVM-backed route/index/size/capacity metadata,
    `backing_count`, backing payloads, and handle payloads signed until their
    own rows.
  - Follow-on migrated group: `HakoAllocOsVmBackedHandle.requested_size` in
    `294x-52-HAKO-ALLOC-USIZE-OSVM-BACKED-HANDLE-REQUESTED-SIZE.md`.
  - Stop line: keep OSVM-backed page/block id fields, backing payloads,
    size/capacity metadata, and OSVM byte-length seams signed until their own
    rows.
  - Follow-on migrated group: `HakoAllocOsVmBackedFastPathHeap` size/capacity
    metadata (`block_size`, `page_capacity`) plus
    `HakoAllocOsVmPageBacking.bytes` and the page-source policy byte-length
    params in
    `294x-54-HAKO-ALLOC-USIZE-OSVM-BACKED-BYTE-LENGTH-SEAM.md`.
  - Follow-on migrated group: `HakoAllocOsVmBackedFastPathHeap.backing_count`
    in
    `294x-63-HAKO-ALLOC-USIZE-OSVM-BACKING-COUNT-ID-SEAM.md`.
  - Stop line: keep OSVM-backed `bin`, `next_page_id`, backing `page_id` /
    `base`, and handle page/block ids signed.
  - Follow-on migrated group: `HakoAllocPageMapReleaseSeam.page_count` in
    `294x-65-HAKO-ALLOC-USIZE-PAGE-MAP-RELEASE-PAGE-COUNT.md`.
  - Stop line: keep page-map entry ids, block ids, pointer-like fields, and
    binary flags signed until their own rows.
  - Follow-on selected group: `HakoAllocFastPathHeap.next_page_id` in
    `294x-66-HAKO-ALLOC-USIZE-FAST-PATH-NEXT-PAGE-ID-SELECTION.md`.
  - Follow-on migrated group: `HakoAllocFastPathHeap.next_page_id` in
    `294x-67-HAKO-ALLOC-USIZE-FAST-PATH-NEXT-PAGE-ID.md`.
  - Stop line: keep fast-path `bin`, handle page/block ids, and OSVM-backed
    `next_page_id` signed until their own rows.
  - Follow-on migrated group: `HakoAllocSecureFreeListDiagnostics` diagnostic
    counters (`scan_count`, `ok_count`, `fail_count`,
    `out_of_range_free_block_count`, `duplicate_free_block_count`,
    `live_block_in_free_list_count`, `free_count_mismatch_count`,
    `local_free_count_mismatch_count`) in
    `294x-39-HAKO-ALLOC-USIZE-SECURE-LIST-DIAGNOSTIC-COUNTERS.md`.
  - Stop line: keep secure-list `last_*` observation flags signed until bool /
    flag semantics are split.
  - Follow-on migrated group: `HakoAllocPageMapReleaseObserver` observer
    counters (`observe_count`, `success_count`, `reject_count`) in
    `294x-40-HAKO-ALLOC-USIZE-PAGE-MAP-RELEASE-OBSERVER-COUNTERS.md`.
  - Stop line: keep release observer before-snapshots, sentinels, statuses, and
    signed deltas as `i64`.
  - Follow-on probe row: `HakoAllocUsizeFieldProbe` stack-top fields
    (`free_top`, `local_free_top`) and reject counters
    (`free_top_underflow_reject_count`, `local_free_overflow_reject_count`,
    `local_free_underflow_reject_count`) in
    `294x-41-HAKO-ALLOC-USIZE-STACK-TOP-PROBE.md`.
  - Stop line: production page stack-top, live-count, capacity, byte-length,
    and remote-free mailbox fields remain signed until their owner-local rows.
  - Follow-on probe row: `HakoAllocUsizeFieldProbe` exact `usize` stack-top
    values used as `ArrayBox.get/set` indexes in
    `294x-42-HAKO-ALLOC-USIZE-STACK-ARRAY-INDEX-PROBE.md`.
  - Stop line: production page stack fields still do not migrate in this row.
  - Follow-on migrated group: `HakoAllocPageModel` stack-top and occupancy
    fields (`used`, `free_top`, `local_free_top`, `peak_used`) in
    `294x-43-HAKO-ALLOC-USIZE-PAGE-MODEL-STACK-OCCUPANCY.md`.
  - Stop line: keep page identity, block size, capacity, reserved count,
    lifecycle state flags, byte-length fields, queue indexes, and remote-free
    mailbox fields signed until their own owner-local rows.
  - Follow-on probe row: `HakoAllocUsizeFieldProbe` exact `usize` capacity
    bound checks with signed loop/index values in
    `294x-44-HAKO-ALLOC-USIZE-CAPACITY-BOUND-PROBE.md`.
  - Stop line: production page capacity/reserved fields still do not migrate
    in this row.
  - Follow-on migrated group: `HakoAllocPageModel` capacity fields
    (`capacity`, `reserved`) in
    `294x-45-HAKO-ALLOC-USIZE-PAGE-MODEL-CAPACITY.md`.
  - Stop line: keep page identity, block size, lifecycle state flags,
    byte-length fields, queue indexes, and remote-free mailbox fields signed.
  - Follow-on probe row: `HakoAllocUsizeFieldProbe` request-size /
    block-size compare and accepted-request byte-sum accumulation in
    `294x-46-HAKO-ALLOC-USIZE-REQUEST-BYTE-SUM-PROBE.md`.
  - Stop line: production page `block_size` and `requested_bytes` still do not
    migrate in this row.
  - Follow-on migrated group: `HakoAllocPageModel` size/byte fields
    (`block_size`, `requested_bytes`) in
    `294x-47-HAKO-ALLOC-USIZE-PAGE-MODEL-SIZE-BYTES.md`.
  - Stop line: keep page identity, lifecycle state flags, queue indexes, and
    remote-free mailbox fields signed.
- [ ] Keep allocator-provider activation out of scope.
- [x] Resume M167+ mimalloc algorithm rows only after the resume gate.
- [x] Land M168 OSVM page-source composition without new native leaves.
- [x] Land M169 local-free collection and retire observation.
- [x] Land M170 remote-free integration through existing pointer atomics only.
- [x] Land M171 page-map model owner.
- [x] Land M172 page-map-backed release seam before scheduling realloc/aligned/page-map/huge-page rows.
- [x] Land M173 pre-realloc release invariant freeze before the realloc body.
- [x] Land M174 realloc same-class/no-move path before alloc-copy-release fallback.
- [x] Land M175 realloc alloc-copy-release fallback before the negative matrix.
- [x] Land M176 realloc negative matrix / failure contract before aligned allocation work.
- [x] Land M177 alignment policy object before aligned execution.
- [x] Land M178 aligned allocation small path before huge routing.

## Open Design Questions

- Decision: VM exact `usize` uses a tagged exact numeric payload shared by all
  exact integer widths.
- Should plain typed arithmetic always checked-fail-fast, or should release
  rows later opt into wrapping with explicit intrinsics?
- Does Program(JSON v0) carry param/return metadata directly, or does phase
  294x introduce a side table to avoid broad schema churn?
- Is the first accepted target 64-bit only, with 32-bit targets fail-fast, or
  should both widths be modeled from the start?
- Which hako_alloc fields can migrate before low-level helper APIs grow
  `usize` variants?
