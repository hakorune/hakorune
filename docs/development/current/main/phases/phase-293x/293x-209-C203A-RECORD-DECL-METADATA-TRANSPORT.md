# 293x-209: C203a Record Declaration Metadata Transport

Status: Complete

## Purpose

Carry `record` declarations through the compiler metadata surfaces without
turning records into ordinary user boxes.

C202 made `record Name { field: Type }` a source-level aggregate declaration.
C203a gives that declaration its own metadata-only lane so later rows can build
record layout plans and local scalar replacement from explicit record facts.

## Transport Lane

`record_decls` is present in:

- Program JSON v0 authority output
- JSON v0 bridge input and MIR metadata lowering
- MIR `ModuleMetadata`
- MIR JSON output

The lane preserves:

- record name
- generic type parameters
- field order
- typed field declarations
- field indexes

## Stop Line

C203a does not add:

- record layout plans
- local scalar replacement
- packed `ArrayBox` storage
- record construction or field access lowering
- ordinary `box` identity erasure
- typed-object-plan reuse for records
- allocator-specific record migration

## Acceptance

- Program JSON v0 emits `record_decls` separately from `user_box_decls`.
- JSON v0 bridge preserves `record_decls` as MIR metadata only.
- MIR JSON emits sorted `record_decls`.
- Existing record declarations remain excluded from runtime user-box factory and
  ordinary user-box declaration indexing.
- No `record_decls` matcher leaks into `.inc` shims.
