//! 引数検証ユーティリティ
//!
//! メソッド呼び出しの引数数を検証する共通処理を提供します。

use super::super::*;
use crate::mir::ValueId;

impl MirInterpreter {
    /// 引数が正確にN個であることを検証
    ///
    /// # Arguments
    /// * `method` - メソッド名（エラーメッセージ用）
    /// * `args` - 引数リスト
    /// * `expected` - 期待する引数数
    ///
    /// # Returns
    /// 引数数が期待値と一致する場合はOk(())、そうでない場合はエラー
    #[inline]
    pub(crate) fn validate_args_exact(
        &self,
        method: &str,
        args: &[ValueId],
        expected: usize,
    ) -> Result<(), VMError> {
        if args.len() != expected {
            return Err(VMError::InvalidInstruction(format!(
                "{} expects {} arg(s), got {}",
                method,
                expected,
                args.len()
            )));
        }
        Ok(())
    }

    /// 引数がmin～max個であることを検証
    ///
    /// # Arguments
    /// * `method` - メソッド名（エラーメッセージ用）
    /// * `args` - 引数リスト
    /// * `min` - 最小引数数
    /// * `max` - 最大引数数
    ///
    /// # Returns
    /// 引数数が範囲内の場合はOk(())、そうでない場合はエラー
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn validate_args_range(
        &self,
        method: &str,
        args: &[ValueId],
        min: usize,
        max: usize,
    ) -> Result<(), VMError> {
        let len = args.len();
        if len < min || len > max {
            return Err(VMError::InvalidInstruction(format!(
                "{} expects {}-{} arg(s), got {}",
                method, min, max, len
            )));
        }
        Ok(())
    }

    /// 引数が最低N個あることを検証
    ///
    /// # Arguments
    /// * `method` - メソッド名（エラーメッセージ用）
    /// * `args` - 引数リスト
    /// * `min` - 最小引数数
    ///
    /// # Returns
    /// 引数数が最小値以上の場合はOk(())、そうでない場合はエラー
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn validate_args_min(
        &self,
        method: &str,
        args: &[ValueId],
        min: usize,
    ) -> Result<(), VMError> {
        if args.len() < min {
            return Err(VMError::InvalidInstruction(format!(
                "{} expects at least {} arg(s), got {}",
                method,
                min,
                args.len()
            )));
        }
        Ok(())
    }
}
