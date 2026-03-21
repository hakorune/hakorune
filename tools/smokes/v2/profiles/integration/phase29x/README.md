# phase29x smoke family

Semantic integration families split out of `tools/smokes/v2/profiles/integration/apps/`
for the phase-29x runtime and lane contracts.

## Layout

- `vm_hako/`: vm-hako S6 and NewClosure contract gates
- `derust/`: de-rust route and lane contract gates. Landed and parked as a stable pin family.
- `observability/`: route observability and strict/default priority gates. Landed and parked as a stable pin family.
- `optimization/`, `rc/`, `runtime/`, `cache/`, `core/`, `llvm/`, `abi/`, `l1/`, `l2/`, `l3/`: remaining phase29x residual families after `observability`

## Contract

- `derust/` and `observability/` are the last landed live slices from the first `phase29x` wave.
- Treat further `phase29x` residual families as parked backlog unless the lane is explicitly reopened.
- Keep the family separate from `vm_hako/` and from the remaining `phase29x` residual buckets.
