---
Status: Ready after design stop; implementation row
Date: 2026-08-08
Decision: accepted — explicit V2 typed wire, local CallSlot, and TextEq only
Parent: `loop-recipe-typed-call-value-d0-design-task-2026-08-08.md`
---

# LOOP-RECIPE-V2-TYPED-SCHEMA-CALLSLOT-I0

## Current Capsule

- **Current decision:** widen the portable Recipe vocabulary only through an
  explicit V2 artifact; never reinterpret the numeric V1 wire.
- **Current implementation status:** no V2 code or fixture is landed.
- **Next ordered task:** add schema/artifact types and structural verifier;
  then open the resolver instance-target row.
- **Production stop line:** no resolver lookup, source observer, Builder/MIR,
  physicalizer, selector, fallback, or production caller.
- **Retirement finish line:** typed legacy scan facts are removed only after
  S6C parity, production selection, callers-zero evidence, and a separate
  retirement row.

## Source -> Facts -> Recipe -> fail-fast

This row has no source observer. Synthetic Builder-free values are accepted
only as structural verifier inputs. The row maps a V2 wire value/operation
directly to a typed structural verifier and fails before any source-bound or
physical owner is opened.

## Scope

Add only:

```text
LOOP_RECIPE_SCHEMA_VERSION_V2 = 2
LoopRecipeArtifactV2
LoopRecipeV2
LoopValueClassV2::{I64, Bool, Unit, Text}
LoopOperationV2::{existing numeric operations, CallSlot, TextEq}
LoopRecipeBindingV2 / LoopRecipeValueV2 / LoopRecipeCarrierV2
LoopRecipeVerifierV2
```

`CallSlot` fields are exactly:

```text
receiver: Option<LoopValueKeyV1>
args: Vec<LoopValueKeyV1>
result: Option<LoopValueKeyV1>
```

It contains no names, target handles, ABI, Home, effects, suspension, or
physical identity. `TextEq` is exactly `Text × Text -> Bool`.

The V2 verifier must reject missing/foreign value keys, duplicate value
definitions, invalid numeric domains, non-Text `TextEq`, and unsupported schema
versions. It may verify key ordering and referenced-operation structure only.

## Explicit non-claims

```text
resolver instance target = 0
source-bound call relation = 0
parameter/input-source relation = 0
ScanWithInit observer/producer = 0
Loop return / callable Tail / Completion = 0
Builder/MIR/CFG/PHI/ABI/ownership lowering = 0
physical or production activation = 0
fallback/retry/opaque value route = 0
legacy deletion = 0
```

Do not reuse the If-only direct-call schema, add a method-name lookup, widen
V1 in place, or publish guessed ScanWithInit counts.

## Acceptance

1. A minimal V2 recipe containing `Text`, one `CallSlot`, and `TextEq` round
   trips through JSON with schema version `2`.
2. A V1 artifact remains accepted only by the V1 verifier and is not silently
   decoded as V2.
3. Wrong `TextEq` operand/result classes reject before any Builder effect.
4. Missing/duplicate CallSlot value references reject deterministically.
5. Unknown wire fields reject through `deny_unknown_fields`.
6. All changed Rust files stay below the 760-line design trigger and 800-line
   hard boundary.
7. The focused schema/verifier tests, pointer guard, diff check, and release
   build are green.
8. This task updates `src/mir/loop_recipe_contract/README.md` and
   `docs/reference/mir/loop-recipe-contract.md` in the same implementation
   commit. No later unnamed docs task may be used to close the contract.

## Ordered follow-up

```text
LOOP-RESOLVER-INSTANCE-CALL-TARGET-I0
  -> LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-I0
  -> LOOP-RECIPE-TYPED-INPUT-RELATION-D0
  -> S6C ScanWithInit Facts/producer
```
