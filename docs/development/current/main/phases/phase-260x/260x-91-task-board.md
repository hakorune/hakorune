# Phase 260x Task Board

## Landed

- [x] `M0` owner seam and stats surface
- [x] `M1` same-block private-carrier store-to-load forwarding
- [x] `M2` same-block private-carrier redundant load elimination
- [x] `M3` overwritten-store / DSE widening beyond the landed private same-block cut

## Next

- [ ] hand off to `escape / barrier -> LLVM attrs`

## Notes

- keep the memory work out of `semantic simplification bundle`
- keep the handoff narrow enough that it can be scheduled independently
- do not widen into hoist/sink legality until the memory-effect handoff is settled
