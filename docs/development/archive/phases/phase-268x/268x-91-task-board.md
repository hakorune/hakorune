# Phase 268x Task Board

Status: LANDED

## Done

- [x] choose compare/select candidates as the next widening slice
- [x] carry `numeric_select_value_ids` from the loop proof seam
- [x] lower `int_compare_select_candidate` contracts into conservative `llvm.loop` metadata
- [x] cover the compare/select metadata lowering seam with focused tests

## Closeout

- [x] numeric loop / SIMD is closed out after this cut
- [x] floating-point widening remains outside the current lane
