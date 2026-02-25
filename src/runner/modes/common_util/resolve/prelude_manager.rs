//! Prelude Manager Box - 綺麗綺麗なプレリュード統合専門家！📦
//!
//! テキストマージとASTマージを分離して、
//! 保守性とテスト容易性を向上させるにゃ！

use crate::runner::modes::common_util::resolve::using_resolution::UsingResolutionBox;
use crate::runner::NyashRunner;

/// 📦 PreludeManagerBox - プレリュード統合の専門家！
///
/// テキストベースとASTベースの両方の統合を
/// 統一インターフェースで提供する箱にゃ！
pub struct PreludeManagerBox<'a> {
    runner: &'a NyashRunner,
}

/// 🎯 MergeStrategy - 統合戦略！
#[derive(Debug, Clone)]
pub enum MergeStrategy {
    /// 🚀 テキストベース統合（高速）
    Text,
    /// 🧠 ASTベース統合（高機能）
    Ast,
}

/// 📊 MergeResult - 統合結果！
#[derive(Debug)]
pub struct MergeResult {
    pub merged_content: String,
    pub strategy: MergeStrategy,
    pub prelude_count: usize,
    pub total_bytes: usize,
}

impl<'a> PreludeManagerBox<'a> {
    /// 🌟 新しいPreludeManagerBoxを作るにゃ！
    pub fn new(runner: &'a NyashRunner) -> Self {
        Self { runner }
    }

    /// 🚀 テキストベース統合を実行するにゃ！
    pub fn merge_text(
        &self,
        source: &str,
        filename: &str,
        prelude_paths: &[String],
    ) -> Result<MergeResult, String> {
        let trace = crate::config::env::resolve_trace();

        if prelude_paths.is_empty() {
            return Ok(MergeResult {
                merged_content: source.to_string(),
                strategy: MergeStrategy::Text,
                prelude_count: 0,
                total_bytes: source.len(),
            });
        }

        if trace {
            crate::runner::trace::log(format!(
                "[prelude/text] {} prelude files for '{}'",
                prelude_paths.len(),
                filename
            ));
        }

        // テキスト統合ロジック
        let merged = self.build_text_merged(source, filename, prelude_paths, trace)?;
        let total_bytes = merged.len();

        Ok(MergeResult {
            merged_content: merged,
            strategy: MergeStrategy::Text,
            prelude_count: prelude_paths.len(),
            total_bytes,
        })
    }

    /// 🧠 ASTベース統合を実行するにゃ！
    pub fn merge_ast(
        &self,
        source: &str,
        filename: &str,
        prelude_paths: &[String],
    ) -> Result<MergeResult, String> {
        let trace = crate::config::env::resolve_trace();

        if prelude_paths.is_empty() {
            return Ok(MergeResult {
                merged_content: source.to_string(),
                strategy: MergeStrategy::Ast,
                prelude_count: 0,
                total_bytes: source.len(),
            });
        }

        if trace {
            crate::runner::trace::log(format!(
                "[prelude/ast] {} prelude files for '{}'",
                prelude_paths.len(),
                filename
            ));
        }

        // TODO: AST統合ロジックをここに実装
        // 今はテキスト統合にフォールバック
        self.merge_text(source, filename, prelude_paths)
    }

