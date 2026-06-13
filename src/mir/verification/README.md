# MIR Verification Boundary

This directory owns MIR verifier checks that run from
`src/mir/verification.rs`.

## Entry Points

| Entry | Owner | Purpose | Gate |
| --- | --- | --- | --- |
| `MirVerifier::verify_module` | MIR verifier | module contracts plus per-function MIR checks | disabled only by `NYASH_STAGEB_DEV_VERIFY=0` |
| `MirVerifier::verify_function` | MIR verifier | function-local MIR correctness | used directly by VM verifier gate |
| `runner::modes::common_util::verifier_gate` | runner | optional VM / VM-Hako verifier enforcement | `NYASH_VM_VERIFY_MIR=1` |
| `builder/control_flow/verify` | CorePlan verifier | verifies CorePlan before lowering | always-on for verified plan paths |
| `tools/hako_check` | tooling | observation/report checks | read-only, not semantic truth |

## Function Check Groups

`verify_function` keeps error precision by running separate checks. Treat these
as visible groups, not as permission to merge unrelated diagnostics.

```text
core_graph:
  SSA form
  dominance
  control flow
  PHI predecessor coverage
  merge-block value use

runtime_safety:
  WeakRef / Barrier
  barrier-context diagnostic
  legacy-op rejection
  await checkpoints

optional_dev:
  PHI-off edge-copy strict
  return-block purity

semantic_contracts:
  string kernel plans
  rune contracts
  required inline plans
  FastMemory regions
```

## Module Check Groups

```text
module_contracts:
  exact numeric field assignments
  module metadata invariants
  hako_alloc metadata invariants
  hako_alloc page lifecycle invariants
```

## Rules

```text
allowed:
  group helper functions for readability
  boundary docs and gate inventory
  env helper centralization with same defaults

forbidden:
  remove checks in a thinning card
  merge checks if error owner becomes less precise
  treat hako_check as verifier truth
  make VM verifier gate silently stricter than the documented contract
```

SSOT:

- `docs/development/current/main/design/compiler-pipeline-thinning-ssot.md`
- `docs/development/current/main/design/recipe-first-entry-contract-ssot.md`
- `docs/development/current/main/design/joinir-observation-layer-ssot.md`
