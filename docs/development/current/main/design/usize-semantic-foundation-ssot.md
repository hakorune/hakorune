---
Status: SSOT
Decision: accepted
Date: 2026-05-12
Scope: exact `usize` / pointer-sized unsigned integer semantics before the
  mimalloc `.hako` migration uses `usize` in live allocator state.
Related:
  - docs/reference/language/types.md
  - docs/reference/runtime/substrate-capabilities.md
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - docs/development/current/main/phases/phase-294x/README.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# Usize Semantic Foundation SSOT

## Decision

Hakorune should grow real `usize` semantics before the mimalloc `.hako` port
migrates live allocator state from `i64` to `usize`.

The previous 293x decision remains correct as a stop line: `usize` is currently
accepted as syntax / annotation metadata, while runtime values still execute on
the dynamic `Integer(i64)` lane. Phase 294x exists to remove that semantic gap
deliberately.

VM role decision:

```text
VM is not a product owner.
VM is a semantic reference executor.
```

Japanese mirror:

```text
VMは本線実装者ではない。
VMは意味論の参照実行器。
```

## Current Truth

Live today:

- numeric substrate names are classified by `src/mir/numeric_substrate.rs`;
- `usize` is recognized as unsigned pointer-width metadata;
- target pointer width and target-resolved numeric kind metadata are owned by
  `NumericTarget` / `NumericKind` in `src/mir/numeric_substrate.rs`;
- field annotations can preserve declared type names;
- method, constructor, interface, static method, and top-level function
  parameter annotations are preserved in AST parameter metadata while the
  existing `params` surface remains names-only;
- accepted return type annotations are preserved in AST metadata;
- AST JSON and Stage1 Program(JSON) carry declared parameter metadata and
  accepted return type annotations, with legacy names-only JSON still readable;
- MIR owns side-car exact numeric type metadata that preserves source spelling
  plus target-resolved signedness/width distinctly from `MirType::Integer`;
- exact numeric constant metadata and dynamic `Integer(i64)` conversion helpers
  range-check values against signedness and resolved width;
- the MIR verifier rejects statically known exact numeric field writes when the
  base object and assigned integer value resolve through same-function
  `NewBox` / Box-typed parameter / `Copy` and `Const(Integer)` / `Copy` chains;
- the MIR verifier rejects unchecked dynamic writes to exact numeric fields
  whose value range does not cover every possible dynamic `Integer(i64)` value;
- function metadata owns `ExactNumericRuntimeCheckContract::DynamicIntegerRange`
  as the verifier/lowering contract for those dynamic exact numeric writes;
- MIR semantic refresh attaches `DynamicIntegerRange` contracts for real
  exact numeric `FieldSet` producers after optimization and before verification,
  with verifier checks consuming the same field-write facts owner;
- the VM interpreter executes matching `DynamicIntegerRange` contracts at
  `FieldSet` sites and rejects non-integer, negative-unsigned, and out-of-range
  dynamic values before field mutation;
- unsupported non-VM backend routes fail fast with
  `[freeze:contract][exact-numeric/runtime-check-unsupported-backend]` when a
  module still carries exact numeric runtime-check contracts;
- `src/mir/numeric_substrate.rs` owns checked exact numeric add/sub/mul policy
  for same-type operands, rejecting type mismatch and out-of-range results;
- `src/mir/numeric_substrate.rs` owns exact numeric compare and unsigned
  logical right-shift policy, rejecting type mismatch, signed logical shift,
  and shift counts at or above the exact type width;
- `src/mir/exact_numeric_unification.rs` owns PHI/Select exact numeric merge
  policy, preserving exact facts only for identical incoming exact types;
- `src/mir/exact_numeric_value_facts.rs` owns first exact numeric per-value
  facts for exact numeric declared params, field reads, `Copy`, and
  conservative `Phi`/`Select` merges, including metadata rejections for
  exact/dynamic or exact/exact mismatches;
