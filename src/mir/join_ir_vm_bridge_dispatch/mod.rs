//! Phase 30 F-4.4: JoinIR VM Bridge Dispatch
//!
//! VM runner から JoinIR 詳細を隠蔽し、関数名ベースのルーティングを一箇所に集約する。
//!
//! VM execution targets are a two-row table. The neutral five-name Loop
//! classification lives in `join_ir::lowering::loop_target_policy`.
//!
//! 将来は LoopScopeShape / ExitAnalysis ベースの構造判定に差し替え予定。

mod env_flags;
mod exec_routes;
mod targets;

use env_flags::JoinIrEnvFlags;
use exec_routes::{try_run_skip_ws, try_run_trim};
use targets::find_joinir_target;

use crate::mir::MirModule;
use crate::runtime::get_global_ring0;

/// JoinIR VM ブリッジ候補を判定し、マッチすれば JoinIR→VM を実行する。
///
/// # Arguments
/// - `module`: MIR モジュール
/// - `quiet_pipe`: 出力を抑制するかどうか
///
/// # Returns
/// - `true`: an Exec bridge route handled its attempt; release success exits,
///   while dev/trace success remains an observation before ordinary VM runs.
/// - `false`: the explicit compatibility lane continues through ordinary VM.
///
/// The VM execution table contains only the two routes that may execute.
pub fn try_run_joinir_vm_bridge(module: &MirModule, quiet_pipe: bool) -> bool {
    let flags = JoinIrEnvFlags::from_env();
    let strict = bridge_exec_strict_enabled();

    // Phase 32 L-4: テーブルから対象関数を探す
    let Some(target) = find_joinir_target(module) else {
        return false;
    };

    // Phase 32 L-4: 有効化条件チェック
    // - env フラグが有効
    //
    // NOTE:
    // `try_run_joinir_vm_bridge` は VM 実行の「前」に走るため、
    // 実行してしまうと stdout/exit_code を汚染し、gate や通常実行の意味論を壊しやすい。
    // そのため、このブリッジは常に明示的な env トグルでのみ有効化する（no silent enable）。
    let is_enabled = flags.is_bridge_enabled();
    if !is_enabled {
        return false;
    }

    let handled = match target.func_name {
        "Main.skip/1" => try_run_skip_ws(module, quiet_pipe),
        "FuncScannerBox.trim/1" => try_run_trim(module, quiet_pipe),
        _ => false,
    };

    if !handled {
        // Phase 80/81: Strict mode では本線対象関数の失敗でパニック
        if bridge_exec_failure_requires_exit(strict) {
            get_global_ring0().log.error(&format!(
                "[joinir/bridge] ERROR: target={} lowering/exec failed (strict, no fallback)",
                target.func_name
            ));
            std::process::exit(1);
        } else {
            return false;
        }
    }
    true
}

/// Exec bridge failures use the established JoinIR strict aliases locally.
///
/// This intentionally does not alter the historical NYASH-only helper used by
/// other JoinIR families.
fn bridge_exec_strict_enabled() -> bool {
    crate::config::env::joinir_dev::strict_enabled()
}

fn bridge_exec_failure_requires_exit(strict: bool) -> bool {
    strict
}

#[cfg(test)]
mod tests {
    use super::{bridge_exec_failure_requires_exit, bridge_exec_strict_enabled};

    #[test]
    fn bridge_exec_strict_accepts_both_established_aliases() {
        crate::test_support::with_env_vars(
            &[
                ("HAKO_JOINIR_STRICT", Some("1")),
                ("NYASH_JOINIR_STRICT", None),
            ],
            || assert!(bridge_exec_strict_enabled()),
        );
        crate::test_support::with_env_vars(
            &[
                ("HAKO_JOINIR_STRICT", None),
                ("NYASH_JOINIR_STRICT", Some("1")),
            ],
            || assert!(bridge_exec_strict_enabled()),
        );
    }

    #[test]
    fn bridge_exec_failure_requires_strict() {
        crate::test_support::with_env_vars(
            &[("HAKO_JOINIR_STRICT", None), ("NYASH_JOINIR_STRICT", None)],
            || {
                assert!(!bridge_exec_failure_requires_exit(false));
                assert!(bridge_exec_failure_requires_exit(true));
            },
        );
    }
}
