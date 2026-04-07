use crate::backend::MirInterpreter;
use crate::box_trait::{BoolBox, IntegerBox};
use crate::mir::{MirModule, VerificationError};
use crate::runner::modes::common_util::{
    emit_direct, safety_gate, verifier_gate, vm_user_factory::VmUserFactoryState,
};
use std::process;

/// Execute a compiled VM module with the shared post-compile gates and exit handling.
pub(crate) fn run_vm_compiled_module(
    route: &str,
    quiet_pipe: bool,
    emit_trace: bool,
    emit_mir_json: Option<&str>,
    emit_exe: Option<&str>,
    emit_exe_nyrt: Option<&str>,
    emit_exe_libs: Option<&str>,
    verification_result: &Result<(), Vec<VerificationError>>,
    mut module_vm: MirModule,
    vm_user_factory: &VmUserFactoryState,
    run_joinir_bridge: bool,
) -> ! {
    // Optional barrier-elision for parity with fallback path
    if crate::config::env::env_bool("NYASH_VM_ESCAPE_ANALYSIS") {
        let removed = crate::mir::passes::escape::escape_elide_barriers_vm(&mut module_vm);
        if removed > 0 {
            crate::cli_v!(
                "[{}] escape_elide_barriers: removed {} barriers",
                route,
                removed
            );
        }
    }

    // CLI emit: MIR JSON / EXE
    // NOTE: These flags are CLI-level and should work regardless of selected backend.
    // Treat them as explicit proof/compat lanes and exit early.
    emit_direct::maybe_emit_mir_json_and_exit(
        emit_mir_json,
        verification_result,
        route,
        quiet_pipe,
        |out_path| {
            crate::runner::mir_json_emit::emit_mir_json_for_harness_bin(&module_vm, out_path)
        },
    );
    emit_direct::maybe_emit_exe_and_exit(
        emit_exe,
        verification_result,
        route,
        quiet_pipe,
        |exe_out| {
            crate::runner::modes::common_util::exec::ny_llvmc_emit_exe_bin(
                &module_vm,
                exe_out,
                emit_exe_nyrt,
                emit_exe_libs,
            )
        },
    );

    // Optional: dump MIR for diagnostics
    // Phase 25.1: File dump for offline analysis (ParserBox等)
    if let Ok(path) = std::env::var("RUST_MIR_DUMP_PATH") {
        if let Ok(mut f) = std::fs::File::create(&path) {
            let p = crate::mir::MirPrinter::new();
            let _ = std::io::Write::write_all(&mut f, p.print_module(&module_vm).as_bytes());
            crate::runtime::ring0::get_global_ring0()
                .log
                .info(&format!("[{}] MIR dumped to: {}", route, path));
        }
    }
    // Existing: NYASH_VM_DUMP_MIR dumps to stderr
    if crate::config::env::env_bool("NYASH_VM_DUMP_MIR") {
        let p = crate::mir::MirPrinter::new();
        crate::runtime::ring0::get_global_ring0()
            .log
            .debug(&p.print_module(&module_vm));
    }

    // Execute via MIR interpreter
    let mut vm = MirInterpreter::new();

    // Register static box declarations for singleton persistence (AST-based)
    vm_user_factory.register_static_box_decls(&mut vm);

    // Optional verifier gate (single-entry SSOT across VM lanes)
    verifier_gate::enforce_vm_verify_gate_or_exit(&module_vm, route);
    safety_gate::enforce_vm_lifecycle_safety_or_exit(&module_vm, route);

    if std::env::var("NYASH_DUMP_FUNCS").ok().as_deref() == Some("1") {
        let ring0 = crate::runtime::ring0::get_global_ring0();
        ring0
            .log
            .debug(&format!("[{}] functions available:", route));
        for k in module_vm.functions.keys() {
            ring0.log.debug(&format!("  - {}", k));
        }
    }

    // Phase 33-10.0: If lowering ドライラン統合（箱化版）
    // JoinIR dev + IfSelect 有効時に IfLoweringDryRunner を使用
    if crate::config::env::joinir_dev_enabled() && crate::config::env::joinir_if_select_enabled() {
        let debug_level = crate::config::env::joinir_debug_level();
        let runner =
            crate::mir::join_ir::lowering::if_dry_runner::IfLoweringDryRunner::new(debug_level);
        let stats = runner.scan_module(&module_vm.functions);
        runner.print_stats(&stats);
    }

    if run_joinir_bridge {
        // Phase 30 F-4.4: JoinIR VM Bridge experimental path (consolidated dispatch)
        // Activated when NYASH_JOINIR_EXPERIMENT=1 AND NYASH_JOINIR_VM_BRIDGE=1
        // Routing logic is centralized in join_ir_vm_bridge_dispatch module
        crate::mir::join_ir_vm_bridge_dispatch::try_run_joinir_vm_bridge(&module_vm, quiet_pipe);
    }

    if emit_trace {
        let ring0 = crate::runtime::ring0::get_global_ring0();
        ring0.log.info(&format!(
            "[runner/{}:emit-trace] phase=execute.begin",
            route
        ));
    }
    match vm.execute_module(&module_vm) {
        Ok(ret) => {
            if emit_trace {
                let ring0 = crate::runtime::ring0::get_global_ring0();
                ring0.log.info(&format!(
                    "[runner/{}:emit-trace] phase=execute.done result={}",
                    route,
                    ret.to_string_box().value
                ));
            }

            // Extract exit code from return value
            let exit_code = if let Some(ib) = ret.as_any().downcast_ref::<IntegerBox>() {
                ib.value as i32
            } else if let Some(bb) = ret.as_any().downcast_ref::<BoolBox>() {
                if bb.value {
                    1
                } else {
                    0
                }
            } else {
                // For non-integer/bool returns, default to 0 (success)
                0
            };

            // Optional: print lightweight VM counters for diagnostics
            if crate::config::env::env_bool("NYASH_VM_STATS") {
                let ring0 = crate::runtime::ring0::get_global_ring0();
                let (inst, br, cmp) = vm.stats_counters();
                ring0.log.debug(&format!(
                    "[{}/stats] inst={} compare={} branch={}",
                    route, inst, cmp, br
                ));
            }

            // Quiet mode: suppress "RC:" output for JSON-only pipelines
            if !quiet_pipe {
                // Phase 105.5: Unified console output via macro
                crate::console_println!("RC: {}", exit_code);
            }
            if std::env::var("NYASH_EMIT_MIR_TRACE").ok().as_deref() == Some("1") {
                let ring0 = crate::runtime::ring0::get_global_ring0();
                ring0
                    .log
                    .debug(&format!("[runner/{}] exit_code={}", route, exit_code));
            }

            // Phase 285: Emit leak report before exit (if enabled)
            crate::runtime::leak_tracker::emit_leak_report();

            // Exit with the return value as exit code
            process::exit(exit_code);
        }
        Err(e) => {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            if std::env::var("NYASH_EMIT_MIR_TRACE").ok().as_deref() == Some("1") {
                ring0
                    .log
                    .debug(&format!("[runner/{}] vm_error={}", route, e));
            }
            ring0.log.error(&format!("❌ [{}] VM error: {}", route, e));
            process::exit(1);
        }
    }
}