- decimal integer literal suffixes such as `0usize` are accepted on the Rust
  parser front, range-check through the exact numeric substrate, and publish
  MIR exact const facts while still emitting current-lane `Integer(i64)`
  constants;
- typed-object layout plans preserve exact numeric storage names such as
  `usize` distinctly from legacy `i64`, while current execution still uses the
  dynamic integer lane;
- non-VM backend routes use a MIR-owned exact numeric backend capability gate
  and fail fast before exact numeric typed-object storage or exact numeric op
  route facts silently lower through legacy `Integer(i64)`;
- RawBuf and OSVM expose first byte-length `usize` facades whose live v0
  meaning is the non-negative current-lane i64 subset;
- MIR raw-layout plans accept `usize` / `isize` fields by resolving them
  through `NumericTarget` pointer-width layout rules to target-sized scalar
  storage. This remains metadata-only and adds no source syntax or
  backend-native field execution;
- `lang/src/hako_alloc/memory/NUMERIC_FIELDS.md` classifies all current
  `hako_alloc` numeric stored fields before any live field migration;
- `HakoAllocPageQueue.direct_page_index` no longer stores `-1`; direct-page
  presence is split into an explicit `has_direct_page` field while not-found
  return sentinels remain signed for later API-shape rows;
- `HakoAllocUsizeFieldProbe` owns an isolated proof-only migration probe for
  capacity/count/byte-length `usize` stored fields without changing production
  allocator state;
- production `hako_alloc` `usize` field migration is blocked until native exact
  numeric typed-object slots and exact field get/set ABI exist; mimalloc
  algorithm rows may continue with production fields on `i64`;
- `FunctionMetadata` preserves MIR-side declared parameter/return annotation
  text, and exact numeric return annotations publish function-level advisory
  return facts without changing runtime lowering;
- exact numeric `BinOp::Add` sites publish MIR-owned route facts and exact
  numeric result value facts only when both operands already share one exact
  numeric type;
- VM exact numeric work in this phase is reference execution only. VM rows may
  consume MIR-owned facts/contracts but do not make VM-only behavior complete
  product support;
- typed-object planning preserves numeric annotations as exact numeric storage
  names in layout metadata;
- VM runtime values use `Integer(i64)`;
- current `>>` is signed i64 arithmetic right shift.

Not live today:

- param/local verifier checks, runtime-check insertion/lowering beyond exact
  numeric field-write contracts, non-VM backend lowering/execution of those
  contracts, and exact runtime unsigned range-check construction;
- `.hako` parser-front parity for numeric literal suffixes;
- backend exact numeric arithmetic/compare/shift lowering, native exact numeric
  typed-object slots, and explicit wrapping vocabulary;
- RawArray index/length/capacity `usize` variants and bounds verifier `usize`
  variants;
- MIR JSON exact-width numeric const tags;
- native typed-object exact numeric slots distinct from the current integer
  lane;
- backend lowering to native pointer-sized integer classes.

## Target Meaning

`usize` means an unsigned integer with the width of the current compilation
target pointer size.

Minimum target contract:

- allowed value range is `0..=usize::MAX` for the target;
- `-1` and any negative value are invalid for `usize`;
- comparisons are unsigned;
- right shift is logical;
- overflow behavior is explicit and never inferred from the old i64 lane;
- backends that cannot lower `usize` must fail fast with a stable diagnostic;
- source spelling must remain visible in metadata until exact lowering consumes
  it.

Phase 294x may initially support only 64-bit targets if that is the active
compiler/backend reality, but the target-width owner must make that explicit.
It must not silently call a 64-bit implementation "pointer-sized" on a 32-bit
target.

## VM Reference Executor Gate

A VM row may land only if:

- a MIR-owned fact, policy, or contract already exists;
- the VM only executes that MIR-owned semantic contract;
- unsupported non-VM backend routes fail fast or have a visible lowering row;
- hako_alloc live field migration is not included in the same row;
- the next backend/lowering row is visible in the taskboard.

