# Phase 262x Task Board

Status: LANDED

## Done

- [x] decide that the first numeric-loop / SIMD cut is a policy seam, not a new lowering route
- [x] centralize LLVM vectorization knobs behind one helper
- [x] add focused policy tests for the helper

## Next

- [x] keep fast-math / FMA widening out of the first seam
- [x] widen only when a concrete numeric-loop or SIMD proof justifies it
