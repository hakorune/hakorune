JoinIR VM Bridge Dispatch  

Purpose:
- Centralize JoinIR→VM routing away from the VM runner.
- Table-driven mapping of MIR function names to JoinIR lowering/exec behavior.
- Keep Exec vs LowerOnly paths explicit and opt-in via env flags or defaults.

Layout:
- `mod.rs`: public entry (`try_run_joinir_vm_bridge`) + shared routing glue
- `env_flags.rs`: env flag evaluation (`NYASH_JOINIR_EXPERIMENT`, `NYASH_JOINIR_VM_BRIDGE`)
- `targets.rs`: descriptor table (`JOINIR_TARGETS`, `JoinIrBridgeKind`, `JoinIrTargetDesc`)
- `exec_routes.rs`: Exec-capable routes (skip_ws, trim)
- `lower_only_routes.rs`: LowerOnly routes (Stage1/StageB) for structural verification only
