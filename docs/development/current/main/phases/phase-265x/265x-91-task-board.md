# Phase 265x Task Board

Status: LANDED

## Done

- [x] decide that the next numeric-loop / SIMD slice is the LoopSimdContract owner seam
- [x] pin proof / policy / lowering / diag as separate responsibilities
- [x] keep actual widening and fast-math out of this seam
- [x] materialize the first code-side LoopSimdContract seam and cache it by loop header id

## Next

- [x] decide the first actual widening slice under LoopSimdContract
- [x] prefer integer map loop widening before reduction widening
- [x] keep LLVM metadata in the lowering layer only