Do not read VM green as hako_alloc/mimalloc green. hako_alloc live field
migration waits for backend fail-fast/lowering and typed-object exact numeric
storage.

## Overflow Policy

The safe default is checked/fail-fast arithmetic for typed `usize` operations.
The MIR numeric substrate already owns the first exact add/sub/mul policy:
operands must share the same exact numeric type, and the result must fit the
target-resolved range for that type.

Wrapping behavior is allowed only through explicit vocabulary added by a later
row, for example `wrapping_add_usize` / `checked_add_usize` helpers or
intrinsics. Plain `+`, `-`, `*`, shift, and conversion rows must not silently
wrap unless this SSOT is updated by an accepted decision card.

This keeps mimalloc policy code honest: size arithmetic failures surface near
the typed integer operation instead of turning into allocator corruption.

## Signed Sentinel Policy

Sentinel-bearing indexes stay signed.

Examples:

- `direct_page_index: i64 = -1`;
- not-found indexes;
- negative error codes;
- deltas that may be negative.

Non-negative quantities are migration candidates:

- sizes;
- capacities;
- counts;
- byte lengths;
- page ids when no negative sentinel is used;
- bin indexes when the not-found state is represented separately.

If a value currently uses `-1` as a sentinel, migrate the state shape first
using an explicit `has_*` flag, enum/Option-like state, or a signed companion.
Do not store sentinel values in `usize`.

## Required Feature Inventory

### 1. Spec And Docs

- Define fixed-width and pointer-sized integer semantics in
  `docs/reference/language/types.md`.
- Define backend obligations and fail-fast behavior in
  `docs/reference/runtime/substrate-capabilities.md`.
- Keep `usize` separate from the legacy `Integer(i64)` lane in wording.
- Document where `i64` remains the right spelling.
- Document sentinel-bearing fields as signed.
- Add migration criteria for hako_alloc/mimalloc fields.

### 2. Parser And Metadata Preservation

- Preserve method parameter declared type names.
- Preserve `birth` parameter declared type names.
- Preserve return type annotations if the parser accepts `method(...): Type`.
- Preserve static-box method annotations with the same representation as box
  methods.
- Keep declared numeric metadata round-tripping through AST JSON /
  Program(JSON).
- Keep Rust and `.hako` parser fronts aligned.
- Keep annotation parsing structural: do not add by-name special cases for
  mimalloc or allocator code.

### 3. MIR Type Model

- Keep an exact numeric MIR type representation instead of collapsing all
  numeric substrate names to `MirType::Integer`.
- Preserve signedness and width.
- Represent pointer-width integers through target-width metadata.
- Add typed constants or constant metadata for exact numeric values.
- Keep conversions between dynamic `Integer(i64)` and exact numeric metadata
  range-checked.
- Define PHI / Select unification rules for exact numeric types.
- Define route facts for numeric param and return types.
- Keep facts and lowerers in one SSOT path so type acceptance and lowering do
  not drift.

### 4. Runtime / VM Semantics

- Add a runtime representation for exact `usize` or an equivalent typed
  numeric value.
- Implement construction/conversion from literals and dynamic integers with
  range checks.
- Implement equality and unsigned comparison.
- Implement `+`, `-`, `*`, `/`, `%` with checked/fail-fast behavior.
- Implement bitwise `&`, `|`, `^`.
- Implement logical right shift for unsigned types.
- Validate shift counts.
- Define truthiness if exact numeric values can flow into conditions.
- Define string/debug formatting without losing the numeric kind.
- Add stable runtime diagnostics for range, overflow, and invalid shift.

### 5. Literal And Const Evaluation

- Accept numeric literal suffixes only when their exact type is implemented.
- Reject out-of-range suffixed literals at parse/const-eval time when static.
- Reject negative unsigned literals unless an explicit conversion row accepts a
  checked form.
- Extend static const table element types beyond `u16` only after exact numeric
  consts exist.
- Define whether unsuffixed integer literals in a `usize` context are checked
  contextual conversions or remain dynamic `Integer(i64)` until assignment.

