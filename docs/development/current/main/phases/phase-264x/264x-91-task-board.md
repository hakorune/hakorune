# Phase 264x Task Board

Status: ACTIVE

## Done

- [x] decide that the next numeric-loop / SIMD cut is a reduction-recognition proof seam
- [x] annotate simple while plans with conservative reduction candidates
- [x] cache the annotated plan on the function-local lowering context
- [x] add focused tests for the numeric-loop reduction proof seam

## Next

- [ ] keep SIMD widening out of this proof seam
- [ ] widen only when a concrete numeric-loop or SIMD proof justifies it
