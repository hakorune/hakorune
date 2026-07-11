# 3503 - LANGV1-WEAK-FIELD-PRODUCT-BACKEND-SELECTION-DESIGN-STOP-001

## Status

Active design consultation stop. Do not change backend lowering, runtime
support, selfhost claims, or capability flags before this decision is accepted.

## Review Corrective Queue

Fable5 review found two pre-existing issues while this stop is active:

```text
R1 stop-the-line gate repair:
  naming_charter_guard pinned the deleted literal comment "syntax-3 path"
  after canonical break/continue replaced that compatibility branch.
  Remove the obsolete literal requirement; do not restore a false comment.
  Run naming guard from the Language v1 FULL sensitive-path gate so the
  grammar and dev-gate lanes cannot remain split.

R2 queued semantic corrective:
  untyped legacy field read returned through a method can panic at
  phi_type_inference.rs instead of producing a stable structured rejection.
  Queue LANGV1-UNTYPED-FIELD-PHI-FAILFAST-CORRECTIVE-001 after this design
  decision. It requires a checked-in fixture, a Result-bearing owner boundary,
  one stable type/* tag, and zero Rust panic/fallback-to-Unknown behavior.
```

R1 changes guard wiring only and does not answer the product-backend decision.
R2 is not implemented inside this design-stop card.

The first full `dev_gate quick` rerun also exposed MIR-root allowlist drift from
the earlier contract-family cards. The corrective keeps operation IDs/kinds
and refresh orchestration at the root, removes carrier bundle re-exports, and
records the classification in the MIR facade SSOT. This is gate convergence,
not product-backend implementation.

Corrective evidence:

```text
naming_charter_guard = green
MIR root facade/import guards = green; exports=120
dev_gate quick = green
LANGV1_GRAMMAR_FULL = naming green + 38/38 + 15/15 green
product backend behavior changed = 0
```

## Closed Basis

3501 and 3502 already close the language/runtime-reference law:

```text
source owner = WeakFieldContractSpec
write owner = WeakFieldWrite
storage owner = InstanceBox declaration-indexed WeakSlotState
runtime policy owner = WeakFieldRuntime
empty read = unified absence
dead/freed target = stable WeakRef token; upgrade returns absence
runtime check elision = 0
Rust VM = semantic-reference consumer only
non-VM backend = reject before effects
```

The VM is not selected as a product backend and must not acquire planner,
layout, optimization, or fallback policy in this row.

## Current Inventory

```text
MIR JSON:
  transports WeakFieldContractSpec, WeakFieldWriteContract, WeakFieldWrite

Wasm:
  has representation-level WeakRef(New) support only
  no declaration-indexed weak slot or WeakFieldWrite consumer

LLVM / ny-llvmc / AOT / PyVM:
  no complete WeakFieldWrite carrier consumer found

central preflight:
  weak_field_runtime_guard_v1 accepts mir-interpreter only
  every product backend rejects before effects
```

Representation-level weak handles are not proof of the field contract.
`WeakFieldWrite -> FieldSet + Barrier` projection remains forbidden.

## Consultation Questions

1. Should the next implementation target be the active product EXE/AOT lane,
   or should Weak fields remain reference-only while Language v1 proceeds to
   the next semantic row?
2. If a product backend is selected, which backend is the single first owner:
   `ny-llvmc EXE/OBJ`, Wasm, or another current product lane?
3. Must the product runtime reproduce the declaration-indexed
   `WeakSlotState::{Empty, Occupied}` layout directly, or may it use a different
   representation behind the same carrier and observable laws?
4. Which component owns lifecycle bookkeeping and stable target-token equality
   in the product runtime? Backend-local inference is forbidden.
5. What is the exact pre-effect capability proof for dynamic alias writes and
   reads when no statically resolved `WeakFieldWrite` exists?
6. Is a native `WeakFieldWrite` lowering mandatory, or may a validated helper
   ABI consume the carrier? A generic `FieldSet + Barrier` lowering is not an
   acceptable candidate.
7. What minimum parity fixtures are required before changing
   `weak_field_runtime_guard_v1` from unsupported to supported?

## Candidate Directions

```text
A. Defer product lowering.
   Keep VM as a narrow oracle, keep all product backends fail-fast, and advance
   to the next Language v1 semantic decision.

B. Select one product backend with native WeakFieldWrite support.
   Add one carrier-aware lowering, one declaration-layout runtime owner, and
   parity fixtures before enabling its capability.

C. Select one product backend with a validated WeakField helper ABI.
   The helper must consume source-derived field identity and preserve atomic
   validate/publication semantics. Generic FieldSet projection stays forbidden.
```

## Recommendation

Prefer A unless Weak fields are a concrete blocker for the active selfhost EXE
front. If they are a blocker, select exactly one current product backend and
choose B or C from measured call/runtime constraints. Do not extend the VM to
stand in for missing product support.

## Minimum Next Slice

The accepted decision must name exactly one of:

```text
defer_with_failfast
native_product_weak_field_consumer
validated_product_weak_field_helper_abi
```

If implementation is selected, the card must include one backend, one runtime
owner, one capability row, positive/negative parity fixtures, and no fallback.

## Non-Claims

```text
product_weak_field_backend_selected = 0
product_weak_field_backend_supported = 0
vm_product_backend = 0
backend_weak_field_projection = 0
runtime_backend_fallback = 0
ownership_kernel_activation = 0
selfhost_claim = 0
```
