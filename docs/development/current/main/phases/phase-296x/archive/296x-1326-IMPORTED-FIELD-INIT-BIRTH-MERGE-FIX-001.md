# 296x-1326 Imported Field Init Birth Merge Fix

Status: closed
Date: 2026-06-20

## Purpose

Close `IMPORTED-STATIC-GLOBAL-CALLEE-ROUTE-PROBE-001` by fixing the app import
bundle path so imported static factory methods preserve normal constructor
lifecycle semantics.

## Finding

The focused field-initializer probe showed:

```text
same-file direct new: green
same-file static factory new: green
same-file birth(value): green
imported static factory: stopped before runtime
```

After registering the probe module root, the imported static factory route
resolved to a direct function call, but runtime behavior still failed.

The real owner was declaration lowering order in App mode:

```text
static box method lowered before later instance box constructors
  -> new ImportedDefaultBox()
  -> ImportedDefaultBox.birth/0 not yet lowered into current module
  -> birth-call injection skipped
```

Standalone library compilation used Script/Test mode and lowered instance box
constructors before the static factory body, so it kept the birth call.

## Change

In App mode, non-Main static box method lowering is deferred until after all
instance box declarations and constructors have been lowered.

This keeps the existing `new` lifecycle owner:

```text
new Box(args)
  -> NewBox
  -> Box.birth/arity when the user constructor exists
```

No field-initializer semantics changed.

## Acceptance

```bash
cargo check -q --lib
cargo build --release --bin hakorune
bash apps/field_initializer_route_probe/smoke.sh
bash apps/lib/collections/smoke_ordered_map.sh
bash apps/constructor-lifecycle-probe/smoke.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Result:

```text
field-initializer-route/smoke summary=ok
ordered-map/smoke summary=ok
constructor-lifecycle/smoke summary=ok
```

## Stop Line

```text
MapBox changed=0
OrderedMapBox API changed=0
ring0/ring1 provider changed=0
field initializer semantics changed=0
birth logic moved into field initializer=0
benchmark/source-name branch added=0
```

## Next

Return to:

```text
CREAT-SUBSET-PILOT-SELECTION-001
```
