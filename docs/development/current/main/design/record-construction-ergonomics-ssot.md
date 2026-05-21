# Record Construction Ergonomics SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: ARG-DATA-003

## Purpose

Wide allocator report rows should not require every field to be repeated at
every construction site. The immediate pain is data construction noise, not a
need for broader runtime record objects or named arguments.

This row accepts a narrow record construction ergonomics slice:

```hako
record ReportFields {
    accepted: i64 = 0
    reason: i64 = 0
    run_count: i64 = 0
}

local base = ReportFields {}
local report = ReportFields { accepted: 1, reason }
local next = report with { run_count: 1 }
```

## Accepted Surface

### Record Field Defaults

Record declarations may attach default expressions to fields:

```hako
record ReportFields {
    accepted: i64 = 0
    reason: i64 = 0
}
```

MVP defaults are scalar literal defaults only. They are evaluated when a record
literal omits that field. They are not methods, stored runtime defaults, or
ordinary box field initializers.

### Empty Record Literal

`RecordName {}` constructs a tracked local record value using declared defaults
for all fields. It fail-fasts when any omitted field has no default.

### Same-Name Field Shorthand

Inside record literals and record updates:

```hako
ReportFields { reason }
```

means:

```hako
ReportFields { reason: reason }
```

The field name remains explicit. There is no wildcard or implicit all-field
copy.

### Record `with` Update

`record_value with { field: expr }` creates a new tracked local record value by
copying the base record fields and replacing the listed fields. It does not
mutate the base value.

`with` is record-only. Ordinary boxes are identity/resource boundaries, so
`box_value with { field: expr }` is rejected rather than shallow-copying,
deep-copying, re-running construction, or silently calling `new`.

## Stop Lines

- No `...fields` spread.
- No named function arguments.
- No automatic record-to-box copy.
- No ordinary-box `with` copy/update.
- No `::default()` constructor surface.
- No runtime record materialization.
- No record return ABI.
- No ordinary call argument escape for record-local values.
- No backend record lowering route.
- No packed `ArrayBox` storage behavior.

## Validation

ARG-DATA-003 must add parser and MIR-builder coverage proving:

- record field defaults parse
- `RecordName {}` fills defaults
- same-name shorthand parses and lowers as `field: field`
- `with` update lowers by copying tracked record-local fields and replacing
  listed fields
- missing non-defaulted fields still fail-fast
