# DYNAMIC-CARRIER-LIFECYCLE-VOCABULARY-R0

Status: selected after the Dynamic operator Decision closeout
Date: 2026-08-10
Depends on: `DYNAMIC-INVOCATION-RESULT-LIFECYCLE-I0` closed

## Change

Move the already-live lifecycle obligation out of the invocation-specific
module into one neutral owner:

```text
src/mir/dynamic_carrier_contract/
  DynamicCarrierLifecycleObligationV1
    EndExactlyOnceUnlessForwarded
```

The Dynamic invocation envelope and its exact I6/V10/I7/V11 lifecycle wrapper
consume this shared vocabulary. Delete the old invocation-specific enum and
all duplicate spellings in the same commit.

## Acceptance

- behavior, source mapping, Recipe, JoinSig, Fault catalog, and row count are
  unchanged;
- exactly one production enum defines the shared obligation;
- no `Home`, `Unique`, `Shared`, `Weak`, runtime tag, provider, or physical end
  mechanism enters the new module;
- focused invocation lifecycle tests, structural guard, `cargo check --lib`,
  pointer guard, line-count check, and `git diff --check` are green;
- module README and language references are updated in the same commit;
- split source near 650-700 lines, stop additions at 760, hard maximum 800.

## Stop

Any need to classify payload Home, infer from a runtime implementation, change
Fault behavior, or add the V9/V17 operator rows returns to design. This row is
a behavior-neutral vocabulary move only.
