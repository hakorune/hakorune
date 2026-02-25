//! Phase 30 F-4.4: JoinIR VM Bridge Dispatch
//!
//! VM runner から JoinIR 詳細を隠蔽し、関数名ベースのルーティングを一箇所に集約する。
//!
//! ## Phase 32 L-4: Descriptor テーブル導入
//!
//! 関数名→役割のマッピングを `JOINIR_TARGETS` テーブルで管理し、
//! 「どの関数が Exec（実行可能）か LowerOnly（検証のみ）か」を明示する。
//!
//! 将来は LoopScopeShape / ExitAnalysis ベースの構造判定に差し替え予定。

mod env_flags;
mod exec_routes;
mod lower_only_routes;
mod targets;

use env_flags::JoinIrEnvFlags;
use exec_routes::{try_run_skip_ws, try_run_trim};
use lower_only_routes::{
    try_run_stage1_usingresolver, try_run_stageb_body, try_run_stageb_funcscanner,
};
use targets::find_joinir_target;
pub use targets::{
    is_if_lowered_function, JoinIrBridgeKind, JoinIrTargetDesc, JOINIR_IF_TARGETS, JOINIR_TARGETS,
};

use crate::mir::MirModule;
use crate::runtime::get_global_ring0;

/// JoinIR VM ブリッジ候補を判定し、マッチすれば JoinIR→VM を実行する。
///
/// # Arguments
/// - `module`: MIR モジュール
/// - `quiet_pipe`: 出力を抑制するかどうか
///
/// # Returns
/// - `true`: JoinIR 経路で実行完了（process::exit 呼び出し済み）
/// - `false`: JoinIR 経路は使わない（通常 VM にフォールバック）
///
/// # Phase 32 L-4: テーブル駆動ルーティング
///
/// `JOINIR_TARGETS` テーブルから対象関数を探し、`JoinIrBridgeKind` に応じて
/// Exec（実行）または LowerOnly（検証のみ）のパスに分岐する。
pub fn try_run_joinir_vm_bridge(module: &MirModule, quiet_pipe: bool) -> bool {
    let flags = JoinIrEnvFlags::from_env();
    let strict = crate::config::env::joinir_strict_enabled();

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

    // Phase 32 L-4: テーブル駆動ディスパッチ
    // 関数名でルーティング（将来は lowering テーブルベースに差し替え予定）
    let handled = match target.func_name {
        "Main.skip/1" => try_run_skip_ws(module, quiet_pipe),
        "FuncScannerBox.trim/1" => try_run_trim(module, quiet_pipe),
        "Stage1UsingResolverBox.resolve_for_source/5" => {
            try_run_stage1_usingresolver(module, quiet_pipe)
        }
        "StageBBodyExtractorBox.build_body_src/2" => try_run_stageb_body(module, quiet_pipe),
        "StageBFuncScannerBox.scan_all_boxes/1" => try_run_stageb_funcscanner(module, quiet_pipe),
        _ => false,
    };

    if !handled {
        // Phase 80/81: Strict mode では本線対象関数の失敗でパニック
        let should_panic =
            crate::mir::join_ir::lowering::should_panic_on_joinir_failure(target.func_name, true);
        if strict || should_panic {
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
