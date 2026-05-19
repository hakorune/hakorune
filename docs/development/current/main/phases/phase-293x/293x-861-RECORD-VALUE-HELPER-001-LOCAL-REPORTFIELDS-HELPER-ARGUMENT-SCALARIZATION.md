# 293x-861 RECORD-VALUE-HELPER-001 Local ReportFields Helper Argument Scalarization

Status: landed
Date: 2026-05-19

## Decision

Open the smallest compiler acceptance row needed before allocator owners pass a
local `ReportFields` record value to a helper.

This is a compiler expressivity row, not an allocator behavior row.

## Why This Exists

C205b record lowering keeps record values as builder-local scalarization
carriers. A constructed record currently becomes a placeholder `ValueId` with a
side table of field operands:

```text
record local value
  -> placeholder ValueId
  -> builder-local field operand table
```

That is safe for direct field reads in the same function:

```hako
local fields = SomeReportFields { ... }
result.reason = fields.reason
```

It is not safe to pass `fields` through a helper by simply disabling the escape
guard, because there is no runtime record object for the callee to receive.

## Scope

Add a narrow acceptance contract for this source shape:

```hako
local fields = SomeReportFields { ... }
return me.makeReport(fields)
```

The initial accepted shape is intentionally limited to:

- a builder-local record argument created in the caller;
- a same-owner helper call;
- a helper parameter declared with that record type;
- helper body uses the record parameter only as a field-read base;
- helper body copies those field reads into an existing returned report box.

## Required Compiler Shape

The implementation must preserve the C205b invariant that no runtime record
object is materialized. The accepted helper form must be represented as an
explicit scalarization contract, for example:

```text
caller local ReportFields
  -> helper argument scalarization plan
  -> callee record parameter field reads bind to caller field operands
  -> returned report box remains the existing ordinary box
```

The exact implementation may choose an inline/specialized helper plan or an
equivalent MIR-owned route, but it must make the scalarization owner explicit.

## Stop Lines

- Do not remove or bypass `[record-value/escape]` for general variables.
- Do not treat record values as `NewBox` / typed-object instances.
- Do not store a record value into a box field, ArrayBox, MapBox, or global.
- Do not return a record value directly.
- Do not allow record helper arguments to pass onward to another helper.
- Do not add backend `.inc` owner-name matchers.
- Do not add packed ArrayBox residence or inline-record storage.
- Do not replace allocator report boxes with record returns.
- Do not open cross-function `Result` direct ABI or runtime sum materialization.

## Required Evidence

```text
bash tools/checks/k2_wide_allocator_record_construction_read_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Add or extend a narrow guard so it proves both:

```text
positive:
  same-owner helper accepts a ReportFields record argument and field-reads it

negative:
  return/store/pass-onward record value escapes still fail fast
```

## Progress

Landed changes:

- Added a compiler-internal lowered method AST index for same-module helper
  inspection.
- Added `RecordHelperArgumentScalarizationBox`.
- Added a dedicated call-argument fail-fast:

```text
[record-helper-arg/unsupported]
```

This keeps unsupported helper passing from falling through the older generic
`[record-value/escape]` path or, worse, from becoming a fake runtime record
argument.

- Added the positive same-owner helper path by inlining the helper body in the
  caller while binding the record parameter to the caller's builder-local field
  operands.
- Factored the modeled local-free reuse ledger release-apply report copy through
  `makeReleaseApplyReport(fields)`.

Evidence:

```text
bash tools/checks/k2_wide_allocator_record_construction_read_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
cargo build --release --bin hakorune
git diff --check
```

## Completion Criteria

- The release-apply owner can factor the repeated report-copy block through a
  same-owner helper using the local `ReportFields` record carrier.
- The MIR JSON contains no `NewBox` for the record type.
- Existing direct field-read scalarization stays green.
- General record escape diagnostics remain green and specific.

## Next

Select one more existing `ReportFields` owner and migrate only that owner to the
same helper-argument scalarization pattern. Do not broaden to all ReportFields
owners in one row.
