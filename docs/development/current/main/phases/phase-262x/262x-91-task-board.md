# Phase 262x Task Board

Status: ACTIVE

## Done

- [x] decide that the first numeric-loop / SIMD cut is a policy seam, not a new lowering route
- [x] centralize LLVM vectorization knobs behind one helper
- [x] add focused policy tests for the helper

## Next

- [ ] keep fast-math / FMA widening out of the first seam
- [ ] widen only when a concrete numeric-loop or SIMD proof justifies it
