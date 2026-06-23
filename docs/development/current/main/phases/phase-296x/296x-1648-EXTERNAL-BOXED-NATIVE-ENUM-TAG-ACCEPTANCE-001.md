# 296x-1648: External Boxed Native Enum Tag Acceptance

Status: Active
Date: 2026-06-23
Token: EXTERNAL-BOXED-NATIVE-ENUM-TAG-ACCEPTANCE-001

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

Add a generic backend contract for external/boxed native enum values:

```text
variant_tag.external_boxed
variant_project.external_boxed
```

The contract must be route/fact driven. It must not infer enum layout from
family names or from the RegionObserver source path.

## Acceptance

```text
Option<MirType> parameter guard-let EXE/AOT green
MapBox-returned MirType classifier EXE/AOT green
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
