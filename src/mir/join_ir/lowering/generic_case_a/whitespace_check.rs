//! Phase 192: Generic Case A - Whitespace Character Detection
//!
//! 責務: trim操作での空白文字判定ロジックの集約
//! - Space/Tab/Newline/CR の判定を統一
//! - skip_leading と loop_step で重複していた処理を統一

use crate::mir::ValueId;

/// Whitespace判定フラグ
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct WhitespaceCheckResult {
    /// Space (0x20)
    pub is_space: bool,
    /// Tab (0x09)
    pub is_tab: bool,
    /// Newline (0x0A)
    pub is_newline: bool,
    /// Carriage return (0x0D)
    pub is_carriage_return: bool,
}

impl WhitespaceCheckResult {
    /// いずれかの空白判定がtrueか確認
    #[allow(dead_code)]
    pub fn is_whitespace(&self) -> bool {
        self.is_space || self.is_tab || self.is_newline || self.is_carriage_return
    }

    /// 空白判定を実行
    #[allow(dead_code)]
    pub fn check(ch: char) -> Self {
        Self {
            is_space: ch == ' ',
            is_tab: ch == '\t',
            is_newline: ch == '\n',
            is_carriage_return: ch == '\r',
        }
    }

    /// 複数文字をチェック（shorthand）
    #[allow(dead_code)]
    pub fn check_byte(byte: u8) -> Self {
        Self::check(byte as char)
    }
}

/// Whitespace検出処理の共通ユーティリティ
#[allow(dead_code)]
pub struct WhitespaceDetector;

impl WhitespaceDetector {
    /// 指定されたValueIdが空白文字かどうかを判定する式を構築
    ///
    /// この関数は複数の条件（Space/Tab/Newline/CR）をORで繋いで
    /// 統一的な空白判定式を生成する
    ///
    /// # Note
    /// 具体的な JoinInst 生成は呼び出し側で行う。
    /// ここは判定ロジック（どの文字を空白と判定するか）を記録する。
    #[allow(dead_code)]
    pub fn build_whitespace_check_expr(ch_value: ValueId, _debug: bool) -> Option<ValueId> {
        // NOTE: JoinInst を生成する実装は呼び出し側で行う
        // ここは判定ロジック（どの文字を空白と判定するか）を記録

        // Space (0x20)
        let _space_check = ch_value;
        // Tab (0x09)
        let _tab_check = ch_value;
        // Newline (0x0A)
        let _newline_check = ch_value;
        // Carriage return (0x0D)
        let _cr_check = ch_value;

        // これらを OR で繋ぐ（具体的な JoinInst 生成は呼び出し側）
        Some(ch_value) // Placeholder
    }

    /// Whitespace判定に必要な文字リスト
    #[allow(dead_code)]
    pub fn whitespace_chars() -> &'static [u8] {
        b" \t\n\r"
    }

    /// Whitespace判定で使用される文字定数のリスト（JoinIR生成用）
    #[allow(dead_code)]
    pub fn whitespace_string_constants() -> Vec<&'static str> {
        vec![" ", "\\t", "\\n", "\\r"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace_check_space() {
        let result = WhitespaceCheckResult::check(' ');
        assert!(result.is_space);
        assert!(result.is_whitespace());
    }

    #[test]
    fn test_whitespace_check_tab() {
        let result = WhitespaceCheckResult::check('\t');
        assert!(result.is_tab);
        assert!(result.is_whitespace());
    }

    #[test]
    fn test_whitespace_check_newline() {
        let result = WhitespaceCheckResult::check('\n');
        assert!(result.is_newline);
        assert!(result.is_whitespace());
    }

    #[test]
    fn test_whitespace_check_carriage_return() {
        let result = WhitespaceCheckResult::check('\r');
        assert!(result.is_carriage_return);
        assert!(result.is_whitespace());
    }

    #[test]
    fn test_whitespace_check_non_whitespace() {
        let result = WhitespaceCheckResult::check('x');
        assert!(!result.is_whitespace());
    }

    #[test]
    fn test_whitespace_check_byte() {
        let result = WhitespaceCheckResult::check_byte(b' ');
        assert!(result.is_space);
        assert!(result.is_whitespace());
    }

    #[test]
    fn test_whitespace_detector_chars() {
        let chars = WhitespaceDetector::whitespace_chars();
        assert!(chars.contains(&b' '));
        assert!(chars.contains(&b'\t'));
        assert!(chars.contains(&b'\n'));
        assert!(chars.contains(&b'\r'));
        assert_eq!(chars.len(), 4);
    }

    #[test]
    fn test_whitespace_detector_constants() {
        let constants = WhitespaceDetector::whitespace_string_constants();
        assert_eq!(constants.len(), 4);
        assert!(constants.contains(&" "));
        assert!(constants.contains(&"\\t"));
        assert!(constants.contains(&"\\n"));
        assert!(constants.contains(&"\\r"));
    }
}
