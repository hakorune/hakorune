# 293x-224: C207 Packed ArrayBox Auto-Use Eligibility

Status: Planned

## Purpose

C207 is the first production compiler row after the C206 cleanup/probe lane.
It does not enable packed `ArrayBox` storage. It only teaches the compiler to
classify when a record-array site is eligible for a future packed inline-record
auto-use path.

The output of this row is metadata:

```text
eligible(reason)
rejected(reason)
fail_fast_required(reason)
```

## Eligibility Decision Table

| Shape | C207 decision | Reason |
| --- | --- | --- |
| concrete `record` layout, all fields use integer lanes, matching `ArrayRecordStoragePlan`, append values have one layout, indexed direct field reads only, no element escape | `eligible` | future packed storage can stay columnar and non-materializing |
| handle column, weak/reflection field, unsupported storage class | `rejected(unsupported-column-kind)` | C207 only covers integer-lane columns |
| mixed record layouts appended to one array | `rejected(layout-mismatch)` | packed columns require one concrete layout |
| record element returned, assigned to visible object storage, passed across a host/backend boundary, or read through public `ArrayBox.get(i)` as a value | `rejected(materialization-required)` | C208 must define the materialization / escape boundary first |
| element field write-through, mutation through an extracted element, dynamic reflection, or unknown array operation | `rejected(unsupported-operation-shape)` | no silent boxed fallback |
| target backend lacks the future packed inline-record lowering capability | `fail_fast_required(backend-unsupported)` | VM/reference success must not imply backend completion |

## Scope

C207 may:

- add a compiler-owned eligibility planner for `array_record_storage_plans`.
- expose candidate/rejection/fail-fast metadata in MIR metadata and MIR JSON.
- add positive/negative fixtures for eligibility decisions.
- add a local-run guard that fixes the decision table.

C207 must not:

- construct `ArrayStorage::InlineRecord` in production code.
- migrate `hako_alloc` metadata stores.
- expose public `ArrayBox` APIs for packed records.
- materialize record elements.
- add backend lowering or boxed fallback behavior.
- touch provider activation, hooks, process allocator replacement, or `.inc`
  allocator/provider/hook matchers.

## First Files

Start here:

```text
docs/development/current/main/design/record-and-packed-array-lowering-ssot.md
docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
src/mir/array_record_storage_plan.rs
src/mir/function/types.rs
src/runner/mir_json_emit/metadata.rs
src/runner/mir_json_emit/tests/*
tools/checks/k2_wide_arraybox_inline_record_autouse_eligibility_guard.sh
docs/tools/check-scripts-index.md
```

Do not touch `lang/src/hako_alloc/**` in C207 unless the row is split and a
new card explicitly changes the scope.

## Acceptance

- Positive fixture: `record Meta { ptr: i64, size: i64 }`, array append of the
  same record layout, and indexed direct field reads produce one eligible
  metadata row.
- Negative fixture: a record with a handle column is rejected with
  `unsupported-column-kind`.
- Negative fixture: mixed record layouts are rejected with `layout-mismatch`.
- Negative fixture: public record element escape is rejected with
  `materialization-required`.
- The guard confirms no production `ArrayStorage::InlineRecord` auto-use, no
  `hako_alloc` migration, no backend lowering, and no `.inc` matcher growth.
