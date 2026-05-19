# Record-Local Scalarization SSOT

Decision: accepted
Date: 2026-05-20

## Purpose

Record-local scalarization is the compiler-owned contract for source records
that are used as identity-free, builder-local value carriers.

This is not a runtime record representation and not a second MIR dialect.

```text
record literal / record constructor
  -> builder-local placeholder ValueId
  -> builder-local field ValueId table
  -> direct field reads or same-owner helper scalarization
```

## Owners

```text
RecordValueScalarizationBox
  code: src/mir/builder/record_values.rs
  owns: local record construction, direct field-read scalarization, escape
        guards, and field assignment rejection.

RecordHelperArgumentScalarizationBox
  code: src/mir/builder/record_helper_args.rs
  owns: narrow same-owner helper calls where a local record argument is
        scalarized by binding helper parameter field reads to caller field
        operands.

RecordLocalFacts
  current storage: src/mir/builder/compilation_context.rs
  current PHI hook: src/mir/utils/phi_helpers.rs
  owns: register/copy/PHI propagation rules for builder-local record facts.
```

Storage may remain in `CompilationContext` for now. The rules above are the
conceptual owner and must not spread into backend or runtime code.

## Allowed Shapes

Direct local field read:

```hako
local fields = SomeReportFields { ... }
local reason = fields.reason
```

Same-owner helper argument scalarization:

```hako
local fields = SomeReportFields { ... }
return me.makeReport(fields)
```

The helper parameter must declare the exact record type:

```hako
makeReport(fields: SomeReportFields): SomeReportBox {
    local result = new SomeReportBox()
    result.reason = fields.reason
    return result
}
```

## Helper Body Shape

The accepted helper body is narrow.

Allowed:

```text
- local scalar temporaries
- field reads from declared record helper arguments
- construction / mutation of the returned ordinary report box
- return of the ordinary report box or scalar value
```

Forbidden:

```text
- passing a record helper argument onward to another helper
- returning a record value directly
- storing a record value into a box field, ArrayBox, MapBox, or global
- assignment to record fields
- loops as part of the record helper contract
- await / nowait / co / Channel / sync box / context boundaries
- extern calls or substrate calls
- backend / provider / OSVM / atomic / worker behavior
```

If a future row needs a broader helper body shape, it must add a named
`RecordHelperBodyShape` profile first, with explicit guards.

## PHI Propagation

Record-local facts may survive a PHI only when every incoming record-local
payload is exactly the same:

```text
same record name
same field count
same field names
same declared field type names
same field ValueIds
```

Any mismatch means the PHI destination is not record-local. There is no runtime
fallback and no partial record fact.

## Receiver Rule

Record helper scalarization is same-owner only.

Accepted receiver:

```text
normalized SelfReceiver
```

Current implementation may reach that through `me.helper(...)` or a receiver
value proven equal to current `me`, but the long-term semantic rule is:

```text
record helper scalarization accepts only normalized SelfReceiver
```

Arbitrary receiver expressions, cross-owner helpers, field receivers, and call
result receivers are out of scope.

## Stop Lines

```text
1. No runtime record object.
2. No NewBox for record values.
3. No typed_object_plan for record helper carriers.
4. No backend lowering route for record helper carriers.
5. No MIR instruction that represents a record aggregate value.
6. No record-local placeholder ValueId escape:
   - no return
   - no ordinary call arg
   - no pass-onward helper arg
   - no field set
   - no array/map storage
   - no object field storage
7. Same-owner helper scalarization only.
8. Narrow helper body shape only unless a named profile is added.
9. PHI propagation only for exact same payload.
10. No cross-function record-local ABI.
11. No runtime materialization fallback.
12. No backend owner-name matcher.
```

## Guard Expectations

Rows that add or migrate record-local helper usage should prove at least:

```text
- direct record construction/read guard stays green
- target owner guard stays green
- MIR JSON contains no NewBox for the record carrier
- MIR JSON contains no typed_object_plan for the record carrier
- unsupported record value escapes remain fail-fast
```

Closeout rows should also cover the first PHI-crossing helper case and any
newly accepted helper body profile.
