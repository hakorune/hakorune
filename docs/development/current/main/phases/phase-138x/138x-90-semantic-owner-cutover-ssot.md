# Phase 138x-90: semantic owner cutover SSOT

## Goal

Turn the landed four-bucket split into a stable final owner model before reopening perf work.

## Final Reading

### Permanent owners

1. `Rust host microkernel`
2. `.hako semantic kernel`
3. `native accelerators`

### Auxiliary surfaces

- `ABI facade`
- `compat quarantine`

## Constraints

- keep the `phase-134x` split as the refactor seam
- do not restart broad perf work before the owner graph is fixed
- do not broaden `.hako` migration into hot leaf substrate
- do not let `compat quarantine` become a permanent owner

## First Concrete Cutover

1. `Array owner`
2. `Map owner`
3. `String` semantic boundary review

## Success Condition

- current docs read `semantic owner cutover`, not `perf reopen`
- Rust permanent zones are locked
- `.hako` semantic owner corridor is fixed
- `phase-139x array owner pilot` is the next implementation lane
