JoinIR VM Bridge Dispatch  

Purpose:
- Centralize JoinIR→VM routing away from the VM runner.
- Table-driven mapping of MIR function names to JoinIR lowering/exec behavior.
- Keep Exec vs LowerOnly paths explicit. Loop bridge execution is opt-in via
  `NYASH_JOINIR_VM_BRIDGE`; If mainline defaults are owned by the target table.

Layout:
- `mod.rs`: public entry (`try_run_joinir_vm_bridge`) + shared routing glue
- `env_flags.rs`: bridge enablement wrapper (`NYASH_JOINIR_VM_BRIDGE`; core
  always-on/deprecation handling stays in `src/config/env/joinir_flags.rs`)
- `targets.rs`: descriptor table (`JOINIR_TARGETS`, `JoinIrBridgeKind`, `JoinIrTargetDesc`)
- `exec_routes.rs`: Exec-capable routes (skip_ws, trim)
- `lower_only_routes.rs`: LowerOnly routes (Stage1/StageB) for structural verification only

Routing rule:
- `Exec` targets may handle process output/exit through JoinIR VM bridge.
- `LowerOnly` targets may run structural lowering observation, but must always
  return to normal VM Route A, including strict mode.

## P5 Crate Split Prep

`join_ir_vm_bridge_dispatch/` stays inside the future `hakorune-mir-joinir` boundary for
now. The prep step is to keep routing tables explicit and stable before any
packaging move.

SSOT:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`

Prep rule:

- do not split dispatch routing away from the bridge yet
- keep env-flag handling and route descriptors narrow and table-driven
