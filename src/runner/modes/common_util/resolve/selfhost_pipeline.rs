//! Selfhost Pipeline Box - 綺麗綺麗なセルフホストパイプライン専門家！📦
//!
//! セルフホストコンパイルの複雑な処理を箱に閉じ込めて、
//! 保守性とテスト容易性を向上させるにゃ！

use crate::runner::modes::common_util::resolve::prelude_manager::{
    MergeStrategy, PreludeManagerBox,
};
use crate::runner::NyashRunner;

/// 📦 SelfhostPipelineBox - セルフホストパイプラインの専門家！
///
/// コンパイラーパイプライン全体を管理する箱にゃ！
pub struct SelfhostPipelineBox<'a> {
    runner: &'a NyashRunner,
    prelude_manager: PreludeManagerBox<'a>,
}

/// 🎯 CompilationResult - コンパイル結果！
#[derive(Debug)]
pub struct CompilationResult {
    pub success: bool,
    pub final_code: String,
    pub merge_strategy: MergeStrategy,
    pub prelude_count: usize,
    pub processing_time_ms: u64,
}

/// ⚙️ PipelineConfig - パイプライン設定！
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub enable_using: bool,
    pub enable_ast_merge: bool,
    pub trace_execution: bool,
    pub debug_mode: bool,
}

impl<'a> SelfhostPipelineBox<'a> {
    /// 🌟 新しいSelfhostPipelineBoxを作るにゃ！
    pub fn new(runner: &'a NyashRunner) -> Self {
        let prelude_manager = PreludeManagerBox::new(runner);

        Self {
            runner,
            prelude_manager,
        }
    }

    /// 🚀 セルフホストパイプラインを実行するにゃ！
    pub fn execute_pipeline(
        &mut self,
        code: &str,
        filename: &str,
    ) -> Result<CompilationResult, String> {
        let start_time = std::time::Instant::now();
        let config = self.build_config();

        // usingが無効ならそのまま返す
        if !config.enable_using {
            return Ok(CompilationResult {
                success: true,
                final_code: code.to_string(),
                merge_strategy: MergeStrategy::Text, // デフォルト
                prelude_count: 0,
                processing_time_ms: start_time.elapsed().as_millis() as u64,
            });
        }

        // 第1フェーズ：using文解析とプレリュードパス収集
        let (cleaned_main, prelude_paths) = self.collect_and_resolve_using(code, filename)?;

        // 第2フェーズ：プレリュード統合
        let merge_result = if config.enable_ast_merge {
            self.prelude_manager
                .merge_ast(&cleaned_main, filename, &prelude_paths)?
        } else {
            self.prelude_manager
                .merge_text(&cleaned_main, filename, &prelude_paths)?
        };

        let processing_time = start_time.elapsed().as_millis() as u64;

        Ok(CompilationResult {
            success: true,
            final_code: merge_result.merged_content,
            merge_strategy: merge_result.strategy,
            prelude_count: merge_result.prelude_count,
            processing_time_ms: processing_time,
        })
    }

    /// 📋 パイプライン設定を構築するにゃ！
    fn build_config(&self) -> PipelineConfig {
        PipelineConfig {
            enable_using: crate::config::env::enable_using(),
            enable_ast_merge: crate::config::env::using_ast_enabled(),
            trace_execution: crate::config::env::resolve_trace(),
            debug_mode: crate::config::env::resolve_seam_debug(),
        }
    }

    /// 🔍 using文を収集して解決するにゃ！
    fn collect_and_resolve_using(
        &mut self,
        code: &str,
        filename: &str,
    ) -> Result<(String, Vec<String>), String> {
        // 既存のresolve_prelude_paths_profiledを使用
        crate::runner::modes::common_util::resolve::strip::resolve_prelude_paths_profiled(
            &self.runner,
            code,
            filename,
        )
    }

    /// 📊 パイプライン統計を表示するにゃ！
    pub fn print_pipeline_stats(&self, result: &CompilationResult) {
        let strategy_str = match result.merge_strategy {
            MergeStrategy::Text => "text",
            MergeStrategy::Ast => "ast",
        };

        crate::runtime::get_global_ring0().log.info(&format!(
            "[selfhost-pipeline] ✅ Completed in {}ms (strategy: {}, preludes: {})",
            result.processing_time_ms, strategy_str, result.prelude_count
        ));
    }

    /// 🚨 エラーハンドリングとフォールバックするにゃ！
    pub fn handle_fallback(
        &self,
        error: &str,
        original_code: &str,
        _filename: &str,
    ) -> CompilationResult {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .warn(&format!("[selfhost-pipeline] ⚠️ Error: {}", error));
        ring0
            .log
            .warn("[selfhost-pipeline] 🔄 Falling back to original code");

        CompilationResult {
            success: false,
            final_code: original_code.to_string(),
            merge_strategy: MergeStrategy::Text, // フォールバックはテキスト
            prelude_count: 0,
            processing_time_ms: 0,
        }
    }

    /// 🧪 パイプラインを検証するにゃ！（テスト用）
    pub fn validate_pipeline(&self, code: &str, filename: &str) -> Result<Vec<String>, String> {
        let mut issues = Vec::new();

        // usingシステムの検証
        if crate::config::env::enable_using() {
            // using文があるかチェック
            let using_count = code
                .lines()
                .filter(|line| line.trim().starts_with("using "))
                .count();

            if using_count > 0 {
                // プレリュード解決を試みる
                match crate::runner::modes::common_util::resolve::strip::resolve_prelude_paths_profiled(
                    &self.runner,
                    code,
                    filename,
                ) {
                    Ok((_, paths)) => {
                        if paths.is_empty() {
                            issues.push("using statements found but no preludes resolved".to_string());
                        }
                    }
                    Err(e) => {
                        issues.push(format!("using resolution failed: {}", e));
                    }
                }
            }
        }

        Ok(issues)
    }

    /// 📊 パフォーマンスプロファイリングするにゃ！
    pub fn profile_pipeline(&mut self, _code: &str, _filename: &str) -> Result<String, String> {
        // プロファイル機能を実装（別途）
        // TODO: プロファイル機能を追加
        Err("Profiling not yet implemented".to_string())
    }
}
