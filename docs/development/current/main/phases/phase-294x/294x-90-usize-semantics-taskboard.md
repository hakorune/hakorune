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
- Production `hako_alloc` fields remain `i64`.
- Mimalloc `.hako` algorithm rows may continue, but they must not claim
  production `usize` field migration yet.
- Native exact numeric typed-object slot representation exists in
  `nyash_kernel`.
- The next `usize` completion work is exact field get/set ABI.

## Next Implementation Queue

| Order | Row | Status | Implementation Boundary |
| --- | --- | --- | --- |
| 1 | `294x-19b` | Pending | Exact numeric field get/set ABI exists, including range/overflow failure contracts. |
| 2 | `294x-19c` | Future | Production `hako_alloc` non-negative field migration reopens one field group at a time. |
| 3 | `M168+` | Future | Mimalloc `.hako` OSVM page source, local-free retire, and remote-free rows consume the completed substrate. |

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
| `294x-19` | Blocked | hako_alloc production facade migration | waits for native exact numeric typed-object slots and exact field get/set ABI |
| `294x-19a` | Complete | native exact numeric typed-object slots | kernel typed-object storage records exact slot kinds and legacy i64 helpers do not mutate exact numeric slots |
| `294x-19b` | Pending | exact numeric field get/set ABI | non-VM backends can read/write exact numeric slots with range/overflow contracts |
| `294x-20` | Complete | mimalloc row resume gate | M167+ mimalloc implementation resumes with clear `usize` support boundaries and production fields still on `i64` |

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
- [ ] Keep Rust and `.hako` parser fronts aligned.

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
- [ ] Add field get/set ABI for exact numeric slots.
- [ ] Lower LLVM/native unsigned compare and shift.
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
- [x] Mark production `usize` field migration blocked on non-VM exact numeric
  storage and field get/set ABI.
- [ ] Update proof apps per field group.
- [ ] Keep allocator-provider activation out of scope.
- [x] Resume M167+ mimalloc algorithm rows only after the resume gate.

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
