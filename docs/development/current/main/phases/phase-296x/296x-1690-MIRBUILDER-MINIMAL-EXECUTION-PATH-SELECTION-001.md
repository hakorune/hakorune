# 296x-1690 MIRBUILDER-MINIMAL-EXECUTION-PATH-SELECTION-001

Status: Landed
Date: 2026-06-25
Token: MIRBUILDER-MINIMAL-EXECUTION-PATH-SELECTION-001

## Purpose

Select the first minimal MirBuilder execution path without creating a temporary
runner or promoting bundle size to semantic proof.

## Selected Profile

```text
entry:
  PreparedMirBuilderStateV1

source entry:
  MirBuilder::build_module(ASTNode::Literal(Integer(0)))

initial state:
  current_module = Absent
  current_function = Absent
  current_block = Absent
  reserved_value_ids = Empty
  source_file = None
```

## Authority

```text
source call order:
  live Rust source

capability availability:
  explicit family artifact contracts

bundle:
  membership only

frontier result:
  derived output
```

## Result

The analyzer derives the first unsupported live edge:

```text
callsite:
  MirBuilder::prepare_module -> MirModule::new

reason:
  UnsupportedTypeTransport

detail:
  MirModuleMinimalShellTransportRequired

next slice:
  MIR-MODULE-MINIMAL-SHELL-TRANSPORT-001
```

Later edges are marked `NotReached`, not silently green.

## Files

```text
tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-execution-path-plan-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-first-red-edge-result-v0.json
```

## Non-Claims

```text
generated Hako change = 0
backend route = 0
ABI = 0
runtime fallback = 0
mainline selection = 0
source selfhost claim = 0
full MirBuilder::new claim = 0
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
python3 -m py_compile tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/current_state_pointer_guard.sh
```
