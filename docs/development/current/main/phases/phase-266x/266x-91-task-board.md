# Phase 266x Task Board

Status: ACTIVE

## Done

- [x] choose integer map loops as the first actual widening slice
- [x] lower `int_map_candidate` contracts into conservative `llvm.loop` metadata
- [x] keep reduction candidates deferred in lowering
- [x] cover the metadata lowering seam with focused tests

## Next

- [ ] widen integer sum reductions under LoopSimdContract
- [ ] keep compare/select expansion separate
