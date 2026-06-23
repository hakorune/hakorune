# 296x-1637 MIRBUILDER-AGGREGATE-BORROW-USE-ELIMINATION-001

Status: Closed
Date: 2026-06-23

## Decision

Adopt Option A+ for aggregate returned read borrows:

```text
standalone aggregate borrow
  -> Deny(ReturnedReadBorrow)

known call-local consumer
  -> eliminate the borrow with BorrowUseFacts + BorrowLoweringDecision
```

No live read view, lease counter, or lifetime framework is introduced.

This is a Rust adapter decision, not a universal pointer model. Source
language references are normalized before Hako lowering:

```text
Rust borrow / Go pointer-slice-map / C pointer
  -> StorageAccessFacts
  -> Elide / ReadFold / OwnedSnapshot / SharedHandle / SharedCell / Span /
     UnsafeCapability / Deny
```

Do not add general Hako pointer syntax for this lane:

```text
no general &
no general *
no arrow / ->
no raw pointer syntax in safe Hako
```

Future Go/C/unsafe-Rust support should enter through access/capability facts.
Raw memory remains behind `Deny(UnsafeOrFFI)` with
`detail=RequireUnsafeCapabilityBoundary` until an explicit unsafe capability
boundary is designed.

## Implemented Slice

Selected consumer:

```rust
builder.metadata_ctx.current_region_stack().last().copied()
```

Lowering:

```text
BorrowLoweringDecision = ElideToLeafProjection
operation = SequenceLastOption
generated API = MetadataContextApi.current_parent_region(ctx): Option<i64>
```

Standalone `MetadataContext::current_region_stack()` remains
`Deny(ReturnedReadBorrow)`.

## Acceptance

```text
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family metadata-context-region-parent --check
bash tools/checks/rust_lifecycle_metadata_context_region_parent_derived_artifact_guard.sh
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

Required generated-source properties:

```text
current_parent_region(ctx): Option<i64>
raw ArrayBox return = 0
ReadView/lease claim = 0
runtime fallback = 0
```

## Next

```text
value_origin_callers().get(...).cloned()
  -> BorrowUseFacts
  -> StorageAccessFacts
  -> ElideToLeafProjection
```
