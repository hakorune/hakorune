use crate::config::env;
use crate::config::env::{joinir_dev_enabled, joinir_strict_enabled};
use crate::mir::join_ir::{lower_funcscanner_trim_to_joinir, lower_skip_ws_to_joinir, JoinFuncId};
use crate::mir::join_ir_ops::JoinValue;
use crate::mir::join_ir_vm_bridge::run_joinir_via_vm;
use crate::mir::MirModule;
use crate::runtime::get_global_ring0;
use std::process;

/// Main.skip/1 用 JoinIR ブリッジ（Exec: JoinIR→VM 実行まで対応）
///
/// Note: PHI canary として使用しているため、default_enabled=false。
/// env フラグ（NYASH_JOINIR_EXPERIMENT=1 & NYASH_JOINIR_VM_BRIDGE=1）が必須。
pub(crate) fn try_run_skip_ws(module: &MirModule, quiet_pipe: bool) -> bool {
    let log_enabled = !quiet_pipe && (env::joinir_vm_bridge_debug() || env::cli_verbose_enabled());
    if log_enabled {
        get_global_ring0()
            .log
            .info("[joinir/vm_bridge] Attempting JoinIR path for Main.skip");
    }

    let Some(join_module) = lower_skip_ws_to_joinir(module) else {
        if log_enabled {
            get_global_ring0()
                .log
                .warn("[joinir/vm_bridge] lower_skip_ws_to_joinir returned None");
            get_global_ring0()
                .log
                .info("[joinir/vm_bridge] Falling back to normal VM path");
        }
        return false;
    };

    // 入力引数を取得（環境変数またはデフォルト）
    let input =
        crate::config::env::joinir_dev::input_mode().unwrap_or_else(|| "   abc".to_string());
    if log_enabled {
        get_global_ring0()
            .log
            .debug(&format!("[joinir/vm_bridge] Input: {:?}", input));
    }

    let dev_bridge = joinir_dev_enabled() || env::env_bool("NYASH_EMIT_MIR_TRACE");
    let strict = joinir_strict_enabled();

    match run_joinir_via_vm(&join_module, JoinFuncId::new(0), &[JoinValue::Str(input)]) {
        Ok(result) => {
            let exit_code = match &result {
                JoinValue::Int(v) => *v as i32,
                JoinValue::Bool(b) => {
                    if *b {
                        1
                    } else {
                        0
                    }
                }
                _ => 0,
            };
            if log_enabled {
                get_global_ring0().log.info(&format!(
                    "[joinir/vm_bridge] ✅ JoinIR result: {:?}",
                    result
                ));
            }
            if dev_bridge {
                // Devモード: 結果を出しても exit しない（後続パスを継続して観測する）
                if !quiet_pipe {
                    println!("RC: {}", exit_code);
                }
                return true;
            } else if strict {
                if !quiet_pipe {
                    println!("RC: {}", exit_code);
                }
                process::exit(exit_code);
            } else {
                if !quiet_pipe {
                    println!("RC: {}", exit_code);
                }
                process::exit(exit_code);
            }
        }
        Err(e) => {
            if log_enabled {
                get_global_ring0().log.warn(&format!(
                    "[joinir/vm_bridge] ❌ JoinIR execution failed: {:?}",
                    e
                ));
                get_global_ring0()
                    .log
                    .info("[joinir/vm_bridge] Falling back to normal VM path");
            }
            false
        }
    }
}

/// FuncScannerBox.trim/1 用 JoinIR ブリッジ（Exec: JoinIR→VM 実行まで対応）
///
/// A/B 実証済み。ただし VM bridge 実行は env フラグが必要。
pub(crate) fn try_run_trim(module: &MirModule, quiet_pipe: bool) -> bool {
    let log_enabled = !quiet_pipe && (env::joinir_vm_bridge_debug() || env::cli_verbose_enabled());
    if log_enabled {
        get_global_ring0()
            .log
            .info("[joinir/vm_bridge] Attempting JoinIR path for FuncScannerBox.trim");
    }

    let Some(join_module) = lower_funcscanner_trim_to_joinir(module) else {
        if log_enabled {
            get_global_ring0()
                .log
                .warn("[joinir/vm_bridge] lower_funcscanner_trim_to_joinir returned None");
            get_global_ring0()
                .log
                .info("[joinir/vm_bridge] Falling back to normal VM path");
        }
        return false;
    };

    // 入力引数を取得（環境変数またはデフォルト）
    let input =
        crate::config::env::joinir_dev::input_mode().unwrap_or_else(|| "   abc  ".to_string());
    if log_enabled {
        get_global_ring0()
            .log
            .debug(&format!("[joinir/vm_bridge] Input: {:?}", input));
    }

    let dev_bridge = joinir_dev_enabled() || env::env_bool("NYASH_EMIT_MIR_TRACE");
    let strict = joinir_strict_enabled();

    match run_joinir_via_vm(&join_module, JoinFuncId::new(0), &[JoinValue::Str(input)]) {
        Ok(result) => {
            if log_enabled {
                get_global_ring0().log.info(&format!(
                    "[joinir/vm_bridge] ✅ JoinIR trim result: {:?}",
                    result
                ));
            }
            if dev_bridge {
                if !quiet_pipe {
                    match &result {
                        JoinValue::Str(s) => println!("{}", s),
                        _ => println!("{:?}", result),
                    }
                }
                return true;
            } else if strict {
                if !quiet_pipe {
                    match &result {
                        JoinValue::Str(s) => println!("{}", s),
                        _ => println!("{:?}", result),
                    }
                }
                process::exit(0);
            } else {
                if !quiet_pipe {
                    match &result {
                        JoinValue::Str(s) => println!("{}", s),
                        _ => println!("{:?}", result),
                    }
                }
                process::exit(0);
            }
        }
        Err(e) => {
            if log_enabled {
                get_global_ring0().log.warn(&format!(
                    "[joinir/vm_bridge] ❌ JoinIR trim failed: {:?}",
                    e
                ));
                get_global_ring0()
                    .log
                    .info("[joinir/vm_bridge] Falling back to normal VM path");
            }
            false
        }
    }
}
