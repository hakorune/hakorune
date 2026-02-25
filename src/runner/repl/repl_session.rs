//! ReplSessionBox - Session state for REPL mode
//!
//! Box-First Design: Encapsulates REPL-specific state
//! Phase 288 P2
//!
//! **重要**: ValueId は MIR コンパイル単位でのみ有効なので、
//! 実行時の値（VMValue）を保持する。

use crate::backend::VMValue;
use std::collections::BTreeMap;

/// REPL session context - isolated from file mode
/// Phase 288.1: Made public for ExternCall bridge access
#[derive(Debug, Default)]
pub struct ReplSessionBox {
    /// Session-level variables (runtime values, persists across evaluations)
    pub variables: BTreeMap<String, VMValue>,

    /// Last expression value (for `_` variable)
    pub last_value: Option<VMValue>,

    /// Evaluation counter
    pub eval_count: usize,
}

impl ReplSessionBox {
    pub(super) fn new() -> Self {
        Self::default()
    }

    /// REPL set: 変数に実行時の値を保存
    /// Phase 288.1: Made public for ExternCall bridge (__repl.set)
    pub fn set(&mut self, name: String, value: VMValue) {
        self.variables.insert(name, value);
    }

    /// REPL get: 変数の実行時の値を取得（未定義は None）
    /// Phase 288.1: Made public for ExternCall bridge (__repl.get)
    pub fn get(&self, name: &str) -> Option<&VMValue> {
        self.variables.get(name)
    }

    /// セッションに変数が存在するか確認
    #[allow(dead_code)]
    pub(super) fn has(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    pub(super) fn set_last_value(&mut self, value: VMValue) {
        self.last_value = Some(value.clone());
        self.variables.insert("_".to_string(), value);
    }

    pub(super) fn reset(&mut self) {
        self.variables.clear();
        self.last_value = None;
        self.eval_count = 0;
    }
}