### 6. Verifier

- Consume runtime-check contracts in lowering/runtime rather than treating them
  as silent permission to use the old `Integer(i64)` lane.
- Extend numeric verifier coverage to annotated params and locals.
- Keep rejecting negative assignment to `usize`.
- Keep rejecting `-1` sentinel assignment to `usize`.
- Detect plain arithmetic overflow when statically knowable.
- Insert runtime checks only through an explicit lowering contract.
- Reject unsupported backend routes with stable diagnostics.
- Add strict/dev gates before broad production acceptance.

### 7. Typed Object And Storage

- Add typed-object storage classes for exact numeric fields, at minimum
  `usize` and likely `u64`.
- Keep `i64` storage for signed values and sentinel-bearing fields.
- Define field get/set ABI for exact numeric slots.
- Preserve declared numeric kind in typed-object plans.
- Ensure EXE runtime storage is not a silent `Vec<i64>` alias for `usize`.
- Update typed-object plan tests for field get/set, birth params, method params,
  PHI, and global-call propagation.

### 8. Backend And ABI

- Lower `usize` to the backend target's pointer-sized integer class.
- For LLVM/native backends, distinguish signed/unsigned comparisons and shifts.
- For WASM or other backends, document i32/i64 target behavior and fail fast if
  unsupported.
- Publish backend capability checks so unsupported `usize` code fails before
  silent wrong-code.
- Keep C ABI / externcall mappings explicit for size_t-like rows.
- Keep backend `.inc` files free of provider, mimalloc, hook, or allocator-name
  matchers.

### 9. Raw Layout And Low-Level Capabilities

- Accept pointer-sized raw-layout fields only after target-width semantics are
  live.
- Define alignment and size-of behavior for `usize` fields.
- Add `usize` variants for low-level helpers only when their semantics are
  exact:
  - raw buffer length/capacity;
  - raw array length/capacity/index;
  - OSVM page size and byte length helpers;
  - bounds checks;
  - pointer slot capacity;
  - atomic fixed-slot or pointer-sized counters where needed.
- Keep existing `*_i64` helper names valid until explicit migration rows move
  call sites.

### 10. Hako Alloc / Mimalloc Migration

- Inventory every hako_alloc numeric field before migration.
- Classify each field as signed sentinel, signed delta, non-negative count,
  size, capacity, index, or byte length.
- Migrate only non-negative fields first.
- Split sentinel state before migrating sentinel-bearing fields.
- Update proof apps per migration slice.
- Keep allocator-provider activation and host allocator replacement out of
  scope.
- Keep `hako_alloc` state boxes on Unified Members and stored field
  initializers.

### 11. Test And Guard Surface

- Parser tests for param/return metadata preservation.
- AST JSON / Program(JSON) round-trip tests for numeric declared types.
- MIR unit tests for numeric type classification and exact type retention.
- VM tests for range, overflow, unsigned compare, logical shift, and formatting.
- Verifier tests for negative assignment and sentinel rejection.
- Typed-object EXE tests for exact numeric field get/set.
- Backend fail-fast tests for unsupported targets.
- hako_alloc proof apps for each migrated field group.
- Guard against silent `usize` aliasing to `Integer(i64)` after the exact row
  lands.

## Non-Goals

- No full static type checker.
- No blanket migration of all integer code.
- No process allocator replacement.
- No provider activation, hook install, or global allocator.
- No app-side workaround to make mimalloc pass.
- No `usize` spelling in hako_alloc live state before the relevant migration
  row.

## Exit Criteria

Phase 294x is complete when:

- `usize` has exact semantics in the accepted compiler/runtime/backend set;
- unsupported backends fail fast;
- hako_alloc can migrate non-negative size/capacity fields without lying about
  runtime behavior;
- sentinel-bearing fields remain signed or are structurally split;
- docs and proof apps make the migration path clear enough that mimalloc rows
  can resume without reopening the semantic question.
