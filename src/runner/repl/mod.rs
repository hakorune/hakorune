//! REPL Module - Box-First Architecture
//!
//! Phase 288: Box化モジュール化
//! - ReplRunnerBox: REPL実行器の完全隔離
//! - ReplSessionBox: セッション状態の管理
//!
//! 公開API: run_repl() のみ

mod ast_rewriter;
mod repl_runner;
mod repl_session; // Phase 288.1: AST rewriting for session variable bridge

use crate::cli::CliConfig;
use repl_runner::ReplRunnerBox;
pub use repl_session::ReplSessionBox; // Phase 288.1: Export for ExternCall bridge

/// Phase 288: REPL モード起動（公開API）
///
/// REPL ループを開始し、プログラムは終了しない（never returns）。
/// `.exit` コマンドで終了する。
pub(crate) fn run_repl(config: CliConfig) -> ! {
    let runner = ReplRunnerBox::new(config);
    runner.run()
}
