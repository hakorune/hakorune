# 293x-210: C203b Record Layout Plans

Status: Complete

## Purpose

Derive backend-readable record layout metadata from `record_decls` without
making records ordinary user boxes.

C203a transported declarations. C203b is the first consumer of that transport:
it creates `record_layout_plans` for concrete records whose fields all have a
known storage class.

## Layout Lane

`record_layout_plans` carries:

- record name
- stable layout id
- layout kind `record_value_aggregate_v0`
- field count
- field slots
- declared type names
- storage classes

Generic or otherwise unsupported field types are skipped for now instead of
guessing a layout.

## Stop Line

C203b does not add:

- record construction lowering
- record field access lowering
- record local scalar replacement
- packed `ArrayBox` storage
- typed-object-plan reuse for records
- ordinary `box` identity erasure
- allocator metadata migration

## Acceptance

- MIR builds `record_layout_plans` from concrete `record_decls`.
- JSON v0 bridge derives the same metadata-only record layout lane.
- MIR JSON emits `record_layout_plans`.
- `typed_object_plans` remains the ordinary user-box layout lane.
- No `record_layout_plans` matcher leaks into `.inc` shims.
