# 296x-1691 MIR-MODULE-MINIMAL-SHELL-TRANSPORT-001

Status: Landed
Date: 2026-06-25
Token: MIR-MODULE-MINIMAL-SHELL-TRANSPORT-001

## Purpose

Close the first derived minimal-execution frontier edge:

```text
MirBuilder::prepare_module -> MirModule::new
```

The slice models only the constructor shell transport. It does not claim
source-file assignment, function insertion, globals publication, metadata
publication, finalize behavior, generated Hako, backend routes, ABI changes,
runtime fallback, or mainline selection.

## Source Authority

```text
src/mir/function/module_impl.rs::MirModule::new
src/mir/function/types.rs::MirModule
```

## Transport Plan

```text
MirModuleMinimalShellTransport:
  name      = parameter:name, ModuleNameStringAtom
  functions = BTreeMap::new, EmptyFunctionTable
  globals   = HashMap::new, EmptyGlobalConstTable
  metadata  = ModuleMetadata::default, ModuleMetadataDefaultShell
```

`ModuleMetadata::default()` is observed only as default shell state. The
profile-specific `source_file=None` assignment is handled by the minimal
execution path analyzer as `ProfileExcluded`, not by this transport plan.

## Frontier Result

After this plan is available, the minimal execution path analyzer advances to:

```text
callsite:
  MirBuilder::prepare_module -> MirFunction::new

reason:
  UnsupportedTypeTransport

detail:
  MirFunctionConstructorTransportRequired

next slice:
  MIR-FUNCTION-CONSTRUCTOR-COMPOSITION-001
```

## Files

```text
tools/rust_lifecycle/mirbuilder_mir_module_minimal_shell_transport.py
tools/checks/rust_lifecycle_mir_module_minimal_shell_transport_guard.sh
docs/development/current/main/design/fixtures/rust-lifecycle/mir-module-minimal-shell-transport-plan-v0.json
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_mir_module_minimal_shell_transport_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
python3 -m py_compile tools/rust_lifecycle/mirbuilder_mir_module_minimal_shell_transport.py
bash tools/checks/current_state_pointer_guard.sh
```
