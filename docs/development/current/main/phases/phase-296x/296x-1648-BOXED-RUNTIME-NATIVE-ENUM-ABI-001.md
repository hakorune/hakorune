# 296x-1648: Boxed Runtime Native Enum ABI

Status: Landed
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

Focused boxed enum probes are green for cross-function unit enums,
handle-payload projection, MapBox-returned enums, and Option-wrapped map
results. The full generated RegionObserver SlotMetadata artifact is also
green through LLVM/AOT:

```text
region_observer_slot_metadata_artifact=ok
Result: 0
```

The final blocker was generic mixed runtime value transport:

```text
MapBox i64-key raw load preserves negative typed-object / boxed enum handles.
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

   Status: landed.

   Scope:

   ```text
   payload-less native enum
   boxed VariantMake
   cross-function parameter transport
   boxed VariantTag
   ```

2. `Implement boxed native enum handle projection`

   Status: landed.

   Scope:

   ```text
   handle-payload enum
   boxed VariantProject
   nested enum tag after projection
   ```

3. `Close boxed enum container round trip`

   Status: landed.

   Scope:

   ```text
   MapBox-returned enum
   enum nested in Option
   native enum function parameter
   native enum return
   enum stored in typed object field
   RegionObserver SlotMetadata artifact
   ```

4. `Close RegionObserver SlotMetadata artifact`

   Status: landed.

   Scope:

   ```text
   verified source-ordered read-fold
   SlotClassifierApi.classify/2
   ArrayBox<SlotMetadataBox> output
   VM/MIR/EXE/AOT green
   ```

   Closed blocker:

   ```text
   ny-llvmc failure retains the exact temp MIR JSON and prints
   retained_mir=<path>. The retained evidence selected generic MapBox mixed
   runtime value publication as the fix, not source-shape changes.

   forbidden fixes:
     generated Hako source-shape workaround
     RegionObserver / MirType backend branch
     OrderedMapBox-only backend branch
   ```

5. `Move slot classification policy into verified operation data`

   Status: landed.

   Scope after RegionObserver AOT is green:

   ```text
   live Rust classification facts
     -> ClassifyEnumVariants operation data
     -> emitter renders table only

   Some(MirType::Unknown) -> NonRef
   None + "args" -> StrongRoot
   fallback names live in operation data, not the emitter
   ```

6. `Select the next post-RegionObserver converter slice`

   Status: design stop.

## Acceptance

```text
same-function local enum route still green
cross-function unit enum route EXE/AOT green
handle-payload projection EXE/AOT green
MapBox-returned enum classifier EXE/AOT green
ny-llvmc failure retains exact input MIR path
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
