JoinIR VM Bridge Dispatch  

Purpose:
- Centralize JoinIR→VM routing away from the VM runner.
- Table-driven mapping of MIR function names to JoinIR execution behavior.
- Keep VM execution separate from neutral loop-target classification. Loop
  bridge execution is opt-in via
  `NYASH_JOINIR_VM_BRIDGE`; If mainline defaults are owned by the target table.

Layout:
- `mod.rs`: public entry (`try_run_joinir_vm_bridge`) + shared routing glue
- `env_flags.rs`: bridge enablement wrapper (`NYASH_JOINIR_VM_BRIDGE`; core
  always-on/deprecation handling stays in `src/config/env/joinir_flags.rs`)
- `targets.rs`: two-row VM execution table (`JOINIR_VM_EXEC_TARGETS`) plus the
  separate If target table
- `exec_routes.rs`: Exec-capable routes (skip_ws, trim)

Routing rule:
- registered VM targets may handle process output/exit through JoinIR VM bridge.
- Exec failure is fail-fast for either established JoinIR strict alias;
  non-strict continuation remains the bounded explicit-VM compatibility path.
- Stage1/StageB lowerers remain direct evidence assets but are not VM dispatch
  targets.

## P5 Crate Split Prep

`join_ir_vm_bridge_dispatch/` stays inside the future `hakorune-mir-joinir` boundary for
now. The prep step is to keep routing tables explicit and stable before any
packaging move.

SSOT:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`

Prep rule:

- do not split dispatch routing away from the bridge yet
- keep env-flag handling and route descriptors narrow and table-driven