    /// 🏗️ テキスト統合を組み立てるにゃ！
    fn build_text_merged(
        &self,
        source: &str,
        filename: &str,
        prelude_paths: &[String],
        trace: bool,
    ) -> Result<String, String> {
        let mut merged = String::new();
        let mut spans: Vec<crate::runner::modes::common_util::resolve::LineSpan> = Vec::new();
        let mut current_line: usize = 1;

        // プレリュードをDFS順に追加
        for (idx, path) in prelude_paths.iter().enumerate() {
            let content = std::fs::read_to_string(path)
                .map_err(|e| format!("using: failed to read '{}': {}", path, e))?;

            // using行を除去して正規化
            let _using_resolver = UsingResolutionBox::new(&self.runner, path)?;
            let (cleaned_raw, _nested) = self.collect_using_and_strip_internal(&content, path)?;
            let cleaned = self.normalize_text_for_inline(&cleaned_raw);

            if trace {
                crate::runner::trace::log(format!(
                    "[prelude/text] [{}] '{}' ({} bytes)",
                    idx + 1,
                    path,
                    cleaned.len()
                ));
            }

            merged.push_str(&cleaned);
            merged.push('\n');
            let added = cleaned.lines().count();
            if added > 0 {
                spans.push(crate::runner::modes::common_util::resolve::LineSpan {
                    file: path.clone(),
                    start_line: current_line,
                    line_count: added,
                });
                current_line += added + 1; // +1 for the extra '\n'
            } else {
                current_line += 1;
            }
        }

        // デバッグモードなら境界マーカーを追加
        if crate::config::env::resolve_seam_debug() {
            merged.push_str("\n/* --- using prelude/main boundary --- */\n\n");
            // boundary line(s) are attributed to a synthetic "<boundary>" pseudo-file
            let boundary_lines = 3usize;
            spans.push(crate::runner::modes::common_util::resolve::LineSpan {
                file: "<prelude/main-boundary>".to_string(),
                start_line: current_line,
                line_count: boundary_lines,
            });
            current_line += boundary_lines;
        }

        // メインソースを正規化して追加
        let cleaned_main = self.normalize_text_for_inline(source);
        merged.push_str(&cleaned_main);
        let main_lines = cleaned_main.lines().count();
        if main_lines > 0 {
            spans.push(crate::runner::modes::common_util::resolve::LineSpan {
                file: filename.to_string(),
                start_line: current_line,
                line_count: main_lines,
            });
            current_line += main_lines;
        }
        let _ = current_line;

        if trace {
            crate::runner::trace::log(format!(
                "[prelude/text] final merged: {} bytes ({} prelude + {} main)",
                merged.len(),
                merged.len() - cleaned_main.len(),
                cleaned_main.len()
            ));
        }

        crate::runner::modes::common_util::resolve::set_last_text_merge_line_spans(spans);

        Ok(self.normalize_text_for_inline(&merged))
    }

    /// 🧹 using行を収集して除去するにゃ！（内部実装）
    fn collect_using_and_strip_internal(
        &self,
        code: &str,
        filename: &str,
    ) -> Result<(String, Vec<String>), String> {
        // 既存のcollect_using_and_strip関数を呼び出す
        // TODO: 将来的にはUsingResolutionBox経由に置き換える
        let (cleaned, prelude_paths, _imports) =
            crate::runner::modes::common_util::resolve::strip::collect_using_and_strip(
                &self.runner,
                code,
                filename,
            )?;
        Ok((cleaned, prelude_paths))
    }

    /// 🔧 テキストを正規化するにゃ！
    fn normalize_text_for_inline(&self, s: &str) -> String {
        let mut out = s.replace("\r\n", "\n").replace("\r", "\n");

        // `}` の前の `;` を除去（複数回パス）
        for _ in 0..2 {
            let bytes = out.as_bytes();
            let mut tmp: Vec<u8> = Vec::with_capacity(bytes.len());
            let mut i = 0usize;

            while i < bytes.len() {
                if bytes[i] == b';' {
                    // 先読みしてスペース/改行をスキップ
                    let mut j = i + 1;
                    while j < bytes.len() {
                        let c = bytes[j];
                        if c == b' ' || c == b'\t' || c == b'\n' {
                            j += 1;
                        } else {
                            break;
                        }
                    }
                    if j < bytes.len() && bytes[j] == b'}' {
                        // `;` をドロップ
                        i += 1;
                        continue;
                    }
                }
                tmp.push(bytes[i]);
                i += 1;
            }
            out = String::from_utf8(tmp).expect("normalize_text_for_inline: invalid UTF-8");
        }

        // ファイル末尾に改行を追加
        if !out.ends_with('\n') {
            out.push('\n');
        }

        out
    }

    /// 📊 最適な統合戦略を選択するにゃ！
    pub fn select_strategy(&self, prelude_count: usize) -> MergeStrategy {
        // 環境変数でAST統合が強制されている場合はASTを選択
        if crate::config::env::using_ast_enabled() {
            return MergeStrategy::Ast;
        }

        // プレリュード数が多い場合はテキスト統合を選択（高速）
        if prelude_count > 5 {
            return MergeStrategy::Text;
        }

        // デフォルトはテキスト統合
        MergeStrategy::Text
    }

    /// 🚀 自動戦略選択で統合を実行するにゃ！
    pub fn merge_auto(
        &self,
        source: &str,
        filename: &str,
        prelude_paths: &[String],
    ) -> Result<MergeResult, String> {
        let strategy = self.select_strategy(prelude_paths.len());

        match strategy {
            MergeStrategy::Text => self.merge_text(source, filename, prelude_paths),
            MergeStrategy::Ast => self.merge_ast(source, filename, prelude_paths),
        }
    }
}
