# Pre-loop located argument descent D0

```text
Decision: pending
Status: design stop
Parent: RAW-INSTANCE-METHOD-SOURCE-SITE-CARRY0-I0
Reason: source-only association is sealed, but the real MeStandardUnified
        route erases the selected Argument(1) into legacy AST before descent.
```

## Established evidence

`PreparedPreloopNestedResultAssociationV1` now co-seals the selected
pre-loop contract and `RawLocatedMethodCallInputV1` by the same declaration
catalog allocation, declaration row, caller, structural site, and borrowed
`MethodCall` node. The parked loop-refresh site rejects before lowering.

The real route is:

```text
AssociatedMethodCallArgumentsV1::lower_all()
  -> drive_call_arguments_v1()
  -> argument_expression_input()
  -> drive_legacy_expression_v1()
```

At this boundary, `RawLegacyMethodCallInputV1` owns cloned syntax and the
exact located child site is no longer available. Passing the association
through that erasure would make the source-to-physical claim unprovable.

## Decision needed

Choose one bounded way to preserve only the selected pre-loop Argument(1)
through the existing method-call descent, without changing the physical Call
writer or type publication.

### A — shared source-neutral argument-completion capability

Extract a small trait from `AssociatedMethodCallArgumentsV1` that supplies
ordered argument descent. The normal owner and a candidate-only owner both
implement it; the candidate owns the opaque association and returns its
located argument only at index 1.

```text
Pros: the existing MeStandardUnified policy depends only on descent capability.
Risk: member-route and handler signatures widen; scope must remain bounded.
```

### B — candidate-only indexed override on the existing stack owner

Keep `AssociatedMethodCallArgumentsV1` concrete and add an optional indexed
opaque descent capability used only by the candidate ingress.

```text
Pros: smaller immediate signature surface.
Risk: an optional policy slot can become a second source association channel.
```

### Forbidden

```text
RawLocated -> RawLegacy conversion
AST re-walk / path reconstruction / name or ordinal matching
Builder-wide site map or source-site -> ValueId map
new Call writer, unified-emitter source lookup, type_ctx write
LocatedLegacy activation, loop-refresh connection, fallback or retry
```

## Required acceptance after the decision

```text
selected Body(3).Value.Argument(1) remains one located input until the
existing actual generic Call seam
default raw route stays byte-for-byte on its legacy facade
loop-refresh stays parked
Call receipt and Integer publication remain zero in this row
all candidate failures retain/discard only; live Builder and publication zero
```
