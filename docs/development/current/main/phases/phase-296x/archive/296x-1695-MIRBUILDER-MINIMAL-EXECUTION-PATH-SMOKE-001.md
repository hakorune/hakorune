# 296x-1695 MIRBUILDER-MINIMAL-EXECUTION-PATH-SMOKE-001

Status: Landed
Date: 2026-06-25
Token: MIRBUILDER-MINIMAL-EXECUTION-PATH-SMOKE-001

## Purpose

Close the minimal execution path smoke edge:

```text
PreparedMirBuilderStateV1
  -> build_module(ASTNode::Literal(Integer(0)))
```

The smoke observes the current Rust execution path and records that the
previous PlanOnly capabilities are sufficient to build the minimal literal
module shape. It is not a mainline-selection claim.

## Smoke Observation

```text
input:
  ASTNode::Literal(Integer(0))

observed:
  main function exists
  main return type = MirType::Integer
  main emits ConstValue::Integer(0)
  main returns that literal const dst
  condition_fn injection remains source-required
```

## Contract

```text
MinimalExecutionPathSmoke:
  provided by:
    tests/mirbuilder_minimal_execution_path_smoke.rs
    mirbuilder-minimal-execution-path-smoke-result-v0.json

  consumed by:
    minimal-mirbuilder-execution-path selection analyzer
```

After this smoke is available, the minimal execution path analyzer advances to:

```text
callsite:
  MirBuilder allocation policy mainline pilot selection

reason:
  UnsupportedDirectShape

detail:
  MainlineSelectionRequired

next slice:
  MIRBUILDER-ALLOCATION-POLICY-MAINLINE-PILOT-001
```

## Non-Claims

```text
full MirBuilder::new authority = 0
generated Hako change = 0
mainline selected = 0
new backend route = 0
new ABI = 0
runtime fallback = 0
source selfhost claim = 0
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_smoke_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
python3 -m py_compile \
  tools/rust_lifecycle/mirbuilder_minimal_execution_path_smoke.py \
  tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/current_state_pointer_guard.sh
```
