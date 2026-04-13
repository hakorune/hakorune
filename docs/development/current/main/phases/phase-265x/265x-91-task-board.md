# Phase 265x Task Board

Status: ACTIVE

## Done

- [x] decide that the next numeric-loop / SIMD slice is the LoopSimdContract owner seam
- [x] pin proof / policy / lowering / diag as separate responsibilities
- [x] keep actual widening and fast-math out of this seam
- [x] materialize the first code-side LoopSimdContract seam and cache it by loop header id

## Next

- [ ] decide the first actual widening slice under LoopSimdContract
- [ ] prefer integer map loop widening before reduction widening
- [ ] keep LLVM metadata in the lowering layer only
