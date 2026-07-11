# 191x-90: loop-carried same-root local field pruning SSOT

Goal
- land the first loop/backedge local-field DCE slice from `phase190x` lane A without widening into overwrite propagation or generic memory

Contract
- a dead local `FieldGet` or unobserved local `FieldSet` remains removable when:
  - the base value is a loop-carried same-root phi
  - that phi resolves to the same definitely non-escaping local root
  - the loop use is otherwise unobserved and has no escape use

Out of scope
- loop-round overwritten write pruning
- mixed-root backedge carriers
- generic `Store` / `Load`
- `Debug` and terminators
