---
Status: Landed
Date: 2026-06-23
Scope: MirBuilder Rust-to-Hako converter ordering capability IR
---

# 296x-1645: Define Total Text Ordering Capability

## Result

The RegionObserver variable-map route now carries structured ordering facts:

```text
KeyAscending(RustStringOrdV1)
```

The ordered read-fold compiler no longer reports the blocker as key transport
failure. String keys are transportable; the missing requirement is executable
ordering capability.

Current fail-fast route:

```text
Deny(UnsupportedOrderCapability)
detail=ComparatorUnavailable
comparator=RustStringOrdV1
required_tiers=VM,EXE,AOT
```

## Code Boundary

The intermediate model is under `tools/rust_lifecycle`:

```text
mirbuilder_ordering_capability.py
```

No backend behavior changed in this slice.

## Next Blocker

```text
IMPLEMENT-BACKEND-ACCEPTED-TOTAL-TEXT-ORDERING-CAPABILITY-001
```

Next task:

```text
Implement CompareTotal(RustStringOrdV1) with VM / EXE / AOT acceptance.
```

Stop line remains:

```text
source_ordered_read_fold_claim=0
generated_region_observer_artifact=0
runtime_fallback=0
```
