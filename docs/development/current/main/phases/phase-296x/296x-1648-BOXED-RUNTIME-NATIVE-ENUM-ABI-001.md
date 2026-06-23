# 296x-1648: Boxed Runtime Native Enum ABI

Status: Active
Date: 2026-06-23
Token: BOXED-RUNTIME-NATIVE-ENUM-ABI-001

## Decision

RegionObserver `Vec<SlotMetadata>` output transport is selected:

```text
RefSlotKind
  = native enum

SlotMetadata
  = semantic OwnedProduct

current execution transport
  = ArrayBox<SlotMetadataBox>

future optimized transport
  = InlineRecord / packed / SoA, without changing read-fold semantics
```

This closes the prior output-transport design stop. Do not replace native enum
transport with manual i64 tags just to make the current AOT backend pass.

## Blocker

The generated RegionObserver SlotMetadata artifact reaches MIR, but EXE/AOT
stops at the first `variant_tag` in:

```text
SlotClassifierApi.classify/2
```

Current backend support covers native enum values that are local aggregates
created by same-function `variant_make`. The RegionObserver classifier needs
native enum values that cross function/container boundaries:

```text
Option<MirType> parameter
MapBox.get(... MirType ...) -> Option::Some(MirType)
Option::Some payload -> MirType variant_tag / variant_project
```

## Required Contract

Do not add new canonical MIR instructions. These stay canonical:

```text
VariantMake
VariantTag
VariantProject
```

Add representation selection and an ABI plan:

```text
SumValueRepresentation =
  LocalAggregate(layout)
  BoxedRuntime(abi_plan_id)

BoxedSumAbiPlanV1 =
  plan_id
  enum_name
  runtime_type_id
  runtime_box_name
  tag_storage
  variants[]
```

Selected route names:

```text
variant_make.boxed_runtime_v1
variant_tag.boxed_runtime_v1
variant_project.boxed_runtime_v1
```

The backend consumes the plan and site route facts. It must not infer enum
layout from family names, `MirType`, `Option`, or the RegionObserver source
path.

## Task Order

1. `Implement boxed native enum make/tag ABI`

   Scope:

   ```text
   payload-less native enum
   boxed VariantMake
   cross-function parameter transport
   boxed VariantTag
   ```

2. `Implement boxed native enum handle projection`

   Scope:

   ```text
   handle-payload enum
   boxed VariantProject
   nested enum tag after projection
   ```

3. `Close boxed enum container round trip`

   Scope:

   ```text
   MapBox-returned enum
   enum nested in Option
   native enum function parameter
   native enum return
   enum stored in typed object field
   RegionObserver SlotMetadata artifact
   ```

## Acceptance

```text
same-function local enum route still green
cross-function unit enum route EXE/AOT green
handle-payload projection EXE/AOT green
MapBox-returned enum classifier EXE/AOT green
RegionObserver SlotMetadata artifact EXE/AOT green
raw aggregate variable_map return = 0
manual i64 enum-tag workaround = 0
RegionObserver backend branch = 0
MirType backend branch = 0
unknown enum ABI -> Deny(UnsupportedEnumValueTransport)
runtime fallback = 0
```

## Parked

```text
read-view / lease framework
record-in-ArrayBox claim
InlineRecord / packed / SoA optimization
full RegionObserver native authority adoption
```
