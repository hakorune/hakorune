# phase29x smoke family

Semantic integration families split out of `tools/smokes/v2/profiles/integration/apps/`
for the phase-29x runtime and lane contracts.

## Layout

- `vm_hako/`: vm-hako S6 and NewClosure contract gates
- `derust/`: next residual phase29x family after `vm_hako`.
- `observability/`, `optimization/`, `rc/`, `runtime/`, `cache/`, `core/`, `llvm/`, `abi/`, `l1/`, `l2/`, `l3/`: remaining phase29x residual families still parked under `integration/apps`.
